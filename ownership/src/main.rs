fn main() {
     {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    // Strings are mutable. Even python does not allow that lol
    // Note the difference between literals and Strings
    // println!("Hello") is a literal
    // let mut s = String::from("World") is a string

    //Lemme explain what String::from() does

    // String::from() looks for memory in the heap that will fit the size of our string
    // As you can imagine... strings are slow... cuz they use heap
    // literals are hardcoded into the binary since the size is known at compile time

    //it's called borrowing because after the code is done with it... the space
    // in the park is freed for more people to watch the cherry blosoms

    //POINTERS
    let s1 = String::from("hello");
    let s2 = s1;

    //pointers are LAZYYY they point to where the data is and don't do any work themselves.
    //you can have multiple pointers pointing to the same data
    
    // watch this
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    // This won't work because s1 is FULLY DEAD. We didn't COPY data we MOVED data.

    // lemme show borrowing
    let s = String::from("Hello")
    println!("{&s}") // This is good because println is just borrowing it for a sec

    // imagine timmy. Timmy wants to borrow poor s's toy and tamper with it
    let timmy = &s
    timmy.push_str(", World!")
    // Rust says nah and s is safe and timmy gets yelled at by the compiler
    // But you can change this is you add mut

    let ss = String::from("Hello")
    let billy = &mut ss
    // billy is a good boy
    // Thomas wants in on the fun!
    let thomas = &mut ss
    // Wait that's not allowed! Only one person can borrow at a time

    // let's fix this!
    {
        let billy = &mut ss
        billy.push_str(", World")
    }
    {
        let thomas = &mut ss;
        thomas.push_str(", and all other planets!");
    }
    // They both got to play with the toy can break it as much as they wanted
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String {
        let s = String::from("hello");
    
        &s
    }

}
