use std::time::Duration;
use super::Floatable;

/// Doing math with std::time::Duration is sort of a pain, so this wraps it up in an easy package.
/// Create a timer set how you like it, call `.update(delta)` with a delta duration each time around
/// the main loop, check `.ready` to see if the timer has gone off, and call `.reset()` to start
/// over.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    time : Duration,
    time_left : Duration,
    /// True if the timer has gone off.  Timer counts down to zero.
    pub ready : bool,
}

impl Timer {
    /// Set the timer based on milliseconds
    pub fn from_millis(ms : u64) -> Self {
        let duration = Duration::from_millis(ms);
        Self {
            time : duration,
            time_left : duration,
            ready : false,
        }
    }

    /// Set the timer based on nanoseconds.
    pub fn from_nanos(nanos : u64) -> Self {
        let duration = Duration::from_nanos(nanos);
        Self {
            time : duration,
            time_left : duration,
            ready : false,
        }
    }

    /// Resets the timer as if it were newly created.
    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.time;
    }

    /// This is how the timer counts-down. You have got to call this repeatedly as time passes.
    pub fn update(&mut self, delta : Duration) {
        if self.ready {
            return;
        }
        if let Some(result) = self.time_left.checked_sub(delta) {
            self.time_left = result;
        } else {
            self.ready = true;
        }
    }
}

impl Floatable for Timer {
    /// The time left on the timer as an f32
    fn f32(&self) -> f32 {
        self.time_left.as_secs() as f32 + self.time_left.subsec_nanos() as f32 * 1e-9
    }
}