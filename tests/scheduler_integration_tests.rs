use datadog_agent::collector::scheduler::{Scheduler, EXECUTE_CHECK_COUNT};
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[test]
fn scheduler_initialization() {
    let scheduler = Scheduler::new();
    assert!(scheduler.is_job_queues_empty(), "Scheduler job_queues should be initialized as empty");
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
    thread::sleep(Duration::from_millis(350)); // Allow time for checks to be executed
    assert!(EXECUTE_CHECK_COUNT > 0, "Checks should have been executed");
}

#[test]
fn scheduler_stops_correctly() {
    let mut scheduler = Scheduler::new();
    let _ = scheduler.add_check("test_check".to_string(), Duration::from_millis(100));
    thread::sleep(Duration::from_millis(350)); // Allow time for checks to be executed
    let count_before_stop = EXECUTE_CHECK_COUNT;
    scheduler.stop_check("test_check");
    thread::sleep(Duration::from_millis(200)); // Allow more time to confirm no checks are executed
    assert_eq!(EXECUTE_CHECK_COUNT, count_before_stop, "No further checks should be executed after stopping the scheduler");
}

#[test]
fn telemetry_and_statistics_tracking() {
    let mut scheduler = Scheduler::new();
    let _ = scheduler.add_check("test_check".to_string(), Duration::from_secs(1));
    thread::sleep(Duration::from_secs(2)); // Allow time for checks to be executed more than once
    assert!(EXECUTE_CHECK_COUNT > 1, "Multiple executions of 'test_check' should be tracked in statistics");
}

#[test]
fn concurrent_check_addition() {
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    let scheduler_clone = Arc::clone(&scheduler);
    let handle = thread::spawn(move || {
        let mut scheduler = scheduler_clone.lock().unwrap();
        scheduler.add_check("test_check_1".to_string(), Duration::from_millis(100));
    });

    let mut scheduler = scheduler.lock().unwrap();
    scheduler.add_check("test_check_2".to_string(), Duration::from_millis(100));
    handle.join().unwrap();

    assert!(scheduler.has_ticker("test_check_1"), "Check test_check_1 should be added to the scheduler");
    assert!(scheduler.has_ticker("test_check_2"), "Check test_check_2 should be added to the scheduler");
}

#[test]
fn concurrent_check_execution() {
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    let scheduler_clone = Arc::clone(&scheduler);
    let execute_count = Arc::new(AtomicUsize::new(0));
    let execute_count_clone = Arc::clone(&execute_count);

    let handle = thread::spawn(move || {
        let mut scheduler = scheduler_clone.lock().unwrap();
        scheduler.add_check("test_check_1".to_string(), Duration::from_millis(100));
        drop(scheduler); // Explicitly drop the lock before sleeping
        thread::sleep(Duration::from_millis(500)); // Allow time for checks to be executed
        execute_count_clone.fetch_add(EXECUTE_CHECK_COUNT as usize, Ordering::SeqCst);
    });

    let mut scheduler = scheduler.lock().unwrap();
    scheduler.add_check("test_check_2".to_string(), Duration::from_millis(100));
    drop(scheduler); // Explicitly drop the lock before sleeping
    thread::sleep(Duration::from_millis(500)); // Allow time for checks to be executed
    execute_count.fetch_add(EXECUTE_CHECK_COUNT as usize, Ordering::SeqCst);

    handle.join().unwrap();

    assert!(execute_count.load(Ordering::SeqCst) > 0, "Checks should have been executed concurrently");
}
