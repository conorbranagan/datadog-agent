use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

// Placeholder function representing the execution of a check
fn execute_check(check_name: &str) {
    println!("Executing check: {}", check_name);
    // Here the actual check execution logic will be implemented
}

pub struct Scheduler {
    // This struct will hold the state and functionality for the scheduler
    tickers: HashMap<String, (Duration, Sender<()>)>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        // Initialization of the scheduler
        Scheduler {
            tickers: HashMap::new(),
        }
    }

    pub fn add_check(&mut self, check_name: String, interval: Duration) {
        // Method to add a new check to the scheduler
        let (tx, rx) = mpsc::channel();
        self.tickers.insert(check_name.clone(), (interval, tx));
        self.start_ticker(check_name, rx, interval);
    }

    fn start_ticker(&self, check_name: String, rx: Receiver<()>, interval: Duration) {
        // Method to start a ticker for a check
        thread::spawn(move || {
            let mut next_tick = Instant::now();
            loop {
                next_tick += interval;
                let now = Instant::now();
                if let Ok(_) = rx.try_recv() {
                    // Stop signal received
                    break;
                }
                if now >= next_tick {
                    // Time to run the check
                    execute_check(&check_name);
                    next_tick = now;
                }
                thread::sleep(next_tick - now);
            }
        });
    }

    pub fn stop_check(&mut self, check_name: &str) {
        // Method to stop a ticker and remove a check from the scheduler
        if let Some((_, tx)) = self.tickers.remove(check_name) {
            let _ = tx.send(()); // Send stop signal to the ticker thread
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn scheduler_initialization() {
        let scheduler = Scheduler::new();
        assert!(scheduler.tickers.is_empty(), "Scheduler tickers should be initialized as empty");
    }

    #[test]
    fn add_and_stop_check() {
        let mut scheduler = Scheduler::new();
        scheduler.add_check("test_check".to_string(), Duration::from_secs(1));
        assert!(scheduler.tickers.contains_key("test_check"), "Check should be added to the scheduler");

        scheduler.stop_check("test_check");
        assert!(!scheduler.tickers.contains_key("test_check"), "Check should be removed from the scheduler");
    }
}
