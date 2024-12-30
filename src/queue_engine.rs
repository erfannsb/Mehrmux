use std::thread::{sleep};
use std::time::{Duration, Instant};
use crate::process_gen::{build_test_process, Process, ProcessStatus, ProcessType};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
// Important Description ---------------------------------------------------------------------------
//All queue structs should have these properties:

// processes: vector of processes
//current_process: to keep track of the current process running
//current_time: to measure the time passed while running the queue algorithm
//context_switch_duration: arbitrary duration for hypothetical context switching process


//every queue should inherit from Queue trait and define its own enqueue and dequeue method

//every queue should implement start method in which there will be an infinite loop which never
//stops (by adding stop_flag in start method parameter we can then pass an atomic boolean to stop
//the loop). the loop would do nothing unless there is at least one process in the processes vector!
//if processes vector isn't empty then the process with the right priority based on the algorithm
//would be chosen in the dequeue method. then the chosen process will be executed by calling
//the run method.


// FIFO Algorithm ----------------------------------------------------------------------------------

pub struct FIFO {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl FIFO {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process)
    }

    fn dequeue(&mut self) -> Option<Process> {
        if self.processes.is_empty() {
            None
        } else {
            Some(self.processes.remove(0))
        }
    }

    pub(crate) fn start(&mut self) {
        if self.processes.is_empty(){
            return;
        }
        let time_passed = Instant::now();
        loop {
            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    println!("🔲 Process: [{}] Stared Running at {:?}",
                             self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                            self.current_time.clone()
                    );
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     self.current_process.as_ref().unwrap().cpu_burst_time,
                                     self.current_process.as_ref().unwrap().waiting_time
                            );
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    pub fn init() -> Self {
        Self {
            processes: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
        }
    }
}

// SPN Algorithm -----------------------------------------------------------------------------------
//Erfun

pub struct SPN {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl SPN {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process);
    }
    fn dequeue(&mut self) -> Option<Process> {
        self.processes.sort_by_key(|p| p.cpu_burst_time);
        if self.processes.is_empty() {
            None
        } else {
            Some(self.processes.remove(0))
        }
    }
    pub(crate) fn start(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {

            if self.processes.is_empty() {
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    println!("🔲 Process: [{}] Stared Running",
                             self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                    );
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     self.current_process.as_ref().unwrap().cpu_burst_time,
                                     self.current_process.as_ref().unwrap().waiting_time
                            );
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    pub fn init() -> Self {
        SPN {
            processes: vec![],
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
        }
    }
}

// FCFS Algorithm ----------------------------------------------------------------------------------
// Meownoosh

pub struct FCFS {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl FCFS {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process)
    }

    fn dequeue(&mut self) -> Option<Process> {
        if self.processes.is_empty() {
            None
        } else {
            Some(self.processes.remove(0))
        }
    }

    pub(crate) fn start(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if self.processes.is_empty() {
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    println!("🔲 Process: [{}] Stared Running at {:?}",
                             self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                             self.current_time.clone()
                    );
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     self.current_process.as_ref().unwrap().cpu_burst_time,
                                     self.current_process.as_ref().unwrap().waiting_time
                            );
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn start_and_end(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {

            if self.processes.is_empty() {
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    println!("🔲 Process: [{}] Stared Running",
                             self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                    );
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     self.current_process.as_ref().unwrap().id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     self.current_process.as_ref().unwrap().cpu_burst_time,
                                     self.current_process.as_ref().unwrap().waiting_time
                            );
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    pub fn init() -> Self {
        Self {
            processes: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_millis(10),
        }
    }
}

// SJF Algorithm -----------------------------------------------------------------------------------
// Erfun

pub struct SJF {
    processes: Vec<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}

impl SJF {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process);
    }
    fn dequeue(&mut self) -> Option<&mut Process> {
        self.processes.sort_by_key(|p| p.cpu_burst_time);
        self.processes.first_mut() // Return a mutable reference.
    }

    pub(crate) fn start(&mut self) {
        let time_passed = Instant::now();
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                break;
            }

            let time_quantum = self.time_quantum;
            let mut to_remove = None; // Track which process to remove.

            if let Some(process) = self.dequeue() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running, CBT: {:?}",
                         process.id.clone().to_string()[0..7].to_string(),
                        process.cpu_burst_time.clone()
                );
                let result = process.run_with_interrupt(time_quantum);
                self.current_time = time_passed.elapsed();
                let copy = self.current_time;  // Access current_time before mutably borrowing self

                if let Some(mut process) = self.dequeue() {
                    match result {
                        Ok(_) => {
                            if process.processed_time == process.cpu_burst_time {
                                process.status = ProcessStatus::Terminated;
                                to_remove = Some(process.id);
                                let ps = process.clone();
                                println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                         ps.id.to_string()[0..7].to_string(),
                                         copy,
                                         ps.cpu_burst_time,
                                         ps.waiting_time
                                );
                            } else {
                                process.status = ProcessStatus::Waiting;
                                let ps = process.clone();
                                self.current_time = time_passed.elapsed();

                                println!("🟦 Process [{}] Stopped Because It Reached Time Quantum, Processed Time: {:?}, CBT: {:?}", ps.id.to_string()[0..7].to_string(),
                                    ps.processed_time.clone(), ps.cpu_burst_time.clone());
                            }
                        }
                        Err(_) => {
                            eprintln!("Error running process {:?}", process.id);
                        }
                    }
                }
            } else {
                println!("No processes left to process.");
                break;
            }

            // Remove the process after the mutable borrow ends.
            if let Some(id) = to_remove {
                if let Some(pos) = self.processes.iter().position(|p| p.id == id) {
                    self.processes.remove(pos);
                }
            }

            sleep(self.context_switch_duration); // Hypothetical Context Switching Process ...
        }
    }

    pub(crate) fn init() -> Self {
        SJF {
            processes: vec![],
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration::from_millis(100)
        }
    }
}

