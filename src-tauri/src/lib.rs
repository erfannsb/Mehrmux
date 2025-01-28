use std::time::Duration;
use crate::cli::Queues;
use tauri::Window;

mod cli;
mod process_gen;
mod queue_engine;
mod simulator;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_simulation(
    window: Window,
    at_lambda: f64,
    cbt_lambda: f64,
    num_of_prcss: i32,
    queue: &str,
    context_switch: u64,
    time_quantum: u64,
    list_of_discipline: Option<[String; 4]>
) {
    let time_quantum = Duration::from_millis(time_quantum);
    let context_switch = Duration::from_millis(context_switch);
    let mut sim = simulator::Simulator::init(at_lambda, cbt_lambda);
    let queue = match queue {
        "FCFS" => Queues::FCFS,
        "FIFO" => Queues::FIFO,
        "SPN" => Queues::SPN,
        "SJF" => Queues::SJF,
        "HRRN" => Queues::HRRN,
        "RR" => Queues::RR,
        "SRTF" => Queues::SRF,
        "MLQ" => Queues::MLQ,
        "MLFQ" => Queues::MLFQ,
        _ => Queues::FIFO
    };
    sim.run_simulate(num_of_prcss, queue, window, context_switch, time_quantum, list_of_discipline);
}

#[tauri::command]
fn run_with_parameters(
    window: Window,
    array_of_processes: Vec<(u64, u64, Option<String>)>,
    queue: &str,
    context_switch: u64,
    time_quantum: u64,
    list_of_discipline: Option<[String; 4]>,
) {
    let time_quantum = Duration::from_millis(time_quantum);
    let context_switch = Duration::from_millis(context_switch);
    println!("time_quantum: {:?}, contextsiwthc: {:?}", time_quantum, context_switch);
    let mut sim = simulator::Simulator::init(1.0,1.0);
    let queue = match queue {
        "FCFS" => Queues::FCFS,
        "FIFO" => Queues::FIFO,
        "SPN" => Queues::SPN,
        "SJF" => Queues::SJF,
        "HRRN" => Queues::HRRN,
        "RR" => Queues::RR,
        "SRTF" => Queues::SRF,
        "MLQ" => Queues::MLQ,
        "MLFQ" => Queues::MLFQ,
        _ => Queues::FIFO
    };
    sim.run_with_predefined_processes(queue, window, context_switch, time_quantum, array_of_processes, list_of_discipline)
}

#[tauri::command]
fn on_exit(window: Window) {
    window.close().unwrap();
}

#[tauri::command]
fn on_max(window: Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn on_min(window: Window) {
    window.minimize().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            run_simulation,
            run_with_parameters,
            on_exit,
            on_max,
            on_min
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}