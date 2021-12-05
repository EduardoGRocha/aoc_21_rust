use csv::{ReaderBuilder, Reader};
use serde::Deserialize;
use std::fs;
use itertools::izip;
use std::vec::Vec;
use std::path::Path;

fn read_file(file_name: &str) -> String {
    let file_name_formatted = format!("data/{}", file_name);
    
    fs::read_to_string(file_name_formatted)
        .expect("Can't read file")
} 

fn read_csv_content(file_name: &str) -> Reader<fs::File> {
    // Should overload the / operator to join paths, like Python
    let file_name = format!("data/{}", file_name);
    let file_path = Path::new(&file_name);

    ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(file_path)
        .expect("Couldn't read CSV")
}

fn day01() {
    println!("::: Day 01");

    let file_contents = read_file("day01-1").lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    {
        // Part 1
        let a = &file_contents[0..file_contents.len()-1];
        let b = &file_contents[1..file_contents.len()];
        
        let count = izip!(a, b)
            .map(|(x, y)|  x < y)
            .filter(|x| *x)
            .count();

        println!("\t Part 1: {}", &count);
    }
    
    {
        // Part 2
        let a = &file_contents[0..file_contents.len()-3];
        let b = &file_contents[1..file_contents.len()-2];
        let c = &file_contents[2..file_contents.len()-1];
        let d = &file_contents[3..file_contents.len()];
        
        let count = izip!(a, b, c, d)
            .map(|(w, x, y, z)| (w + x + y, x + y + z))
            .filter(|(x, y)| x < y)
            .count();
    
        println!("\t Part 2: {}", &count);
    }
}

fn day02() {
    println!("::: Day 02");

    // This will look nicer when moved into its own file
    #[derive(Deserialize)]
    struct Direction {
        direction: String,
        amplitude: i32,
    }

    struct Location {
        x: i32,
        y: i32,
        aim: i32,
    }
    

    impl Location { 
        fn move_to_direction(&mut self, direction: &Direction) {
            match direction.direction.as_str() {
                "forward" => self.x += direction.amplitude,
                "down" => self.y += direction.amplitude,
                "up" => self.y -= direction.amplitude,
                _ => {}
            }
        }

        fn move_to_direction_with_aim(&mut self, direction: &Direction) {
            match direction.direction.as_str() {
                "down" => self.aim += direction.amplitude,
                "up" => self.aim -= direction.amplitude,
                "forward" => {
                    self.x += direction.amplitude;
                    self.y += direction.amplitude * self.aim;
                },
                _ => {}
            }
        }
    }

    let mut reader = read_csv_content("day02-1");

    let mut vec = Vec::<Direction>::new();
    for direction in reader.deserialize::<Direction>() { 
        let direction = direction.expect("Error when dealing with result");
        vec.push(direction);
    }

    {
        // Part 1
        let location = vec.iter().fold(
            Location {x:0 , y: 0, aim: 0}, 
            |mut acc, x| { acc.move_to_direction(x); acc }
        );
        println!("\t Part 1: {}", location.x * location.y);
    }

    {
        // Part 2
        let location = vec.iter().fold(
            Location {x:0 , y: 0, aim: 0}, 
            |mut acc, x| { acc.move_to_direction_with_aim(x); acc }
        );
        println!("\t Part 2: {}", location.x * location.y);
    }
}

