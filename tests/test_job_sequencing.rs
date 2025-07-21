#![allow(unused_imports)]
use code::optional::set14::job_sequencing::*;

fn run_both(jobs: &[(usize, usize)], expected: (usize, usize)) {
    assert_eq!(job_scheduling_greedy(jobs), expected);
    assert_eq!(job_sequencing_heap(jobs), expected);
}

#[test]
fn test_basic_case() {
    let jobs = vec![(2, 100), (1, 19), (2, 27), (1, 25), (3, 15)];
    // Optimal: (2,100) at t=2, (2,27) at t=1, (3,15) at t=3 -> Total: 3 jobs, 142 profit
    run_both(&jobs, (142, 3));
}

#[test]
fn test_all_same_deadline() {
    let jobs = vec![(1, 10), (1, 20), (1, 30), (1, 5)];
    // Only one can be chosen, pick the most profitable
    run_both(&jobs, (30, 1));
}

#[test]
fn test_no_jobs() {
    let jobs = vec![];
    run_both(&jobs, (0, 0));
}

#[test]
fn test_all_can_be_done() {
    let jobs = vec![(1, 10), (2, 20), (3, 30)];
    // Each job has its own slot
    run_both(&jobs, (60, 3));
}

#[test]
fn test_unschedulable_jobs() {
    let jobs = vec![(1, 10), (1, 5), (1, 1), (1, 20)];
    // Only 1 job can be scheduled, pick the highest profit
    run_both(&jobs, (20, 1));
}

#[test]
fn test_complex_case() {
    let jobs = vec![
        (4, 70),
        (1, 80),
        (1, 30),
        (2, 100),
        (3, 60),
        (2, 20),
        (4, 40),
        (1, 90),
    ];
    // Greedy: Schedule best profits in latest possible available slots
    // Expected scheduling might yield max profit of 320 from 4 jobs
    run_both(&jobs, (320, 4));
}

#[test]
fn test_high_deadlines_low_profits() {
    let jobs = vec![(5, 10), (5, 20), (5, 5), (5, 1)];
    // All can be scheduled since deadline is high
    run_both(&jobs, (36, 4));
}

#[test]
fn test_duplicate_jobs() {
    let jobs = vec![(2, 50), (2, 50), (2, 50), (2, 50)];
    // Can schedule 2 jobs with deadline 2: t=2 and t=1
    run_both(&jobs, (100, 2));
}
