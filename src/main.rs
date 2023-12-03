use regex::Regex;

mod part1;
mod part2;
fn main() {
    // let filepath = "input.txt";
    let text = "one, two, three, and seven are numbers";
    let re = Regex::new(r"\b(one|two|three|seven)\b").unwrap();

    for cap in re.find_iter(text) {
        println!("Found: {}", &cap.as_str());
    }




    // part1::read_file_line_by_line(filepath).expect("Could not read file");
    // part2::replace_each_line_with_numbers(filepath).expect("Could not read file");
}
