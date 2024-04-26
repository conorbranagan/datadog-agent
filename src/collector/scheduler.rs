use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::thread::JoinHandle;

pub static mut EXECUTE_CHECK_COUNT: u32 = 0;

fn execute_check(_check_name: &str) {
    println!("execute_check called for {}", _check_name); // Added for debugging
    unsafe {
        EXECUTE_CHECK_COUNT += 1;
        println!("EXECUTE_CHECK_COUNT incremented to {}", EXECUTE_CHECK_COUNT); // Added for debugging
    }
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

    pub fn add_check(&mut self, check_name: String, interval: Duration) -> JoinHandle<()> {
        // Method to add a new check to the scheduler
        if interval == Duration::from_secs(0) {
            panic!("Interval cannot be zero");
        }
        let (tx, rx) = mpsc::channel();
        self.tickers.insert(check_name.clone(), (interval, tx));
        self.start_ticker(check_name, rx, interval)
    }

    fn start_ticker(&self, check_name: String, rx: Receiver<()>, interval: Duration) -> JoinHandle<()> {
        // Method to start a ticker for a check
        thread::spawn(move || {
            let mut next_tick = Instant::now();
            loop {
                match rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        println!("Stop signal received, breaking loop...");
                        break;
                    },
                    Err(mpsc::TryRecvError::Empty) => {
                        let now = Instant::now();
                        if now >= next_tick {
                            execute_check(&check_name);
                            next_tick = Instant::now() + interval;
                        }
                        let sleep_duration = if now < next_tick { next_tick - now } else { Duration::from_millis(0) };
                        thread::sleep(sleep_duration);
                    }
                }
            }
        })
    }

    pub fn stop_check(&mut self, check_name: &str) {
        // Method to stop a ticker and remove a check from the scheduler
        if let Some((_, tx)) = self.tickers.remove(check_name) {
            let _ = tx.send(()); // Send stop signal to the ticker thread
        }
    }

    pub fn has_ticker(&self, check_name: &str) -> bool {
        self.tickers.contains_key(check_name)
    }

    pub fn get_ticker_count(&self) -> usize {
        self.tickers.len()
    }
}

// Mock check for testing purposes
pub struct MockCheck {
    pub name: String,
    pub interval: Duration,
}

impl MockCheck {
    pub fn new(name: String, interval: Duration) -> MockCheck {
        MockCheck { name, interval }
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
        let _ = scheduler.add_check("test_check".to_string(), Duration::from_secs(1));
        assert!(scheduler.has_ticker("test_check"), "Check should be added to the scheduler");

        scheduler.stop_check("test_check");
        assert!(!scheduler.has_ticker("test_check"), "Check should be removed from the scheduler");
    }

    #[test]
    #[should_panic(expected = "Interval cannot be zero")]
    fn add_check_with_invalid_interval() {
        let mut scheduler = Scheduler::new();
        let _ = scheduler.add_check("test_check".to_string(), Duration::from_secs(0)); // Zero interval is invalid
    }

    #[test]
    fn add_check_with_valid_interval() {
        let mut scheduler = Scheduler::new();
        let _ = scheduler.add_check("test_check".to_string(), Duration::from_secs(5));
        assert!(scheduler.has_ticker("test_check"), "Check should be added to the scheduler");
    }

    #[test]
    fn cancel_scheduled_check() {
        let mut scheduler = Scheduler::new();
        let _ = scheduler.add_check("test_check".to_string(), Duration::from_secs(5));
        scheduler.stop_check("test_check");
        assert!(!scheduler.has_ticker("test_check"), "Scheduled check should be canceled");
    }

    #[test]
    fn scheduler_starts_and_runs_checks() {
        let mut scheduler = Scheduler::new();
        let _ = scheduler.add_check("test_check".to_string(), Duration::from_millis(100));
        unsafe {
            println!("EXECUTE_CHECK_COUNT before sleep: {}", EXECUTE_CHECK_COUNT); // Added for debugging
        }
        thread::sleep(Duration::from_millis(350)); // Allow time for checks to be executed
        unsafe {
            println!("EXECUTE_CHECK_COUNT after sleep: {}", EXECUTE_CHECK_COUNT); // Added for debugging
            assert!(EXECUTE_CHECK_COUNT > 0, "Checks should have been executed");
        }
    }

    #[test]
    fn scheduler_stops_correctly() {
        let mut scheduler = Scheduler::new();
        let handle = scheduler.add_check("test_check".to_string(), Duration::from_millis(100));
        thread::sleep(Duration::from_millis(350)); // Allow time for checks to be executed
        scheduler.stop_check("test_check");
        handle.join().unwrap(); // Ensure the ticker thread has finished
        let count_before_stop;
        unsafe {
            count_before_stop = EXECUTE_CHECK_COUNT;
        }
        thread::sleep(Duration::from_millis(200)); // Allow more time to confirm no checks are executed
        unsafe {
            assert_eq!(EXECUTE_CHECK_COUNT, count_before_stop, "No further checks should be executed after stopping the scheduler");
        }
    }
}
