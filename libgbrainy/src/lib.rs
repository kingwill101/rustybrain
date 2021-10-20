pub mod engine;
pub mod models;
pub mod reader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello world");
}
