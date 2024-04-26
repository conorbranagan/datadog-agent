use datadog_agent::collector::scheduler::{Scheduler, EXECUTE_CHECK_COUNT};
use std::time::Duration;
use std::thread;

#[test]
fn scheduler_initialization() {
    let scheduler = Scheduler::new();
    assert!(scheduler.get_ticker_count() == 0, "Scheduler tickers should be initialized as empty");
}

#[test]
fn add_and_stop_check() {
    let mut scheduler = Scheduler::new();
    let result = scheduler.add_check("test_check".to_string(), Duration::from_secs(1));
    assert!(result.is_ok(), "Check with valid interval should be added");
    assert!(scheduler.has_ticker("test_check"), "Check should be added to the scheduler");

    scheduler.stop_check("test_check");
    assert!(!scheduler.has_ticker("test_check"), "Check should be removed from the scheduler");
}

#[test]
fn add_check_with_invalid_interval() {
    let mut scheduler = Scheduler::new();
    let result = scheduler.add_check("test_check".to_string(), Duration::from_secs(0)); // Zero interval is invalid
    assert!(result.is_err(), "Adding check with zero interval should return an error");
}

#[test]
fn add_check_with_valid_interval() {
    let mut scheduler = Scheduler::new();
    let result = scheduler.add_check("test_check".to_string(), Duration::from_secs(5));
    assert!(result.is_ok(), "Adding check with valid interval should be successful");
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
    unsafe {
        assert!(EXECUTE_CHECK_COUNT > 0, "Checks should have been executed");
    }
}

#[test]
fn scheduler_stops_correctly() {
    let mut scheduler = Scheduler::new();
    let _ = scheduler.add_check("test_check".to_string(), Duration::from_millis(100));
    thread::sleep(Duration::from_millis(350)); // Allow time for checks to be executed
    let count_before_stop;
    unsafe {
        count_before_stop = EXECUTE_CHECK_COUNT;
    }
    scheduler.stop_check("test_check");
    thread::sleep(Duration::from_millis(200)); // Allow more time to confirm no checks are executed
    unsafe {
        assert_eq!(EXECUTE_CHECK_COUNT, count_before_stop, "No further checks should be executed after stopping the scheduler");
    }
}
