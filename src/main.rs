use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;
use std::io::Read;

fn _guessing_game(){
    println!("Guess the number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Please input your guess:\n");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("Das right!");
                break;
            }
        }

    }


}

fn _shadowing(){
    let x = 5;
    println!("x = {}", x);

    let x = x + 1;
    println!("x = {}", x);

    let x = x * 2;
    println!("x = {}", x);

    let x = 69;
    println!("x = {}", x);

}

fn _tuples_and_arrays(){
    //tuples are fixed size/length collections of possibly different types
    let tup = (420, 69, 6.10);
    let (x, y, z) = tup;

    println!("x: {}, y: {}, z:{}", x, y, z);

    let ex: (i32, u8, f64) = (500, 1, 6.4);
    let five_hundo = ex.0;

    println!("{}", five_hundo);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                            "August", "September", "October", "November", "December"];
    let array: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
    let same_value = [3; 5];
    //                   [value, size]
    for element in same_value.iter(){
        println!("same_value: {}", element);
    }
}

fn _addition(x: i32, y: i32) -> i32{
           //notice lack of semicolon, semicolon would make it a statement instead of expression
    x + y //same as "return x + y;"
}

fn _looping(){
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("the result is: {}", result); //20


/*    let mut number = 3;
    while number != 0{
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");*/


    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!");


    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is {}", element);
    }
}

fn _string_slicing(){
    let s = String::from("Hello World!");

    let hello = &s[0..5]; //&s[..5]; works as well (no need for the first 0)
    let world = &s[6..12];//&s[6..]; works here too!
    println!("{} {}", hello, world);
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

//related functions
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
//methods
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn show(&self){
        println!("{}x{} with area {}", self.width, self.height, self.area());
    }

}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) and Area: {}", self.width, self.height, self.area())
    }
}

fn _struct_example(){
    let r = Rectangle{
        width:  32,
        height: 64,
    };

    let r2 = Rectangle::new(128, 128);


    r.show();
    r2.show();

    //#[derive(Debug)] gives access to printing structs with debug "{:?} or pretty debug "{:#?}"
    println!("{:?}", r);
    println!("{:#?}", r2);

    //impl Display trait gives access to println!ing struct without debug
    println!("{}", r);
}

fn _match_example(){
    let pair = (4, -5);
    match pair{
        (x, y) if x == y     => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal 0"),
        (x, _) if x % 2 == 0        => println!("X is even"),
        _                                  => println!("no match"),
    }
}

#[derive(Debug)]
enum Shape{
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
}

impl Shape{
    fn area(&self) -> f64{
        match *self{
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

// enum Option<T>{
//     Some(T),
//     None,
// }

fn division(x: f64, y: f64) -> Option<f64>{
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn _enums_and_options(){
    let r = Shape::Rectangle {width: 10, height: 70};
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);

    // println!("{}", r.area());
    // println!("{}", s.area());
    // println!("{}", c.area());

    let result = division(5.0, 7.0);
    match result{
        Some(x) => println!("{}", x),
        None => println!("cannot divide by 0!"),
    }

}

fn _vectors(){
    let x = vec![1, 2, 3, 4];
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    for i in &v{
        println!("{}", i);
    }
    //debug print
    println!("{:?} {} {}", &v, v.len(), v.capacity());

    let enum_vector = vec![
        Shape::Circle(4.2),
        Shape::Square(69),
        Shape::Rectangle {width: 6, height: 10},
    ];
    println!("{:?}", &enum_vector);
}

fn _hash_maps(){
    use std::collections::hash_map;

    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (k, v) in &hm{
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")){
        Some(&n) => println!("{}", n),
        None => println!("no match"),
    }
}

fn _result_example(){
    use std::fs::File;

    let f = File::open("text.txt");

    let f = match f{
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening file: {:#?}", error)
        },
    };
}
fn main() {
    _guessing_game();
    //shadowing();
    //tuples_and_arrays();
    //println!("5 + 6 = {}", addition(5, 6));
    //looping();
    //string_slicing();
    //_struct_example();
    //_match_example();
    //_enums_and_options();
    //_vectors();
    //_hash_maps();
    //_result_example();
}
