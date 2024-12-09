use std::{collections::VecDeque, fs::File, io::{BufReader, Read}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (file_spaces, free_spaces) = read_file(path)?;
    
    // dump_spaces(&file_spaces, &free_spaces);

    let cmpact_space = compaction(file_spaces, free_spaces);

    // dump_compact(&cmpact_space);

    let mut i: usize = 0;
    let mut checksum: usize = 0;

    for space in & cmpact_space {
        checksum += 
            std::iter::repeat(space.id).take(space.len)
            .enumerate()
            .map(|(offset, id)| id * (i + offset))
            .sum::<usize>()
        ;
        i += space.len;
    }
    
    Ok(checksum)
}

#[allow(unused)]
fn dump_spaces(file_spaces: &VecDeque<DiskMap>, free_spaces: &VecDeque<DiskMap>) {
    let iter1 = file_spaces.into_iter()
        .filter_map(|x| match x {
            DiskMap::Fill(space) => Some(space),
            DiskMap::Vacant(_) => None,
        })
        .fuse();
    let mut iter2 = free_spaces.into_iter()
        .filter_map(|x| match x {
            DiskMap::Fill(_) => None,
            DiskMap::Vacant(size) => Some(size),
        })
        .fuse();

    for space in iter1 {
        eprint!("{}", (format!("[{}]", space.id).repeat(space.len)));
        if let Some(size) = iter2.next() {
            eprint!("{}", ".".repeat(*size));
        }
    }
    eprintln!("\n");
}

#[allow(unused)]
fn dump_compact(files: &Vec<Space>) {
    let dump = files.into_iter()
        .flat_map(|space| {
            std::iter::repeat(space.id).take(space.len).collect::<Vec<_>>()
        })
    ;
    
    for id in dump {
        eprintln!("{}", id);
    }
}

#[derive(PartialEq, Debug)]
struct Space {
    id: usize,
    len: usize,
}

#[derive(PartialEq, Debug)]
enum DiskMap {
    Fill(Space),
    Vacant(usize),
}

fn read_file<P>(path: P) -> Result<(VecDeque<DiskMap>, VecDeque<DiskMap>), Box<dyn std::error::Error>> 
where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = vec![];
    let mut file_spaces = VecDeque::<DiskMap>::new();
    let mut free_spaces = VecDeque::<DiskMap>::new();

    _ = reader.read_to_end(&mut buf)?;

    for (id, chunk) in buf.chunks(2).enumerate() {
        file_spaces.push_back(DiskMap::Fill(Space{id, len: (chunk[0] - b'0') as usize }));

        if (chunk.len() > 1) && (chunk[1] != b'\n') {
            free_spaces.push_back(DiskMap::Vacant((chunk[1] - b'0') as usize));
        };
    }

    Ok((file_spaces, free_spaces))
}

