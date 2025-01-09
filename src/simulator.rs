use crate::cli::Queues;
use crate::process_gen::{Process};
use rand_distr::{Distribution, Exp};
use std::time::{Duration, Instant};
use crate::queue_engine::{QueueDiscipline, ReadyQueue, MLFQ, MLQ};

// Utils -------------------------------------------------------------------------------------------
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
            if result != 0.0 {
                // ensure that the generated value isn't 0
                return result;
            }
        }
    }

    pub fn generate_accumulative(&self, size: usize) -> Vec<f64> {
        let mut initial_value = 0.0; // Initialize the first arrival time
        let mut result = Vec::with_capacity(size);

        for _ in 0..size {
            let value = self.generate();
            initial_value += value; // Accumulate values to ensure they are ascending
            result.push(initial_value);
        }

        result
    }
}

enum Queue {
    // since there are multiple queue types rust prevent us to use a single value to store
    // ready queue, mlq, mlfq in single variable since they are different structs.
    // therefore using a multi type Queue enum is necessary
    ReadyQueue(ReadyQueue),
    MultiLevel(MLQ),
    MultiLevelFeedBack(MLFQ),
}

// Simulation: -------------------------------------------------------------------------------------
pub struct Simulator {
    lambda_rate_arrival: f64,
    lambda_rate_cbt: f64,
}

impl Simulator {
    pub fn generate_queue(queue_discipline: Queues) -> Queue {
        //generating The Ready queue based on single or multi level
        let mut queue: Queue;
        if let Queues::MLFQ = queue_discipline {
            queue = Queue::MultiLevelFeedBack(MLFQ::init(QueueDiscipline::RR, QueueDiscipline::RR, QueueDiscipline::FCFS));
        } else if let Queues::MLQ = queue_discipline {
            queue = Queue::MultiLevel(MLQ::init(QueueDiscipline::RR, QueueDiscipline::RR, QueueDiscipline::FCFS));
        } else {
            //convert queue type to queues_discipline
            let queue_discipline = match queue_discipline {
                Queues::FIFO => QueueDiscipline::FIFO,
                Queues::FCFS => QueueDiscipline::FCFS,
                Queues::SPN => QueueDiscipline::SPN,
                Queues::SJF => QueueDiscipline::SJF,
                Queues::HRRN => QueueDiscipline::HRRN,
                Queues::RR => QueueDiscipline::RR,
                Queues::SRF => QueueDiscipline::SRF,
                _ => QueueDiscipline::FCFS
            };
            queue = Queue::ReadyQueue(ReadyQueue::new(queue_discipline));
        }
        queue
    }

    fn enqueue_queue(q: &mut Queue, process: Process) {
        match q {
            Queue::ReadyQueue(ref mut R) => R.enqueue(process),
            Queue::MultiLevel(ref mut MLQ) => MLQ.enqueue(process),
            Queue::MultiLevelFeedBack(ref mut MLFQ) => MLFQ.enqueue(process)
        };
    }

    fn execute_queue(q: &mut Queue) {
        match q {
            Queue::ReadyQueue(ref mut R) => {R.execute_next();},
            Queue::MultiLevel(ref mut MLQ) => {MLQ.execute_next();},
            Queue::MultiLevelFeedBack(ref mut MLFQ) => {MLFQ.execute_next();}
        };
    }

    fn is_queue_empty(q: &Queue) -> bool {
        match q {
            Queue::ReadyQueue(R) => R.is_queue_empty(),
            Queue::MultiLevel(MLQ) => MLQ.is_queue_empty(),
            Queue::MultiLevelFeedBack(MLFQ) => MLFQ.is_queue_empty()
        }
    }

    pub(crate) fn run_simulate(&mut self, num_of_processes: i32, queue_discipline: Queues) {
        let lambda_rate_arrival = self.lambda_rate_arrival.clone();
        let lambda_rate_cbt = self.lambda_rate_cbt.clone();

        // generating random numbers: --------------------------------------------------------------
        let exp_for_arrival = ExponentialGenerator::new(lambda_rate_arrival);
        let exp_for_cbt = ExponentialGenerator::new(lambda_rate_cbt);
        let mut arrival_randoms = exp_for_arrival
            .unwrap()
            .generate_accumulative(num_of_processes as usize);
        let mut generated_random_numbers: Vec<(f64, f64)> =
            Vec::with_capacity(num_of_processes as usize);
        for element in arrival_randoms {
            let cpu_burst_time = exp_for_cbt.as_ref().unwrap().generate();
            generated_random_numbers.push((element, cpu_burst_time));
        }

        //generating queue: ------------------------------------------------------------------------
        let mut queue = Simulator::generate_queue(queue_discipline);

        // running the simulation ------------------------------------------------------------------
        let right_now = Instant::now();
        loop {
            if generated_random_numbers.is_empty() {
                break;
            }
            let current_time = right_now.elapsed();

            //check if the current time passed the first arrival time
            if current_time >= Duration::from_millis(generated_random_numbers.get(0).unwrap().0 as u64) {
                let generated_couple = generated_random_numbers.remove(0); // remove the arrival time and cbt from the vec
                let process = Process::new(Duration::from_millis(generated_couple.1 as u64));
                Simulator::enqueue_queue(&mut queue, process); // enqueue process at this point.
            }

            //check if queue isn't empty execute the next process:
            if !Simulator::is_queue_empty(&queue) {
                Simulator::execute_queue(&mut queue)
            }
        }

        println!("bro what the {:?}", generated_random_numbers);
    }

    pub(crate) fn init(lambda_rate_arrival: f64, lambda_rate_cbt: f64) -> Self {
        Simulator {
            lambda_rate_arrival,
            lambda_rate_cbt,
        }
    }
}
