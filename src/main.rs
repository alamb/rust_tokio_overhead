//! This program is designed to measure the per-task overhead of the tokio runtime
//! for CPU bound tasks
//!
//! It does so by processing 100
//!
//! Rows per second means "how fast can two arrays of integers be compared
//!
//! Then we will chart
//!
//! x-axis: batch size
//! y-axis: total rows/second
//!
//! The intercept will then give us some idea of
//! how many rows/second
//! compute 100x the intercept (so how many rows /

use std::{sync::Arc, time::{Instant, Duration}};

use arrow::{array::{ArrayRef, Int64Array}, compute::eq_dyn};
use rand::{distributions::{Distribution, Uniform}, prelude::ThreadRng};

/// what we are measuring
fn do_work(left: ArrayRef, right: ArrayRef) {
    let cmp = eq_dyn(left.as_ref(), right.as_ref()).unwrap();
    assert_eq!(cmp.len(), left.len());
    assert_eq!(cmp.len(), right.len());
}

/// async version of `do_work` (copy / pasted)
async fn do_async_work(left: ArrayRef, right: ArrayRef) {
    let cmp = eq_dyn(left.as_ref(), right.as_ref()).unwrap();
    assert_eq!(cmp.len(), left.len());
    assert_eq!(cmp.len(), right.len());
}

fn random_array(rng: &mut ThreadRng, num_rows: u32) -> ArrayRef {
    let between = Uniform::from(0..i64::MAX);
    let array: Int64Array = (0..num_rows)
        .map(|_| between.sample(rng))
        .map(Some)
        .collect();
    Arc::new(array)
}

const NUM_RUNS: u32 = 10_000_000;
const NUM_ROWS: u32 = 100;

fn main() {
    let mut rng = rand::thread_rng();

    // create an array of num_ros
    println!("Setting up a1...");
    let a1 = random_array(&mut rng, NUM_ROWS);
    println!("Setting up a2...");
    let a2 = random_array(&mut rng, NUM_ROWS);

    test(a1.clone(), a2.clone());
    async_test(a1.clone(), a2.clone());

    async_test2(a1.clone(), a2.clone());
}

fn test(a1: ArrayRef, a2: ArrayRef) {
    println!("Begin non async...");
    let total_time: Duration = (0..NUM_RUNS)
        .map(|_| {
            let start = Instant::now();
            do_work(a1.clone(), a2.clone());
            start.elapsed()
        })
        .sum();

    println!("ran {} runs in {:?}", NUM_RUNS, total_time);
    let time_per_run = total_time  / NUM_RUNS;

    println!("average time per run: {:?}", time_per_run);
}


fn async_test(a1: ArrayRef, a2: ArrayRef) {

    println!("Begin async...");

    // now run with tokio
    let builder = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    builder.block_on(async move {

        let mut total_time: Duration = Default::default();
        for _ in 0..NUM_RUNS {
            let start = Instant::now();
            do_async_work(a1.clone(), a2.clone()).await;
            total_time += start.elapsed()
        }

        println!("ran {} runs in {:?}", NUM_RUNS, total_time);
        let time_per_run = total_time  / NUM_RUNS;

        println!("average time per run: {:?}", time_per_run);
    });
}


fn async_test2(a1: ArrayRef, a2: ArrayRef) {

    println!("Begin async with multi-threads...");

    // now run with tokio
    let builder = tokio::runtime::Builder::new_multi_thread()
        .build()
        .unwrap();

    builder.block_on(async move {

        let mut total_time: Duration = Default::default();
        for _ in 0..NUM_RUNS {
            let start = Instant::now();
            do_async_work(a1.clone(), a2.clone()).await;
            total_time += start.elapsed()
        }

        println!("ran {} runs in {:?}", NUM_RUNS, total_time);
        let time_per_run = total_time  / NUM_RUNS;

        println!("average time per run: {:?}", time_per_run);
    });
}
