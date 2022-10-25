use std::io;

fn main() {

    loop {
        println!("Which number of the Fibonacci Sequence do you want?");
        println!("or type 'quit' to stop the program");

        let mut n: String = String::new();

        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        if n.trim() == "quit" {
            break;
        }

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let result = fibbo(n);

        let result = fibbo_iter(n);

        println!("Result is: {result}");
    }

    
}

// Recursive function.

// fn fibbo(x: u32) -> u32 {
    
//     match x {
//         0 => 1,
//         1 => 1,
//         x => fibbo(x - 1) + fibbo(x - 2),
//     }
    
// } 

// iterative function is much quicker.

fn fibbo_iter(y: u64) -> u64 {

    let mut first_number: u64 = 0;
    let mut second_number: u64 = 0;
    let mut current_number: u64 = 1;

    let mut i: u64 = 1;

    while i < y {
        first_number = second_number;
        second_number = current_number;
        current_number = first_number + second_number;

        i += 1;
    }
    return current_number;
}
