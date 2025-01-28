use crate::cli::Queues;
use crate::process_gen::{Process, ProcessType};
use crate::queue_engine::{QueueDiscipline, ReadyQueue, MLFQ, MLQ};
use rand_distr::{Distribution, Exp};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use tauri::{Emitter, Window};

// Utils -------------------------------------------------------------------------------------------
pub struct ExponentialGenerator {
    rate: f64,
}

impl ExponentialGenerator {
    pub fn new(rate: f64) -> Result<Self, &'static str> {
        if rate <= 0.0 {
            return Err("Rate parameter must be positive");
        }
        Ok(Self { rate })
    }

    pub fn generate(&self) -> Duration {
        loop {
            let exp = Exp::new(self.rate).unwrap();
            let mut rng = rand::thread_rng();
            let result = exp.sample(&mut rng);

            // Break the result into the integer part (seconds) and the fractional part (nanoseconds)
            let secs = result.floor() as u64; // Integer part as seconds
            let nanos = ((result - secs as f64) * 1e9).round() as u32; // Fractional part as nanoseconds

            if secs > 0 || nanos > 0 {
                return Duration::new(secs, nanos);
            }
        }
    }

    pub fn generate_accumulative(&self, size: usize) -> Vec<Duration> {
        let mut initial_value = Duration::new(0, 0); // Initialize the first arrival time
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
    pub fn generate_queue(
        queue_discipline: Queues,
        context_switch: Duration,
        time_quantum: Duration,
        list_of_disciplines: Option<[String; 4]>,
    ) -> Queue {
        //generating The Ready queue based on single or multi level
        let mut selected_disciplines: [QueueDiscipline; 4] = [QueueDiscipline::RR; 4]; // Initialize with default values (e.g., RR)

        if let Some(list) = list_of_disciplines {
            for (index, element) in list.iter().enumerate() {
                selected_disciplines[index] = match element.trim() {
                    "RR" => QueueDiscipline::RR,
                    "SJF" => QueueDiscipline::SJF,
                    "SRTF" => QueueDiscipline::SRF, // Fixed typo
                    "FCFS" => QueueDiscipline::FCFS,
                    "SPN" => QueueDiscipline::SPN,
                    "HRRN" => QueueDiscipline::HRRN,
                    _ => QueueDiscipline::RR, // Default case
                };
            }
        }

        let mut queue: Queue;
        if let Queues::MLFQ = queue_discipline {
            queue = Queue::MultiLevelFeedBack(MLFQ::init(
                selected_disciplines[0],
                selected_disciplines[1],
                selected_disciplines[2],
                selected_disciplines[3],
                context_switch,
                time_quantum,
            ));
        } else if let Queues::MLQ = queue_discipline {
            queue = Queue::MultiLevel(MLQ::init(
                selected_disciplines[0],
                selected_disciplines[1],
                selected_disciplines[2],
                selected_disciplines[3],
                context_switch,
                time_quantum,
            ));
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
                _ => QueueDiscipline::FCFS,
            };
            queue = Queue::ReadyQueue(ReadyQueue::new(
                queue_discipline,
                context_switch,
                time_quantum,
            ));
        }
        queue
    }

    fn enqueue_queue(q: &mut Queue, process: Process) {
        match q {
            Queue::ReadyQueue(ref mut R) => R.enqueue(process),
            Queue::MultiLevel(ref mut MLQ) => MLQ.enqueue(process),
            Queue::MultiLevelFeedBack(ref mut MLFQ) => MLFQ.enqueue(process),
        };
    }

    fn execute_queue(q: &mut Queue, window: &Window) {
        match q {
            Queue::ReadyQueue(ref mut R) => {
                R.execute_next(window, false);
            }
            Queue::MultiLevel(ref mut MLQ) => {
                MLQ.execute_next(window);
            }
            Queue::MultiLevelFeedBack(ref mut MLFQ) => {
                MLFQ.execute_next(window);
            }
        };
    }

    fn is_queue_empty(q: &Queue) -> bool {
        match q {
            Queue::ReadyQueue(R) => R.is_queue_empty(),
            Queue::MultiLevel(MLQ) => MLQ.is_queue_empty(),
            Queue::MultiLevelFeedBack(MLFQ) => MLFQ.is_queue_empty(),
        }
    }

    fn update_data(q: &mut Queue, window: &Window) {
        match q {
            Queue::ReadyQueue(ref mut R) => {
                let data = R.get_data();
                window.emit("update_process", data).unwrap();
            }
            _ => {}
        };
    }

    fn calc_q_metrics(q: &mut Queue, window: &Window) {
        match q {
            Queue::ReadyQueue(ref mut R) => {
                let data = R.calculate_metrics();
                window.emit("send_metrics", data).unwrap();
            }
            Queue::MultiLevel(ref mut MLQ) => {
                let data = MLQ.calculate_metric();
                window.emit("send_metrics_mlq", data).unwrap();
            }
            Queue::MultiLevelFeedBack(ref mut MLFQ) => {
                let data = MLFQ.calculate_metric();
                window.emit("send_metrics_mlfq", data).unwrap()
            }
        }
    }

    pub(crate) fn run_simulate(
        &mut self,
        num_of_processes: i32,
        queue_discipline: Queues,
        window: Window,
        context_switch: Duration,
        time_quantum: Duration,
        list_of_disciplines: Option<[String; 4]>,
    ) {
        let lambda_rate_arrival = self.lambda_rate_arrival.clone();
        let lambda_rate_cbt = self.lambda_rate_cbt.clone();

        thread::spawn(move || {
            // generating random numbers: --------------------------------------------------------------
            let exp_for_arrival = ExponentialGenerator::new(lambda_rate_arrival);
            let exp_for_cbt = ExponentialGenerator::new(lambda_rate_cbt);
            let this_time = SystemTime::now();
            let arrival_randoms = exp_for_arrival
                .unwrap()
                .generate_accumulative(num_of_processes as usize);
            let mut generated_random_numbers: Vec<(SystemTime, Duration)> =
                Vec::with_capacity(num_of_processes as usize);
            for element in arrival_randoms {
                let cpu_burst_time = exp_for_cbt.as_ref().unwrap().generate();
                let element = this_time + element;
                generated_random_numbers.push((element, cpu_burst_time));
            }

            println!("{:?}", generated_random_numbers);
            //generating queue: ------------------------------------------------------------------------
            let mut queue = Simulator::generate_queue(
                queue_discipline,
                context_switch,
                time_quantum,
                list_of_disciplines,
            );

            // running the simulation ------------------------------------------------------------------
            loop {
                let right_now = SystemTime::now();

                if generated_random_numbers.is_empty() && Simulator::is_queue_empty(&queue) {
                    Simulator::calc_q_metrics(&mut queue, &window);
                    break;
                }

                //check if the current time passed the first arrival time
                if (!generated_random_numbers.is_empty()) {
                    while right_now
                        >= generated_random_numbers
                            .get(0)
                            .unwrap_or(&(SystemTime::now(), Duration::from_secs(0)))
                            .0
                    {
                        if generated_random_numbers.is_empty() {
                            break;
                        }
                        {
                            let (at, cbt) = generated_random_numbers.remove(0); // remove the arrival time and cbt from the vec
                            let process = Process::new(cbt, at, None);
                            Simulator::enqueue_queue(&mut queue, process); // enqueue process at this point.
                        }
                    }
                }

                //check if queue isn't empty execute the next process:
                if !Simulator::is_queue_empty(&queue) {
                    Simulator::update_data(&mut queue, &window);
                    Simulator::execute_queue(&mut queue, &window);
                    Simulator::update_data(&mut queue, &window);
                }
            }
        });
    }

    pub(crate) fn run_with_predefined_processes(
        &mut self,
        queue_discipline: Queues,
        window: Window,
        context_switch: Duration,
        time_quantum: Duration,
        mut processes_to_be_generated: Vec<(u64, u64, Option<String>)>,
        list_of_disciplines: Option<[String; 4]>,
    ) {
        // process_to_be_generated is a vector of tuples (arrival_time, cbt)
        thread::spawn(move || {
            //generating queue: ------------------------------------------------------------------------
            let mut queue = Simulator::generate_queue(
                queue_discipline,
                context_switch,
                time_quantum,
                list_of_disciplines,
            );
            let this_time = SystemTime::now();
            let mut processes_to_be_generated: Vec<(SystemTime, Duration, Option<String>)> =
                processes_to_be_generated
                    .iter()
                    .map(|p| {
                        (
                            this_time + Duration::from_millis(p.0),
                            Duration::from_millis(p.1),
                            p.2.clone(),
                        )
                    })
                    .collect();

            // running the simulation ------------------------------------------------------------------
            loop {
                let right_now = SystemTime::now();

                if processes_to_be_generated.is_empty() && Simulator::is_queue_empty(&queue) {
                    Simulator::calc_q_metrics(&mut queue, &window);
                    break;
                }

                //check if the current time passed the first arrival time
                if (!processes_to_be_generated.is_empty()) {
                    while right_now
                        >= processes_to_be_generated
                            .get(0)
                            .unwrap_or(&(
                                SystemTime::now(),
                                Duration::from_secs(0),
                                Some(String::from("")),
                            ))
                            .0
                    {
                        if processes_to_be_generated.is_empty() {
                            break;
                        }
                        {
                            let (at, cbt, pt) = processes_to_be_generated.remove(0); // remove the arrival time and cbt from the vec
                            let selected_process_type: Option<ProcessType>;
                            if let Some(process_type) = pt {
                                selected_process_type = match process_type.trim() {
                                    "system" => Some(ProcessType::SystemProcess),
                                    "batch" => Some(ProcessType::BatchProcess),
                                    "interactive" => Some(ProcessType::InteractiveProcess),
                                    "student" => Some(ProcessType::StudentProcess),
                                    _ => None,
                                }
                            } else {
                                selected_process_type = None
                            }

                            let process = Process::new(cbt, at, selected_process_type);
                            Simulator::enqueue_queue(&mut queue, process); // enqueue process at this point.
                        }
                    }
                }

                //check if queue isn't empty execute the next process:
                if !Simulator::is_queue_empty(&queue) {
                    Simulator::update_data(&mut queue, &window);
                    Simulator::execute_queue(&mut queue, &window);
                    Simulator::update_data(&mut queue, &window);
                }
            }
        });
    }

    pub(crate) fn init(lambda_rate_arrival: f64, lambda_rate_cbt: f64) -> Self {
        Simulator {
            lambda_rate_arrival,
            lambda_rate_cbt,
        }
    }
}
