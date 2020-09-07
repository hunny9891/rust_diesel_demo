extern crate diesel_demo;
extern crate diesel;

use std::time::{Instant};

use self::diesel_demo::*;

fn main() {
    let connection = establish_connection();
    let now = Instant::now();

    println!("Test Started.");
    insert_batch(&connection);
    insert_batch(&connection);
    insert_batch(&connection);
    insert_batch(&connection);
    println!("Test Finished ... Time Elapsed {}s", now.elapsed().as_secs());
}