// HRRN Algorithm ----------------------------------------------------------------------------------
// Meownoosh

pub struct HRRN {
    processes: Vec<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}

impl HRRN {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process);
    }

    fn dequeue(&mut self) -> Option<Process> {
        // Sorting Processes based on the highest response ratio first.
        if self.processes.is_empty() {
            None
        } else {
            let current_time = Instant::now();

            for p in self.processes.iter_mut() {
                p.waiting_time = current_time.duration_since(p.arrival_time);
            }

            self.processes.sort_by(|p1, p2| {
                let p1_ratio = (p1.waiting_time.as_millis() as f64 + p1.cpu_burst_time.as_millis() as f64) / p1.cpu_burst_time.as_millis() as f64;
                let p2_ratio = (p2.waiting_time.as_millis() as f64 + p2.cpu_burst_time.as_millis() as f64) / p2.cpu_burst_time.as_millis() as f64;
                p2_ratio.partial_cmp(&p1_ratio).unwrap_or(std::cmp::Ordering::Equal)
            });
            // Return a mutable reference to the first process
            Some(self.processes.remove(0))
        }
    }

    pub(crate) fn start(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if self.processes.is_empty() {
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    process.status = ProcessStatus::Running;
                    println!("🔲 Process: [{}] Stared Running, CBT: {:?}",
                             process.id.clone().to_string()[0..7].to_string(),
                             process.cpu_burst_time.clone()
                    );
                    let result = process.run();
                    match result {
                        Ok(_) => {
                            process.status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     process.id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     process.cpu_burst_time,
                                     process.waiting_time
                            );
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    pub(crate) fn init() -> Self {
        HRRN {
            processes: vec![],
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration::from_millis(100)
        }
    }
}


// RR Algorithm ------------------------------------------------------------------------------------
// Meownoosh

pub struct RR {
    processes: Vec<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}

impl RR {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process)
    }

    fn dequeue(&mut self) -> Option<Process> {
        if self.processes.is_empty() {
            None
        } else {
            Some(self.processes.remove(0))
        }
    }

    pub(crate) fn start(&mut self) {
        let right_now = Instant::now();
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                break;
            }

            if let Some(mut process) = self.dequeue() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running, CBT: {:?}",
                         process.id.clone().to_string()[0..7].to_string(),
                         process.cpu_burst_time.clone()
                );
                let result = process.run_with_interrupt(self.time_quantum);

                match result {
                    Ok(_) => {
                        if process.processed_time == process.cpu_burst_time {
                            process.status = ProcessStatus::Terminated;
                            self.current_time = right_now.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     process.id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     process.cpu_burst_time,
                                     process.waiting_time
                            );
                        } else {
                            process.status = ProcessStatus::Waiting;
                            let ps = process.clone();
                            self.processes.push(process); // push process to the end of the processes vector
                            println!("🟦 Process [{}] Stopped Because It Reached Time Quantum, Processed Time: {:?}, CBT: {:?}", ps.id.to_string()[0..7].to_string(),
                                     ps.processed_time.clone(), ps.cpu_burst_time.clone());
                        }
                    }
                    Err(_) => {
                        eprintln!("Error running process {:?}", process.id);
                    }
                }
            } else {
                println!("No processes left to process.");
                break;
            }

            sleep(self.context_switch_duration); // Hypothetical Context Switching Process ...
        }
    }

    fn start_and_end(&mut self) {
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                break;
            }

            if let Some(mut process) = self.dequeue() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running",
                         process.id.clone().to_string()[0..7].to_string(),
                );
                let result = process.run_with_interrupt(self.time_quantum);

                match result {
                    Ok(_) => {
                        if process.processed_time == process.cpu_burst_time {
                            process.status = ProcessStatus::Terminated;
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     process.id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     process.cpu_burst_time,
                                     process.waiting_time
                            );
                            } else {
                            process.status = ProcessStatus::Waiting;
                            let ps = process.clone();
                            self.processes.push(process); // push process to the end of the processes vector
                            println!("🟦 Process [{}] Stopped Because It Reached Time Quantum, Processed Time: {:?}, CBT: {:?}", ps.id.to_string()[0..7].to_string(),
                                     ps.processed_time.clone(), ps.cpu_burst_time.clone());
                        }
                    }
                    Err(_) => {
                        eprintln!("Error running process {:?}", process.id);
                    }
                }
            } else {
                println!("No processes left to process.");
                break;
            }

            sleep(self.context_switch_duration); // Hypothetical Context Switching Process ...
        }
    }

    fn start_and_return(&mut self) -> Option<Process>{
        let right_now = Instant::now();
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                return None;
            }

            if let Some(mut process) = self.dequeue() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running",
                         process.id.clone().to_string()[0..7].to_string(),
                );
                let result = process.run_with_interrupt(self.time_quantum);

                match result {
                    Ok(_) => {
                        if process.processed_time == process.cpu_burst_time {
                            process.status = ProcessStatus::Terminated;
                            self.current_time = right_now.elapsed();
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     process.id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     process.cpu_burst_time,
                                     process.waiting_time
                            );
                        } else {
                            process.status = ProcessStatus::Waiting;
                            self.current_time = right_now.elapsed();
                            let ps = process.clone();
                            println!("🟦 Process [{}] Stopped Because It Reached Time Quantum, Processed Time: {:?}, CBT: {:?}", ps.id.to_string()[0..7].to_string(),
                                     ps.processed_time.clone(), ps.cpu_burst_time.clone());
                            return Some(process);

                        }
                    }
                    Err(_) => {
                        eprintln!("Error running process {:?}", process.id);
                    }
                }
            } else {
                println!("No processes left to process.");
                return None;
            }

            sleep(self.context_switch_duration); // Hypothetical Context Switching Process ...
        }

    }

    pub(crate) fn init() -> Self {
        RR {
            processes: vec![] ,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration::from_millis(100)
        }
    }
}

