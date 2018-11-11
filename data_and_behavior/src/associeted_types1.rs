trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for FourIntegers {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {}
}
