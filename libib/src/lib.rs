mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            use crate::outermost;
            outermost::middle_function();
            outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function(); // Error
    outermost::inside::inner_function(); // Error 
    // outermost::inside::secret_function(); // Error
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
