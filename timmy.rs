fn main() {
    let s = "Hello"; // This is stack
    let timmy = &s; //Huh?? You can't use that!
    timmy.push_str(", World");
}