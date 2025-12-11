/*
 * Day 1: Secret Entrance
 * Ref: https://adventofcode.com/2025/day/1
 */

use std::fs;

pub(crate) struct Day1 {}

impl Day1 {
    pub(crate) fn get_results() -> [String; 2] {
        let rotations = Self::get_input().unwrap();
        return [Self::part1(&rotations), Self::part2(&rotations)];
    }

    fn part1(rotations: &Vec<(String, i32)>) -> String {
        let mut dial_position = 50;
        let mut zero_counter = 0;
        for i in 0..rotations.len() {
            Self::update_dial_position(&mut dial_position, &mut zero_counter, &rotations[i], false);
            if dial_position == 0 {
                zero_counter += 1;
            }
        }
        return zero_counter.to_string();
    }

    fn part2(rotations: &Vec<(String, i32)>) -> String {
        let mut dial_position = 50;
        let mut zero_counter = 0;
        for i in 0..rotations.len() {
            print!("Line: {} ", i);
            Self::update_dial_position(&mut dial_position, &mut zero_counter, &rotations[i], true);
        }
        return zero_counter.to_string();
    }

    fn update_dial_position(
        dial_position: &mut i32,
        zero_counter: &mut i32,
        rotation: &(String, i32),
        is_part_two: bool,
    ) {
        let old_dial_position = *dial_position;
        match rotation.0.as_str() {
            "L" => {
                *dial_position = ((*dial_position + 100) - (rotation.1 % 100)) % 100;
                if is_part_two {
                    *zero_counter += rotation.1 / 100;
                    if (*dial_position == 0 && rotation.1 % 100 != 0)
                        || ((*dial_position > old_dial_position) && old_dial_position != 0)
                    {
                        *zero_counter += 1;
                    }
                }
            }
            "R" => {
                *dial_position = (*dial_position + (rotation.1 % 100)) % 100;
                if is_part_two {
                    *zero_counter += rotation.1 / 100;
                    if (*dial_position < old_dial_position) {
                        *zero_counter += 1;
                    }
                }
            }
            _ => panic!("Invalid direction"),
        }

        println!(
            "{:?}: Old {}, New {}, Total Zeroes {}",
            rotation, old_dial_position, *dial_position, *zero_counter
        );

        *dial_position = dial_position.abs();
    }

    fn get_input() -> Result<(Vec<(String, i32)>), std::io::Error> {
        return match fs::read_to_string("./inputs/day1/day1.txt") {
            Ok(file_contents) => {
                let mut rotations: Vec<(String, i32)> = Vec::new();

                for line in file_contents.lines() {
                    let (direction, stride) = line.split_at_checked(1).unwrap();
                    let stride_int = stride.parse::<i32>().unwrap();
                    rotations.push((direction.to_string(), stride_int));
                }

                Ok(rotations)
            }
            Err(err) => Err(err),
        };
    }
}
