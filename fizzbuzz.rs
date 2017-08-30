fn main() {
    let f = "Fizz";
    let b = "Buzz";
    for i in 1..100 {
        if i%3 == 0 {
            print!("{}", f);
        }
        if i%5 == 0 {
            print!("{}", b);
        }
        if i%3 != 0 && i%5 != 0 {
            print!("{}", i);
        }
        println!("");
    }
}
