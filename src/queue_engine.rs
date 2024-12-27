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
}

impl FIFO {
    fn init() -> Self {
        Self {
            queue: vec![] ,
            current_process: None,
            current_time: Duration::from_secs(0),
        }
    }

    fn run(&mut self) {
        self.current_time = Duration::from_millis(0);
        let time_passed = Instant::now();
        loop { // in this loop we process all processes until there is no process left
            self.current_time = time_passed.elapsed();

            if self.queue.is_empty() {
                break;
            }

            match self.dequeue() {
                Some(mut process) => {
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
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}

// SPN Algorithm -----------------------------------------------------------------------------------

struct SPN {

}

pub fn test() {
    let mut fifo: FIFO = FIFO::init();
    let list_of_processes = vec![build_test_process(), build_test_process(), build_test_process()];
    println!("{:?}", &list_of_processes);
    fifo.queue.extend(list_of_processes);
    fifo.run();
}