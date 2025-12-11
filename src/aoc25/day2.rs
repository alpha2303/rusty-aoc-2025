/*
 * Day 1: Secret Entrance
 * Ref: https://adventofcode.com/2025/day/1
 */

use std::fs;

pub(crate) struct Day2 {}

impl Day2 {
    pub(crate) fn get_results() -> [String; 2] {
        let id_ranges = Self::get_input().unwrap();
        println!("{:?}", id_ranges);
        return [Self::part1(&id_ranges), Self::part2(&id_ranges)];
    }

    fn part1(id_ranges: &Vec<(String, String)>) -> String {
        let mut zero_counter = 0;
        return zero_counter.to_string();
    }

    fn part2(id_ranges: &Vec<(String, String)>) -> String {
        let mut zero_counter = 0;
        return zero_counter.to_string();
    }

    fn get_input() -> Result<(Vec<(String, String)>), std::io::Error> {
        return match fs::read_to_string("./inputs/day2/day2.txt") {
            Ok(file_contents) => {
                let range_line = file_contents.lines().nth(0).unwrap();
                let id_ranges: Vec<(String, String)> = range_line
                    .split(",")
                    .map(|range| {
                        let parts: Vec<&str> = range.split("-").collect();
                        (parts[0].to_string(), parts[1].to_string())
                    })
                    .collect();

                Ok(id_ranges)
            }
            Err(err) => Err(err),
        };
    }
}
