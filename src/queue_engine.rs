use crate::process_gen::{Metrics, Process, ProcessStatus, ProcessType};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub enum QueueDiscipline {
    FIFO,
    SPN,
    FCFS,
    SJF,
    HRRN,
    RR,
    SRF,
}
#[derive(Debug)]
enum MetricValue {
    DurationValue(Duration),
    PercentageValue(f64),
    IntegerValue(i32),
}

pub struct ReadyQueue {
    processes: Vec<Process>,
    discipline: QueueDiscipline,
    time_quantum: Duration,
    context_switch: Duration,
    finished_processes: Vec<Process>,
}

impl ReadyQueue {
    pub fn new(discipline: QueueDiscipline) -> Self {
        ReadyQueue {
            processes: Vec::new(),
            discipline,
            time_quantum: Duration::from_millis(100),
            context_switch: Duration::from_millis(10),
            finished_processes: Vec::new(),
        }
    }

    pub fn sort(&mut self) {
        match self.discipline {
            QueueDiscipline::FIFO => {}
            QueueDiscipline::SPN => {
                self.processes.sort_by_key(|p| p.cpu_burst_time);
            }
            QueueDiscipline::FCFS => {}
            QueueDiscipline::SJF => {
                self.processes.sort_by_key(|p| p.cpu_burst_time);
            }
            QueueDiscipline::HRRN => {
                self.processes.sort_by(|p1, p2| {
                    let p1_ratio = (p1.waiting_time.as_millis() as f64
                        + p1.cpu_burst_time.as_millis() as f64)
                        / p1.cpu_burst_time.as_millis() as f64;
                    let p2_ratio = (p2.waiting_time.as_millis() as f64
                        + p2.cpu_burst_time.as_millis() as f64)
                        / p2.cpu_burst_time.as_millis() as f64;
                    p2_ratio
                        .partial_cmp(&p1_ratio)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            QueueDiscipline::RR => {}
            QueueDiscipline::SRF => {
                self.processes.sort_by(|p1, p2| {
                    let p1_remaining_time = p1.cpu_burst_time - p1.processed_time;
                    let p2_remaining_time = p2.cpu_burst_time - p2.processed_time;
                    p1_remaining_time
                        .partial_cmp(&p2_remaining_time)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
    }

    pub fn is_preemptive(&self) -> bool {
        match self.discipline {
            QueueDiscipline::FIFO => false,
            QueueDiscipline::SPN => false,
            QueueDiscipline::FCFS => false,
            QueueDiscipline::SJF => true,
            QueueDiscipline::HRRN => false,
            QueueDiscipline::RR => true,
            QueueDiscipline::SRF => true,
        }
    }

    pub fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        process.arrival_time = Instant::now();
        self.processes.push(process);
    }

    pub fn dequeue(&mut self) -> Option<Process> {
        self.sort();
        if self.processes.is_empty() {
            None
        } else {
            Some(self.processes.remove(0))
        }
    }

    pub fn execute_next(&mut self) -> Option<Process> {
        if let Some(mut process) = self.dequeue() {
            let right_now = Instant::now();
            process.status = ProcessStatus::Running;
            println!("--------------------------------------");
            println!("process {:?} is running", process);

            let result;
            if self.is_preemptive() {
                result = process.run_with_interrupt(&self.time_quantum, &right_now);
                println!("process is preemptive");
            } else {
                result = process.run(&right_now);
            }

            match result {
                Ok(()) => {
                    return if process.processed_time == process.cpu_burst_time {
                        process.status = ProcessStatus::Terminated;
                        println!("process {:?} is finished running", process);
                        self.finished_processes.push(process.clone());
                        Some(process)
                    } else {
                        println!("process {:?} is waiting", process);
                        process.status = ProcessStatus::Waiting;
                        self.processes.push(process.clone());
                        Some(process)
                    }
                }
                Err(e) => {
                    eprintln!("process {}, terminated with error: {:?}", process.id, e);
                }
            }
        } else {
            eprintln!("No Process Found!");
        }

        sleep(self.context_switch); // simulating context_switch
        None
    }

    pub fn calculate_metrics(&self) -> HashMap<String, MetricValue> {


        let process_metrics: Vec<Metrics> = self
            .finished_processes
            .iter()
            .map(|process| process.metrics)
            .collect();
        let length_of_processes: u32 = process_metrics.len() as u32;
        let mut average_metrics: HashMap<String, MetricValue> = HashMap::new();

        average_metrics.insert(
            String::from("average_turnaround_time"),
            MetricValue::DurationValue(
            process_metrics
                .iter()
                .map(|m| m.total_time)
                .sum::<Duration>()
                / length_of_processes
            ));
        average_metrics.insert(
            String::from("average_waiting_time"),
            MetricValue::DurationValue(
            process_metrics
                .iter()
                .map(|m| m.total_waiting_time)
                .sum::<Duration>()
                / length_of_processes
            ));
        average_metrics.insert(
            String::from("average_response_time"),
            MetricValue::DurationValue(
            process_metrics
                .iter()
                .map(|m| m.response_time)
                .sum::<Duration>()
                / length_of_processes
            ));
        let total_cpu_burst_time = self
            .finished_processes
            .iter()
            .map(|p| p.cpu_burst_time)
            .sum::<Duration>();
        let total_time = self
            .finished_processes
            .iter()
            .map(|p| p.metrics.total_time)
            .sum::<Duration>();
        average_metrics.insert(
            String::from("cpu_utilization"),
            MetricValue::PercentageValue(
                (total_cpu_burst_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0
            ));
        average_metrics
    }

    pub fn is_queue_empty(&self) -> bool {
        self.processes.is_empty()
    }
}

// MLQ Algorithm -----------------------------------------------------------------------------------

pub struct MLQ {
    pub queue_1: ReadyQueue,
    pub queue_2: ReadyQueue,
    pub queue_3: ReadyQueue,
}

impl MLQ {
    pub(crate) fn init(
        q1_d: QueueDiscipline,
        q2_d: QueueDiscipline,
        q3_d: QueueDiscipline,
    ) -> Self {
        MLQ {
            queue_1: ReadyQueue::new(q1_d),
            queue_2: ReadyQueue::new(q2_d),
            queue_3: ReadyQueue::new(q3_d),
        }
    }
    pub(crate) fn enqueue(&mut self, process: Process) {
        match process.process_type {
            ProcessType::SystemProcess => self.queue_1.enqueue(process),
            ProcessType::InteractiveProcess => self.queue_2.enqueue(process),
            ProcessType::BatchProcess => self.queue_3.enqueue(process),
        }
    }

    pub(crate) fn execute_next(&mut self) {
        if !self.queue_1.processes.is_empty() {
            self.queue_1.execute_next();
        } else if !self.queue_2.processes.is_empty() {
            self.queue_2.execute_next();
        } else if !self.queue_3.processes.is_empty() {
            self.queue_3.execute_next();
        }
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue_1.processes.is_empty() &&
        self.queue_2.processes.is_empty() &&
        self.queue_3.processes.is_empty()
    }
}

// MLFQ Algorithm ----------------------------------------------------------------------------------

pub struct MLFQ {
    pub queue_1: ReadyQueue,
    pub queue_2: ReadyQueue,
    pub queue_3: ReadyQueue,
}

impl MLFQ {
    pub(crate) fn init(
        q1_d: QueueDiscipline,
        q2_d: QueueDiscipline,
        q3_d: QueueDiscipline,
    ) -> Self {
        let mut q1 = ReadyQueue::new(q1_d);
        q1.time_quantum = Duration::from_millis(25);
        let mut q2 = ReadyQueue::new(q2_d);
        q2.time_quantum = Duration::from_millis(50);
        let mut q3 = ReadyQueue::new(q3_d);
        q3.time_quantum = Duration::from_millis(100);

        MLFQ {
            queue_1: q1,
            queue_2: q2,
            queue_3: q3,
        }
    }

    pub(crate) fn enqueue(&mut self, process: Process) {
        self.queue_1.enqueue(process);
    }

    pub(crate) fn execute_next(&mut self) {
        if !self.queue_1.processes.is_empty() {
            let process = self.queue_1.execute_next();
            if let Some(process) = process {
                self.queue_2.enqueue(process)
            }
        } else if !self.queue_2.processes.is_empty() {
            let process = self.queue_2.execute_next();
            if let Some(process) = process {
                if process.waiting_time >= process.cpu_burst_time {
                    self.queue_1.enqueue(process)
                } else {
                    self.queue_3.enqueue(process)
                }
            }
        } else if !self.queue_3.processes.is_empty() {
            let process = self.queue_3.execute_next();
            if let Some(process) = process {
                if process.waiting_time >= process.cpu_burst_time {
                    self.queue_2.enqueue(process)
                } else {
                    self.queue_3.enqueue(process)
                }
            }
        }
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue_1.processes.is_empty() &&
            self.queue_2.processes.is_empty() &&
            self.queue_3.processes.is_empty()
    }
}
