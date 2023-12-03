mod part1;
mod part2;
fn main() {

    let filepath = "input.txt";
    part1::read_file_line_by_line(filepath).expect("Could not read file");
    // part2::replace_each_line_with_numbers(filepath).expect("Could not read file");
}
