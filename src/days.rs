#![allow(dead_code)]

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub fn day1() {
    day1_p1();
    day1_p2();
}

pub fn day2() {
    day2_p1();
    day2_p2()
}

pub fn day3() {
    day3_p1();
}

fn day1_p1() {
    let file_name: &str = "data/day1.txt";

    let contents: String = fs::read_to_string(file_name).expect("Found file");
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let lines = contents.split('\n');
    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();
        if nums.len() < 2 {
            break;
        }
        let num1: i32 = nums[0].parse::<i32>().unwrap();
        let num2: i32 = nums[1].parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    let mut total: i32 = 0;
    list1.sort();
    list2.sort();

    for (num1, num2) in zip(&list1, &list2) {
        total += (num1 - num2).abs()
    }
    println!("Day 1 Part 1 Answer: {}", total)
}

fn day1_p2() {
    let file_name = "data/day1.txt";
    let contents: String = fs::read_to_string(file_name).expect("Found file");
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let lines = contents.split('\n');
    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();
        if nums.len() < 2 {
            break;
        }
        let num1: i32 = nums[0].parse::<i32>().unwrap();
        let num2: i32 = nums[1].parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    let mut map: HashMap<&i32, i32> = HashMap::new();
    for num in list2.iter() {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut total = 0;
    for num in list1.iter() {
        let multiplier = match map.get(num) {
            Some(n) => *n,
            None => 0,
        };
        total += num * multiplier;
    }
    println!("Day 1 Part 2 Answer: {}", total);
}

fn day2_p1() {
    let file_name = "data/day2.txt";
    let contents: String = fs::read_to_string(file_name).expect("Found file");

    let lines = contents.split('\n');

    let mut num_safe: i32 = 0;

    for line in lines {
        let arr: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if arr.len() <= 1 {
            break;
        }

        let is_positive = arr[0] < arr[1];
        let mut is_safe = true;

        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff == 0 {
                is_safe = false;
                break;
            }
            if diff < 0 && is_positive {
                is_safe = false;
                break;
            }

            if diff > 0 && !is_positive {
                is_safe = false;
                break;
            }

            if diff.abs() != 1 && diff.abs() != 2 && diff.abs() != 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            num_safe += 1;
        }
    }

    println!("Day 2 Part 1 Answer: {}", num_safe);
}

fn day2_p2() {
    let file_name = "data/day2.txt";
    let contents: String = fs::read_to_string(file_name).expect("Found file");

    let lines = contents.split('\n');

    let mut num_safe: i32 = 0;

    for line in lines {
        let arr: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if arr.len() <= 1 {
            break;
        }

        let is_positive = arr[0] < arr[1];
        let mut is_safe = true;
        let mut rem_safe = true;

        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff == 0
                || diff < 0 && is_positive
                || diff > 0 && !is_positive
                || diff.abs() != 1 && diff.abs() != 2 && diff.abs() != 3
            {
                if rem_safe {
                    rem_safe = false;
                } else {
                    is_safe = false;
                    break;
                }
            }
        }

        if is_safe {
            num_safe += 1;
        }
    }

    println!("Day 2 Part 2 Answer: {}", num_safe);
}

fn day3_p1() {
    let file_name = "data/day3.txt";
    let contents: String = fs::read_to_string(file_name).expect("Found file");

    let lines = contents.split('\n');

    let mut result: i32 = 0;

    for line in lines {
        let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

        for cap in re.captures_iter(line) {
            let num1_str = &cap[1];
            let num2_str = &cap[2];

            // Parse the captured strings into numbers
            let num1: i32 = num1_str.parse().unwrap();
            let num2: i32 = num2_str.parse().unwrap();
            result += num1 * num2;
        }
    }

    println!("Day3 Part 1 Answer: {}", result);
}
