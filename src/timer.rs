use crate::game::Floatable;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Doing math with [std::time::Duration](https://doc.rust-lang.org/std/time/struct.Duration.html)
/// is sort of a pain, so this wraps it up in an easy package.
/// Create a timer set how you like it, call `.update(delta)` with a delta duration each time around
/// the main loop, check `.ready` to see if the timer has gone off, and call `.reset()` to start
/// over.
///
/// You still have to keep track of time by getting an
/// [Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) and storing its `elapsed()`
/// duration to use during your game loop.  It looks like this:
///
/// ```
/// use std::time::{Duration, Instant};
/// use rusty_sword_arena::timer::Timer;
///
/// // Create some timers
/// let mut timer1 = Timer::from_millis(100);
/// let mut timer2 = Timer::from_millis(200);
///
/// // The current time on the clock
/// let mut instant = Instant::now();
///
/// // The "delta time", or time it took to make it around the loop last time
/// let mut dt = Duration::from_secs(0);
///
/// // Your game loop
/// loop {
///     // All your game logic, including pumping your timers like this...
///     timer1.update(dt);
///     timer2.update(dt);
///
///     // Get the delta time for the next loop iteration
///     dt = instant.elapsed();
///     instant = Instant::now();
/// #   break
/// }
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    time: Duration,
    time_left: Duration,
    /// True while the timer is at zero.  Once the timer reaches zero, it stays at zero until you
    /// call `.reset()` or `.set_millis_transient()`
    pub ready: bool,
}

impl Timer {
    /// Create a timer, initializing with a value in milliseconds.
    pub fn from_millis(ms: u64) -> Self {
        let duration = Duration::from_millis(ms);
        Self {
            time: duration,
            time_left: duration,
            ready: false,
        }
    }

    /// Create a timer, initializing with a value in nanoseconds.
    pub fn from_nanos(nanos: u64) -> Self {
        let duration = Duration::from_nanos(nanos);
        Self {
            time: duration,
            time_left: duration,
            ready: false,
        }
    }

    /// Resets the timer back to the starting time.  `ready` goes back to `false`
    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.time;
    }

    /// Just like `.reset()` but the timer is set to an arbitrary time.  Calling `.reset()` will
    /// still use the original starting time.
    pub fn set_millis_transient(&mut self, ms: u64) {
        self.ready = false;
        self.time_left = Duration::from_millis(ms);
    }

    /// IMPORTANT! You must call this method in your game loop!  This is how the timer counts-down.
    /// Every time you call this, the timer counts down the amount in `delta`.  If the timer reaches
    /// zero, `ready` becomes true and the timer stays at zero until `.reset()` or
    /// `.set_millis_transient()` is called.
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

    /// How much time is left as an f32 percentage from 0.0 to 1.0.  Very useful if your timer is
    /// being used for some sort of animation.
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
