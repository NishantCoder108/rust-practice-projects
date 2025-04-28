use std::io::{self, Read};

fn main() {
    promise_world();
}

fn promise_world() {
    loop {
        println!("\n\n\n📝 WELCOME TO YOUR PROMISE WORLD!");
        println!("=================================");
        println!("\nA place where you promise yourself to complete what matters.\n");

        println!("1. ➕ Add Your Promise");
        println!("2. 📋 View Your Promises");
        println!("3. ✅ Complete Your Promise");
        println!("4. 🔄 Reaffirm Your Promise");
        println!("5. 🚪 Exit\n");

        //Here give choices:
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // let choice: u8 = choice.trim().parse().expect("Please type number!");
        let choice: u8 = if let Ok(num) = choice.trim().parse() {
            num
        } else {
            println!("Invalid number! , Please try again.");
            continue;
        };

        println!("Choices : {:?}", choice);
    }
}
