#![allow(unused)]

async fn g1() -> u32 {
    1
}
async fn g2() -> u32 {
    2
}
async fn g3() -> u32 {
    3
}

// Make coffee
// 
async fn f() {
    // Wait step by step
    // let r1 = g1().await;
    // let r2 = g2().await;

    // Run both of functions simultaneous
    let (r1, r2) = tokio::join!(g1(), g2());

    let r3 = g3().await;
    println!("{r1} {r2} {r3}")
}

#[tokio::main]
async fn main() {
    f().await;
}
