use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn write_to_file(content: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub struct TOutput {
    pub t_input: String,   // Path to the file
    pub t_output: String,  // Output content (to be filled)
    pub t_error: String,   // Error message (if any)
}

pub fn read_input(input: String) -> TOutput {
    let input = input.trim().to_string(); // Remove leading/trailing whitespaces
    let mut output = TOutput {
        t_input: input.clone(),
        t_output: String::new(),
        t_error: String::new(),
    };

    match File::open(&output.t_input) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        output.t_output.push_str(&line);
                        output.t_output.push('\n');
                    }
                    Err(e) => {
                        output.t_error.push_str(&e.to_string());
                        output.t_error.push('\n');
                    }
                }
            }
        }
        Err(e) => {
            output.t_error = e.to_string(); // Store file open error
        }
    }

    return output;
}
