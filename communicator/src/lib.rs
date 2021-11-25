pub mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {
            crate::outermost::middle_secret_function();
        }
        fn secret_function() {}
    }
}

fn test() {
    crate::outermost::middle_function();
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
