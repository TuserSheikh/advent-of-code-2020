use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_one() -> i32 {
    let file = File::open("input/day_01.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let expenses = buf_reader
        .lines()
        .map(|e| e.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for i in &expenses {
        for j in &expenses {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    1
}

pub fn part_two() -> i32 {
    let file = File::open("input/day_01.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let expenses = buf_reader
        .lines()
        .map(|e| e.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for i in &expenses {
        for j in &expenses {
            for k in &expenses {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }

    1
}
