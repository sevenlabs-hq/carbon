//! System capability helpers for sizing the firehose runtime.
use std::cmp;
/// Environment variable that overrides detected network throughput in megabytes.
const NETWORK_CAPACITY_OVERRIDE_ENV: &str = "JETSTREAMER_NETWORK_CAPACITY_MB";
const DEFAULT_NETWORK_CAPACITY_MB: u64 = 1_000;
const DEFAULT_BUFFER_WINDOW_FALLBACK_BYTES: u64 = 512 * 1024 * 1024;
const DEFAULT_BUFFER_WINDOW_PERCENT_NUMERATOR: u64 = 15;
const DEFAULT_BUFFER_WINDOW_PERCENT_DENOMINATOR: u64 = 100;
const DEFAULT_BUFFER_WINDOW_MAX_BYTES: u64 = 4 * 1024 * 1024 * 1024;

/// Calculates an optimal number of firehose threads for the current machine.
///
/// The heuristic picks whichever constraint is tighter between CPU availability
/// and network capacity using:
///
/// `min(num_cpu_cores * 4, network_interface_bandwidth_capacity_megabytes / 285)`
///
/// The returned thread count is always in the inclusive range
/// `[1, num_cpu_cores * 4]`. The network capacity defaults to an assumed
/// 1,000 MB/s link unless overridden via the
/// `JETSTREAMER_NETWORK_CAPACITY_MB` environment variable.
#[inline]
pub fn optimal_firehose_thread_count() -> usize {
    compute_optimal_thread_count(detect_cpu_core_count(), detect_network_capacity_megabytes())
}

/// Returns the default ripget sequential download window size in bytes.
///
/// The default is the lower of 15% of detected available RAM and 4 GiB, falling back to 512 MiB
/// when RAM cannot be detected.
#[inline]
pub fn default_firehose_buffer_window_bytes() -> u64 {
    default_buffer_window_bytes()
}

/// Parses a human-readable byte size (for example `4GiB`, `512mb`, or `1073741824`).
///
/// Returns `None` when the input is invalid or smaller than two bytes.
#[inline]
pub fn parse_buffer_window_bytes(value: &str) -> Option<u64> {
    parse_byte_size(value).filter(|parsed| *parsed >= 2)
}

#[inline(always)]
fn detect_cpu_core_count() -> usize {
    std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
}

#[inline(always)]
fn detect_network_capacity_megabytes() -> Option<u64> {
    network_capacity_override().or(Some(DEFAULT_NETWORK_CAPACITY_MB))
}

fn network_capacity_override() -> Option<u64> {
    std::env::var(NETWORK_CAPACITY_OVERRIDE_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn default_buffer_window_bytes() -> u64 {
    let computed = detect_available_memory_bytes()
        .map(compute_default_buffer_window_bytes)
        .unwrap_or(DEFAULT_BUFFER_WINDOW_FALLBACK_BYTES);
    computed.max(2)
}

#[inline(always)]
fn compute_default_buffer_window_bytes(available_memory_bytes: u64) -> u64 {
    let window = (available_memory_bytes as u128)
        .saturating_mul(DEFAULT_BUFFER_WINDOW_PERCENT_NUMERATOR as u128)
        / (DEFAULT_BUFFER_WINDOW_PERCENT_DENOMINATOR as u128);
    window.min(DEFAULT_BUFFER_WINDOW_MAX_BYTES as u128) as u64
}

/// Formats a byte count as a human-readable string (e.g. `1.5 GiB`).
pub fn format_byte_size(bytes: u64) -> String {
    const GIB: u64 = 1_073_741_824;
    const MIB: u64 = 1_048_576;
    const KIB: u64 = 1_024;
    if bytes >= GIB && bytes.is_multiple_of(GIB) {
        format!("{} GiB", bytes / GIB)
    } else if bytes >= MIB && bytes.is_multiple_of(MIB) {
        format!("{} MiB", bytes / MIB)
    } else if bytes >= KIB && bytes.is_multiple_of(KIB) {
        format!("{} KiB", bytes / KIB)
    } else if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn parse_byte_size(value: &str) -> Option<u64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let split_idx = trimmed
        .char_indices()
        .find_map(|(idx, ch)| {
            if ch.is_ascii_digit() || ch == '_' {
                None
            } else {
                Some(idx)
            }
        })
        .unwrap_or(trimmed.len());
    let (number_part, suffix_part) = trimmed.split_at(split_idx);
    let number: u64 = number_part.replace('_', "").parse().ok()?;
    let suffix = suffix_part.trim().to_ascii_lowercase();

    let multiplier = match suffix.as_str() {
        "" | "b" => 1u64,
        "k" | "kb" => 1_000u64,
        "m" | "mb" => 1_000_000u64,
        "g" | "gb" => 1_000_000_000u64,
        "ki" | "kib" => 1_024u64,
        "mi" | "mib" => 1_048_576u64,
        "gi" | "gib" => 1_073_741_824u64,
        _ => return None,
    };
    number.checked_mul(multiplier)
}

#[cfg(target_os = "linux")]
fn detect_available_memory_bytes() -> Option<u64> {
    let mut info = std::mem::MaybeUninit::<libc::sysinfo>::uninit();
    let rc = unsafe { libc::sysinfo(info.as_mut_ptr()) };
    if rc != 0 {
        return None;
    }
    let info = unsafe { info.assume_init() };
    let freeram = u64::try_from(info.freeram).ok()?;
    let mem_unit = u64::from(info.mem_unit.max(1));
    freeram.checked_mul(mem_unit)
}

#[cfg(all(unix, not(target_os = "linux")))]
fn detect_available_memory_bytes() -> Option<u64> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size <= 0 {
        return None;
    }
    // `_SC_AVPHYS_PAGES` is not available on all Unix targets (including macOS), so use
    // physical pages as a portable fallback for default sizing.
    let pages = unsafe { libc::sysconf(libc::_SC_PHYS_PAGES) };
    if pages <= 0 {
        return None;
    }
    let bytes = (pages as u128).saturating_mul(page_size as u128);
    Some(bytes.min(u64::MAX as u128) as u64)
}

#[cfg(not(unix))]
fn detect_available_memory_bytes() -> Option<u64> {
    None
}

#[inline(always)]
fn compute_optimal_thread_count(
    cpu_cores: usize,
    network_capacity_megabytes: Option<u64>,
) -> usize {
    let cpu_limited = cmp::max(1, cpu_cores.saturating_mul(4));

    if let Some(capacity) = network_capacity_megabytes.filter(|value| *value > 0) {
        let network_limited = cmp::max(1u64, capacity / 250);
        cmp::min(cpu_limited as u64, network_limited)
            .max(1u64)
            .min(usize::MAX as u64) as usize
    } else {
        cpu_limited
    }
}
