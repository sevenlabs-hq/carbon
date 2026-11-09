//! Archive backend configuration that allows the firehose to fetch CARs and indexes
//! from either HTTP mirrors or authenticated S3-compatible storage.

use once_cell::sync::Lazy;
use reqwest::Url;
use std::env;
use url::ParseError;

#[cfg(feature = "s3-backend")]
use std::sync::Arc;

#[cfg(feature = "s3-backend")]
use {
    aws_credential_types::{Credentials, provider::SharedCredentialsProvider},
    aws_sdk_s3::Client as S3Client,
    aws_sdk_s3::config::{BehaviorVersion, Builder as S3ConfigBuilder},
    aws_types::region::Region,
};

use crate::jetstreamer_firehose::epochs::BASE_URL;

/// Lazily resolved CAR location.
static CAR_LOCATION: Lazy<Location> = Lazy::new(|| {
    resolve_location(LocationKind::Car).unwrap_or_else(|err| {
        panic!("failed to resolve CAR archive location: {err}");
    })
});

/// Lazily resolved index location.
static INDEX_LOCATION: Lazy<Location> = Lazy::new(|| {
    resolve_location(LocationKind::Index).unwrap_or_else(|err| {
        panic!("failed to resolve index archive location: {err}");
    })
});

/// Returns the configured CAR archive location (HTTP or S3).
pub fn car_location() -> &'static Location {
    &CAR_LOCATION
}

/// Returns the configured compact index location (HTTP or S3).
pub fn index_location() -> &'static Location {
    &INDEX_LOCATION
}

/// Location of a remote archive.
#[derive(Debug)]
pub struct Location {
    url: Url,
    kind: LocationBackend,
}

impl Location {
    /// Creates a new HTTP location.
    pub(crate) fn http(url: Url) -> Self {
        Self {
            url,
            kind: LocationBackend::Http,
        }
    }

    #[cfg(feature = "s3-backend")]
    fn s3(url: Url, cfg: S3Location) -> Self {
        Self {
            url,
            kind: LocationBackend::S3(Arc::new(cfg)),
        }
    }

    /// Returns the display URL for this location.
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Indicates whether this location uses the HTTP backend.
    pub const fn is_http(&self) -> bool {
        matches!(self.kind, LocationBackend::Http)
    }

    /// Returns the S3-backed configuration if available.
    #[cfg(feature = "s3-backend")]
    pub fn as_s3(&self) -> Option<Arc<S3Location>> {
        match &self.kind {
            LocationBackend::Http => None,
            LocationBackend::S3(cfg) => Some(Arc::clone(cfg)),
        }
    }
}

#[derive(Debug)]
enum LocationBackend {
    Http,
    #[cfg(feature = "s3-backend")]
    S3(Arc<S3Location>),
}

#[derive(Clone, Copy, Debug)]
enum LocationKind {
    Car,
    Index,
}

fn resolve_location(kind: LocationKind) -> Result<Location, LocationError> {
    let backend_hint =
        env::var("JETSTREAMER_ARCHIVE_BACKEND").unwrap_or_else(|_| "http".to_owned());
    let raw = match kind {
        LocationKind::Car => env::var("JETSTREAMER_HTTP_BASE_URL").unwrap_or_else(|_| {
            env::var("JETSTREAMER_ARCHIVE_BASE").unwrap_or_else(|_| BASE_URL.to_string())
        }),
        LocationKind::Index => env::var("JETSTREAMER_COMPACT_INDEX_BASE_URL")
            .or_else(|_| env::var("JETSTREAMER_ARCHIVE_BASE"))
            .unwrap_or_else(|_| car_location().url().as_str().to_owned()),
    };

    if raw.starts_with("s3://") {
        #[cfg(feature = "s3-backend")]
        {
            let cfg = build_s3_location(kind, Some(raw.as_str()))?;
            return Ok(Location::s3(cfg.display_url.clone(), cfg));
        }
        #[cfg(not(feature = "s3-backend"))]
        {
            return Err(LocationError::S3FeatureDisabled);
        }
    }

    if backend_hint.eq_ignore_ascii_case("s3") {
        #[cfg(feature = "s3-backend")]
        {
            let cfg = build_s3_location(kind, None)?;
            return Ok(Location::s3(cfg.display_url.clone(), cfg));
        }
        #[cfg(not(feature = "s3-backend"))]
        {
            return Err(LocationError::S3FeatureDisabled);
        }
    }

    let url = Url::parse(&raw).map_err(|err| LocationError::InvalidUrl(raw.clone(), err))?;
    Ok(Location::http(url))
}

