use std::fs::read_to_string;

pub fn find_safe_reports(input_path: &str) -> u32 {
    return 0;
}

fn extract_puzzle(input_path: &str) -> Puzzle {
    let mut puzzle = Puzzle::new();
    read_to_string(input_path)
    .unwrap()
    .lines()
    .for_each(|line| puzzle.add_report(line));

    puzzle
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Puzzle {
    reports: Vec<Report>
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Report {
    values: Vec<u32>
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            reports: Vec::new()
        }
    }

    fn add_report(&mut self, line: &str) {
        let numbers: Vec<u32> = line
        .split_whitespace() 
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

        self.reports.push(Report {
            values: numbers
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_puzzle() {
        assert_eq!(extract_puzzle("tests/resources/puzzle.txt"), 
        Puzzle {
            reports: vec![Report {
                values: vec![7, 6, 4, 2, 1]
            },
            Report {
                values: vec![1, 2, 7, 8, 9]
            },
            Report {
                values: vec![9, 7, 6, 2, 1]
            },
            Report {
                values: vec![1, 3, 2, 4, 5]
            },
            Report {
                values: vec![8, 6, 4, 4, 1]
            },
            Report {
                values: vec![1, 3, 6, 7, 9]
            }]
        })
        
    }
    
}