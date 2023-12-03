use std::fmt::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let filepath = "input.txt";
    read_file_line_by_line(filepath).expect("Could not read file");
}


fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let number = grab_all_numbers_from_each_line(&line)?;
        println!("{}", number);
    }

    Ok(())
}


/*
    This function will take a line of text and return the number that is in the line
    If there is no number in the line, it will return an error
*/
fn grab_all_numbers_from_each_line(line: &str) {
    println!("{}", line);

}
