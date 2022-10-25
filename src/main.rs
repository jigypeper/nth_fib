use std::io;

fn main() {

    loop {
        println!("Which number of the Fibonacci Sequence do you want?");

        let mut n: String = String::new();

        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = fibbo(n);

        println!("Result is: {result}");
        break;
    }

    
}

fn fibbo(x: u32) -> u32 {
    
    match x {
        0 => 1,
        1 => 1,
        x => fibbo(x - 1) + fibbo(x - 2),
    }
    
} 
