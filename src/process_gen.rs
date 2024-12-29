use std::{time::Duration};
use std::thread::sleep;
use std::time::Instant;
use uuid::Uuid;
use rand::prelude::*;

pub enum ProcessErrors {
    UnknownError,
}

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    New,
    Ready,
    Running,
    Waiting,
    Terminated
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessType {
    SystemProcess,
    InteractiveProcess,
    BatchProcess
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: Uuid,
    pub arrival_time: Instant,
    pub cpu_burst_time: Duration,
    pub status: ProcessStatus,
    pub waiting_time: Duration,
    pub processed_time: Duration,
    pub process_type: ProcessType,
    pub last_execution: Option<Instant>,
}

impl Process {
    pub fn calculate_waiting_time(&mut self) {
        // Getting the current time:
        let current_time = Instant::now();

        if self.last_execution.is_none() {
            self.last_execution = Some(current_time);
            self.waiting_time += self.arrival_time.duration_since(self.last_execution.unwrap());
        }

        if let Some(last_exec) = self.last_execution {
            let time_passed_between_execution = current_time.duration_since(last_exec);
            self.waiting_time += time_passed_between_execution;
            self.last_execution = Some(current_time);
        }
    }

    pub fn run_with_interrupt(&mut self, mut quantum_time: Duration) -> Result<(), ProcessErrors> {

        // calculate waiting time
        self.calculate_waiting_time();

        // running the process

        let remaining_time = self.cpu_burst_time - self.processed_time;
        if remaining_time >= quantum_time {
            sleep(quantum_time);
            self.processed_time += quantum_time;
        } else{
            sleep(remaining_time);
            self.processed_time += remaining_time;
        }

        Ok(()) // Returns Ok(()) if there is no error
    }

    pub fn run(&mut self) -> Result<(), ProcessErrors> {
        //simulating process work ...
        sleep(self.cpu_burst_time);
        Ok(())  // Returns Ok(()) if there is no error
    }

    pub fn new(cbt: Duration, process_type: ProcessType) -> Self {
        Process {
            id: Uuid::new_v4(),
            cpu_burst_time: cbt,
            arrival_time: Instant::now(),
            status: ProcessStatus::New,
            processed_time: Duration::from_secs(0),
            waiting_time: Duration::from_secs(0),
            last_execution: None,
            process_type,
        }
    }
}

pub fn build_test_process() -> Process {
    let mut rng = thread_rng();
    let process_variants = [ProcessType::BatchProcess, ProcessType::SystemProcess, ProcessType::SystemProcess];
    Process::new(
        Duration::from_millis(rng.gen_range(0..500)),
        process_variants[rng.gen_range(0..process_variants.len())]
    )
}