use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let pairs = read_file(path)?;

    let connections = match_connection(&pairs);
    let total = count_initial(&connections, 't');

    Ok(total)
}

fn read_file<P>(path: P) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{

    let reader = BufReader::new(File::open(path)?);
    
    let pairs = reader.lines()
        .map(|s| s.unwrap().split("-").map(String::from).collect::<Vec<_>>())
        .map(|pair| (pair[0].to_string(), pair[1].to_string()))
        .collect::<Vec<_>>()
    ;

    Ok(pairs)
}

#[derive(Eq, Debug)]
struct Connection(String, String, String);

impl Connection {
    fn new(name1: &str, name2: &str, name3: &str) -> Self {
        Self(name1.to_string(), name2.to_string(), name3.to_string())
    }

    fn has_initial(&self, needle: char) -> bool {
        let Self(name_1, name_2, name_3) = self;

        name_1.starts_with(needle) || name_2.starts_with(needle) || name_3.starts_with(needle)
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        let Self(name_1, name_2, name_3) = self;
        let Self(other_name_1, other_name_2, other_name_3) = other;

        let haystack = HashSet::from([other_name_1, other_name_2, other_name_3]);

        haystack.contains(&name_1) && haystack.contains(&name_2) && haystack.contains(&name_3)
    }
}
impl std::hash::Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self(name_1, name_2, name_3) = self;

        let mut names = [name_1, name_2, name_3];
        names.sort();
        names.hash(state);
    }
}

fn match_connection(pairs: &[(String, String)]) -> Vec<Connection> {
    let mut connections = HashSet::new();

    let mut lookup = HashMap::<String, HashSet<String>>::new();

    for (name1, name2) in pairs {
        lookup.entry(name1.clone())
            .and_modify(|peers: &mut _| {
                peers.insert(name2.clone());
            })
            .or_insert(HashSet::from([name2.clone()]))
        ;
        lookup.entry(name2.clone())
            .and_modify(|peers| {
                peers.insert(name1.clone());
            })
            .or_insert(HashSet::from([name1.clone()]))
        ;
    }

    for (name1, name2) in pairs {
        if let Some((peers1, peers2)) = lookup.get(&name1.clone()).zip(lookup.get(&name2.clone())) {
            let peers = peers1.intersection(&peers2).cloned().collect::<Vec<_>>();

            for peer in &peers {
                connections.insert(Connection::new(name1, name2, peer));
            }
        }
    }

    connections.into_iter().collect()
}

fn count_initial(connections: &[Connection], needle: char) -> usize {
    connections.into_iter()
        .filter(|conn| conn.has_initial(needle))
        .count()
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(7, solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let pairs = read_file("./aoc_input_example.txt")?;
        let expect_pairs = vec![
            ("kh", "tc"), ("qp", "kh"), ("de", "cg"), ("ka", "co"), ("yn", "aq"), ("qp", "ub"), ("cg", "tb"), ("vc", "aq"), 
            ("tb", "ka"), ("wh", "tc"), ("yn", "cg"), ("kh", "ub"), ("ta", "co"), ("de", "co"), ("tc", "td"), ("tb", "wq"), 
            ("wh", "td"), ("ta", "ka"), ("td", "qp"), ("aq", "cg"), ("wq", "ub"), ("ub", "vc"), ("de", "ta"), ("wq", "aq"), 
            ("wq", "vc"), ("wh", "yn"), ("ka", "de"), ("kh", "ta"), ("co", "tc"), ("wh", "qp"), ("tb", "vc"), ("td", "yn"), 
        ].into_iter().map(|(name1, name2)| (name1.to_string(), name2.to_string())).collect::<Vec<_>>();

        assert_eq!(expect_pairs, pairs);
        Ok(())
    }

    #[test]
    fn match_connection_example() -> Result<(), Box<dyn std::error::Error>> {
        let pairs = vec![
            ("kh", "tc"), ("qp", "kh"), ("de", "cg"), ("ka", "co"), ("yn", "aq"), ("qp", "ub"), ("cg", "tb"), ("vc", "aq"), 
            ("tb", "ka"), ("wh", "tc"), ("yn", "cg"), ("kh", "ub"), ("ta", "co"), ("de", "co"), ("tc", "td"), ("tb", "wq"), 
            ("wh", "td"), ("ta", "ka"), ("td", "qp"), ("aq", "cg"), ("wq", "ub"), ("ub", "vc"), ("de", "ta"), ("wq", "aq"), 
            ("wq", "vc"), ("wh", "yn"), ("ka", "de"), ("kh", "ta"), ("co", "tc"), ("wh", "qp"), ("tb", "vc"), ("td", "yn"), 
        ].into_iter().map(|(name1, name2)| (name1.to_string(), name2.to_string())).collect::<Vec<_>>();

        let connections = match_connection(&pairs);
        let expect_connections = vec![
            ("aq","cg","yn"), 
            ("aq","vc","wq"), 
            ("co","de","ka"), 
            ("co","de","ta"), 
            ("co","ka","ta"), 
            ("de","ka","ta"), 
            ("kh","qp","ub"), 
            ("qp","td","wh"), 
            ("tb","vc","wq"), 
            ("tc","td","wh"), 
            ("td","wh","yn"), 
            ("ub","vc","wq"),             
        ].into_iter().map(|(name1, name2, name3)| Connection::new(name1, name2, name3)).collect::<Vec<_>>();

        assert_eq_unordered!(expect_connections, connections);
        Ok(())
    }

    #[test]
    fn count_t_example() -> Result<(), Box<dyn std::error::Error>> {
        let connections = vec![
            ("aq","cg","yn"), 
            ("aq","vc","wq"), 
            ("co","de","ka"), 
            ("co","de","ta"), 
            ("co","ka","ta"), 
            ("de","ka","ta"), 
            ("kh","qp","ub"), 
            ("qp","td","wh"), 
            ("tb","vc","wq"), 
            ("tc","td","wh"), 
            ("td","wh","yn"), 
            ("ub","vc","wq"),             
        ].into_iter().map(|(name1, name2, name3)| Connection::new(name1, name2, name3)).collect::<Vec<_>>();
        
        assert_eq!(7, count_initial(&connections, 't'));

        Ok(())
    }
}