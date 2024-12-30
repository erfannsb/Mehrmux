use std::env::current_exe;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand_distr::{Distribution, Exp};
use crate::cli::Queues;
use crate::process_gen::{build_test_process, Process, ProcessType};
use crate::queue_engine::{FCFS, FIFO, HRRN, MLFQ, MLQ, RR, SJF, SPN, SRF};

pub struct ExponentialGenerator {
    rate: f64, // Rate parameter (lambda) for the exponential distribution
}

impl ExponentialGenerator {
    pub fn new(rate: f64) -> Result<Self, &'static str> {
        if rate <= 0.0 {
            return Err("Rate parameter must be positive");
        }
        Ok(Self { rate })
    }

    pub fn generate(&self) -> f64 {
        let exp = Exp::new(self.rate).unwrap();
        let mut rng = rand::thread_rng();
        exp.sample(&mut rng)
    }

    pub fn generate_accumulative(&self, size: usize) -> Vec<f64> {
        let mut initial_value = 0.0;  // Initialize the first arrival time
        let mut result = Vec::with_capacity(size);

        for _ in 0..size {
            let value = self.generate();  // Generate next arrival time
            initial_value += value;  // Accumulate arrival times to ensure they are ascending
            result.push(initial_value);
        }

        result
    }
}

pub struct Simulator {
    lambda_rate_arrival: f64,
    lambda_rate_cbt: f64,
    fifo: FIFO,
    fcfs: FCFS,
    spn: SPN,
    sjf: SJF,
    hrrn: HRRN,
    rr: RR,
    srf: SRF,
    mlq: MLQ,
    mlfq: MLFQ,
}

impl Simulator {
    pub(crate) fn run_simulate(self, num_of_processes: i32, queue_type: Queues) {
        let lambda_rate_arrival = self.lambda_rate_arrival.clone();
        let lambda_rate_cbt = self.lambda_rate_cbt.clone();
        // Wrap 'self' in Arc and Mutex
        let self_arc = Arc::new(Mutex::new(self));

        // Wrap 'queue_type' in Arc
        let queue_type_arc = Arc::new(queue_type);

        // generating random numbers:
        let exp_for_arrival = ExponentialGenerator::new(lambda_rate_arrival);
        let exp_for_cbt = ExponentialGenerator::new(lambda_rate_cbt);
        let mut arrival_randoms = exp_for_arrival.unwrap().generate_accumulative(num_of_processes as usize);
        let mut generated_random_numbers: Vec<(f64, f64)> = Vec::with_capacity(num_of_processes as usize);
        for element in arrival_randoms {
            let cpu_burst_time = exp_for_cbt.as_ref().unwrap().generate();
            generated_random_numbers.push((element, cpu_burst_time));
        }

        let right_now = Instant::now();
        let mut current_time = Duration::from_millis(0);
        let mut self_locked = self_arc.lock().unwrap();
        println!("-------------------------------------------------");
        println!("Generating Random Processes");
        loop {
            if generated_random_numbers.is_empty() {
                break;
            }
            current_time = right_now.elapsed();
            // Check if it's time to process the next process
            if current_time >= Duration::from_millis(generated_random_numbers.get(0).unwrap().0 as u64) {
                let random_numbers = generated_random_numbers.remove(0);
                let process = Process::new(Duration::from_millis(random_numbers.1 as u64));
                println!("🔻 Process Entered The Queue: id: {}, at: {:?}", &process.clone().id.to_string()[0..7].to_string(), current_time);
                // Process according to the queue type
                match *queue_type_arc {
                    Queues::FIFO => self_locked.fifo.enqueue(process),
                    Queues::SPN => self_locked.spn.enqueue(process),
                    Queues::FCFS => self_locked.fcfs.enqueue(process),
                    Queues::SJF => self_locked.sjf.enqueue(process),
                    Queues::HRRN => self_locked.hrrn.enqueue(process),
                    Queues::RR => self_locked.rr.enqueue(process),
                    Queues::SRF => self_locked.srf.enqueue(process),
                    Queues::MLQ => self_locked.mlq.enqueue(process),
                    Queues::MLFQ => self_locked.mlfq.enqueue(process),
                }
            }
        }

        println!("-------------------------------------------------");
        println!("Starting The Queue");

        match *queue_type_arc {
            Queues::FIFO => {
                self_locked.fifo.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                 self_locked.fifo.metrics.TotalWaitingTime / num_of_processes as u32,
                 self_locked.fifo.metrics.TotalTime / num_of_processes as u32,
                 self_locked.fifo.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::SPN => {
                self_locked.spn.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.spn.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.spn.metrics.TotalTime / num_of_processes as u32,
                         self_locked.spn.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::FCFS => {
                self_locked.fcfs.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.fcfs.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.fcfs.metrics.TotalTime / num_of_processes as u32,
                         self_locked.fcfs.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::SJF => {
                self_locked.sjf.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.sjf.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.sjf.metrics.TotalTime / num_of_processes as u32,
                         self_locked.sjf.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::HRRN => {
                self_locked.hrrn.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.hrrn.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.hrrn.metrics.TotalTime / num_of_processes as u32,
                         self_locked.hrrn.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::RR => {
                self_locked.rr.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.rr.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.rr.metrics.TotalTime / num_of_processes as u32,
                         self_locked.rr.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::SRF => {
                self_locked.srf.start();
                println!("Metrics:\nAverage Waiting Time: {:?}, Average Total Time: {:?}, Average Response Time: {:?}",
                         self_locked.srf.metrics.TotalWaitingTime / num_of_processes as u32,
                         self_locked.srf.metrics.TotalTime / num_of_processes as u32,
                         self_locked.srf.metrics.ResponseTime / num_of_processes as u32,
                )
            },
            Queues::MLQ => {
                self_locked.mlq.start();
            },
            Queues::MLFQ =>{
                self_locked.mlfq.start();
            },
        }

    }

    pub(crate) fn init(lambda_rate_arrival: f64, lambda_rate_cbt: f64) -> Self {
        Simulator {
            lambda_rate_arrival,
            lambda_rate_cbt,
            fifo: FIFO::init(),
            fcfs: FCFS::init(),
            spn: SPN::init(),
            sjf: SJF::init(),
            hrrn: HRRN::init(),
            rr: RR::init(),
            srf: SRF::init(),
            mlq: MLQ::init(),
            mlfq: MLFQ::init(),
        }
    }
}



pub fn test2() {
    // Shared variable wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Clone the Arc to pass to threads
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    // Thread 1 - runs forever and modifies the counter
    let handle1 = thread::spawn(move || {
        loop {
            let mut num = counter1.lock().unwrap();
            *num += 1;
            println!("Thread 1: Counter = {}", *num);
            thread::sleep(Duration::from_secs(1)); // Simulate some work
        }
    });

    // Thread 2 - runs forever and reads the counter
    let handle2 = thread::spawn(move || {
        loop {
            let num = counter2.lock().unwrap();
            println!("Thread 2: Counter = {}", *num);
            thread::sleep(Duration::from_secs(1)); // Simulate some work
        }
    });

    // Wait for both threads to finish (they never will in this case)
    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn test() {
    let sim = Simulator::init(0.01, 0.001);
    sim.run_simulate(10, Queues::SPN);
}