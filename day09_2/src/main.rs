use std::{collections::VecDeque, fs::File, io::{BufReader, Read}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let files = read_file(path)?;
    
    let cmpact_space = compaction(files);
    
    Ok(checksum(&cmpact_space))
}

#[allow(unused)]
fn dump_spaces(files: &VecDeque<DiskMap>) {
    for dm in files {
        match dm {
            DiskMap::Fill(space) => {
                eprint!("{}", (format!("[{}]", space.id).repeat(space.len)));
            }
            DiskMap::Vacant(size) => {
                eprint!("{}", ".".repeat(*size));
            }
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

#[derive(PartialEq, Clone, Debug)]
struct Space {
    id: usize,
    len: usize,
}

#[derive(PartialEq, Debug)]
enum DiskMap {
    Fill(Space),
    Vacant(usize),
}

fn read_file<P>(path: P) -> Result<Vec<DiskMap>, Box<dyn std::error::Error>> 
where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = vec![];
    let mut files = Vec::<DiskMap>::new();

    _ = reader.read_to_end(&mut buf)?;

    for (id, chunk) in buf.chunks(2).enumerate() {
        files.push(DiskMap::Fill(Space{id, len: (chunk[0] - b'0') as usize }));

        if (chunk.len() > 1) && (chunk[1] != b'\n') {
            files.push(DiskMap::Vacant((chunk[1] - b'0') as usize));
        };
    }

    Ok(files)
}

fn compaction(files: Vec<DiskMap>) -> Vec<DiskMap> {
    let mut files = files;
    let mut i: usize = files.len();

    while i > 0 {
        i -= 1;
        let space = {
            let Some(DiskMap::Fill(space)) = files.get(i) else {
                continue;
            };
            space.clone()
        };
        
        let vacant = files[0..i].iter().position(|dm| match dm {
            DiskMap::Vacant(free_space) if space.len <= *free_space => true,
            _ => false,
        });

        if let Some(vacant) = vacant {
            if let Some(&mut DiskMap::Vacant(free_space)) = files.get_mut(vacant) {
                match (free_space, space.len) {
                    (free_space, len) if free_space == len => {
                        files[vacant] = DiskMap::Fill(space.clone());
                    }
                    (free_space, len) => {
                        files[vacant] = DiskMap::Vacant(free_space - len);
                        files.insert(vacant, DiskMap::Fill(space.clone()));
                        i += 1;
                    }
                }
                files[i] = DiskMap::Vacant(space.len);
           }
        }
    }

    files
}

fn checksum(files: &[DiskMap]) -> usize {
    let mut checksum: usize = 0;
    let mut i: usize = 0;

    for dm in files {
        match dm {
            DiskMap::Fill(space) => { 
                checksum += 
                    std::iter::repeat(space.id).take(space.len)
                    .enumerate()
                    .map(|(offset, id)| id * (i + offset))
                    .sum::<usize>()
                ;
                i += space.len;
            }
            DiskMap::Vacant(size) => {
                i += size;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[ignore]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(1928, actual);
        Ok(())
    }

    #[test]
    fn test_compaction_example_0() -> Result<(), Box<dyn std::error::Error>> {
        let file_space = vec![
            DiskMap::Fill(Space{id: 0, len: 1}), 
            DiskMap::Vacant(2), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Vacant(4), 
            DiskMap::Fill(Space{id: 2, len: 5}), 
        ];
        //0..111....22222

        let expect_file_space = vec![
            DiskMap::Fill(Space{id: 0, len: 1}), 
            DiskMap::Vacant(2), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Vacant(4), 
            DiskMap::Fill(Space{id: 2, len: 5}), 
        ];
        // 022111222

        let file_space = compaction(file_space);

        assert_eq!(expect_file_space, file_space);
        Ok(())
    }

    #[test]
    fn test_compaction_example() -> Result<(), Box<dyn std::error::Error>> {
        let file_spaces = vec![
            DiskMap::Fill(Space{id: 0, len: 2}), 
            DiskMap::Vacant(3), 
            DiskMap::Fill(Space{id: 1, len: 3}), 
            DiskMap::Vacant(3), 
            DiskMap::Fill(Space{id: 2, len: 1}), 
            DiskMap::Vacant(3), 
            DiskMap::Fill(Space{id: 3, len: 3}), 
            DiskMap::Vacant(1), 
            DiskMap::Fill(Space{id: 4, len: 2}), 
            DiskMap::Vacant(1), 
            DiskMap::Fill(Space{id: 5, len: 4}), 
            DiskMap::Vacant(1), 
            DiskMap::Fill(Space{id: 6, len: 4}), 
            DiskMap::Vacant(1), 
            DiskMap::Fill(Space{id: 7, len: 3}), 
            DiskMap::Vacant(1), 
            DiskMap::Fill(Space{id: 8, len: 4}), 
            DiskMap::Vacant(0), 
            DiskMap::Fill(Space{id: 9, len: 2}), 
        ];
        // 00...111...2...333.44.5555.6666.777.888899

        let expect_file_space = vec![
            DiskMap::Fill(Space{id: 0, len: 2}),
            DiskMap::Fill(Space{id: 9, len: 2}),
            DiskMap::Fill(Space{id: 2, len: 1}),
            DiskMap::Fill(Space{id: 1, len: 3}),
            DiskMap::Fill(Space{id: 7, len: 3}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 4, len: 2}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 3, len: 3}),
            DiskMap::Vacant(1),
            DiskMap::Vacant(2),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 5, len: 4}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 6, len: 4}),
            DiskMap::Vacant(1),
            DiskMap::Vacant(3),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 8, len: 4}),
            DiskMap::Vacant(0), 
            DiskMap::Vacant(2),
        ];
        // 00992111777.44.333....5555.6666.....8888..

        let file_space = compaction(file_spaces);

        assert_eq!(expect_file_space, file_space);
        Ok(())
    }

    #[test]
    fn checksum_example() -> Result<(), Box<dyn std::error::Error>> {
        let file_space = vec![
            DiskMap::Fill(Space{id: 0, len: 2}),
            DiskMap::Fill(Space{id: 9, len: 2}),
            DiskMap::Fill(Space{id: 2, len: 1}),
            DiskMap::Fill(Space{id: 1, len: 3}),
            DiskMap::Fill(Space{id: 7, len: 3}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 4, len: 2}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 3, len: 3}),
            DiskMap::Vacant(4),
            DiskMap::Fill(Space{id: 5, len: 4}),
            DiskMap::Vacant(1),
            DiskMap::Fill(Space{id: 6, len: 4}),
            DiskMap::Vacant(5),
            DiskMap::Fill(Space{id: 8, len: 4}),
            DiskMap::Vacant(2),
        ];

        assert_eq!(2858, checksum(&file_space));
        Ok(())
    }
}

