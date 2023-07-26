fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // THANK LORD WE DON'T NEED TO USE std::cmp::Ordering and match and cmp and stuff

    // let's run down rq all the important operators
    // != is not equal obviously
    // < and > I don't need to explain
    // == is the expression version of equals

    // WOAH this is crazy check this out
    let condition = true; // lowercase bools btw
    let number = if condition { 5 } else { 6 }; // this is python for x = 5 if True else 10
    println!("The value of number is: {number}");

    //let number = if condition { 5 } else { 5.00001 }; // This is illegal because not same type
    
    // just do this smh
    if condition {
        let number = 5;
    }
    else {
        let number = "six";
    }

    // Now this is just absurd...
    let mut average_american_weight = 1; // not true lol
    let mass_of_earth = loop{
        average_american_weight += 1 ;
        average_american_weight *= 10;
        if average_american_weight > 1000 {
            break average_american_weight ^ 2;
        }
    };
    println!("{mass_of_earth}");
    
    // if that's not crazy idk what is...

    // While loops too
    const ME: &str = "tall";
    while ME == "tall" {
        break;
    }
    for number in (1..10).rev() {
        println!("{number}")
    }
    println!("Liftoff!")
}
