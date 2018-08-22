//      Chapter 8.1: Vectors
/*
enum Spreadsheet {
    Int(i32),
    Float(f64),
    Text(String),
}

struct Order {
    name:String,
    cost:f64,
    quantity:u32,
}
use Spreadsheet::*;
fn main() {
    //let v1: Vec<i32> = Vec::new();
    let v2= vec![5, 6, 7, 1];
    let mut v3 = vec![1, 2, 3];
    cycle(v2);
    vector_thingy(v3);
    let mut row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("blue")),
        Spreadsheet::Float(10.12),
    ];
    let mut GroceryList = vec![Order{name:String::from("apple"), cost:0.99, quantity:5}];
    let x = Float(4.257);
    row.push(Int(4));
    row.push(x);


}


fn cycle(v: Vec<i32>) {
    for i in 0..v.len() {
        println!("{}", i);
    }
}

fn cycle1(v: Vec<i32>) {
    //Another implementation of cycle
    for i in &v{
        println!("{}", i);
    }
}

fn vector_thingy(v: Vec<i32>) {
    for i in 0..5 {
        let mut x = v.get(i);
        match x {
            None => println!("Nothing at index: {}", i),
            _i32 => println!("Found {:?} at index: {}", x, i),
        }
    }
}
*/
//      EE 312 Midterm Exam 1 Spring 2018, Question 1 - potential Rust implementation - shelved for now
/*
fn main() {}
struct Order {
    name: String,
    pricePerUnit:f32,
    quantity:f32,
}
struct groceryList {
    list:Vec<(&Order)>,
    numItems:i32,
}
// a)
fn createOrder(name: String, pricePerUnit:f32, quantity:f32) -> &Order {
    &Order  {
        name,
        pricePerUnit,
        quantity,
    }

}
// b)
fn getCost(Order: &Order) -> f32 {
    Order.pricePerUnit * Order.quantity
}
// c)
*/
//      Chapter 8.2: Strings
/*
fn main() {
    let mut s = String::new();
    let data = "initial";
    s = data.to_string(); //equivalent: let mut S = String::from(data);
    s = String::from(data);
    s.push_str(" contents");
    println!("{}",s);
    let s1 = String::from("Hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;

    let s4 = String::from("Johnson");
    let s5 = String::from("Johnson");
    let s7 = format!("{} & {}", s4, s5);
    let s6 = s4 + " & " + &s5;
    println!("format doesn't take ownership of contents: {}. add does :( {}",s7, s6);
    let s8 = &s2[0..5]; // use with caution
    for c in s2.chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
*/
//      Chapter 8.3: Hash Maps

use std::collections::HashMap;
fn main() {
    let mut books = HashMap::new();
    books.insert(String::from("John Green"), vec![ "Will Grayson, Will Grayson".to_string(), "Paper Towns".to_string() ]);
    books.insert(String::from("J.K. Rowling"), vec![ "Harry Potter 1".to_string(), "Harry Potter 2".to_string()]);
    //books.insert(String::from("J.K. Rowling"), vec!["Harry Potter 1, 2, 3, 4".to_string()]); //overwrites previous line
    let book1 = &(books.get(&("John Green".to_string())));
    println!("{:?}", book1);

    for (author, book) in &books {
        for title in book {
            println!("{} by {}", title, author);
        }
    }
    let navy_seals_pasta = "What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little clever comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo.";
    let mut copypasta_map = HashMap::new();
    for word in navy_seals_pasta.split_whitespace() {
        let count = copypasta_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", copypasta_map);
    let mut max = 0;
    let mut most_word = "";
    for (word, count) in &copypasta_map {
        if *count > max {
            max = *count;
            most_word = *word;
        }
    }
    println!("'{}' is most common, printed {} times.", most_word, max);
}
