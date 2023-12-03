use std::fmt::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;



pub fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut total_amount: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let row_amount = grab_all_numbers_from_each_line(&line);
        total_amount += row_amount;

    }
    println!("Total Rows: {}", total_amount);
    Ok(())
}


/*
    This function will take a line of text and return the number that is in the line
    If there is no number in the line, it will return an error
*/
fn grab_all_numbers_from_each_line(line: &str) -> u32 {

    



    let mut numbers_as_chars: Vec<char> = Vec::new();

    for character in line.chars() {
        if character.is_numeric() {
            numbers_as_chars.push(character);
        }
    }
    println!("{:?}", numbers_as_chars);

    let length = numbers_as_chars.len();

    let first = numbers_as_chars[0];
    if length == 0 {
        println!("No numbers in line");
        return 0;
    }

    if length == 1 {
        numbers_as_chars.push(first);
        let row_amount = format!("{}{}", numbers_as_chars[0], numbers_as_chars[length - 1]);
        return row_amount.parse::<u32>().expect("Could not parse number");
    }
    
    let row_amount = format!("{}{}", numbers_as_chars[0], numbers_as_chars[length - 1]);
    return row_amount.parse::<u32>().expect("Could not parse number");

}
