extern crate rand;

use std::io;
use rand::Rng;


fn main() {
   
    let mut numbers: Vec<i32> = Vec::new();
    
    println!("Enter numbers to add to the vector. Enter q to stop");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_input) => (),
        };
        if input.trim().len() > 0 && (&input.trim()[..1] == "q" || &input.trim()[..1] == "Q") {
            break;
        }
        let input : i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue
            },
        };

        numbers.push(input);      
    }    

    println!("Numbers = {:?}", numbers);


    println!("The average of {:?} is {:.3}",numbers,average(&numbers));

    user_sort(&mut numbers);
    println!("The sorted vector is {:?}", numbers);

    println!("The median value is {}",numbers[numbers.len()/2]);

}

fn average(numbers : &[i32]) -> f64 {
    let mut average: i32 = 0;
    for item in numbers {
        average = average + item;
    }
    let average: f64 = (average as f64) / (numbers.len() as f64);
    average
}

fn user_sort(numbers: &mut Vec<i32>) {
    for index in 0..numbers.len() {
        let mut smallest = numbers[index];
        let mut smallest_index = index;
        for subindex in (index+1)..numbers.len() {
            if numbers[subindex] < smallest {
                smallest = numbers[subindex];
                smallest_index = subindex;
            }
        }
        smallest = numbers.remove(smallest_index);
        numbers.insert(index,smallest);
    }
}
