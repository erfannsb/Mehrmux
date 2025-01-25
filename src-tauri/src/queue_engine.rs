use tauri::{window, Emitter, Window};
use std::collections::BinaryHeap;
use crate::process_gen::{Metrics, Process, ProcessStatus, ProcessType, SerializableProcess};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use serde::de::Unexpected::Str;
use serde::Serialize;
// Utils -------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum QueueDiscipline {
    /// queue discipline is the way cpu scheduler prioritize processes
    /// with this enum we choose the algorithm and pass it to the ready
    /// queue
    FIFO,
    SPN,
    FCFS,
    SJF,
    HRRN,
    RR,
    SRF,
}
#[derive(Debug, Serialize, Clone)]
pub enum MetricValue {
    // Since metric values' types differ. in order to store these metrics in a single
    // hashmap we need to define this enum to store multiple types inside of it.
    DurationValue(Duration),
    PercentageValue(f64),
    IntegerValue(i32),
}

struct DataToBeSent {
    queue: Vec<Process>,
    finished: Vec<Process>,
}

// Ready Queue -------------------------------------------------------------------------------------

pub struct ReadyQueue {
    processes: Vec<Process>,
    discipline: QueueDiscipline,
    time_quantum: Duration,
    context_switch: Duration,
    finished_processes: Vec<Process>, // processes that finished their process time are pushed
                                      // into this vector so rust won't drop their value.
}

impl ReadyQueue {
    pub fn new(discipline: QueueDiscipline, context_switch: Duration, time_quantum: Duration) -> Self {
        ReadyQueue {
            processes: Vec::new(),
            discipline,
            time_quantum,
            context_switch,
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
            QueueDiscipline::RR => {
                self.processes.sort_by(|p1,p2| {
                    let p1_le = p1.last_execution.unwrap_or(SystemTime::UNIX_EPOCH);
                    let p2_le = p2.last_execution.unwrap_or(SystemTime::UNIX_EPOCH);

                    p1_le.partial_cmp(&p2_le).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
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
        println!("Process {}, enqueued", process.id);
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

    pub fn execute_next(&mut self, window: &Window) -> Option<Process> {
        if let Some(mut process) = self.dequeue() {
            if process.processed_time == process.cpu_burst_time {
                return None;
            }
            let right_now = Instant::now();
            process.status = ProcessStatus::Running;
            println!("--------------------------------------");
            println!("process {:#?} is running in queue: {:?}", process, self.discipline);

            let result;
            if self.is_preemptive() {
                result = process.run_with_interrupt(&self.time_quantum, &right_now);
            } else {
                result = process.run(&right_now);
            }

            match result {
                Ok(()) => {
                    window.emit("process_stopped", process.clone().to_serializable()).unwrap();
                    println!("{:#?}", process.to_serializable());
                    println!("----------------------------------------");
                    return if process.processed_time == process.cpu_burst_time {
                        process.status = ProcessStatus::Terminated;
                        self.finished_processes.push(process.clone());
                        let data_to_be_sent: Vec<SerializableProcess> = self.finished_processes.iter().map(|p| p.to_serializable()).collect();
                        window.emit("finished_process", data_to_be_sent).unwrap();
                        println!("this process {:?} is finished", process.id);
                        println!("this is the quqeuueueueueueue: {:?}", self.processes);
                        sleep(self.context_switch); // simulating context_switch
                        None
                    } else {
                        // println!("process {:?} is waiting", process);
                        process.status = ProcessStatus::Waiting;
                        let copy = process.clone();
                        println!("Process: {} Pushed Back TO Queue", process.id);
                        self.processes.push(process);
                        println!("this is the quqeuueueueueueue: {:?}", self.processes);
                        sleep(self.context_switch); // simulating context_switch
                        Some(copy)
                    };
                }
                Err(e) => {
                    eprintln!("process {}, terminated with error: {:?}", process.id, e);
                }
            }
        } else {
            eprintln!("No Process Found!");
        }

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
        if length_of_processes == 0 {
            average_metrics.insert(String::from("average_turnaround_time"), MetricValue::DurationValue(Duration::from_secs(0)));
            average_metrics.insert(String::from("average_waiting_time"), MetricValue::DurationValue(Duration::from_secs(0)));
            average_metrics.insert(String::from("average_response_time"), MetricValue::DurationValue(Duration::from_secs(0)));
            average_metrics.insert(String::from("cpu_utilization"), MetricValue::PercentageValue(0.0));
            return average_metrics;
        }
        average_metrics.insert(
            String::from("average_turnaround_time"),
            MetricValue::DurationValue(
                process_metrics
                    .iter()
                    .map(|m| m.total_time)
                    .sum::<Duration>()
                    / length_of_processes,
            ),
        );
        average_metrics.insert(
            String::from("average_waiting_time"),
            MetricValue::DurationValue(
                process_metrics
                    .iter()
                    .map(|m| m.total_waiting_time)
                    .sum::<Duration>()
                    / length_of_processes,
            ),
        );
        average_metrics.insert(
            String::from("average_response_time"),
            MetricValue::DurationValue(
                process_metrics
                    .iter()
                    .map(|m| m.response_time)
                    .sum::<Duration>()
                    / length_of_processes,
            ),
        );
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
                (total_cpu_burst_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0,
            ),
        );
        average_metrics
    }

    pub fn is_queue_empty(&self) -> bool {
        self.processes.is_empty()
    }

    pub fn get_data(&self) -> Vec<SerializableProcess> {
        self.processes.iter().map(|p| p.to_serializable()).collect()
    }
}

// MultiLevel Queue --------------------------------------------------------------------------------

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
        context_switch: Duration,
        time_quantum: Duration,
    ) -> Self {
        let q1 = ReadyQueue::new(q1_d, time_quantum.clone(), context_switch.clone());
        let q2 = ReadyQueue::new(q2_d, time_quantum.clone(), context_switch.clone());
        let q3 = ReadyQueue::new(q3_d, time_quantum, context_switch);

        MLQ {
            queue_1: q1,
            queue_2: q2,
            queue_3: q3,
        }
    }
    pub(crate) fn enqueue(&mut self, process: Process) {
        match process.process_type {
            ProcessType::SystemProcess => self.queue_1.enqueue(process),
            ProcessType::InteractiveProcess => self.queue_2.enqueue(process),
            ProcessType::BatchProcess => self.queue_3.enqueue(process),
        }
    }

    pub(crate) fn execute_next(&mut self, window: &Window) {
        if !self.queue_1.processes.is_empty() {
            self.queue_1.execute_next(window);
        } else if !self.queue_2.processes.is_empty() {
            self.queue_2.execute_next(window);
        } else if !self.queue_3.processes.is_empty() {
            self.queue_3.execute_next(window);
        }
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue_1.processes.is_empty()
            && self.queue_2.processes.is_empty()
            && self.queue_3.processes.is_empty()
    }

    pub fn calculate_metric(&self) -> [HashMap<String, MetricValue>; 3] {
        let m1 = self.queue_1.calculate_metrics();
        let m2 = self.queue_2.calculate_metrics();
        let m3 = self.queue_3.calculate_metrics();

        return [m1,m2,m3]
    }
}

// MultiLevel FeedBack Algorithm -------------------------------------------------------------------

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
        context_switch: Duration,
        time_quantum: Duration
    ) -> Self {

        let q1 = ReadyQueue::new(q1_d, time_quantum.clone(), context_switch.clone());
        let q2 = ReadyQueue::new(q2_d, time_quantum.clone(), context_switch.clone());
        let q3 = ReadyQueue::new(q3_d, time_quantum, context_switch);


        MLFQ {
            queue_1: q1,
            queue_2: q2,
            queue_3: q3,
        }
    }

