/*
cargo run -p data_and_behavior --bin make_true
*/
fn make_true(input: &str) -> String {
    format!("{}!!", input)
}

fn main() {
    println!("Hello, {}!", make_true("world"));
}
