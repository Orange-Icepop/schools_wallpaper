mod services;

use services::date;

fn main() {
    println!("Hello, world!");
    println!("Today is day {} of the week.", date::get_weekday());
}
