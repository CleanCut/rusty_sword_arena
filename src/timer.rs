use std::time::Duration;
use super::Floatable;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    time : Duration,
    time_left : Duration,
    pub ready : bool,
}

impl Timer {
    pub fn from_millis(ms : u64) -> Self {
        let duration = Duration::from_millis(ms);
        Self {
            time : duration,
            time_left : duration,
            ready : false,
        }
    }
    pub fn from_nanos(nanos : u64) -> Self {
        let duration = Duration::from_nanos(nanos);
        Self {
            time : duration,
            time_left : duration,
            ready : false,
        }
    }
    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.time;
    }

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
    fn f32(&self) -> f32 {
        self.time_left.as_secs() as f32 + self.time_left.subsec_nanos() as f32 * 1e-9
    }
}