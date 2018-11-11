/*
cargo run -p traits --bin print_news2
*/
use std::fmt;

pub trait Truth {
    fn make_true(&self) -> Self;
}

fn print_news<T>(facts: &[T]) where T: Truth + fmt::Display {
    for fact in facts {
        let fact = fact.make_true();
        println!("{}", fact);
    }
}
