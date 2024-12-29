use rand_distr::{Distribution, Exp};
use crate::process_gen::Process;
use crate::cli::Queues;

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

struct Simulate {}

impl Simulate {
    fn run_simulate(queue_type: Queues, num_of_processes: i32, sim_type: i32) {

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