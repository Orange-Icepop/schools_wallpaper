use chrono::Datelike;
use chrono::Local;
pub fn get_weekday() -> u32 {
    let now = Local::now();
    now.weekday().num_days_from_monday()
}
