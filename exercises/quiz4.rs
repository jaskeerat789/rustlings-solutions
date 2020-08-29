// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros



macro_rules! my_macro {
    ($message:expr)=>{
        format!("{} {}","Hello",$message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
    
}
