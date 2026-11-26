use {
    agave_snapshots::paths::parse_full_snapshot_archive_filename,
    carbon_core::error::{CarbonResult, Error},
    percent_encoding::percent_decode_str,
    std::{
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    },
    tokio_util::sync::CancellationToken,
};

const MAX_RETRIES: u32 = 3;
const MAX_REDIRECTS: usize = 10;
const PROGRESS_LOG_INTERVAL: u64 = 50 * 1024 * 1024;
const PROGRESS_LOG_TIME_INTERVAL: u64 = 5;

pub async fn download_and_setup_ledger(
    base_url: &str,
    cancellation_token: CancellationToken,
) -> CarbonResult<PathBuf> {
    let temp_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("ledger");
    download_ledger(base_url, temp_dir, cancellation_token).await
}

async fn download_ledger(
    base_url: &str,
    temp_dir: PathBuf,
    cancellation_token: CancellationToken,
) -> CarbonResult<PathBuf> {
    std::fs::create_dir_all(&temp_dir).map_err(|e| {
        Error::FailedToConsumeDatasource(format!("Failed to create temp directory: {e}"))
    })?;

    let snapshots_dir = temp_dir.join("snapshots");
    std::fs::create_dir_all(&snapshots_dir).map_err(|e| {
        Error::FailedToConsumeDatasource(format!("Failed to create snapshots directory: {e}"))
    })?;

    log::info!("Downloading genesis and snapshot from: {base_url}");

    let genesis_url = format!("{}/genesis.tar.bz2", base_url.trim_end_matches('/'));
    let genesis_path = temp_dir.join("genesis.tar.bz2");
    download_file_with_retry(
        &genesis_url,
        &genesis_path,
        false,
        cancellation_token.clone(),
    )
    .await?;

    let snapshot_url = format!("{}/snapshot.tar.bz2", base_url.trim_end_matches('/'));
    let snapshot_path = snapshots_dir.join("snapshot.tar.bz2");
    download_file_with_retry(
        &snapshot_url,
        &snapshot_path,
        true,
        cancellation_token.clone(),
    )
    .await?;

    log::info!("Ledger structure created at: {temp_dir:?}");

    Ok(temp_dir)
}

