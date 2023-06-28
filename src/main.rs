pub mod auth;
pub mod config;

type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() {
    println!("Hello, world!");
}
