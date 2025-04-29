use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions, write};
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
#[derive(Serialize, Deserialize)]
struct Promise {
    text: String,
    time: String,
}

fn main() {
    promise_world();
}

fn promise_world() {
    '_PromiseWorld: loop {
        println!("___________________________________");

        println!("\n\n\n📝 WELCOME TO YOUR PROMISE WORLD!");
        println!("=================================");
        println!("\nA place where you promise yourself to complete what matters.\n");

        println!("1. ➕ Add Your Promise");
        println!("2. 📋 View Your Promises");
        println!("3. ✅ Complete Your Promise");
        println!("4. 🔄 Reaffirm Your Promise");
        println!("5. 🚪 Exit\n");

        let mut choice = String::new();

        print!("Enter your option: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = if let Ok(num) = choice.trim().parse() {
            num
        } else {
            println!("Invalid number! , Please try again.");
            continue;
        };

        match choice {
            1 => {
                add_promise();
            }
            2 => {
                println!("\n\n📋 Your Promises:\n");
                view_promises();
            }
            3 => println!("Choices : {}", choice),
            4 => println!("Choices : {}", choice),
            5 => {
                println!("Exiting... 👋");
                break '_PromiseWorld;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn add_promise() {
    print!("What is your promise? ");
    io::stdout().flush().unwrap();

    let mut promise_text = String::new();
    io::stdin()
        .read_line(&mut promise_text)
        .expect("Failed to read line");

    print!("When will you start working on it? ");
    io::stdout().flush().unwrap();

    let mut promise_time = String::new();
    io::stdin()
        .read_line(&mut promise_time)
        .expect("Failed to read line");

    let new_promise = Promise {
        text: promise_text.trim().to_string(),
        time: promise_time.trim().to_string(),
    };

    //reading data from file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("promises.json")
        .unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut data: Vec<Promise> = if contents.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap()
    };

    data.push(new_promise);

    let json = serde_json::to_string_pretty(&data).unwrap();
    write("promises.json", json).unwrap();
}

fn view_promises() {
    use std::fs;
    use std::io::{self, Write};
    let contents = match fs::read_to_string("promises.json") {
        Ok(s) => s,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No promises found.");
                println!("Done reflecting on your promises? Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
                return;
            } else {
                eprintln!("Failed to read promises.json: {}", e);
                println!("Done reflecting on your promises? Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
                return;
            }
        }
    };
    let data: Vec<Promise> = if contents.trim().is_empty() {
        Vec::new()
    } else {
        match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse promises.json: {}", e);
                println!("Done reflecting on your promises? Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
                return;
            }
        }
    };
    if data.is_empty() {
        println!("No promises found.");
    } else {
        let time_width = data.iter().map(|p| p.time.len()).max().unwrap_or(4).max(4);
        println!("{:<width$} | {}", "Time", "Promise", width = time_width);
        println!(
            "{:-<width$}-+-{:-<promise_width$}",
            "",
            "",
            width = time_width,
            promise_width = 40
        );
        for promise in data {
            println!(
                "{:<width$} | {}",
                promise.time,
                promise.text,
                width = time_width
            );
        }
    }
    println!("\n\nDone reflecting on your promises? Press Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn complete_promise() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("promises.json")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: Vec<Promise> = serde_json::from_str(&contents).unwrap();
    for promise in data {
        println!("{}", promise.text);
    }
}

// fn reaffirm_promise() {
//     let mut file = std::fs::File::open("promises.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     if contents.is_empty() {
//         let mut file = std::fs::File::create("promises.txt").unwrap();
//         file.write_all(format!("{}\n{}\n", promise, promisetime).as_bytes())
//             .unwrap();
//     } else {
//     }
//     let mut file = std::fs::File::create("promises.txt").unwrap();
//     file.write_all(format!("{}\n{}\n", promise, promisetime).as_bytes())
//         .unwrap();
// }
