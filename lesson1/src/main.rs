// fn main() {
//     println!("Welcome message!!");
//     fizz_buzz();
// }

// fn fizz_buzz() {
//     for i in 1..301 {
//         if i % 3 == 0 {
//             println!("fizz");
//             println!("{}", i);
//         }
//         if i % 5 == 0 {
//             println!("fizz");
//             println!("{}", i);
//         }
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("fizz buzz ");
//             println!("{}", i);
//         }
//         if i == 301 {
//             println!("fizz buzz");
//             println!("{}", i);
//         }
//     }
// }

//Ex1: Done!

/*fn main() {
    call_me();
}

fn call_me(){

}*/

//Ex2: Done!

/*fn main() {
    call_this(3);
}

fn call_this(num: u64) {
    for i in 0..num {
        println!("Loop! number {}", i + 1);
    }
}*/

//Ex3: Done!
/*
fn main() {
    call_this(10);
}

fn call_this(num: u32) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}*/

//Ex4: Done!
/*
fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
} */

//Ex5: Done!
/*
fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}*/

//Ex6: Done!

/*fn main() {
    bigger(1, 2);
}

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        return a;
    }
    b
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}*/

//Ex7: Done!

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
/*
fn main() {
    fizz_if_foo("hn");
}

pub fn fizz_if_foo(fizzish: &str) -> &str {
    // if fizzish == "fizz" {
    //     "foo"
    // } else {
    //     1
    // }
    match fizzish {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz",
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(fizz_if_foo("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(fizz_if_foo("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(fizz_if_foo("literally anything"), "baz")
    }
}
*/

//Ex8: Done!
/*
fn main() {
    let y = 5;
    println!("y has the value {}", y);
}
*/

//Ex9: Done
/*
fn main() {
    let x = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
 */

//Ex10: Done!
/*
fn main() {
    let x: i32 = 1;
    println!("Number {}", x);
}*/

//Ex11: Done!
/*
fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}*/

//Ex12: Done!
/*
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}*/

//Ex13: Done!

const NUMBER: u32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
