use std::{thread::sleep, time::Duration};
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
    id: Uuid,
    arrival_time: Duration,
    priority: i32,
    cpu_burst_time: Duration,
    pub status: ProcessStatus
}

impl Process {
    pub fn run(&self) -> Result<(), ProcessErrors> {
        sleep(self.cpu_burst_time); // Process Code ...
        Ok(()) // Returns Ok(()) if there is no error
    }
}

pub fn build_test_process() -> Process {
    let mut rng = thread_rng();
    Process {id: Uuid::new_v4(), arrival_time: Duration::from_millis(rng.gen_range(0..500)), priority: rng.gen_range(1..10), cpu_burst_time: Duration::from_millis(rng.gen_range(0..500)), status: ProcessStatus::New  }
}