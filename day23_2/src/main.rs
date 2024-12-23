use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let pairs = read_file(path)?;

    let connections = match_connection(&pairs);
    let lans = find_p2p_all(&connections);

    Ok(lans[0].join(","))
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

    #[allow(unused)]
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

#[derive(Eq, Debug)]
struct Star(HashSet<String>);

impl Star {
    fn new(set: &HashSet<String>) -> Self {
        Self(set.clone())
    }

    fn is_p2p(&self, other: &HashSet<String>) -> bool {
        let Self(set) = self;

        *set == *other
    }
}
impl PartialEq for Star {
    fn eq(&self, other: &Self) -> bool {
        let Self(set) = self;
        let Self(other_set) = other;

        set == other_set
    }
}
impl std::hash::Hash for Star {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self(set) = self;
        let mut names = Vec::from_iter(set);
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
            .or_insert_with(|| HashSet::from([name2.clone()]))
        ;
        lookup.entry(name2.clone())
            .and_modify(|peers| {
                peers.insert(name1.clone());
            })
            .or_insert_with(|| HashSet::from([name1.clone()]))
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

fn find_p2p_all(connections: &[Connection]) -> Vec<Vec<String>> {
    let mut lookup = HashMap::<String, HashSet<String>>::new();

    for Connection(p1, p2, p3) in connections {
        for p in &[p1, p2, p3] {
            lookup.entry(p.to_string())
                .and_modify(|set| {
                    set.insert(p1.clone());
                    set.insert(p2.clone());
                    set.insert(p3.clone());
                })
                .or_insert_with(|| HashSet::from([p1.clone(), p2.clone(), p3.clone()]))
            ;
        }
    }

    let mut p2p_map = HashMap::<Star, HashSet<String>>::new();

    for (k, v) in lookup {
        p2p_map.entry(Star::new(&v))
            .and_modify(|peers| {
                peers.insert(k.clone());
            })
            .or_insert_with(|| HashSet::<String>::from([k.clone()]))
        ;
    }

    let mut p2p = p2p_map.into_iter()
        .filter_map(|(star, v)| match star.is_p2p(&v) {
            true => Some(Vec::<String>::from_iter(v)),
            false => None
        })
        .map(|mut star| {
            star.sort();
            star
        })
        .collect::<Vec<_>>()
    ;

    p2p.sort();

    p2p
}

#[allow(unused)]
fn count_initial(connections: &[Connection], needle: char) -> usize {
    connections.into_iter()
        .filter(|conn| conn.has_initial(needle))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("co,de,ka,ta", solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn find_conneced_all_example() -> Result<(), Box<dyn std::error::Error>> {
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

        let connections = find_p2p_all(&connections);
        let expect_connections = vec!["co","de","ka","ta"].into_iter().map(String::from).collect::<Vec<_>>();

        assert_eq!(vec![expect_connections], connections);
        Ok(())
    }
}