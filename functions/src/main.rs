use std::cmp::Ordering;

fn greet_people() {
    println!("I'm too lazy to greet you so here is a rust function instead!");
}

fn check_if_can_drink(age: u16) { // just like python type hinting
    match age.cmp(&80) {
        Ordering::Less => println!("You can drink!"),
        Ordering::Greater => {
            println!("Go drink some \"Reverse aging\" water from ebay cuz ur too old");
        }
        Ordering::Equal => println!("You can drink!")
    }
}
fn plus_one(num: u16) {
    x + 1
}

fn plus_one_bad(num: u16) {
    x + 1;
}
fn main() {
    // Let's call some funcs
    greet_people(); // same as python but with semi-colon
    check_if_can_drink(90);

    // Random things rq
    // say it with me "EXPRESSION IS A QUESTION"
    // expressions are == in python and statements are = in python


    // But here is what's weird...
    let y = {
        let x = 3;
        x + 1
    };
    // Think of the Curly braces as an anonamous function. 
    //Except what ever value pops up in memory is the return value? weird
    println!("The value of y is: {y}"); // This actually works...

    // This right here is CRAZY
    println!("5 plus one is {plus_one(5)}") // This WORKS????

    // To make things crazyer
    println!("5 plus one is {bad_plus_one(5)}") // This BREAKS?!?!??

    // let me explain sorta
    // In Python there is a reference and a call
    // func vs func() is just like x + 1 vs x + 1;
    // In Rust we didn't want to call x + 1 but just mention it to call later...

    println!("I'm out!")
}
