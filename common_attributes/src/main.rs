//      Chapter 10
/*
fn main() {
    // Using main to find largest. Only problem is you need to duplicate to find largest of another list
    let number_list = vec![34, 50, 25, 100, 65];
    let mut max = number_list[0];
    for i in number_list {
        if i > max {
            max = i;
        }
    }
    println!("The largest number is {}", max);

    let number_list0 = vec![34, 50, 25, 100, 65];
    let number_list1 = vec![25, 60, 3, 4, 5, 102];
    println!("Max of list0 = {}. Max of list1 {}.", largest(&number_list0), largest(&number_list1))
}

fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &i in list.iter() {
        if i > max {
            max = i;
        }
    }
    max
}
*/
//      Chapter 10.1
/*
struct Point<T> {
    x: T,
    y: T,
}
struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T, U> Point1<T, U> {
    fn mix_up<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y
        }
    }
}
fn main() {
    let p = Point {x: 5, y: 6};
    println!("The x of Point p is: {}.", p.x());
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['c', 'd', 'e', 'A', 'Z'];
    let integer = Point {x:5, y:10};
    let float = Point {x: 3.14, y: 5.0};
    let both = Point1 {x: 3.14, y: 5};
    let common0 = Point1 {x: 5.99, y: 6.0};
    let common1 = Point1 {x: 5, y: 6};

    let p1 = Point1 {x: 5, y: 10.4};
    let p2 = Point1 {x: "", y: "Matthew"};

    let p3 = p1.mix_up(p2);
    println!("p3.x: {}. p3.y: {}", p3.x, p3.y);

}
// Shelved until we learn what traits are
fn largest<T>(list: &[T]) -> T {
    let mut max = list[0];
    for &i in list.iter() {
        if i > max {
            max = i;
        }
    }
    max
}
*/
//      Chapter 10.2
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let tweet = Tweet {
        username:String::from("m2038943920"),
        content:String::from("#TSMWIN"),
        reply: false,
        retweet: true,
    };
    println!("{}",tweet.summarize());
}