/*
cargo run -p traits --bin print_news5
*/

use std::fmt::Debug;


trait  Foo{

}

impl<T: Debug> Foo for Vec<T> {

}

fn main(){}