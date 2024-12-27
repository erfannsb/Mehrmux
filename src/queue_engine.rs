use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::process_gen::{build_test_process, Process, ProcessStatus};

// Common Queueing Sense ---------------------------------------------------------------------------

trait  Queue {
    fn enqueue(&mut self,process: Process);
    fn dequeue(&mut self) -> Option<Process>;
}

// FIFO Algorithm ----------------------------------------------------------------------------------

struct FIFO {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
    context_switch_duration: Duration
}

impl FIFO {
    fn init() -> Self {
        Self {
            queue: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
            context_switch_duration: Duration::from_millis(10)
        }
    }

    fn run(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop {  // in this loop we process all processes until there is no process left
            if self.queue.is_empty() {
                break;
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
                        Ok(_) => self.current_process.as_mut().unwrap().status = ProcessStatus::Terminated,
                        Err(_) => {}
                    }
                }
                None => {}
            }

            self.current_time = time_passed.elapsed();
            println!("{:?}", self.current_time);
        }
    }
}

impl Queue for FIFO {
    fn enqueue(&mut self, mut process: Process) {
        process.status = ProcessStatus::Ready;
        self.queue.push(process)
    }

    fn dequeue(&mut self) -> Option<Process> {
        sleep(self.context_switch_duration); // context switch process ...
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}

// SPN Algorithm -----------------------------------------------------------------------------------
//Erfun

struct SPN {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl SPN {
}




// FCFS Algorithm ----------------------------------------------------------------------------------
// Meownoosh
struct FCFS {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl FCFS {
}

// SJF Algorithm -----------------------------------------------------------------------------------
// Erfun

struct SJF {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl SJF {
}



// HRRN Algorithm ----------------------------------------------------------------------------------
// Meownoosh

struct HRRN {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl HRRN {
}


// RR Algorithm ------------------------------------------------------------------------------------
// Meownoosh
struct RR {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl RR {
}

// SRF Algorithm -----------------------------------------------------------------------------------
// Erfun

struct SRF {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl SRF {
}

// MLQ Algorithm -----------------------------------------------------------------------------------
// Meownoosh

struct MLQ {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl MLQ {
}

// MLFQ Algorithm ----------------------------------------------------------------------------------
// Erfun
struct MLFQ {
    queue: Vec<Process>,
    current_process: Option<Process>,
    current_time: Duration,
}

impl MLFQ {
}


// Testing -----------------------------------------------------------------------------------------

pub fn test() {
    let mut fifo: FIFO = FIFO::init();
    let mut list_of_processes = vec![build_test_process(), build_test_process(), build_test_process()];
    list_of_processes.sort_by_key(|p| p.arrival_time);
    println!("{:?}", &list_of_processes);
    fifo.queue.extend(list_of_processes);
    fifo.run();
}