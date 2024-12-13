use std::collections::HashMap;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

pub fn day01_01(filename: String) {
    // File hosts.txt must exist in the current path
    if let Ok((left, right)) = read_file(filename) {
        let mut left = left;
        let mut right = right;
        left.sort();
        right.sort();

        let mut sum = 0;
        for i in 0..left.len() {
            sum += (left[i] - right[i]).abs()
        }

        println!("The sum of day01_1 is: {}", sum);
    }

    println!("Sorry there is something wrong reading the file.");
}

pub fn day01_02(filename: String) {
    if let Ok((left, right)) = read_file(filename) {
        let occurrences = count_occurences(left, right);
        let mut sum: i64 = 0;
        for key in occurrences.keys() {
            sum += key * occurrences.get(key).unwrap()
        }
        println!("The sum of day01_2 is: {}", sum);
    }
    println!("Sorry there is something wrong reading the file.");
}

fn count_occurences(left: Vec<i64>, right: Vec<i64>) -> HashMap<i64, i64> {
    let mut occurences: HashMap<i64, i64> = HashMap::new();

    for num in left {
        let count = right.iter().filter(|&&x| x == num).count();
        occurences.insert(num, count as i64);
    }

    occurences
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_file(filename: String) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let (left, right): (Vec<i64>, Vec<i64>) = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(l), Ok(r)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                    Some((l, r))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unzip();
    Ok((left, right))
}
