use std::{time::Duration};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use uuid::Uuid;
use rand::prelude::*;

pub enum ProcessErrors {
    UnknownError,
}

#[derive(Debug)]
pub enum ProcessStatus {
    New,
    Ready,
    Running,
    Waiting,
    Terminated
}

#[derive(Debug)]
pub struct Process {
    pub id: Uuid,
    pub arrival_time: Duration,
    pub priority: i32,
    pub cpu_burst_time: Duration,
    pub status: ProcessStatus,
    pub processed_time: Duration,
}
impl Process {
    pub fn run_with_interrupt(&mut self, interrupt_sings: Arc<AtomicBool>) -> Result<(), ProcessErrors> {
        let right_now = Instant::now();
        let mut current_time = Duration::from_secs(0);

        // Running The Process ...
        loop {
            // interrupting the process with shared Atomic Boolean ------------------

            'interrupt: loop {//
                if interrupt_sings.load(Ordering::Relaxed) {
                    break 'interrupt;
                }
            }

            // Running The Process  -------------------------------------------------
            current_time = right_now.elapsed();
            if current_time >= self.cpu_burst_time {
                break
            }

            self.processed_time = current_time;
        }
        Ok(()) // Returns Ok(()) if there is no error
    }
    pub fn run(&self) -> Result<(), ProcessErrors> {
        let right_now = Instant::now();
        let mut current_time = Duration::from_secs(0);
        // Running The Process ...
        loop {
            current_time = right_now.elapsed();
            if current_time >= self.cpu_burst_time {
                break
            }
        }
        Ok(())
    }
}

pub fn build_test_process() -> Process {
    let mut rng = thread_rng();
    Process {id: Uuid::new_v4(), arrival_time: Duration::from_millis(rng.gen_range(0..500)), priority: rng.gen_range(1..10), cpu_burst_time: Duration::from_millis(rng.gen_range(0..500)), status: ProcessStatus::New, processed_time: Duration::from_secs(0)  }
}