#![feature(test)]

extern crate test;

use rayon::prelude::*;
use std::sync::atomic::*;
use std::sync::Mutex;
use test::bench::Bencher;

fn increment_mutex_serial(x: &Mutex<u64>, n: u64) {
    (0..n).into_iter().for_each(|_| {
        *x.lock().unwrap() += 1;
    });
}

const N: u64 = 1_000;

#[bench]
fn bench_increment_mutex_serial(b: &mut Bencher) {
    let x = Mutex::new(0);
    b.iter(|| {
        increment_mutex_serial(&x, N);
    });
}

fn increment_mutex_parallel(x: &Mutex<u64>, n: u64) {
    (0..n).into_par_iter().for_each(|_| {
        *x.lock().unwrap() += 1;
    });
}

#[bench]
fn bench_increment_mutex_parallel(b: &mut Bencher) {
    let x = Mutex::new(0);
    b.iter(|| {
        increment_mutex_parallel(&x, N);
    });
}

fn increment_atomic_serial(x: &AtomicU64, n: u64) {
    (0..n).into_iter().for_each(|_| {
        x.fetch_add(1, Ordering::SeqCst);
    });
}

#[bench]
fn bench_increment_atomic_serial(b: &mut Bencher) {
    let x = AtomicU64::new(0);
    b.iter(|| {
        increment_atomic_serial(&x, N);
    });
}

fn increment_atomic_parallel(x: &AtomicU64, n: u64) {
    (0..n).into_par_iter().for_each(|_| {
        x.fetch_add(1, Ordering::SeqCst);
    });
}

#[bench]
fn bench_increment_atomic_parallel(b: &mut Bencher) {
    let x = AtomicU64::new(0);
    b.iter(|| {
        increment_atomic_parallel(&x, N);
    });
}