// SRF Algorithm -----------------------------------------------------------------------------------
// Erfun

pub struct SRF {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}

impl SRF {
    pub(crate) fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process)
    }

    fn dequeue(&mut self) -> Option<&mut Process> {
        self.processes.sort_by(|p1, p2| {
            let p1_remaining_time = p1.cpu_burst_time - p1.processed_time;
            let p2_remaining_time = p2.cpu_burst_time - p2.processed_time;
            p1_remaining_time.partial_cmp(&p2_remaining_time).unwrap_or(std::cmp::Ordering::Equal)
        });
        self.processes.first_mut() // Return a mutable reference.
    }

    fn dequeue_remove(&mut self) -> Option<Process> {
        self.processes.sort_by(|p1, p2| {
            let p1_remaining_time = p1.cpu_burst_time - p1.processed_time;
            let p2_remaining_time = p2.cpu_burst_time - p2.processed_time;
            p1_remaining_time.partial_cmp(&p2_remaining_time).unwrap_or(std::cmp::Ordering::Equal)
        });
        Some(self.processes.remove(0))
    }
    pub(crate) fn start(&mut self) {
        let right_now = Instant::now();
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                break;
            }

            let time_quantum = self.time_quantum;
            self.current_time = right_now.elapsed();

            let copy = self.current_time;  // Access current_time after mutable borrow ends
            let mut to_remove = None; // Track which process to remove.

            // Dequeue the process (mutable borrow of self).
            if let Some(mut process) = self.dequeue() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running, CBT: {:?}",
                         process.id.clone().to_string()[0..7].to_string(),
                         process.cpu_burst_time.clone()
                );
                // Run process and handle the result
                let result = process.run_with_interrupt(time_quantum);

                // Access self.current_time here, after process is dequeued and result is processed.

                let ps = process.clone();  // Clone the process for printing later

                match result {
                    Ok(_) => {
                        if process.processed_time == process.cpu_burst_time {
                            process.status = ProcessStatus::Terminated;
                            to_remove = Some(process.id);
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     ps.id.to_string()[0..7].to_string(),
                                     copy,
                                     ps.cpu_burst_time,
                                     ps.waiting_time
                            );
                        } else {
                            process.status = ProcessStatus::Waiting;
                            let ps = process.clone();
                            println!("🟦 Process [{}] Stopped Because It Reached Time Quantum, Processed Time: {:?}, CBT: {:?}", ps.id.to_string()[0..7].to_string(),
                                     ps.processed_time.clone(), ps.cpu_burst_time.clone());
                        }
                    }
                    Err(_) => {
                        eprintln!("Error running process {:?}", process.id);
                    }
                }

                // Remove the process after the mutable borrow ends
                if let Some(id) = to_remove {
                    if let Some(pos) = self.processes.iter().position(|p| p.id == id) {
                        self.processes.remove(pos);
                    }
                }

                // Hypothetical Context Switching Process ...
                sleep(self.context_switch_duration);
            } else {
                println!("No processes left to process.");
                break;
            }
        }
    }

    fn start_and_return(&mut self) -> Option<Process> {
        self.current_time = Duration::from_millis(0);

        loop {
            if self.processes.is_empty() {
                return None;
            }

            if let Some(mut process) = self.dequeue_remove() {
                process.status = ProcessStatus::Running;
                println!("🔲 Process: [{}] Stared Running",
                         process.id.clone().to_string()[0..7].to_string(),
                );
                let result = process.run_with_interrupt(self.time_quantum);

                match result {
                    Ok(_) => {
                        if process.processed_time == process.cpu_burst_time {
                            process.status = ProcessStatus::Terminated;
                            println!("🔸 Process: [{}] Terminated At: {:?}, CBT: {:?}, Waiting Time: {:?}",
                                     process.id.clone().to_string()[0..7].to_string(),
                                     self.current_time,
                                     process.cpu_burst_time,
                                     process.waiting_time
                            );
                        } else {
                            process.status = ProcessStatus::Waiting;
                            return Some(process); // Return the process without borrowing.
                        }
                    }
                    Err(_) => {
                        eprintln!("Error running process {:?}", process.id);
                    }
                }
            } else {
                println!("No processes left to process.");
                return None;
            }

            sleep(self.context_switch_duration); // Hypothetical Context Switching Process...
        }
    }


    pub(crate) fn init() -> Self {
        Self {
            processes: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration::from_millis(100)
        }
    }
}

