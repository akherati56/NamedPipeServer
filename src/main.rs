mod controller;
mod pipe_common;

fn main() {
    println!("[+] Starting Pipe Server (Controller)...");
    if let Err(e) = controller::run_controller() {
        eprintln!("Controller error: {}", e);
    }
}