use crate::simulator::Simulator;
use dialoguer::{theme::ColorfulTheme, Input, Select, Validator};
use std::fmt;
use std::io::Write;

pub enum Queues {
    FIFO,
    SPN,
    FCFS,
    SJF,
    HRRN,
    RR,
    SRF,
    MLQ,
    MLFQ,
}

impl fmt::Display for Queues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Queues::FIFO => write!(f, "FIFO"),
            Queues::SPN => write!(f, "SPN"),
            Queues::FCFS => write!(f, "FCFS"),
            Queues::SJF => write!(f, "SJF"),
            Queues::HRRN => write!(f, "HRRN"),
            Queues::RR => write!(f, "RR"),
            Queues::SRF => write!(f, "SRF"),
            Queues::MLQ => write!(f, "MLQ"),
            Queues::MLFQ => write!(f, "MLFQ"),
        }
    }
}

fn clear_console() {
    // ANSI escape sequence to clear screen
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
}

fn print_variable(n_p: i32, sim_time: i32, queue: Option<Queues>) {
    println!("💻 Operating System Queueing Simulation");
    println!("--- Variables Selected ---");
    println!(
        "Number Of Processes: {}\nSelected Queue: {}\nSimulation Time: {}",
        n_p,
        queue.unwrap(),
        sim_time
    );
    println!("--------------------------");
}

pub fn run() {
    clear_console();
    println!("💻 Operating System Queueing Simulation");
    let mut n_p: i32 = 0;
    let mut queue: usize = 0;
    let mut sim_time: i32 = 0;

    loop {
        // Display a menu of options
        let options = vec![
            "Enter Number Of Processes",
            "Select Queuing Algorithm",
            "Run The Simulation",
            "Exit",
        ];

        // Show the menu
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        let queue_selection_options = vec![
            "FIFO", "SPN", "FCFS", "SJF", "HRRN", "RR", "SRF", "MLQ", "MLFQ",
        ];

        match selection {
            0 => {
                n_p = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter Number Of Processes")
                    .report(false)
                    .interact_text()
                    .unwrap();
            }
            1 => {
                queue = Select::with_theme(&ColorfulTheme::default())
                    .items(&queue_selection_options)
                    .report(true)
                    .default(0)
                    .interact()
                    .unwrap();
            }
            2 => {
                clear_console();
                let selected_queue = match queue {
                    0 => Queues::FIFO,
                    1 => Queues::SPN,
                    2 => Queues::FCFS,
                    3 => Queues::SJF,
                    4 => Queues::HRRN,
                    5 => Queues::RR,
                    6 => Queues::SRF,
                    7 => Queues::MLQ,
                    8 => Queues::MLFQ,
                    _ => Queues::FIFO,
                };
                let sim = Simulator::init(0.01, 0.001);
                if n_p <= 0 {
                    println!("Wrong Number Of Processes Try Again");
                    continue;
                }
                // sim.run_simulate(n_p, selected_queue);
            }
            3 => return,
            _ => println!("Invalid selection."),
        }
    }
}
