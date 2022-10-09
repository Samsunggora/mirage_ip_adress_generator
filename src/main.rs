use std::fs::File;
use std::io;
use std::io::Write;
use rand::prelude::*;

fn main() {
    let mut stack_size = String::new();
    println!("Enter capacity :");
    let mut n: u128 = 0;

    let _user_stack_size = io::stdin().read_line(&mut stack_size).unwrap();
    let mut file = File::create("ips.txt").expect("Error!");

    let mut all_ips = Vec::with_capacity(stack_size.trim().parse().expect("Index entered was not a number"));

    while n < stack_size.trim().parse().expect("Index entered was not a number") {
        let x = ip_generator();
        all_ips.push(x);
        n += 1;
    }

    for content in all_ips {
        file.write_all(content.as_bytes()).expect("Write error!");
    }
}

fn ip_generator() -> String {
    let mut rng = thread_rng();
    let x: u8 = rng.gen_range(0..=255);
    let y: u8 = rng.gen_range(0..=255);
    let z: u8 = rng.gen_range(0..=255);
    let k: u8 = rng.gen_range(0..=255);
    let ip = format!("'{}.{}.{}.{}' ", x, y, z, k);
    return ip;
}