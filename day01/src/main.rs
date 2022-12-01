use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => calorie_count("input.txt"),
        2 => calorie_count(args[0].as_str()),
        _ => println!("Only one input file argument is supported!"),
    }
}

fn calorie_count(input_file_path: &str) {
    let mut elfs_calories = read_calories_per_elf(input_file_path);

    println!(
        "Elf with most calories has {} calories",
        elfs_calories.iter().max().unwrap()
    );

    elfs_calories.sort();
    elfs_calories.reverse();
    let top3: i32 = elfs_calories[0..3].iter().sum();
    println!("Top3 elves have {:?} calories", top3);
}

fn read_calories_per_elf(input_file_path: &str) -> Vec<i32> {
    let lines = read_lines(input_file_path).unwrap();

    let mut cur_elf_sum = 0;
    let mut elfs_calories: Vec<i32> = Vec::new();
    for line in lines.flatten() {
        if let Ok(calories) = line.parse::<i32>() {
            cur_elf_sum += calories
        } else {
            elfs_calories.push(cur_elf_sum);
            cur_elf_sum = 0;
        }
    }
    if cur_elf_sum != 0 {
        elfs_calories.push(cur_elf_sum);
    }

    elfs_calories
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
