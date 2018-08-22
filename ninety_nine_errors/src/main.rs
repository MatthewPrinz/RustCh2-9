//      Chapter 9.1
/*
fn main() {
    panic!("boom");
}

*/
//      Chapter 9.2
/*
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; //works similarly to read_username_from_file above. ? serves as the match function.
    Ok(s);
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    //? can only be used in functions that return result
}

fn main() {
    let f = File::open("hello.txt");
    //other options besides match is expect() and unwrap()
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("I'm basically the same thing as unwrap, but I carry a message");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Could not create file hello.txt, because {:?}", e);
                },
            }
            panic!("There was a problem opening the file {:?}", error);
        },
    };
}
*/
//      Chapter 9.3