    pub(crate) fn enqueue(&mut self, process: Process) {
        self.queue_1.enqueue(process);
    }

    pub(crate) fn execute_next(&mut self, window: &Window) {
        if !self.queue_1.processes.is_empty() {
            let process = self.queue_1.execute_next(window);
            if let Some(process) = process {
                self.queue_2.enqueue(process)
            }
        } else if !self.queue_2.processes.is_empty() {
            let process = self.queue_2.execute_next(window);
            if let Some(process) = process {
                if process.waiting_time >= process.cpu_burst_time {
                    self.queue_1.enqueue(process)
                } else {
                    self.queue_3.enqueue(process)
                }
            }
        } else if !self.queue_3.processes.is_empty() {
            let process = self.queue_3.execute_next(window);
            if let Some(process) = process {
                if process.waiting_time >= process.cpu_burst_time {
                    self.queue_2.enqueue(process)
                } else {
                    self.queue_3.enqueue(process)
                }
            }
        }
    }

    pub fn calculate_metric(&self) -> [HashMap<String, MetricValue>; 3] {
        let m1 = self.queue_1.calculate_metrics();
        let m2 = self.queue_2.calculate_metrics();
        let m3 = self.queue_3.calculate_metrics();

        return [m1,m2,m3]
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue_1.processes.is_empty()
            && self.queue_2.processes.is_empty()
            && self.queue_3.processes.is_empty()
    }
}