fn day03() {
    println!("::: Day 03");

    let file_contents = read_file("day03-1");
    let row_size = file_contents.lines().next().unwrap().len();

    {
        // Part 1
        let file_contents = file_contents.lines();

        let mut zero_counts = vec![0 as usize; row_size];
        let mut one_counts = vec![0 as usize; row_size];

        for line in file_contents {
            line.chars()
                .enumerate()
                .for_each(|(i, c)| {
                    match c {
                        '0' => zero_counts[i] += 1,
                        '1' => one_counts[i] += 1,
                        _ => {},
                    }
                });
        }
        
        let gamma_value = zero_counts.iter()
            .zip(one_counts.iter())
            .map(|(zero_count, one_count)| if zero_count > one_count {'0'} else {'1'})
            .collect::<String>();
        let gamma_value = isize::from_str_radix(&gamma_value, 2).unwrap();

        let epsilon_value = zero_counts.iter()
            .zip(one_counts.iter())
            .map(|(zero_count, one_count)| if zero_count < one_count {'0'} else {'1'})
            .collect::<String>();
        let epsilon_value = isize::from_str_radix(&epsilon_value, 2).unwrap();

        println!("\t Part 1: {}", gamma_value * epsilon_value);
    }

    {
    // Part 2
    let mut file_contents = file_contents.lines()
        .collect::<Vec<_>>(); 
    // It's stupid that rust doesn't have a partition method yet
    //  But sort will be comparabily fast (one extra pass only) if numbers don't repeat
    file_contents.sort_unstable();

    let mut oxygen_slice = &file_contents[0..];

    for i in 0..row_size {
        if oxygen_slice.len() == 1 {
            break;
        }

        // TODO this is slow, but the alternative was using .as_bytes 
        //  or refactoring to use arrays instead 
        let partition_point = oxygen_slice
            .partition_point(|&x| x.chars().nth(i).unwrap() == '0');

        if oxygen_slice.len() >= partition_point * 2 {
            oxygen_slice = &oxygen_slice[partition_point..];
        } else {
            oxygen_slice = &oxygen_slice[..partition_point];
        }
    }

        let mut co2_slice = &file_contents[0..];

        // Here I was a bit tired and just copied. When refactoring this will get better
        for i in 0..row_size {
            if co2_slice.len() == 1 {
                break;
            }

            // TODO this is slow, but the alternative was using .as_bytes 
            //  or refactoring to use arrays instead 
            let partition_point = co2_slice
                .partition_point(|&x| x.chars().nth(i).unwrap() == '0');

            if co2_slice.len() >= partition_point * 2 {
                co2_slice = &co2_slice[..partition_point];
            } else {
                co2_slice = &co2_slice[partition_point..];
            }
        }

        let o2 = isize::from_str_radix(oxygen_slice[0], 2).unwrap();
        let co2 = isize::from_str_radix(co2_slice[0], 2).unwrap();
        println!("\t Part 2: {}", o2 * co2);
    }
}

fn day04() {
    // This approach trades off memory for speed
    println!("::: Day 04");

    // Using fix-sized array, but can easily be adapted to use variable size
    const BOARD_SIZE : usize = 5;

    // We could in theory use tighter data types, but that is not necessary
    type NumberIndex = Vec::<(usize, usize, usize)>;
    type NumbersIndex = Vec::<NumberIndex>;
    type BoardCheck = ([usize; BOARD_SIZE], [usize; BOARD_SIZE]);
    
    // Here, I'll deviate from common wisdom and have it mutable
    let file_contents = read_file("day04-1");
    let mut file_contents = file_contents.lines();

    // The implementation assumes that the input does not contain repeated numbers.
    //  But can be adapted to also cope with repeated numbers
    let numbers = file_contents.next().unwrap().split(',');
    let numbers = numbers.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let max_input = match numbers.iter().max() { 
        None => 0,
        Some(i) => *i,
    };

    let mut numbers_idx = NumbersIndex::new();
    let mut board_checks = Vec::<BoardCheck>::new();
    for _i in 0..=max_input {
        numbers_idx.push(NumberIndex::new());
        board_checks.push(([0; BOARD_SIZE], [0; BOARD_SIZE]));
    }

    let mut board_idx: usize = 0;
    while file_contents.next().is_some() {
        for row_idx in 0..BOARD_SIZE {
            let row = file_contents.next();
            row.unwrap()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(col_idx, value)| {
                    numbers_idx[value].push((board_idx, row_idx, col_idx))
                });
        }
        board_idx += 1;
    }

    // Now we play :D
    {
        // Part 1
        'outer: for number in numbers {
            let number_index = &numbers_idx[number];
            for (board_idx, row_idx, col_idx) in number_index {
                board_checks[*board_idx].0[*row_idx] += 1;
                board_checks[*board_idx].1[*col_idx] += 1;

                if board_checks[*board_idx].0[*row_idx] == BOARD_SIZE 
                    || board_checks[*board_idx].1[*col_idx] == BOARD_SIZE {
                    println!("\t Part 1: {}", board_idx + 1);
                    break 'outer;
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    day01();
    day02();
    day03();
    day04();
}
