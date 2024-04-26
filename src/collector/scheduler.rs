use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use lazy_static::lazy_static;

pub static mut EXECUTE_CHECK_COUNT: u32 = 0;

fn execute_check(_check_name: &str) {
    println!("execute_check called for {}", _check_name); // Added for debugging
    unsafe {
        EXECUTE_CHECK_COUNT += 1;
        println!("EXECUTE_CHECK_COUNT incremented to {}", EXECUTE_CHECK_COUNT); // Added for debugging
    }
}

pub struct JobQueue {
    checks: Vec<String>,
    executed_checks_count: AtomicUsize,
}

impl JobQueue {
    pub fn new() -> JobQueue {
        JobQueue {
            checks: Vec::new(),
            executed_checks_count: AtomicUsize::new(0),
        }
    }

    pub fn add_check(&mut self, check_name: String) {
        self.checks.push(check_name);
    }

    pub fn remove_check(&mut self, check_name: &str) {
        self.checks.retain(|check| check != check_name);
    }

    pub fn execute_checks(&self) {
        for check_name in &self.checks {
            execute_check(check_name);
            self.executed_checks_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn get_stats(&self) -> usize {
        self.executed_checks_count.load(Ordering::SeqCst)
    }
}

lazy_static! {
    static ref ONCE_CHECK_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct Scheduler {
    job_queues: HashMap<Duration, JobQueue>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            job_queues: HashMap::new(),
        }
    }

    pub fn add_check(&mut self, check_name: String, interval: Duration) -> JoinHandle<()> {
        let job_queue = self.job_queues.entry(interval).or_insert_with(|| JobQueue::new());
        job_queue.add_check(check_name.clone());
        let handle = thread::spawn(move || {
            // Logic to execute the check
            execute_check(&check_name);
        });
        handle
    }

    pub fn has_ticker(&self, check_name: &str) -> bool {
        self.job_queues.values().any(|job_queue| job_queue.checks.contains(&check_name.to_string()))
    }

    pub fn stop_check(&mut self, check_name: &str) {
        for job_queue in self.job_queues.values_mut() {
            job_queue.remove_check(check_name);
        }
    }

    pub fn enqueue_once(&self, check_name: String, check_logic: fn(&str)) -> JoinHandle<()> {
        let _guard = ONCE_CHECK_MUTEX.lock().unwrap();
        let (tx, rx) = mpsc::channel();
        let _ = tx.send(()); // Immediately trigger the check execution
        thread::spawn(move || {
            match rx.recv() {
                Ok(_) => {
                    check_logic(&check_name);
                }
                Err(_) => {
                    println!("Failed to receive from channel for one-time execution");
                }
            }
        })
    }

    pub fn get_scheduler_stats(&self) -> HashMap<Duration, usize> {
        let mut stats = HashMap::new();
        for (&interval, job_queue) in &self.job_queues {
            stats.insert(interval, job_queue.get_stats());
        }
        stats
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
        assert!(scheduler.job_queues.is_empty(), "Scheduler job_queues should be initialized as empty");
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
