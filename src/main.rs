use std::fs;
fn main() {
    let mut total = 0;
    let letters: Vec<char> = ('a'..='z').collect();
       
    for line in fs::read_to_string("test.txt").unwrap().lines(){
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
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    println!("{response}");
    return response.to_string();
}
