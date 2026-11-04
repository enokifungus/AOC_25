use itertools::Itertools;
use std::fs::read_to_string;

const DATA:&str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn start_bits(s: &str) -> u16 {
    s.chars().enumerate().map(|(i,c)| if c != '.' { 1<<i } else { 0 }).sum::<u16>()
}

fn toggle_bits(s: &str) -> u16 {
    s.split(',').map(|t| 1<<t.parse::<u16>().unwrap()).sum::<u16>()
}

#[derive(Debug)]
struct Machine {
    st: u16,
    tg: Vec<u16>,
}

impl Machine {
    fn new(sc: &str) -> Self {
        let mut a:Vec<_> = sc.split(' ').collect();
        let st = start_bits(a.remove(0).strip_prefix('[').unwrap().strip_suffix(']').unwrap());
        a.pop();
        let tg:Vec<_> = a.into_iter().map(|s| toggle_bits(s.strip_prefix('(').unwrap().strip_suffix(')').unwrap()))
            .collect();
        Machine { st,tg }
    }
}

fn main() {
    let sc = DATA;
    let sc = read_to_string("data").expect("file 'data'");
    let ar:Vec<_> = sc.lines().map(|s| Machine::new(s)).collect();
    let n1 = ar.iter().map(|m| (0..).filter(|&k| (0..m.tg.len()).combinations(k)
            .any(|a| a.into_iter().map(|i| m.tg[i]).fold(0,|a,b| a^b) == m.st)).next().unwrap()).sum::<usize>();
    println!("{n1}");
}
