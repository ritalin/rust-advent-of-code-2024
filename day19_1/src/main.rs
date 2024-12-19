use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (candidates, patterns) = read_file(path)?;

    let total = patterns.into_iter()
        .filter(|p| {
            match_pattern(&p, &candidates)
        })
        .count()
    ;

    Ok(total)
}

fn read_file<P>(path: P) -> Result<(HashSet<String>, Vec<String>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut candidates = HashSet::<String>::new();
    let mut patterns = vec![];
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim_end();
        if s.len() == 0 { break; }

        s.split(",").for_each(|p| {
            candidates.insert(p.trim().to_string());
        });
        buf.clear();
    }
    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim();
        if s.len() == 0 { break; }
        patterns.push(s.to_string());
        buf.clear();
    }

    Ok((candidates, patterns))
}

fn match_pattern(pattern: &str, candidates: &HashSet<String>) -> bool {
    match_pattern_internal(pattern, candidates, 0)
}

fn match_pattern_internal(pattern: &str, candidates: &HashSet<String>, start: usize) -> bool {
    if start >= pattern.len() {
        return true;
    }

    for end in (start + 1)..=pattern.len() {
        let s = &pattern[start..end];

        if candidates.contains(s) {
            if match_pattern_internal(&pattern, candidates, end) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(6, solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (candidates, patterns) = read_file("./aoc_input_example.txt")?;

        let expect_candidates = 
            ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].into_iter()
            .map(|p| p.to_string()).collect::<HashSet<String>>()
        ;
        let expect_patterns = vec![
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ];

        assert_eq!(expect_candidates, candidates);
        assert_eq!(expect_patterns, patterns);
        Ok(())
    }

    #[test]
    fn match_pattern_example() -> Result<(), Box<dyn std::error::Error>> {
        let candidates = 
            ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].into_iter()
            .map(|p| p.to_string()).collect::<HashSet<String>>()
        ;

        assert_eq!(true, match_pattern("brwrr", &candidates));
        assert_eq!(false, match_pattern("ubwu", &candidates));
        assert_eq!(true, match_pattern("bwurrg", &candidates));
        assert_eq!(false, match_pattern("bbrgwb", &candidates));
   
        Ok(())
    }

    #[test]
    fn match_pattern_input() {
        let candidates = 
            [
                "rrgbg", "rgguubg", "rbru", "rb", "rrrw", "wbu", "gbgb", "uururg", "ubru", "rugb", "bbru", "b", "rggurg", "wgru", "bgrwb", "rrgubg", "ubrrbg", "wgurru", "rrrrw", "rbrwu", "wubwb", "wrbbr", "bgbu", "brrww", "brg", "gbwu", "wrubuur", "gur", "grbr", "gruwrrbg", "bgwr", "wgugbgb", "rwbru", "wuwr", "rrg", "ruwg", "rgwgr", "ubu", "wbr", "bwg", "gbu", "bgrwrb", "wwrw", "bb", "gr", "rug", "grr", "ubwb", "rbruwbu", "guggug", "ugu", "rwbuu", "bbur", "wbrww", "wubw", "br", "gruu", "gwr", "wrrwwu", "wug", "bgu", "bgb", "wugb", "grb", "rbbg", "grgbwgb", "rwu", "bwrbb", "uwrbbru", "bbu", "wrg", "rwg", "ggbgbg", "wbwru", "wrgrw", "bburr", "bgr", "rgbg", "bwwwugrb", "uwu", "rwuwrg", "gguburu", "uwubwu", "wb", "wgguug", "ugrbw", "rbgu", "rwwu", "ggbbgb", "gw", "wgg", "rrw", "uurg", "uubg", "bgurgb", "uwgbr", "wbrwu", "rrbb", "rww", "bwgb", "uugru", "ggrbub", "wubgw", "rwwugr", "uuwur", "burbrg", "bbr", "ggb", "wgr", "gwbgw", "rrbu", "wuwwrwb", "rbbr", "uubw", "buuu", "ubbwggu", "brr", "urguww", "uggwwgw", "ugw", "ug", "rub", "grrrwu", "wurburgg", "wwww", "ruu", "wrb", "rrur", "urggr", "bwbwwwrg", "wwbu", "bbg", "uwb", "ggugu", "gbbr", "rbrgw", "bg", "bbuu", "uwrgrg", "ggbwrgw", "wwgwugr", "wgbwuwb", "gbrwg", "bbw", "wwrgu", "rgrwrbwb", "wwwg", "gugbgbg", "bwrwub", "rw", "gbb", "ggw", "rrb", "urrbbwb", "urbu", "grg", "rbugggb", "bruwg", "uubgwgw", "bw", "uubgu", "rru", "rwww", "gu", "wrbrg", "guu", "wgu", "ruuru", "wubrrg", "brugrg", "uuggb", "wrurg", "rgb", "rbu", "urg", "buw", "grwbg", "gwuw", "grbbg", "wuwu", "gbgbbw", "uugrwrww", "wbguub", "grrgbug", "u", "uug", "uwr", "wrwb", "gbg", "bru", "wwg", "bbwur", "bbbbbbbw", "bugbb", "rwurrgr", "wwb", "ugbbw", "bug", "grgg", "bgwgrrb", "uggu", "rurguru", "wwr", "rbw", "ubwwgw", "wrrw", "rrbg", "g", "rgrbub", "gruggw", "bur", "ubgb", "bgg", "uwur", "ur", "bbbwb", "gwbu", "wuww", "uruuuubb", "wbru", "ugggg", "gbuu", "rgg", "wwwug", "gbrwgg", "guwgr", "urw", "wbur", "bu", "rur", "rrubr", "ugb", "ubrgrb", "rbbgg", "rbwrg", "rggw", "rguug", "wwu", "uur", "wrrguggu", "brbb", "bwwwbr", "guw", "gugw", "wgubwu", "rrr", "ub", "ru", "brrbbruw", "bwwgrrbb", "rubgu", "uggwwg", "wbb", "rwr", "r", "ruuu", "wbggg", "wwwu", "grbrb", "rrrrr", "ruwb", "wuuw", "rbbb", "ugg", "wbg", "rguwr", "wbw", "wbuwg", "ubbgrbb", "urugu", "rbubw", "wrgwuuwu", "grw", "ubbbu", "bwurb", "rwuu", "rgbbwur", "urbuu", "uww", "wuw", "bwurgu", "wur", "wuwwrbur", "wwuu", "wbgw", "grbbbw", "wuub", "urb", "wuuguuwg", "wg", "gwrgr", "gbug", "gbrbw", "rwgg", "brbbu", "bwww", "wrbbbu", "wwgbggb", "burug", "gww", "ubgrrb", "bgw", "rgbb", "brb", "wbuu", "brw", "ggrbrbwg", "uwruw", "wrgurrbb", "guwrgu", "rr", "ugggrbr", "bgrgg", "urgbrbrb", "wrw", "wwubggu", "ugurwg", "grrg", "bwrb", "rrgwuuw", "rrru", "wurg", "bgurwu", "rgurbb", "uuub", "uuwb", "rrrg", "wrgrrrww", "uwg", "bgbguu", "wrgwb", "wwwru", "rg", "rrurwur", "urrbb", "bww", "rrwwbbb", "ubr", "wgrbu", "gru", "ubg", "gb", "ggr", "gbwbwru", "uuw", "rbb", "gugwrg", "gwb", "bbb", "rgrggw", "uw", "wggbbg", "rwgubgb", "gwbw", "brrr", "gugbwu", "rwrr", "uuuwbb", "ugrrr", "rgu", "wubrwr", "wrr", "bwb", "bwbgurb", "rrrub", "wub", "rwgrug", "rurb", "gwrgrbr", "rbrwbb", "wwrgbg", "urr", "bwu", "bwgu", "rbuuw", "bbub", "wbbwubu", "ggg", "wbwwgwb", "buwbg", "wwwuru", "rbg", "gubugg", "gbwb", "www", "rbr", "ubur", "grbbw", "wgw", "gbw", "wgrw", "rguuuu", "rwb", "gbrw", "bbbb", "gururbb", "rbggwu", "bub", "bgbr", "gurb", "rrbgrb", "rurwb", "ugr", "wgwrg", "ruw", "bgrr", "gwu", "gbr", "uwugwru", "wru", "wr", "ugbruw", "grbubb", "gwg", "gwggurg", "gugbr", "ugubbgb", "uwrw", "uru", "bwr", "rgr", "rbgrwg", "buuw", "ugbur", "urgub", "wurrrgww", "rgguuwru", "burr", "rgw", "rbruw", "rwbwbbr", "bwguw", "uub", "wgb", "rgur", "ubb", "urwg", "rwgww", "wgub", "rwwrgu", "gg", "rgug", "gub", "bggr", "uuu", "uu", "rbug"
            ].into_iter()
            .map(|c| c.to_string())
            .collect::<HashSet<_>>()
        ;
            
        let p = "wwgwuurrurrbbgwuwwbgrbbrbruugwwgugbrbrgguuwrugwuguurbgwrru";
        assert_eq!(false, match_pattern(p, &candidates));
    }
}
