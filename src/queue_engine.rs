use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::process_gen::{build_test_process, Process, ProcessStatus};

// Common Queueing Sense ---------------------------------------------------------------------------

trait  Queue {
    fn enqueue(&mut self,process: Process);
    fn dequeue(&mut self) -> Option<Process>;
    fn start(&mut self);
    fn stop(&mut self);
    fn init() -> Self;
}

// Important Description ---------------------------------------------------------------------------
//All queue structs should have these properties:

// processes: vector of processes
//current_process: to keep track of the current process running
//current_time: to measure the time passed while running the queue algorithm
//context_switch_duration: arbitrary duration for hypothetical context switching process
//running: this variable is needed to stop or start running the queue!

//every queue should inherit from Queue trait and define its own enqueue and dequeue method

//every queue should implement start method in which there will be an infinite loop which never
//stops unless "running" boolean flag defined in the struct property is changed to false by stop
//method. the loop would do nothing unless there is at least one process in the processes vector!
//if processes vector isn't empty then the process with the right priority based on the algorithm
//would be chosen in the dequeue method. then the chosen process will be executed by calling
//the run method.


// FIFO Algorithm ----------------------------------------------------------------------------------

struct FIFO {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
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

    fn start(&mut self) {
        self.running = true;
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left

            if !self.running {
                break
            }

            match self.dequeue() {
                Some(mut process) => {

                    if process.arrival_time > time_passed.elapsed() {
                        sleep(process.arrival_time - time_passed.elapsed())
                    }

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

    fn stop(&mut self) {
        self.running = false;
    }

    fn init() -> Self {
        Self {
            processes: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_millis(10),
            running: false
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
    running: bool,
}

impl SPN {
}




// FCFS Algorithm ----------------------------------------------------------------------------------
// Meownoosh
struct FCFS {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
}

impl FCFS {
}

// SJF Algorithm -----------------------------------------------------------------------------------
// Erfun

struct SJF {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
}

impl SJF {
}



// HRRN Algorithm ----------------------------------------------------------------------------------
// Meownoosh

struct HRRN {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
}

impl HRRN {
}


// RR Algorithm ------------------------------------------------------------------------------------
// Meownoosh
struct RR {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
}

impl RR {
}

// SRF Algorithm -----------------------------------------------------------------------------------
// Erfun

struct SRF {
    processes: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration,
    running: bool,
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
    running: bool,
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
    running: bool,
}

impl MLFQ {
}


// Testing -----------------------------------------------------------------------------------------

pub fn test() {
    let mut fifo: FIFO = FIFO::init();
    let mut list_of_processes = vec![build_test_process(), build_test_process(), build_test_process()];
    list_of_processes.sort_by_key(|p| p.arrival_time);
    println!("{:?}", &list_of_processes);
    fifo.processes.extend(list_of_processes);
    fifo.start();
}