/// Errors surfaced while constructing archive locations.
#[derive(Debug, thiserror::Error)]
pub enum LocationError {
    /// Archive URL failed to parse.
    #[error("invalid URL {0}: {1}")]
    InvalidUrl(String, #[source] ParseError),
    #[cfg(feature = "s3-backend")]
    /// Missing required S3 bucket configuration.
    #[error("missing S3 bucket configuration for {0} data")]
    MissingBucket(&'static str),
    #[cfg(feature = "s3-backend")]
    /// Missing S3 credentials.
    #[error("missing S3 credentials ({0})")]
    MissingCredentials(&'static str),
    #[cfg(feature = "s3-backend")]
    /// AWS SDK could not be initialized.
    #[error("failed to initialize S3 client: {0}")]
    ClientInit(String),
    /// S3 backend requested but crate built without support.
    #[error("S3 backend requested but the crate was compiled without the `s3-backend` feature")]
    S3FeatureDisabled,
}

#[cfg(feature = "s3-backend")]
#[derive(Debug)]
/// Fully resolved S3 location (client, bucket, and prefix).
pub struct S3Location {
    /// Shared AWS SDK client.
    pub client: Arc<S3Client>,
    /// Target bucket name.
    pub bucket: Arc<str>,
    /// Optional key prefix.
    pub prefix: Arc<str>,
    /// Display URL (e.g. `s3://bucket/prefix/`).
    pub display_url: Url,
}

#[cfg(feature = "s3-backend")]
impl S3Location {
    /// Generates the fully-qualified object key for a relative path.
    pub fn key_for(&self, path: &str) -> String {
        let trimmed = path.trim_start_matches('/');
        if self.prefix.is_empty() {
            return trimmed.to_string();
        }
        format!("{}{}", self.prefix, trimmed)
    }
}

#[cfg(feature = "s3-backend")]
fn build_s3_location(kind: LocationKind, raw: Option<&str>) -> Result<S3Location, LocationError> {
    let (bucket, prefix) = if let Some(uri) = raw {
        parse_s3_uri(uri)?
    } else {
        load_bucket_from_env(kind)?
    };

    let region = env::var("JETSTREAMER_S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let endpoint = env::var("JETSTREAMER_S3_ENDPOINT").ok();
    let client = build_s3_client(region, endpoint)?;

    let display = Url::parse(&format!(
        "s3://{}/{}",
        bucket,
        if prefix.is_empty() {
            "".to_string()
        } else {
            prefix.clone()
        }
    ))
    .map_err(|err| LocationError::InvalidUrl(format!("s3://{bucket}/{prefix}"), err))?;

    Ok(S3Location {
        client: Arc::new(client),
        bucket: bucket.into(),
        prefix: prefix.into(),
        display_url: display,
    })
}

#[cfg(feature = "s3-backend")]
fn parse_s3_uri(uri: &str) -> Result<(String, String), LocationError> {
    let trimmed = uri.trim_start_matches("s3://");
    if trimmed.is_empty() {
        return Err(LocationError::MissingBucket("archive"));
    }
    let mut parts = trimmed.splitn(2, '/');
    let bucket = parts
        .next()
        .ok_or(LocationError::MissingBucket("archive"))?
        .to_string();
    let prefix = parts
        .next()
        .map(|rest| rest.trim_matches('/').to_string())
        .unwrap_or_default();
    Ok((bucket, normalize_prefix(&prefix)))
}

#[cfg(feature = "s3-backend")]
fn load_bucket_from_env(kind: LocationKind) -> Result<(String, String), LocationError> {
    let (bucket_var, prefix_var, label) = match kind {
        LocationKind::Car => (
            "JETSTREAMER_S3_BUCKET",
            "JETSTREAMER_S3_PREFIX",
            "CAR archive",
        ),
        LocationKind::Index => (
            "JETSTREAMER_S3_BUCKET",
            "JETSTREAMER_S3_INDEX_PREFIX",
            "compact index",
        ),
    };

    let bucket = env::var(bucket_var).map_err(|_| LocationError::MissingBucket(label))?;
    let prefix = env::var(prefix_var).unwrap_or_default();
    Ok((bucket, normalize_prefix(&prefix)))
}

#[cfg(feature = "s3-backend")]
fn normalize_prefix(prefix: &str) -> String {
    let trimmed = prefix.trim_matches('/');
    if trimmed.is_empty() {
        return String::new();
    }
    format!("{trimmed}/")
}

#[cfg(feature = "s3-backend")]
fn build_s3_client(region: String, endpoint: Option<String>) -> Result<S3Client, LocationError> {
    let access_key = env::var("JETSTREAMER_S3_ACCESS_KEY")
        .or_else(|_| env::var("AWS_ACCESS_KEY_ID"))
        .map_err(|_| LocationError::MissingCredentials("access key"))?;
    let secret_key = env::var("JETSTREAMER_S3_SECRET_KEY")
        .or_else(|_| env::var("AWS_SECRET_ACCESS_KEY"))
        .map_err(|_| LocationError::MissingCredentials("secret key"))?;
    let session_token = env::var("JETSTREAMER_S3_SESSION_TOKEN")
        .or_else(|_| env::var("AWS_SESSION_TOKEN"))
        .ok();

    let creds = Credentials::new(access_key, secret_key, session_token, None, "jetstreamer");
    let region = Region::new(region);
    let provider = SharedCredentialsProvider::new(creds);

    let mut builder = S3ConfigBuilder::new()
        .region(region)
        .credentials_provider(provider);

    if let Some(endpoint) = endpoint {
        builder = builder.endpoint_url(endpoint);
    }

    let config = builder.behavior_version(BehaviorVersion::latest()).build();
    Ok(S3Client::from_conf(config))
}
