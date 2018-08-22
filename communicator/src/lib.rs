//      Chapter 7.1
#[cfg(test)]
/*
mod network { //called with network::connect()
    fn connect(){
    }
    mod client { //called with network::client::connect()
        fn connect() {

    }
    }
}
*/
pub mod network;
pub mod client;



mod tests {
    use super::client;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        client::connect();
        /*
        ::client::connect();
        super::client::conect();
        super::deep(); //just need super to call;
        super::connect(); //probably be kind of weird due to multiple connect functions
        */
    }
}
