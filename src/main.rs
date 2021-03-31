fn main() {
    let mut counter: u32 = 0;
    loop {
        counter += 1;
        if counter >= 101 {
            break;
        }

        else if (counter % 3 == 0) && (counter % 5 == 0) {
        println!("Fizzbuzz");
        continue;
        }

        else if counter % 3 == 0 {
            println!("Fizz");
        }
        else if counter % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}",counter);
        }
    }
}
