use rand::prelude::*;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use uuid::Uuid;

// Utils -------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    /// Determines the current state of the process which could be one of these: New,
    /// Ready, Running, Waiting, Terminated
    New,  // The process has been created but not yet ready to run.
    Ready,  // The process is ready to run but is waiting for CPU time.
    Running,  // The process is currently executing.
    Waiting,  // The process is waiting for an event (e.g., I/O completion).
    Terminated,  // The process has finished execution.
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessType {
    /// SystemProcess represents processes that are critical to the system's operation,
    /// often having higher priority and requiring swift execution.
    SystemProcess,

    /// InteractiveProcess represents processes that interact with the user, such as GUI applications.
    /// These processes typically require lower latency to provide a responsive user experience.
    InteractiveProcess,

    /// BatchProcess represents processes that are run in the background, often with lower priority,
    /// and do not require immediate user interaction, such as scheduled tasks or data processing jobs.
    BatchProcess,
}
#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    pub response_time: Duration,  // Time taken from process arrival to its first response.
    pub total_waiting_time: Duration,  // Total time the process spent waiting in queues.
    pub total_time: Duration,  // Total time from process arrival to completion.
}

impl Metrics {
    fn new() -> Self {
        Metrics {
            response_time: Duration::from_millis(0),
            total_time: Duration::from_millis(0),
            total_waiting_time: Duration::from_millis(0),
        }
    }
}

// Process -----------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Process {
    pub id: Uuid,  // Unique identifier for the process.
    pub arrival_time: Instant,  // The time when the process arrived in the system.
    pub cpu_burst_time: Duration,  // The total CPU time required by the process.
    pub status: ProcessStatus,  // Current status of the process.
    pub waiting_time: Duration,  // Accumulated time the process has spent waiting.
    pub processed_time: Duration,  // Total time the process has been executed.
    pub process_type: ProcessType,  // Type of the process (e.g., system, interactive, batch).
    pub last_execution: Option<Instant>,  // The last time the process was executed.
    pub metrics: Metrics,  // Performance metrics related to the process.
}

impl Process {
    pub fn calculate_waiting_time(&mut self, &current_time: &Instant) {
        // Getting the current time:
        let current_time = Instant::now();

        // Check if this is the first time the process is being executed
        if self.last_execution.is_none() {
            self.last_execution = Some(current_time);
            self.waiting_time += self
                .last_execution
                .unwrap()
                .duration_since(self.arrival_time);
            self.metrics.response_time = self.waiting_time;
            self.metrics.total_waiting_time = self.waiting_time;
        }

        // Logic for subsequent executions
        if let Some(last_exec) = self.last_execution {
            let time_passed_between_execution = current_time.duration_since(last_exec);
            self.waiting_time += time_passed_between_execution;
            self.last_execution = Some(current_time);
            self.metrics.total_waiting_time = self.waiting_time;
        }
    }

    pub fn run_with_interrupt(
        &mut self,
        &quantum_time: &Duration,
        &current_time: &Instant,
    ) -> Result<(), Box<dyn Error>> {
        // calculate waiting time
        self.calculate_waiting_time(&current_time);

        //simulating process work ...

        let remaining_time = self.cpu_burst_time - self.processed_time;
        if remaining_time >= quantum_time {
            sleep(quantum_time);
            self.processed_time += quantum_time;
        } else {
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
        Ok(()) // Returns Ok(()) if there is no error
    }

    pub fn new(cbt: Duration) -> Self {
        let mut rng = thread_rng();
        let process_variants = [
            ProcessType::BatchProcess,
            ProcessType::SystemProcess,
            ProcessType::SystemProcess,
        ];
        Process {
            id: Uuid::new_v4(),
            cpu_burst_time: cbt,
            arrival_time: Instant::now(),
            status: ProcessStatus::New,
            processed_time: Duration::from_secs(0),
            waiting_time: Duration::from_secs(0),
            last_execution: None,
            process_type: process_variants[rng.gen_range(0..process_variants.len())],
            metrics: Metrics::new(),
        }
    }
}
