pub(crate) mod error;
pub(crate) mod prelude;

fn main() {
    println!("Hello, world!");

    use crate::prelude::*;

    println!("{}", Error::from("Error!"));
    // Short for
    // Error::Generic("Error!".to_string());

    use crate::error::ToCrateError;

    println!("{}", std::fmt::Error {}.to_crate_error())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generic_error() {
        use crate::prelude::*;
        println!("{}", Error::from("Error!"));
    }

    #[test]
    fn test_other_error() {
        use crate::error::ToCrateError;

        println!("{}", std::fmt::Error {}.to_crate_error())
    }
}
