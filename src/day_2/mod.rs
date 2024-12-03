use crate::utils::write_to_file;
pub struct Day2 {
}

fn word_to_int_arr(content: &str) -> Vec<i32> {
    let numbers = content.split(' ');
    let mut num_list: Vec<i32> = Vec::new();

    numbers.for_each(|n: &str| {
        let num = n.parse::<i32>();
        if !num.is_err() {
            num_list.push(num.unwrap())
        }
    });
    return num_list;
}

pub fn solve_day_part_1(content: &str) -> i32 {
    let mut result = 0;

    content.split("\n").for_each(|l: &str| {
        let num_list = word_to_int_arr(l);
        if num_list.len() < 2 {
            return;
        }
        let mut i = 1;
        let mut dif = num_list[i] - num_list[i-1];
        let direction = dif > 0;

        while i < num_list.len() {
            dif = num_list[i] - num_list[i-1];
            if dif == 0 {
                return
            }
            if dif.abs() < 1 {
                return
            }
            if dif.abs() > 3 {
                return
            }
            if dif > 0 {
                if direction != true {
                    return
                }
            } else {
                if direction != false {
                    return
                }
            }
            i += 1;
        }
        result += 1;
    });

    print!("Result: {}",result);
    let to_file = format!("{}",result);
    write_to_file(&to_file,"./src/day_2/output_part_1.txt");
    return result;
}

fn is_valid_sequence(num_list: &[i32]) -> bool {
    if num_list.len() < 2 {
        return false; // A valid sequence requires at least two numbers.
    }

    let direction = num_list[1] > num_list[0];
    for i in 1..num_list.len() {
        let dif = num_list[i] - num_list[i - 1];
        if dif.abs() < 1 || dif.abs() > 3 || (dif > 0) != direction {
            return false; // Invalid if difference is out of range or direction changes.
        }
    }
    true
}

pub fn solve_day_part_2(content: &str) -> i32 {
    let mut result = 0;

    content.lines().for_each(|line| {
        let original_list = word_to_int_arr(line);

        for remove_index in 0..=original_list.len() {
            let mut num_list = original_list.clone();
            if remove_index < num_list.len() {
                num_list.remove(remove_index); // Remove the element at the given index.
            }

            if is_valid_sequence(&num_list) {
                println!("Valid Sequence after removal: {:?}", num_list);
                result += 1;
                break; // Stop further checks once a valid sequence is found.
            }
        }
    });

    println!("Result: {}", result);
    let to_file = format!("{}", result);
    write_to_file(&to_file, "./src/day_2/output_part_2.txt");
    result
}




















