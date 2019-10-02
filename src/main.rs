extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
   
    let mut numbers: Vec<u32> = Vec::new();
    
    println!("Populating Vector...");

    const LOWER : u32 = 0;
    const UPPER : u32 = 50;

    for _i in 0..500 {
        let input : u32 = rand::thread_rng().gen_range(LOWER,UPPER);
        numbers.push(input);
    }
    println!("Numbers = {:?}", numbers);


    println!("The average is {:.3}",average(&numbers));

    user_sort(&mut numbers);

    println!("The median value is {}",numbers[numbers.len()/2]);

    let frequencies = get_frequencies(&numbers);

    let mode = get_mode(&frequencies);
    match mode.len().cmp(&1) {
        Ordering::Equal => println!("The mode is {}",&mode[0]),
        Ordering::Greater => println!("The modes are {:?}",mode),
        Ordering::Less => println!("No Modes! Error!"),
    }
}

fn average(numbers : &[u32]) -> f64 {
    let mut average: u32 = 0;
    for item in numbers {
        average = average + item;
    }
    let average: f64 = (average as f64) / (numbers.len() as f64);
    average
}

fn user_sort(numbers: &mut Vec<u32>) {
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

fn get_frequencies(numbers: &[u32]) -> HashMap<u32,u32> {
    let mut frequencies : HashMap<u32,u32> = HashMap::new();

    for item in numbers.iter() {
        let count = frequencies.entry(*item).or_insert(0);
        *count += 1;
    }

    frequencies
}

fn get_mode(frequencies: &HashMap<u32,u32>) -> Vec<u32> {
    let mut mode : Vec<u32> = Vec::new();

    let mut largest = 0;

    for (item,frequency) in frequencies {
        match frequency.cmp(&largest) {
            Ordering::Greater => {
                mode.clear();
                mode.push(*item);
                largest = *frequency;
            },
            Ordering::Equal => mode.push(*item),
            Ordering::Less => continue,
        }
    }
    mode
}
       
