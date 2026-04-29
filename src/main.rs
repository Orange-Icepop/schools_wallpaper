mod services;

use services::date;

fn main() {
    println!("Hello, world!");
    let weekday = date::get_weekday();
    println!("Today is day {} of the week.", weekday);
    println!("Reading config...");
    let config = services::configs::init().unwrap();
}