fn compaction(file_spaces: VecDeque<DiskMap>, free_spaces: VecDeque<DiskMap>) -> Vec<Space> {
    let mut free_spaces = free_spaces;
    let mut file_spaces = file_spaces;
    let mut resolved = VecDeque::<DiskMap>::new();

    while free_spaces.len() > 0 {
        if let Some(DiskMap::Vacant(free_size)) = free_spaces.pop_front() {
            if let Some(file_space) = file_spaces.pop_front() { 
                resolved.push_back(file_space);
            };
            if free_size == 0 { 
                continue;
            }

            let Some(DiskMap::Fill(Space{ id, len})) = file_spaces.pop_back() else {
                break;
            };
    
            match (free_size, len) {
                (free_size, len) if free_size > len => {
                    resolved.push_back(DiskMap::Fill(Space{ id, len }));
                    free_spaces.push_front(DiskMap::Vacant(free_size - len));
                    file_spaces.push_front(DiskMap::Vacant(0));
                }
                (free_size, len) if free_size == len => {
                    resolved.push_back(DiskMap::Fill(Space{ id, len }));
                }
                (free_size, len) => {
                    resolved.push_back(DiskMap::Fill(Space{ id, len: free_size }));
                    file_spaces.push_back(DiskMap::Fill(Space{ id, len: len - free_size }));
                }
            }
        }
    }

    resolved.append(&mut file_spaces);

    resolved.into_iter()
        .filter_map(|dm| match dm {
            DiskMap::Fill(space) => Some(space),
            DiskMap::Vacant(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(1928, actual);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (file_spaces, free_spaces) = read_file("./aoc_input_example_0.txt")?;

        let expect_file_spaces = vec![
            DiskMap::Fill(Space{id: 0, len: 1}), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Fill(Space{id: 2, len: 5}), 
        ];
        let expect_free_spaces = vec![
            DiskMap::Vacant(2), DiskMap::Vacant(4), 
        ];

        assert_eq!(expect_file_spaces, Vec::<DiskMap>::from(file_spaces));
        assert_eq!(expect_free_spaces, Vec::<DiskMap>::from(free_spaces));
        Ok(())
    }

    #[test]
    fn test_compaction_example_0() -> Result<(), Box<dyn std::error::Error>> {
        let file_spaces = VecDeque::<DiskMap>::from(vec![
            DiskMap::Fill(Space{id: 0, len: 1}), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Fill(Space{id: 2, len: 5}), 
        ]);
        let free_spaces = VecDeque::<DiskMap>::from(vec![
            DiskMap::Vacant(2), DiskMap::Vacant(4), 
        ]);

        let expect_file_space = vec![
            Space{id: 0, len: 1},
            Space{id: 2, len: 2},
            Space{id: 1, len: 3},
            Space{id: 2, len: 3},
        ];
        // 022111222

        let file_space = compaction(file_spaces, free_spaces);

        assert_eq!(expect_file_space, file_space);
        Ok(())
    }

    #[test]
    fn test_compaction_example() -> Result<(), Box<dyn std::error::Error>> {
        let file_spaces = VecDeque::<DiskMap>::from(vec![
            DiskMap::Fill(Space{id: 0, len: 2}), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Fill(Space{id: 2, len: 1}), 
            DiskMap::Fill(Space{id: 3, len: 3}), 
            DiskMap::Fill(Space{id: 4, len: 2}), 
            DiskMap::Fill(Space{id: 5, len: 4}), 
            DiskMap::Fill(Space{id: 6, len: 4}), 
            DiskMap::Fill(Space{id: 7, len: 3}), 
            DiskMap::Fill(Space{id: 8, len: 4}), 
            DiskMap::Fill(Space{id: 9, len: 2}), 
        ]);
        let free_spaces = VecDeque::<DiskMap>::from(vec![
            DiskMap::Vacant(3), 
            DiskMap::Vacant(3), 
            DiskMap::Vacant(3), 
            DiskMap::Vacant(1), 
            DiskMap::Vacant(1), 
            DiskMap::Vacant(1), 
            DiskMap::Vacant(1), 
            DiskMap::Vacant(1), 
            DiskMap::Vacant(0), 
        ]);
        // 00...111...2...333.44.5555.6666.777.888899

        dump_spaces(&file_spaces, &free_spaces);

        let expect_file_space = vec![
            Space{id: 0, len: 2},
            Space{id: 9, len: 2},
            Space{id: 8, len: 1},
            Space{id: 1, len: 3},
            Space{id: 8, len: 3},
            Space{id: 2, len: 1},
            Space{id: 7, len: 3},
            Space{id: 3, len: 3},
            Space{id: 6, len: 1},
            Space{id: 4, len: 2},
            Space{id: 6, len: 1},
            Space{id: 5, len: 4},
            Space{id: 6, len: 1},
            Space{id: 6, len: 1},
        ];
        // 0099811188827773336446555566

        let file_space = compaction(file_spaces, free_spaces);

        assert_eq!(expect_file_space, file_space);
        Ok(())
    }

}

