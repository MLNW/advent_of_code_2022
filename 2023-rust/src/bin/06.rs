use itertools::Itertools;

advent_of_code::solution!(6);

struct Race {
    total_time: u32,
    record_distance: u32,
}

impl Race {
    fn distance(&self, time_holding_button: u32) -> u32 {
        let left_over_ms = self.total_time - time_holding_button;
        time_holding_button * left_over_ms
    }

    fn simulate(&self) -> Vec<u32> {
        (1..self.total_time - 1)
            .map(|time_holding_button| self.distance(time_holding_button))
            .filter(|distance| distance > &self.record_distance)
            .collect_vec()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_input(input);

    races
        .iter()
        .map(|race| race.simulate().len() as u32)
        .reduce(|acc, possible_wins| acc * possible_wins)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = parse_numbers(times.strip_prefix("Time:").unwrap());
    let distances = parse_numbers(distances.strip_prefix("Distance:").unwrap());

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            total_time: *time,
            record_distance: *distance,
        })
        .collect_vec()
}

fn parse_numbers(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
