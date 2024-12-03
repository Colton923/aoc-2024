mod utils; // Import your utils module for read_input
use utils::read_input; // Import the read_input function
use std::io::{self, Write}; // Importing Write to flush the output buffer
use std::collections::HashMap;

// Define a trait for solving the day's puzzle
trait Solver {
    fn solve(&self, input: &str);
}

mod day_1;

impl Solver for day_1::Day1 {
    fn solve(&self, input: &str) {
        day_1::solve_day_part_1(input);
        day_1::solve_day_part_2(input);
    }
}

mod day_2;

impl Solver for day_2::Day2 {
    fn solve(&self, input: &str) {
        day_2::solve_day_part_1(input);
        day_2::solve_day_part_2(input);
    }
}

mod day_3;

impl Solver for day_3::Day3 {
    fn solve(&self, input: &str) {
        day_3::solve_day_part_1(input);
        day_3::solve_day_part_2(input);
    }
}

fn main() {
    // Prompt user for the day number
    print!("Enter the day number (e.g., 1, 2, 3, ...): ");
    io::stdout().flush().unwrap(); // Ensure prompt is shown before user input

    let mut day_input = String::new();
    io::stdin().read_line(&mut day_input).unwrap();
    let day_input = day_input.trim(); // Remove any leading/trailing whitespace

    // Build the file path dynamically based on the day input
    let file_path = format!("./src/day_{}/input.txt", day_input);

    // Call the read_input function to read the file content
    let result = read_input(file_path.to_string());

    // If there was an error, handle it
    if !result.t_error.is_empty() {
        println!("Error: {}", result.t_error);
        return;
    }

    // If successful, retrieve the output content
    let content = result.t_output;

    // Create a HashMap to map day input to solver
    let mut solvers: HashMap<&str, Box<dyn Solver>> = HashMap::new();
    solvers.insert("1", Box::new(day_1::Day1 {}));
    solvers.insert("2", Box::new(day_2::Day2 {}));
    solvers.insert("3", Box::new(day_3::Day3 {}));

    // Solve based on the day input
    if let Some(solver) = solvers.get(day_input) {
        solver.solve(&content);
    } else {
        println!("No solution for day {}", day_input);
    }
}
