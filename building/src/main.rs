//      Chapter 5.1
/*
struct Penguin {
    name: String,
    alive: bool,
    fish: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let pengu = Penguin {
        name: String::from("pengu"),
        alive: true,
        fish: 3,
    };
    let matt = build_penguin(String::from("Matthew"), 2);
    println!("This penguin's name is {}. He is {}. He has {} fish.", matt.name, matt.alive, matt.fish);
    let copy_pengu = Penguin {
        name: pengu.name,
        alive: pengu.alive,
        fish: pengu.fish,
    };
    let copy_pengu_short = Penguin {
        name: String::from("pengu clone"),
        ..pengu
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); //despite being called as structs, they behave more similarly to tuples
    //as they can be accessed via indexing, destructured, etc. Note that Color and Point are different.

}

fn build_penguin(name: String, fish: u64) -> Penguin {
    Penguin {
        name : name,
        alive : true,
        fish : fish,
    }
}
// with field init shorthand

fn build_penguin_short(name: String, fish: u64) -> Penguin {
    Penguin {
        name,
        alive: true,
        fish,
    }
}
// struct update syntax
*/

//      Chapter 5.2
/*
#[derive(Debug)]
struct triangle {
    base: f32,
    height: f32,
}
fn main(){
    let tri1 = triangle{
        base: 3.0,
        height: 4.0,};
    println!("tri1 is {:?}.", tri1);
    println!("tri1 area is {}.", area(tri1));
    //:? tells compiler we want to use debug. Need to opt-in with the struct, using #[derive(Debug)]
}

fn area(triangle: triangle) -> f32 {
    triangle.base * triangle.height * 0.5
}
*/

//      Chapter 5.3
/*
#[derive(Debug)]
struct triangle {
    base: f32,
    height: f32,
    side1: f32,
    side2: f32,
    side3: f32,
}

impl triangle {
    fn area(&self) -> f32 {
        self.base * self.height * 0.5
    }
    fn perimeter(&self) -> f32 {
        self.side1+self.side2+self.side3
    }
    fn can_hold(&self, tri2: &triangle) -> bool {
        let area1 = self.base * self.height;
        let area2 = tri2.base * tri2.height;
        area1 >= area2
    }
    fn equilateral(size: f32) -> triangle {
        triangle {base: size, height: sqrt(3)/2*size, side1: size, side2: size, side3: size}
    }
}
fn main(){
    let tri1 = triangle{
        base: 3.0,
        height: 4.0,
        side1: 3.0,
        side2: 4.0,
        side3: 5.0,
    };
    let tri2 = triangle{
        ..tri1
    };
    println!("tri1 area is {}.", tri1.area());
    println!("tri1 perimeter is {},", tri1.perimeter());
    println!("{}, tri1 can hold tri2.", tri1.can_hold(&tri2));
    println!("{:?}", triangle::equilateral(4.0));
    //:? tells compiler we want to use debug. Need to opt-in with the struct, using #[derive(Debug)]
}
*/