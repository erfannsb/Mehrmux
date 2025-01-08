use std::env::current_exe;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::{sleep, spawn, Thread};
use std::time::{Duration, Instant};
use rand_distr::{Distribution, Exp};
use crate::cli::Queues;
use crate::process_gen::{build_test_process, Process, ProcessType};

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
        loop {
            let exp = Exp::new(self.rate).unwrap();
            let mut rng = rand::thread_rng();
            let result = exp.sample(&mut rng);
            if result != 0.0 {  // ensure that the generated value isn't 0
                return result
            }
        }
    }

    pub fn generate_accumulative(&self, size: usize) -> Vec<f64> {
        let mut initial_value = 0.0;  // Initialize the first arrival time
        let mut result = Vec::with_capacity(size);

        for _ in 0..size {
            let value = self.generate();
            initial_value += value;  // Accumulate values to ensure they are ascending
            result.push(initial_value);
        }

        result
    }
}

pub struct Simulator {
    lambda_rate_arrival: f64,
    lambda_rate_cbt: f64,
}

impl Simulator {
    pub(crate) fn run_simulate(self, num_of_processes: i32, queue_discipline: Queues) {
        let lambda_rate_arrival = self.lambda_rate_arrival.clone();
        let lambda_rate_cbt = self.lambda_rate_cbt.clone();

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
        //
        // loop {
        //     if generated_random_numbers.is_empty() {
        //         break;
        //     }
        //
        // }

        println!("bro what the {:?}", generated_random_numbers);

    }

    pub(crate) fn init(lambda_rate_arrival: f64, lambda_rate_cbt: f64) -> Self {
        Simulator {
            lambda_rate_arrival,
            lambda_rate_cbt,
        }
    }
}


pub fn test() {
    let sim = Simulator::init(0.01, 0.001);
    sim.run_simulate(10, Queues::SPN);
}