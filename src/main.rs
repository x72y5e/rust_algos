#![allow(dead_code)]
#![allow(unused_imports)]

use fibonacci::fib;
use scss::test_scss;
use knapsack::run;

mod fibonacci;
mod scss;
mod knapsack;


fn main() {
    /*
    for n in 0..50 {
        println!("{}", fib(n as usize));
    }
    */
    //test_scss();
    run();
}
