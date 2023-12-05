use std::fs;
fn main() {
    let mut total = 0;
    let letters: Vec<char> = ('a'..='z').collect();
       
    for line in fs::read_to_string("input.txt").unwrap().lines(){
        println!("{line}");
        
        let ints = replace_spelling_with_ints(&line).replace(&*letters,"");
        println!("{ints}"); 
        let first =  &ints.chars().next().unwrap().to_string();
        let last = &ints.chars().last().unwrap().to_string();
        let result = format!("{first}{last}");
        println!("{result}");
        total += result.parse::<i32>().unwrap();
    }
    
    println!("{}",total);
}

fn replace_spelling_with_ints (input : &str) -> String {
    let  response  = &input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    println!("{response}");
    return response.to_string();
}
