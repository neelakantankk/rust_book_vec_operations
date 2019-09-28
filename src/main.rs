use std::io;

fn main() {
   
    let mut numbers: Vec<i32> = Vec::new();
    
    println!("Enter numbers to add to the vector. Enter q to stop");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_input) => (),
        };
        if &input.trim()[..1] == "q" || &input.trim()[..1] == "Q" {
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
}

fn average(numbers : &[i32]) -> f64 {
    let mut average: i32 = 0;
    for item in numbers {
        average = average + item;
    }
    let average: f64 = (average as f64) / (numbers.len() as f64);
    average
}
