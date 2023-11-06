use std::time::{Duration, Instant};
//use tokio::time;
use std::sync::{Arc, Mutex};
use log::{error, warn};

use super::clock_mode::ClockMode; 



#[derive(Debug)]
pub struct Clock {
    pub clock_mode: ClockMode,
//    tick_size: Duration,
//    start_time: Instant,
//    end_time: Option<Instant>,
//    current_tick: Instant,
//    child_iterators: Vec<Arc<Mutex<TimeIterator>>>,
//    started: bool,
}

impl Clock {
    pub fn clock_mode(&self) -> &ClockMode {
        &self.clock_mode
    }
}
