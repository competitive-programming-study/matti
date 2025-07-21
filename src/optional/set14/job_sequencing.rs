use std::cmp::Reverse as Rev;
use std::collections::BinaryHeap;
///
/// ## Job Sequencing (Greedy)
///
/// Given n jobs represented as tuples: (deadline: usize, profit: usize), find the
/// most profit achievable by a sequence of jobs, given that each job takes 1 unit
/// of time and we can pocket the profit only if a job gets completed before its
/// deadline. Time starts by one. All deadlines are guarantted to be in the interval
/// [1,n]
///
/// ## Returns
/// a tuple with the number of jobs sequenced and the max profit given by
/// the sum of jobs profit that we successfully scheduled.
///
/// ## Strategy
/// The greedy approach would suggest to prioritize jobs with higher profit. The
/// strategy is to try to sequence a job with deadline i, at the time slot i, so
/// to keep earlier slots free for earlier jobs. If the slot i is occupied we try for
/// all slots in the range 0-i.
///
/// To do this we need a memoization array, with a max number of slots as the job length.
/// We can use boolean to specify if each time slot is occupied or free.
///
/// For each job (sorted by descending profit), we try to schedule them in the first slot
/// in the range [1,job.deadline] right to left. If we can successfuly schedule them, we
/// accumulate the profit and increment the count.
///
/// ## Complexity
/// This approach has a quadratic time complexity and linear space relative to the number
/// of jobs
///
pub fn job_scheduling_greedy(jobs: &[(usize, usize)]) -> (usize, usize) {
    let (mut max_profit, mut count) = (0, 0);

    let mut job_vec: Vec<(usize, usize)> = jobs.to_vec();

    //sort by profit
    job_vec.sort_by_key(|&j| j.1);

    //initialize time slots
    let mut time_slots = vec![false; jobs.len()];

    //from most profitable to least profitable
    for &(deadline, profit) in job_vec.iter().rev() {
        //use an iterator so we take only the timeslots before or equal to j.deadline
        for t in time_slots.iter_mut().take(deadline).rev() {
            //claim the latest time_slot
            if !*t {
                *t = true;
                max_profit += profit;
                count += 1;
                break;
            }
        }
    }
    (max_profit, count)
}

///
/// ## Job Sequencing (Greedy)
///
/// Given n jobs represented as tuples: (deadline: usize, profit: usize), find the
/// most profit achievable by a sequence of jobs, given that each job takes 1 unit
/// of time and we can pocket the profit only if a job gets completed before its
/// deadline. Time starts by one. All deadlines are guarantted to be in the interval
/// [1,n]
///
/// ## Returns
/// a tuple with the number of jobs sequenced and the max profit given by
/// the sum of jobs profit that we successfully scheduled.
///
/// ## Strategy
/// This approach uses a min-heap indexed by job deadline. The current length of the
/// heap tells us the current time slots. We iterate through jobs:
/// - if the job deadline is higher than the length of the heap
///   we can insert the job in the heap
/// - else, if the first job in the heap has lower profit than the current we
///   evict it and inset this one
///
/// Iterating on the heap we can pocket the profit of the jobs inside and the length
/// tells us the total number of scheduled job
///
/// ## Complexity
/// This approach has O(n*log(n)) time complexity (heap operations are logarithmic + we iterate
/// one time on the job array) and linear space relative to the number
/// of jobs
///
pub fn job_sequencing_heap(jobs: &[(usize, usize)]) -> (usize, usize) {
    let mut heap: BinaryHeap<Rev<(usize, usize)>> = BinaryHeap::new(); // (deadline, profit)

    let mut sorted_jobs = jobs.to_vec();
    sorted_jobs.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by deadline ascending

    for &(deadline, profit) in &sorted_jobs {
        if heap.len() < deadline {
            heap.push(Rev((deadline, profit)));
        } else if let Some(&Rev((_, min_profit))) = heap.peek() {
            if profit > min_profit {
                heap.pop();
                heap.push(Rev((deadline, profit)));
            }
        }
    }

    let total_profit = heap.iter().map(|Rev((_, p))| p).sum();
    let count = heap.len();
    (total_profit, count)
}
