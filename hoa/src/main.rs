    //      Chapter 4.1
/*fn main() {
    let s = "Hello world!"; // known size at run time
    let d = s; //does not work if s was typecast to String
    let mut a:char;
    let mut me = String::from("matthew");
    let s = me;
    //let myname  = me.clone();
    //me.push_str(" prinz");
    println!("{}", me);
}
*/
/*
fn main(){
    let s = String::from("hello");
    take_ownership(s);
    //println!("{}", s); //does not work, as it was passed to take_ownership
    let x = 5;
    makes_copy(x);
    println!("{}", x); //does work, as because of Copy trait assigned to integers

}
fn take_ownership(s:String){
    println!("{}", s);
}

fn makes_copy(n:i32) {
    println!("{}", n);
}
*/
/*
fn main() {
    let s1 = give_ownership();
    let s2 = String::from("Wtf am I doing");
    let s3 = takes_and_gives_back(s2);

    let mut s4 = String::from("this is trippy");
    s4 = takes_and_gives_back(s4);
    println!("{}", s1);
    //println!("{}", s2); //doesn't work, as it has been moved by takes_and_gives_back
    println!("{}", s3);
    println!("{}", s4); //this does work as it passes the ownership back to s4
}

fn give_ownership() -> String {
    let some_string = String::from("Hello");
    return some_string;
}

fn takes_and_gives_back(s:String) -> String {
    return s;
}
*/

//      Chapter 4.2
/*
fn main() {
    let s1 = String::from("hello");
    //ampersands are references
    //println!("The string I am using is '{}' and its length is {}.",s1, calculate_test(s1)); //throws an error
    println!("The string I am using is '{}' and its length is {}.",s1, calculate_length(&s1));
    //let mut s2 = String::from("goodbye");
    //println!("{}", things_break(&s2)); //does not work because we can not change things which are passed into functions by reference
    // DIFFERENT THAN C!!!
    let mut s2 = String::from("hello");
    let x = things_no_break(&mut s2);
    //println!("{}", x); //cannot print
    let r1:&String = &s2;
    //let r2:&String = &mut s2; //doesn't work, as it is borrowed already as immutable.
    //let potential_dangling_pointer = dangle(); //Rust does not allow for dangling pointers. :')
}

fn calculate_length(s: &String) -> usize { //s is a reference to a string (essentially a pointer)
    return s.len();
    //s goes out of scope, but it's ok because we took in a reference, not the actual string.
}

fn calculate_test(s: String) -> usize {
    return s.len();

}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn things_break(mut s: &String) -> &String {
    //s.push('h');
    return s;
}

fn things_no_break(s: &mut String){
    s.push_str(", world!");
}
fn dangle() -> &String {
    let s = String::from("Hello");
    return &s;
    //s goes out of scope after this function - C would return a pointer which references garbage or nothing. you have to return ownership
    //of the entire string.
}
*/

//      Chapter 4.3
fn main(){
    let s = String::from("Hello world");
    let word = first_word(&s);
    //s.clear(); //wipes the word away, index doesn't do anything
    let my_string = String::from("Hello, world!");
    let word1 = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word2 = first_word(&my_string_literal[..]);

    let word3 = first_word(&my_string_literal); //same, as string literals are slices to start with

    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[..3];

    println!("word2 = {}, word3 = {}.", word2, word3)


}
/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}