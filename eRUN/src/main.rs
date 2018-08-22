//      Chapter 6.1
/*
enum IpAddrKind {
    V4,
    V6,
}

/* //can be expressed in a more concise way
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

/* // Better way to be expressed (in the context of IP addresses)
enum IpAddr{
    V4(String),
    V6(String),
}
*/

enum IpAddr{
    V4(u8, u8, u8, u8), //can associate any type (e.g. enum, struct, String) with enums. This is a tuple.
    V6(String),
}

enum message {
    Quit,
    Move {x: i32, y:i32}, //anonymous struct associated with Move
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl message {
    fn call(&self){
        //some method body
    }
}

fn route(ip_type: IpAddrKind) {}

fn main() {
    let m = message::Write(String::from("Hello world!"));
    m.call();
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let tup:(u8, u8, u32) = (5, 5, 32);
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{}", tup.1);
    /* //Expressed in a better way
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    */
    /*
    let home = IpAddr{
        kind:IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number:Option<i32> = None;
    let he:Option<u32> = None; //need the :Option<i32>, otherwise error throw -> cannot infer type
}
*/
//      Chapter 6.2
/*
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    etc,
}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(whatever) => {
            println!("State quarter from {:?}", whatever);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn five_six_seven(x: i8) -> i8{
    match x {
        5 => {
            println!("Value of five!");
            5
        },
        6 => {
            println!("Value of six!");
            6
        },
        7 => {
            println!("Value of seven!");
            7
        },
        _ => 125,
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five_cents = Coin::Nickel;
    println!("The value of a US nickel is: {} cents.", value_in_cents(five_cents));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five_six_seven returned: {}.", five_six_seven(2));
}

struct Animal{
    status:status_e,
    color:String,
}
enum status_e{
    alive,
    dead,
    unconscious,
}
impl Animal {
    fn check_status(&self) -> String {
        match self.status {
            status_e::alive => String::from("Alive! Woohoo!"),
            status_e::dead => String::from("...."),
            status_e::unconscious => String::from("zzzzz"),
        }
    }
}
fn main() {
    let monkey = Animal{
        status: status_e::alive,
        color: String::from("brown"),
    };
    println!("This animal is: {}.", monkey.check_status() );

}
*/
//      Chapter 6.3
/*
fn main() {
    let some_u8_value = Some(4);
    match some_u8_value{
        Some(4) => println!("FOUR!"),
        _ => println!("NOT FOUR!"),
    }
    //Above is equivalent to the following:
    if let Some(4) = some_u8_value {
        println!("FOUR!");
    }else {
        println!("NOT FOUR!"); }
}
*/