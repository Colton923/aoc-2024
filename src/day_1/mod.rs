use std::{collections::HashMap};

use crate::utils::write_to_file;
pub struct Day1 {
}

pub fn solve_day_part_1(content: &str) -> i32 {
    let mut items_a: Vec<i32> = Vec::new();
    let mut items_b: Vec<i32> = Vec::new();
    let mut result = 0;

    content.split("\n").for_each(|l: &str| {
        let numbers= l.split_once(' ');
        if (numbers.is_none()) {
            return;
        }
        if(numbers.unwrap().0.is_empty() || numbers.unwrap().1.is_empty()) {
            return;
        }

        let first = numbers.unwrap().0.trim().parse::<i32>().unwrap();
        let second = numbers.unwrap().1.trim().parse::<i32>().unwrap();
        
        items_a.push(first);
        items_b.push(second);
    });

    items_a.sort_by(|a, b| a.cmp(&b));
    items_b.sort_by(|a, b| a.cmp(&b));

    let  len = items_a.len();


    for i in 0..len {
        result += (items_a[i] - items_b[i]).abs();
    };
    let to_file = format!("{}",result);

    write_to_file(&to_file,"./src/day_1/output_part_1.txt");
    return result;
}

pub fn solve_day_part_2(content: &str) -> i32 { 
    let mut items_a: HashMap<i32, i32> = HashMap::new();
    let mut items_b: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;

    content.split("\n").for_each(|l: &str| {
        let numbers= l.split_once(' ');
        if (numbers.is_none()) {
            return;
        }
        if(numbers.unwrap().0.is_empty() || numbers.unwrap().1.is_empty()) {
            return;
        }

        let first = numbers.unwrap().0.trim().parse::<i32>().unwrap();
        let second = numbers.unwrap().1.trim().parse::<i32>().unwrap();
        
        if items_a.contains_key(&first) {
            items_a.insert(first, items_a.get(&first).unwrap() + 1);
        } else {
            items_a.insert(first, 1);
        }

        if items_b.contains_key(&second) {
            items_b.insert(second, items_b.get(&second).unwrap() + 1);
        } else {
            items_b.insert(second, 1);
        }
    });

    items_a.iter().for_each(|(k, v)| {
        if items_b.contains_key(k) {
            result += k * v *  items_b.get(k).unwrap()
        }
    });

    let to_file = format!("{}",result);

    write_to_file(&to_file,"./src/day_1/output_part_2.txt");
    return result;
}

