use std::{collections::VecDeque, fs::read_to_string};

pub fn optimize_checksum(input_path: &str) -> u32 {
    let files = extract_original_file(input_path);

    let ordered_files = re_order_files(files);

    ordered_files.into_iter().enumerate().fold(0, |acc, (index, file)| {
        acc + (file.id * index) as u32
    })
}


fn re_order_files(files: Vec<File>) -> Vec<File> {
    if files.len() == 0 {
        return files;
    }
    let mut ordered_files: Vec<File> = vec![files.get(0).unwrap().clone()];
    let mut queue: VecDeque<File> = VecDeque::from(files.clone().into_iter().filter(|file| !file.free_space).collect::<Vec<File>>());


    for file_index in (1..files.len()).rev() {
        let file = files.get(file_index).unwrap().clone();
        if file.free_space {
            let mut capacity_to_insert = file.capacity;

            loop {
                //While capacity not absorbed => get last in queue and replace the last in queu
                let last_pending_in_queue = queue.pop_front();
                if last_pending_in_queue.is_some() {
                    let new_capacity = file.capacity - last_pending_in_queue.unwrap().capacity;
                } else {
                    break;
                }
            }
            
        } else {
            ordered_files.push(file);
        }
    }

    ordered_files
} 

fn extract_original_file(input_path: &str) -> Vec<File> {
    read_to_string(input_path)
    .unwrap()
    .lines()
    .last()
    .unwrap()
    .chars()
    .enumerate()
    .fold(Vec::new(), |mut acc, (index, number)| {
        acc.push(File  {
            id: index,
            capacity: number.to_digit(10).unwrap(),
            free_space: index % 2 != 0
        });
        acc
    })
}

#[derive(PartialEq, Debug, Clone)]
struct File {
    id: usize,
    capacity: u32,
    free_space: bool
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn should_add_last_content_in_free_space_with_remaining() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 1,
                free_space: true
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            
            File {
                id: 3,
                capacity: 2,
                free_space: true
            },
            File {
                id: 4,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            },
            File {
                id: 3,
                capacity: 2,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_add_last_content_in_free_space_without_remaining() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: true
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            
            File {
                id: 3,
                capacity: 2,
                free_space: true
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_put_last_file_into_empty_file_when_same_capacity() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: true
            },
            File {
                id: 2,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 2,
                capacity: 2,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_not_order_when_there_is_no_free_space() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files.clone()), files);
    }

    #[test]
    fn should_ordered_file_return_empty_when_files_empty() {
        assert_eq!(re_order_files(Vec::new()), Vec::new());
    }

    #[test]
    fn should_extract_original_file() {
        assert_eq!(extract_original_file("tests/resources/light_puzzle.txt"), vec![
            File {
                id: 0,
                capacity: 3,
                free_space: false
            },
            File {
                id: 1,
                capacity: 4,
                free_space: true
            },
            File {
                id: 2,
                capacity: 6,
                free_space: false
            },
            File {
                id: 3,
                capacity: 3,
                free_space: true
            },
        ]);
    }
    

}