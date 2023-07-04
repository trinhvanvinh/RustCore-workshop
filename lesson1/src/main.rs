fn main() {
    println!("Welcome message!!");
    fizz_buzz();
}

fn fizz_buzz() {
    for i in 1..301 {
        if i % 3 == 0 {
            println!("fizz");
            println!("{}", i);
        }
        if i % 5 == 0 {
            println!("fizz");
            println!("{}", i);
        }
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz ");
            println!("{}", i);
        }
        if i == 301 {
            println!("fizz buzz");
            println!("{}", i);
        }
    }
}
