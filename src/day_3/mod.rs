use crate::utils::write_to_file;
pub struct Day3 {
}

pub fn solve_day_part_1(content: &str) -> i32 {
    let mut result = 0;

    println!("Result: {}", result);
    let to_file = format!("{}", result);
    write_to_file(&to_file, "./src/day_3/output_part_1.txt");
    result
}

pub fn solve_day_part_2(content: &str) -> i32 {
    let mut result = 0;

    println!("Result: {}", result);
    let to_file = format!("{}", result);
    write_to_file(&to_file, "./src/day_3/output_part_2.txt");
    result
}




















