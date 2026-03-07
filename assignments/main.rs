use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

const FREEZING_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_F
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count: usize = 0;

    for i in 0..words.len() {
        let current_word = words[i];
        let mut count: usize = 0;

        for j in 0..words.len() {
            if words[j] == current_word {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = current_word;
        }
    }

    (max_word.to_string(), max_count)
}

// book catalog
struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");

    for book in books {
        writeln!(file, "{}|{}|{}", book.title, book.author, book.year)
            .expect("Failed to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() == 3 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse().expect("Invalid year"),
            };
            books.push(book);
        }
    }

    books
}

fn main() {
    // Book catalog
    let books = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

    // Temperature converter
    let f: i32 = 32;
    let c = fahrenheit_to_celsius(f as f64);
    let back_to_f = celsius_to_fahrenheit(c);

    println!("\nTemperature check:");
    println!("{f}°F = {c:.2}°C");
    println!("{c:.2}°C = {back_to_f:.2}°F");

    // Word frequency
    let text = "the thing is that the thing is the thing";
    let (word, count) = most_frequent_word(text);
    println!("\nMost frequent word: \"{}\" ({} times)", word, count);

    // Number analyzer
    let num = [10, 12, 50, 43, 0, 97, 67, 69, 21, 80];

    let mut even_count = 0;
    let mut odd_count = 0;

    for n in num {
        if is_even(n) {
            even_count += 1;
        } else {
            odd_count += 1;
        }
    }

    println!("\nNumber analyzer:");
    println!("Even numbers: {}", even_count);
    println!("Odd numbers: {}", odd_count);

    println!("FizzBuzz results:");
    for n in num {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n} -> FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n} -> Fizz");
        } else if n % 5 == 0 {
            println!("{n} -> Buzz");
        }
    }

    let mut i = 0;
    let mut sum = 0;
    let mut largest = num[0];

    while i < num.len() {
        sum += num[i];

        if num[i] > largest {
            largest = num[i];
        }

        i += 1;
    }

    println!("Sum: {}", sum);
    println!("Largest: {}", largest);

    // Guessing game
    let secret: i32 = 7;
    let mut attempts: i32 = 0;

    println!("\nGuessing game:");

    loop {
        println!("Enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct.");
            break;
        } else if result == 1 {
            println!("Too high.");
        } else {
            println!("Too low.");
        }
    }

    println!("It took {} guesses.", attempts);
}