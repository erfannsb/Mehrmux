use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::process_gen::{build_test_process, Process, ProcessStatus};
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

// Common Queueing Sense ---------------------------------------------------------------------------

trait  Queue {
    fn enqueue(&mut self,process: Process); // to add a process to the queue
    fn dequeue(&mut self) -> Option<Process>; // to remove and return a process from queue
    fn start(&mut self, stop_flag: Arc<AtomicBool>); // to start running the queue
    fn init() -> Self; // to initiate an instant of queue
}

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

struct FIFO {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl Queue for FIFO {
    fn enqueue(&mut self, mut process: Process) {
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

    fn start(&mut self ,stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id ,self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
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

struct SPN {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl Queue for SPN {
    fn enqueue(&mut self, mut process: Process) {
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
    fn start(&mut self ,stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id ,self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
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
struct FCFS {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}


impl Queue for FCFS {
    fn enqueue(&mut self, mut process: Process) {
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

    fn start(&mut self ,stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id ,self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
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

struct SJF {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}

impl SJF {
    fn enqueue(&mut self, mut process: Process) {
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
    fn start(&mut self ,stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id ,self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
        SJF {
            processes: vec![],
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration::from_micros(10)
        }
    }
}



// HRRN Algorithm ----------------------------------------------------------------------------------
// Meownoosh


struct HRRN {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl Queue for HRRN {
    fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.processes.push(process);
    }

    fn dequeue(&mut self) -> Option<Process> {
        if self.processes.is_empty() {
            None
        } else {
            let time_elapsed = self.current_time.as_secs() as u32;

            // Calculate response ratio
            // self.processes.iter_mut().for_each(|process| {
            //     if process.arrival_time <= time_elapsed {
            //         process.response_ratio = (time_elapsed - process.arrival_time + process.cpu_burst_time) as f64 / process.cpu_burst_time as f64;
            //     }
            // });

            // Sort by response ratio in descending order
            //self.processes.sort_by(|a, b| b.response_ratio.partial_cmp(&a.response_ratio).unwrap());

            Some(self.processes.remove(0))
        }
    }

    fn start(&mut self, stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();

        loop {
            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    if process.arrival_time > time_passed.elapsed() {
                        sleep(process.arrival_time - time_passed.elapsed());
                    }

                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id, self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
        HRRN {
            processes: vec![],
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
        }
    }
}



// RR Algorithm ------------------------------------------------------------------------------------
// Meownoosh
struct RR {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    time_quantum: Duration,
}


impl Queue for RR {
    fn enqueue(&mut self, mut process: Process) {
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

    fn start(&mut self ,stop_flag: Arc<AtomicBool>) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if stop_flag.load(Ordering::Relaxed) {
                println!("Loop stopped.");
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
                    self.current_process = Some(process);
                    self.current_process.as_mut().unwrap().status = ProcessStatus::Running;
                    let result = self.current_process.as_mut().unwrap().run();
                    match result {
                        Ok(_) => {
                            self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated;
                            self.current_time = time_passed.elapsed();
                            println!("Process: {} Terminated At: {:?}", self.current_process.as_mut().unwrap().id ,self.current_time);
                        },
                        Err(_) => {}
                    }
                }
                None => {}
            }

            sleep(self.context_switch_duration); // context switch process ...
        }
    }

    fn init() -> Self {
        RR {
            processes: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_micros(5),
            time_quantum: Duration:: from_micros(10)
        }
    }
}
// SRF Algorithm -----------------------------------------------------------------------------------
// Erfun

struct SRF {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl SRF {
}

// MLQ Algorithm -----------------------------------------------------------------------------------
// Meownoosh

struct MLQ {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl MLQ {
}

// MLFQ Algorithm ----------------------------------------------------------------------------------
// Erfun
struct MLFQ {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
}

impl MLFQ {
}


// Testing -----------------------------------------------------------------------------------------

pub fn test() {
    let mut spn: SPN = SPN::init();
    let mut list_of_processes = vec![build_test_process(), build_test_process(), build_test_process()];
    list_of_processes.sort_by_key(|p| p.arrival_time);
    println!("{:?}", &list_of_processes);
    spn.processes.extend(list_of_processes);

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    let handle = thread::spawn(move || {
        spn.start(stop_flag_clone); // Pass stop_flag to the start method
    });

    sleep(Duration::from_secs(5));
    stop_flag.store(true, Ordering::Relaxed); // Set the stop flag after 5 seconds

    handle.join().unwrap();
}


fn test_two() {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();

    // Start a thread that will "freeze" and "defrost"
    let handle = thread::spawn(move || {
        let mut count = 0;

        loop {
            // Print the count and then "freeze" by waiting for a message
            println!("Count: {}", count);
            count += 1;

            // This is where the function "freezes" until a message is received
            receiver.recv().unwrap();

            // Simulate some processing
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Let the program "defrost" the function after 3 seconds
    thread::sleep(Duration::from_secs(3));
    println!("Resuming function...");
    sender.send(()).unwrap();

    // Let the function run for a while, then pause it again after another 3 seconds
    thread::sleep(Duration::from_secs(3));
    println!("Pausing function...");
    sender.send(()).unwrap();

    // Let the function continue one more time
    thread::sleep(Duration::from_secs(3));
    println!("Resuming function again...");
    sender.send(()).unwrap();

    handle.join().unwrap();
}
