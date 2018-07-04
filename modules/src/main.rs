// module is in this case another .rs file in the same directory
mod module;

fn main() {
    println!("{}", module::other_file_module::do_something());
    println!("{}", module::free_funcion());
    println!("{}", example::greetins());
}

// In-private module
mod example {
    pub fn greetins() -> String {
        return "Hi there".to_string();
    }
}

// In-file priate module compiled only on test builds
#[cfg(test)]
mod tst {
    #[test]
    fn test_greetings() {
        // Notice the ::example needed to disambiguate
        // the module name
        assert_eq!("Hi there", ::example::greetins());
    }
}