// MLQ Algorithm -----------------------------------------------------------------------------------
// Meownoosh

pub struct MLQ {
    queue_1: RR,
    queue_2: RR,
    queue_3: FCFS,
}

impl MLQ {
    pub(crate) fn init() -> Self {
        MLQ {
            queue_1: RR::init(),
            queue_2: RR::init(),
            queue_3: FCFS::init()
        }
    }
    pub(crate) fn enqueue(&mut self, process: Process) {
        match process.process_type {
            ProcessType::SystemProcess => self.queue_1.enqueue(process),
            ProcessType::InteractiveProcess => self.queue_2.enqueue(process),
            ProcessType::BatchProcess => self.queue_3.enqueue(process)
        }
    }

    pub(crate) fn start(&mut self) {
        loop {
            if self.queue_1.processes.is_empty() &&
                self.queue_2.processes.is_empty() &&
                self.queue_3.processes.is_empty()
            {
                break;
            }
            if !self.queue_1.processes.is_empty() {
                self.queue_1.start_and_end();
            }
            else if !self.queue_2.processes.is_empty() {
                self.queue_2.start_and_end();
            }
            else if !self.queue_3.processes.is_empty() {
                self.queue_3.start_and_end();
            }
        }
    }
}

// MLFQ Algorithm ----------------------------------------------------------------------------------
// Erfun

