use chrono::Utc;
use rand::random;

pub fn unique_file_name(ext: &str) -> String {
    let now = Utc::now();
    let random_number: i32 = random();
    format!("{}_{random_number}.{ext}", now.timestamp())
}
