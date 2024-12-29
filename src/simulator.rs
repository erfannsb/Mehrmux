use std::any::Any;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand_distr::{Distribution, Exp};
use crate::cli::Queues;
use crate::process_gen::{build_test_process, Process};
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
}

struct Simulate {
    lambda_rate: f64,
    fifo: FIFO,
    fcfs: FCFS,
    spn: SPN,
    sjf: SJF,
    hrrn: HRRN,
    rr: RR,
    srf: SRF,
    mlq: MLQ,
    mlfq: MLFQ
}

impl Simulate {


    fn run_simulate(self, queue_type: Queues, num_of_processes: i32, sim_type: i32) {
        let mut test_processes: Vec<Process> = vec![];

        for i in 0..num_of_processes {
            test_processes.push(build_test_process());
        }

        // Wrap 'self' in Arc and Mutex
        let self_arc = Arc::new(Mutex::new(self));

        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = Arc::clone(&stop_flag);
        let stop_flag_clone_second = Arc::clone(&stop_flag);

        let handle = thread::spawn(move || {
            // Lock the mutex to access 'self' safely
            let mut self_locked = self_arc.lock().unwrap();

            match queue_type {
                Queues::FIFO => self_locked.fifo.start(stop_flag_clone),
                Queues::SPN => self_locked.spn.start(stop_flag_clone),
                Queues::FCFS => self_locked.fcfs.start(stop_flag_clone),
                Queues::SJF => self_locked.sjf.start(stop_flag_clone),
                Queues::HRRN => self_locked.hrrn.start(stop_flag_clone),
                Queues::RR => self_locked.rr.start(stop_flag_clone),
                Queues::SRF => self_locked.srf.start(stop_flag_clone),
                Queues::MLQ => self_locked.mlq.start(stop_flag_clone),
                Queues::MLFQ => self_locked.mlfq.start(stop_flag_clone),
            }
        });

        let handle2 = thread::spawn(move || {
            let right_now = Instant::now();
            let mut current_time = Duration::from_millis(0);
            let mut random_generated_numbers: Vec<(f64,f64)> = vec![];
            let mut self_locked = self_arc.lock().unwrap();

            // generating random arrival_times and cpu_burst_time
            for _ in 0..num_of_processes {
                let time_between_arrival = ExponentialGenerator::new(self_locked.lambda_rate).unwrap().generate();
                let cpu_burst_time = ExponentialGenerator::new(self_locked.lambda_rate).unwrap().generate();
                random_generated_numbers.push((time_between_arrival, cpu_burst_time));
            }

            loop {
                current_time += right_now.elapsed();

                if stop_flag_clone_second.load(Ordering::Relaxed) {
                    break
                }

                if current_time >= Duration::from_millis(random_generated_numbers.get(0).unwrap().0 as u64) {

                    let process = Process::new()

                    match queue_type {
                        Queues::FIFO => self_locked.fifo.enqueue(),
                        Queues::SPN => self_locked.spn.enqueue(stop_flag_clone),
                        Queues::FCFS => self_locked.fcfs.enqueue(stop_flag_clone),
                        Queues::SJF => self_locked.sjf.enqueue(stop_flag_clone),
                        Queues::HRRN => self_locked.hrrn.enqueue(stop_flag_clone),
                        Queues::RR => self_locked.rr.enqueue(stop_flag_clone),
                        Queues::SRF => self_locked.srf.enqueue(stop_flag_clone),
                        Queues::MLQ => self_locked.mlq.enqueue(stop_flag_clone),
                        Queues::MLFQ => self_locked.mlfq.enqueue(stop_flag_clone),
                    }
                }

            }
        });

        sleep(Duration::from_secs(sim_type as u64));
        stop_flag.store(true, Ordering::Relaxed); // Set the stop flag after 5 seconds

        handle.join().unwrap();
        handle2.join().unwrap();
    }

}

pub fn test() {
    let generator = ExponentialGenerator::new(2.0).expect("Invalid rate");

    // Generate 10 random numbers
    for _ in 0..10 {
        let value = generator.generate();
        println!("{:.4}", value);
    }
}