fn main() {
    println!("Hello, world!");
}

pub mod collector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
