use super::game::Floatable;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Doing math with [std::time::Duration](https://doc.rust-lang.org/std/time/struct.Duration.html)
/// is sort of a pain, so this wraps it up in an easy package.
/// Create a timer set how you like it, call `.update(delta)` with a delta duration each time around
/// the main loop, check `.ready` to see if the timer has gone off, and call `.reset()` to start
/// over.
///
/// Of course, that means that you still have to keep track of time by getting an
/// [Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) at the start of every loop,
/// and then getting it's `elapsed()` duration before resetting it the next time around the loop.
///
/// If you are only tracking one single thing, the `Instant` and its `elapsed()` are all you need.
/// If you're timing more than one thing, then use the `Instant` and its `elapsed()` for your delta
/// duration, and use it to pump all your timers.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    time: Duration,
    time_left: Duration,
    /// True if the timer has gone off.  Timer counts down to zero.
    pub ready: bool,
}

impl Timer {
    /// Set the timer based on milliseconds
    pub fn from_millis(ms: u64) -> Self {
        let duration = Duration::from_millis(ms);
        Self {
            time: duration,
            time_left: duration,
            ready: false,
        }
    }

    /// Set the timer based on nanoseconds.
    pub fn from_nanos(nanos: u64) -> Self {
        let duration = Duration::from_nanos(nanos);
        Self {
            time: duration,
            time_left: duration,
            ready: false,
        }
    }

    /// Resets the timer as if it were newly created.
    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.time;
    }

    /// Sets the timer to a value, but will still reset back to the initial value
    pub fn set_millis_transient(&mut self, ms: u64) {
        self.ready = false;
        self.time_left = Duration::from_millis(ms);
    }

    /// This is how the timer counts-down. You have got to call this repeatedly as time passes.
    pub fn update(&mut self, delta: Duration) {
        if self.ready {
            return;
        }
        if let Some(result) = self.time_left.checked_sub(delta) {
            self.time_left = result;
        } else {
            self.ready = true;
        }
    }

    /// How much time is left as an f32 percentage from 0.0 to 1.0
    pub fn time_left_percent(&self) -> f32 {
        if self.ready {
            1.0
        } else {
            self.time_left.as_millis() as f32 / self.time.as_millis() as f32
        }
    }
}

impl Floatable for Timer {
    /// The time left on the timer as an f32
    fn f32(&self) -> f32 {
        self.time_left.as_secs() as f32 + self.time_left.subsec_nanos() as f32 * 1e-9
    }
}
