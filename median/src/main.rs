use std::collections::HashMap;

fn median(numbers: &mut Vec<i32>) -> Option<f32> {
    let size = numbers.len();
    numbers.sort();
    if size == 0 {
        return None;
    } else if size == 1 {
        let x = numbers.get(0).expect("");
        return Some(*x as f32);
    } else if size % 2 == 1 {
        let x = numbers.get(size / 2).expect("");
        return Some(*x as f32);
    } else {
        let x = *numbers.get(size / 2 - 1).expect("") as f32;
        let y = *numbers.get(size / 2).expect("") as f32;
        return Some((x + y) / 2.0);
    }
}

fn mode(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.len() == 0 {
        panic!("{}", "Modes of 0-length array does not exist");
    }

    let mut modes = HashMap::new();
    for number in numbers {
        let count = modes.entry(number).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", modes);

    let mut max_key = 0;
    let mut max_value = 0;

    for (key, value) in modes {
        if value > max_value {
            max_value = value;
            max_key = *key;
        }
    }
    return if max_key != 0 { Some(max_key) } else { None };
}

fn main() {
    let mut numbers = vec![1, 4, 2, 4, 87, 8, 3, 14];
    let _median = median(&mut numbers);
    println!("Median: {}", _median.expect(""));

    let _mode = mode(&numbers);
    match _mode {
        None => println!("{}", "No mode found"),
        Some(m) => println!("Mode: {}", m),
    }
}
