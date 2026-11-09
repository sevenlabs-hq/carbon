//! How hard the firehose pushes the archive it streams from.
//! [`Polite`](crate::jetstreamer_firehose::pace::Pace::Polite) is the default and shares the public Old Faithful
//! archive with other users. [`Turbo`](crate::jetstreamer_firehose::pace::Pace::Turbo) runs at full speed for
//! benchmarks and private mirrors.
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

/// Environment variable that selects [`Pace::Turbo`] when set to a truthy value.
pub const TURBO_ENV: &str = "JETSTREAMER_TURBO";

const UNSET: u8 = 0;
const POLITE: u8 = 1;
const TURBO: u8 = 2;

static PACE: AtomicU8 = AtomicU8::new(UNSET);

/// Request pacing profile for the firehose.
///
/// | Setting | Polite | Turbo |
/// |---------|--------|-------|
/// | [`Pace::request_spacing`] | 250 ms | 15 ms |
/// | [`Pace::recycle_interval`] | 2 min | 15 s |
/// | [`Pace::default_spawn_pending`] | 4 | 24 |
/// | [`Pace::retry_backoff_base`] .. [`Pace::retry_backoff_max`] | 2 s .. 120 s | 1 s .. 32 s |
///
/// `JETSTREAMER_SPAWN_PENDING` still overrides the default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pace {
    /// Paced reconnects, rare connection recycling and a slow restart backoff.
    #[default]
    Polite,
    /// Fast reconnects, connection recycling and a short restart backoff.
    Turbo,
}

impl Pace {
    /// Minimum time between two new archive connections (stream opens and seeks) in the process.
    pub const fn request_spacing(self) -> Duration {
        match self {
            Self::Polite => Duration::from_millis(250),
            Self::Turbo => Duration::from_millis(15),
        }
    }

    /// Time between connection-recycle sweeps. Each sweep reconnects the slowest threads.
    pub const fn recycle_interval(self) -> Duration {
        match self {
            Self::Polite => Duration::from_secs(120),
            Self::Turbo => Duration::from_secs(15),
        }
    }

    /// Default not-yet-green thread limit when `JETSTREAMER_SPAWN_PENDING` is unset.
    pub const fn default_spawn_pending(self) -> usize {
        match self {
            Self::Polite => 4,
            Self::Turbo => 24,
        }
    }

    /// Delay before the first restart of a failed firehose thread.
    pub const fn retry_backoff_base(self) -> Duration {
        match self {
            Self::Polite => Duration::from_secs(2),
            Self::Turbo => Duration::from_secs(1),
        }
    }

    /// Upper bound for the restart delay of a firehose thread.
    pub const fn retry_backoff_max(self) -> Duration {
        match self {
            Self::Polite => Duration::from_secs(120),
            Self::Turbo => Duration::from_secs(32),
        }
    }

    /// Lowercase name used in logs.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Turbo => "turbo",
        }
    }
}

/// Returns the pace set by [`set_pace`], else [`Pace::Turbo`] when `JETSTREAMER_TURBO` is truthy,
/// else [`Pace::Polite`].
pub fn pace() -> Pace {
    match PACE.load(Ordering::Relaxed) {
        UNSET => {
            let from_env = encode(pace_from_env());
            // Keep a value that `set_pace` stored after the load above.
            match PACE.compare_exchange(UNSET, from_env, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => decode(from_env),
                Err(current) => decode(current),
            }
        }
        current => decode(current),
    }
}

/// Sets the pace for the whole process. Call it before the firehose starts.
pub fn set_pace(pace: Pace) {
    PACE.store(encode(pace), Ordering::Relaxed);
}

fn pace_from_env() -> Pace {
    let raw = std::env::var(TURBO_ENV).unwrap_or_default();
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Pace::Turbo,
        _ => Pace::Polite,
    }
}

const fn encode(pace: Pace) -> u8 {
    match pace {
        Pace::Polite => POLITE,
        Pace::Turbo => TURBO,
    }
}

const fn decode(raw: u8) -> Pace {
    if raw == TURBO {
        Pace::Turbo
    } else {
        Pace::Polite
    }
}
