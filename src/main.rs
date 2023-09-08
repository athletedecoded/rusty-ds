// Define main function which calls add function from lib.rs
use rust_distro_cicd::add;

fn main() {
    println!("Hello, world!");
    println!("10 + 5 = {}", add(10, 5));
}
