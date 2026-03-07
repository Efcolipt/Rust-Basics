#![allow(unused)]

// Async vs thread
// native thread 
// - good for CPU bound computation
// - limited by memory and OS thread limits
// async programming
// - lower memory consumption
// - no limit on number of threads
// - good for IO bound computations

#[tokio::main]
async fn main() {}