pub struct MLFQ {
    queue_1: SRF,
    queue_2: RR,
    queue_3: SRF
}

impl MLFQ {
    pub(crate) fn init() -> Self {
        let mut srf1 = SRF::init();
        srf1.time_quantum = Duration::from_millis(30);
        let mut rr2 = RR::init();
        rr2.time_quantum = Duration::from_millis(50);
        let mut srf3 = SRF::init();
        srf3.time_quantum = Duration::from_millis(100);

        MLFQ{
            queue_1: srf1,
            queue_2: rr2,
            queue_3: srf3,
        }
    }

    pub(crate) fn enqueue(&mut self, process: Process) {
        self.queue_1.enqueue(process);
    }

        pub(crate) fn start(&mut self) {
            loop {
                if self.queue_1.processes.is_empty() &&
                    self.queue_2.processes.is_empty() &&
                    self.queue_3.processes.is_empty()
                {
                    break;
                }

                if !self.queue_1.processes.is_empty() {
                    let process = self.queue_1.start_and_return();
                    if let Some(process) = process {
                        self.queue_2.enqueue(process)
                    }
                }
                else if !self.queue_2.processes.is_empty() {
                    let process = self.queue_2.start_and_return();
                    if let Some(process) = process {
                        if process.waiting_time >= process.cpu_burst_time {
                            self.queue_1.enqueue(process)
                        } else{
                            self.queue_2.enqueue(process)

                        }
                    }
                }
                else if !self.queue_3.processes.is_empty() {
                    let process = self.queue_3.start_and_return();
                    if let Some(process) = process {
                        if process.waiting_time >= process.cpu_burst_time {
                            self.queue_2.enqueue(process)
                        } else{
                            self.queue_3.enqueue(process)
                        }
                    }
                }
            }
        }
}


// Testing -----------------------------------------------------------------------------------------

pub fn test() {
    let mut sjf = MLFQ::init();
    sjf.enqueue(build_test_process());
    sjf.enqueue(build_test_process());
    sjf.enqueue(build_test_process());
    sjf.enqueue(build_test_process());
    sjf.enqueue(build_test_process());

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    let handle = thread::spawn(move || {
        // sjf.start(stop_flag_clone); // Pass stop_flag to the start method
    });

    sleep(Duration::from_secs(10));
    stop_flag.store(true, Ordering::Relaxed); // Set the stop flag after 5 seconds

    handle.join().unwrap();
}

pub fn test_two() {
    let mut process = build_test_process();
    process.cpu_burst_time = Duration::from_secs(8);

    process.run_with_interrupt(Duration::from_secs(3));
    println!("{:?}", process.processed_time);
    process.run_with_interrupt(Duration::from_secs(3));
    println!("{:?}", process.processed_time);
    process.run_with_interrupt(Duration::from_secs(3));
    println!("{:?}", process.processed_time);

}
