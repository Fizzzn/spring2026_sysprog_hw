
const Freezing_f:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
 (f - Freezing_f) * 5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
 c * 9.0/5.0 + Freezing_f
}

fn is_even(n: i32)-> bool{
     n % 2 == 0
}

fn check_guess(guess: i32, secret:i32) -> i32{
    if guess == secret{ 0}
    else if guess > secret{1}
    else{-1}
}

fn main() {
 //Temperature Converter******************************
 let mut f: i32 = 32;
 let c = fahrenheit_to_celsius(f as f64);
 println!("{f} F = {c:.2} C");

 //5 int
 for following_f in (f+1) ..=(f+5){
    let c0 = fahrenheit_to_celsius(following_f as f64);
    println!("{following_f} f = {c:.2} c");
 }


 //Number Analyzer************************************

//Create an array of 10 integer numbers of your choice.
    let mut num = [10,12,50,43,00,97,67,69,21,80];
//Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
//done
//Use a for loop to iterate through the array and for each number:
    for n in num{
        if is_even(n){
            println!("{n} is even");
        }
        else{println!("{n} is odd")}
    }
    for n in num{
        if n % 3 ==0 && n%5==0{
            println!("FizzBuzz");
        }
        else if n%3 ==0 {
            println!("Fizz");
        }

        else if  n%5==0 {
            println!("Buzz");
        }
        else{
            println!("{n}");
        }

    }
//If the number is divisible by 3, print "Fizz" instead

//If the number is divisible by 5, print "Buzz" instead

//If it's divisible by both 3 and 5, print "FizzBuzz"

//Use a while loop to find and print the sum of all numbers in the array.
    let mut i = 0;
    let mut sum =0;
    //imma combine both largest and sum
    let mut largest = num[0];

    while i<num.len(){
        sum += num[i];

        if num[i] > largest {
            largest = num[i]
        }
        i+=1;
    }
    println!("sum: {}", sum);
    println!("largest: {}", largest);

//Use a loop to find and print the largest number in the array.




 //Guessing Game**************************************

    let secret: i32 =7;
    let mut guess: i32 =1;
    let mut attempts: i32 = 0;

    loop{
        attempts += 1;
        let result = check_guess(guess, secret);
        if guess == secret{
            println!("Correct.");
            break;
        }
        else if guess > secret{
            println!( "Too high.");
        
        }
        else{println!("Too low.");}
        guess += 1;
    }

    println!("it took like {attempts} guesses.");
}
