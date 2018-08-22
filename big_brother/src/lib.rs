//      Chapter 7.2
mod outermost {
    pub fn middle_function(){}

    fn middle_private_function(){}

    pub mod inside {
        pub fn inner_function(){
            ::outermost::middle_private_function();
        }

        fn secret_function(){}
    }
}

mod should_work{
    pub fn please(){}

    fn prob_not(){}
}

fn try_me(){
    outermost::middle_function();
    //outermost::middle_private_function(); private function
    outermost::inside::inner_function();

    //outermost::inside::inner_function(); private function
    //outermost::inside::secret_function(); private function
    should_work::please();
    // should_work::prob_not(); private function
}
/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/