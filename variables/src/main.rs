fn main() {
    /* 
    let x: i16 = 5; // You don't need to specify a type for let
    let x: i16 = 10; // This a reassignment and this is ALLOWED
    x = 15; // This is a MUTATION and is **NOT** allowed

    let mut name = "Hello"
    name = "World" // This is mutation... but it's allowed because the variable is mutatable

    const pi: f32 = 3.14 // remember that types are required for constants
    pi = 3.13 // NOT ALLOWED because that is a mutation!!
    let pi = 3.13 // NOT ALLOWED because no values are allowed to be changed
    const pi = 3.15 // NOT ALLOWED!!! even if you make it a constant still no changing
    */
    let spaces = "    ";
    let spaces = spaces.len(); // len is a method and we are reasigning so all is good
    
    // Watch this tho
    let mut spaces = "   ";
    spaces = spaces.len(); // Mutable values can't change types!!
    
    // This is a bit confusing
    let guess = "42".parse().expect("Not a number!"); // This throws an error
    // I think it's broken because we need to assign "42" to a number but Rust does not know which one!

    // Changing gears... Here is chars
    let letter: char = "A";

    // Finally something fun! Here are tuples!
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // The typing looks pretty funny but it's not needed
    let tup = (500, 3.14, "What's up!")

    // This syntax is so cool
    let (x, y, z) = tup;
    println!("{z}") // What's up!

    // But let's be honest... Single indexing is more useful
    let fun_phrase = tup.2; // it's sorta like elements in python like get_terminal_size().lines

    // Arrays are a homo get it? they can only have the same type...
    let arr= [1, 2, 3, 4, 5];
    // Optional hinting for performance (Rust is stronly typed so it will still check)
    let arr: [u8; 5] = [20, 60, 140, 242, 0];

    // The reason it isnt u8, u8, u8... 
    // is because the array only has 1 data type
    // so it's pointless...

    // here is a cool trick for preallocation
    let arr: [0; 5]; // same as np.zeros(shape=(5,))

    // Indexing is the same! Look!
    zero = arr[0]

    // let's look at this example that the book gave on indexing
    use std::io;

    let a: [u8: 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // Just like in python... We have IndexErrors!!!
    
}