async fn download_file_with_retry(
    url: &str,
    output_path: &Path,
    keep_archive_name: bool,
    cancellation_token: CancellationToken,
) -> CarbonResult<()> {
    let mut retries = 0;

    loop {
        if cancellation_token.is_cancelled() {
            return Err(Error::FailedToConsumeDatasource(
                "Download cancelled".to_string(),
            ));
        }

        match download_file(url, output_path, keep_archive_name, &cancellation_token).await {
            Ok(()) => {
                return Ok(());
            }
            Err(e) if cancellation_token.is_cancelled() => return Err(e),
            Err(e) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    return Err(Error::FailedToConsumeDatasource(format!(
                        "Failed to download {url} after {MAX_RETRIES} retries: {e}"
                    )));
                }

                log::warn!(
                    "Download failed (attempt {retries}/{MAX_RETRIES}), retrying in 2s: {e:?}"
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}

async fn download_file(
    url: &str,
    output_path: &Path,
    keep_archive_name: bool,
    cancellation_token: &CancellationToken,
) -> CarbonResult<()> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(MAX_REDIRECTS))
        .build()
        .map_err(|e| {
            Error::FailedToConsumeDatasource(format!("Failed to create HTTP client: {e}"))
        })?;

    let mut response = tokio::select! {
        response = client.get(url).send() => response
            .map_err(|e| Error::FailedToConsumeDatasource(format!("HTTP request failed: {e}")))?,
        _ = cancellation_token.cancelled() => {
            return Err(Error::FailedToConsumeDatasource(
                "Download cancelled".to_string(),
            ));
        }
    };

    if !response.status().is_success() {
        return Err(Error::FailedToConsumeDatasource(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let final_path = response
        .headers()
        .get("content-disposition")
        .and_then(|h| h.to_str().ok())
        .and_then(extract_filename_from_content_disposition)
        .or_else(|| {
            response
                .url()
                .path_segments()
                .and_then(|mut segments| segments.next_back())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .filter(|_| keep_archive_name)
        .and_then(|filename| {
            let filename = Path::new(&filename).file_name()?.to_str()?;
            parse_full_snapshot_archive_filename(filename).ok()?;
            Some(filename.to_string())
        })
        .map(|filename| output_path.parent().unwrap_or(output_path).join(filename))
        .unwrap_or_else(|| output_path.to_path_buf());

    let total_size = response.content_length().unwrap_or(0);
    let mut file = File::create(&final_path)
        .map_err(|e| Error::FailedToConsumeDatasource(format!("Failed to create file: {e}")))?;

    log::info!("Downloading from {url} (size: {total_size} bytes)");

    let mut written = 0u64;
    let mut last_logged = 0u64;
    let mut last_log_time = std::time::Instant::now();
    let start_time = std::time::Instant::now();

    loop {
        let chunk = tokio::select! {
            chunk = response.chunk() => chunk,
            _ = cancellation_token.cancelled() => {
                return Err(Error::FailedToConsumeDatasource(
                    "Download cancelled".to_string(),
                ));
            }
        };

        match chunk {
            Ok(Some(chunk)) => {
                file.write_all(&chunk).map_err(|e| {
                    Error::FailedToConsumeDatasource(format!("Failed to write to file: {e}"))
                })?;

                written += chunk.len() as u64;

                let time_since_last_log = last_log_time.elapsed().as_secs();
                let should_log = (written - last_logged >= PROGRESS_LOG_INTERVAL)
                    && (time_since_last_log >= PROGRESS_LOG_TIME_INTERVAL);

                if should_log {
                    let progress = if total_size > 0 {
                        (written as f64 / total_size as f64) * 100.0
                    } else {
                        0.0
                    };
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed_mbps = if elapsed > 0.0 {
                        (written as f64 / 1024.0 / 1024.0) / elapsed
                    } else {
                        0.0
                    };

                    let written_mb = written as f64 / 1024.0 / 1024.0;
                    let total_mb = total_size as f64 / 1024.0 / 1024.0;

                    let remaining_bytes = total_size.saturating_sub(written);
                    let eta_seconds = if speed_mbps > 0.0 && total_size > 0 {
                        (remaining_bytes as f64 / 1024.0 / 1024.0) / speed_mbps
                    } else {
                        0.0
                    };
                    let eta_minutes = (eta_seconds / 60.0) as u64;
                    let eta_secs = (eta_seconds % 60.0) as u64;

                    let elapsed_minutes = (elapsed / 60.0) as u64;
                    let elapsed_secs = (elapsed % 60.0) as u64;

                    let (written_size, total_size_display, unit) = if total_mb >= 1024.0 {
                        (written_mb / 1024.0, total_mb / 1024.0, "GB")
                    } else {
                        (written_mb, total_mb, "MB")
                    };

                    log::info!(
                        "Download: {progress:.2}% | {written_size:.2} {unit} / {total_size_display:.2} {unit} | Speed: {speed_mbps:.2} MB/s | Elapsed: {elapsed_minutes}m {elapsed_secs}s | ETA: {eta_minutes}m {eta_secs}s"
                    );
                    last_logged = written;
                    last_log_time = std::time::Instant::now();
                }
            }
            Ok(None) => break, // End of stream
            Err(e) => {
                return Err(Error::FailedToConsumeDatasource(format!(
                    "Download error: {e}"
                )));
            }
        }
    }

    let total_elapsed = start_time.elapsed().as_secs_f64();
    let avg_speed_mbps = if total_elapsed > 0.0 {
        (written as f64 / 1024.0 / 1024.0) / total_elapsed
    } else {
        0.0
    };
    let total_mb = written as f64 / 1024.0 / 1024.0;
    let elapsed_minutes = (total_elapsed / 60.0) as u64;
    let elapsed_secs = (total_elapsed % 60.0) as u64;

    let (size, unit) = if total_mb >= 1024.0 {
        (total_mb / 1024.0, "GB")
    } else {
        (total_mb, "MB")
    };

    log::info!(
        "Download complete: {size:.2} {unit} downloaded in {elapsed_minutes}m {elapsed_secs}s (avg speed: {avg_speed_mbps:.2} MB/s) to {final_path:?}"
    );

    Ok(())
}

fn extract_filename_from_content_disposition(header_value: &str) -> Option<String> {
    for part in header_value.split(';') {
        let part = part.trim();

        if let Some(encoded) = part.strip_prefix("filename*=") {
            if let Some(filename) = encoded.split('\'').nth(2) {
                if let Ok(filename) = percent_decode_str(filename).decode_utf8() {
                    return Some(filename.into_owned());
                }
            }
        }

        if let Some(filename) = part.strip_prefix("filename=") {
            let filename = filename.trim().trim_matches('"').trim_matches('\'');
            if !filename.is_empty() {
                return Some(filename.to_string());
            }
        }
    }

    None
}
