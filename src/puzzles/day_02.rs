use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_one() -> i32 {
    let file = File::open("input/day_02.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let valid_password = buf_reader
        .lines()
        .map(|e| {
            let rules = e.unwrap();
            let rules = rules.split_ascii_whitespace().collect::<Vec<_>>();
            let min_max = rules[0]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let min = min_max[0];
            let max = min_max[1];
            let letter = rules[1].chars().next().unwrap();
            let content = rules[2];
            let appear_times = content.matches(letter).count() as i32;

            if appear_times > max || appear_times < min {
                0
            } else {
                1
            }
        })
        .sum();

    valid_password
}

pub fn part_two() -> i32 {
    let file = File::open("input/day_02.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let valid_password = buf_reader
        .lines()
        .map(|e| {
            let rules = e.unwrap();
            let rules = rules.split_ascii_whitespace().collect::<Vec<_>>();
            let min_max = rules[0]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let min = min_max[0];
            let max = min_max[1];
            let letter = rules[1].chars().next().unwrap();
            let content = rules[2];

            let appear_times = content
                .match_indices(letter)
                .map(|(i, _)| {
                    let pos = i as i32 + 1;
                    if pos == min || pos == max {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>();

            if appear_times == 1 {
                1
            } else {
                0
            }
        })
        .sum();

    valid_password
}
