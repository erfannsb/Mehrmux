use std::{time::Duration};
use std::error::Error;
use std::thread::sleep;
use std::time::Instant;
use uuid::Uuid;
use rand::prelude::*;

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

#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    pub response_time: Duration,
    pub total_waiting_time: Duration,
    pub total_time: Duration,
}

impl Metrics {
    fn new() -> Self {
        Metrics {
            response_time: Duration::from_millis(0),
            total_time: Duration::from_millis(0),
            total_waiting_time: Duration::from_millis(0),
        }
    }

    fn averages(&mut self, number_of_process: usize) -> (Duration, Duration, Duration) {
        (
            self.total_waiting_time / number_of_process as u32,
            self.total_time / number_of_process as u32,
            self.total_waiting_time / number_of_process as u32,
        )
    }
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
    pub metrics: Metrics,
}

impl Process {
    pub fn calculate_waiting_time(&mut self, &current_time: &Instant) {
        // Getting the current time:
        let current_time = Instant::now();

        if self.last_execution.is_none() {
            self.last_execution = Some(current_time);
            self.waiting_time += self.last_execution.unwrap().duration_since(self.arrival_time);
            self.metrics.response_time = self.waiting_time;
            self.metrics.total_waiting_time = self.waiting_time;
        }

        if let Some(last_exec) = self.last_execution {
            let time_passed_between_execution = current_time.duration_since(last_exec);
            self.waiting_time += time_passed_between_execution;
            self.last_execution = Some(current_time);
            self.metrics.total_waiting_time = self.waiting_time;
        }
    }

    pub fn run_with_interrupt(&mut self, &quantum_time: &Duration, &current_time: &Instant) -> Result<(), Box<dyn Error>> {

        // calculate waiting time
        self.calculate_waiting_time(&current_time);

        // running the process

        let remaining_time = self.cpu_burst_time - self.processed_time;
        if remaining_time >= quantum_time {
            sleep(quantum_time);
            self.processed_time += quantum_time;
        } else{
            sleep(remaining_time);
            self.processed_time += remaining_time;
        }
        self.metrics.total_time = self.processed_time + self.waiting_time;
        Ok(()) // Returns Ok(()) if there is no error
    }

    pub fn run(&mut self, &current_time: &Instant) -> Result<(), Box<dyn Error>> {
        //simulating process work ...
        self.calculate_waiting_time(&current_time);
        sleep(self.cpu_burst_time);
        self.processed_time = self.cpu_burst_time;
        self.metrics.total_time = self.cpu_burst_time + self.waiting_time;
        Ok(())  // Returns Ok(()) if there is no error
    }

    pub fn new(cbt: Duration) -> Self {
        let mut rng = thread_rng();
        let process_variants = [ProcessType::BatchProcess, ProcessType::SystemProcess, ProcessType::SystemProcess];
        Process {
            id: Uuid::new_v4(),
            cpu_burst_time: cbt,
            arrival_time: Instant::now(),
            status: ProcessStatus::New,
            processed_time: Duration::from_secs(0),
            waiting_time: Duration::from_secs(0),
            last_execution: None,
            process_type: process_variants[rng.gen_range(0..process_variants.len())],
            metrics: Metrics::new()
        }
    }
}

pub fn build_test_process() -> Process {
    let mut rng = thread_rng();
    Process::new(
        Duration::from_millis(rng.gen_range(0..500)),
    )
}