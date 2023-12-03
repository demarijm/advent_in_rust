use std::{io::{BufReader, BufRead}, fs::File};






pub fn replace_each_line_with_numbers(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut total_amount: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let start = line.find("one").unwrap_or(0);
        println!("{}", start);

    }


    println!("Total Rows: {}", total_amount);
    Ok(())
}


fn grab_all_numbers_from_each_line(line: &str) {

}

fn match_words_to_number (word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}