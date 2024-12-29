use dialoguer::{theme::ColorfulTheme, Select, Input, Validator};
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
    println!("Number Of Processes: {}\nSelected Queue: {}\nSimulation Time: {}", n_p, queue.unwrap(), sim_time);
    println!("--------------------------");
}

pub fn test() {
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
            "Enter Simulation Time (In Seconds)",
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
            Queues::FIFO,
            Queues::SPN,
            Queues::FCFS,
            Queues::SJF,
            Queues::HRRN,
            Queues::RR,
            Queues::SRF,
            Queues::MLQ,
            Queues::MLFQ,
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
                sim_time = Input::new()
                    .with_prompt("Enter Simulation Time (In Seconds)")
                    .report(false)
                    .interact_text()
                    .unwrap();
            }
            3 => {
                clear_console();
                let selected_queue = match queue {
                    0 => Some(Queues::FIFO),
                    1 => Some(Queues::SPN),
                    2 => Some(Queues::FCFS),
                    3 => Some(Queues::SJF),
                    4 => Some(Queues::HRRN),
                    5 => Some(Queues::RR),
                    6 => Some(Queues::SRF),
                    7 => Some(Queues::MLQ),
                    8 => Some(Queues::MLFQ),
                    _ => None
                };
                print_variable(n_p, sim_time, selected_queue);
            }
            4 => {}
            _ => println!("Invalid selection."),
        }
    }
}
