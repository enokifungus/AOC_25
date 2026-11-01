use std::collections::HashSet;
use std::fs::read;

fn parse_source(sc: &[u8]) -> (Vec<&[u8]>,usize) {
    let wd = sc.iter().position(|&i| i == b'\n').unwrap();
    let ar:Vec<_> = (0..(sc.len() + 1)/(wd + 1)).step_by(2).skip(1)
        .map(|i| &sc[i*(wd + 1)..(i*(wd + 1) + wd)]).collect();
    let st = ar[0].iter().position(|&i| i != b'.').unwrap();
    (ar,st)
}

fn part1(sc: &[&[u8]],st: usize) -> usize {
    let mut ty = HashSet::new();
    ty.insert(st);
    let mut n1 = 0;
    for l in sc.iter() {
        for i in (0..l.len()).filter(|&i| l[i] != b'.') {
            if ty.contains(&i) {
                n1 += 1;
                ty.remove(&i);
                ty.insert(i - 1);
                ty.insert(i + 1);
            }
        }
    }
    n1
}

fn main() {
    let sc = read("data").expect("file 'data'");
    let (ar,st) = parse_source(&sc);
    let n1 = part1(&ar,st);
    println!("PART1: {n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&[u8] = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn _1() {
        let sc = DATA;
        let (sc,st) = parse_source(sc);
        let n1 = part1(&sc,st);
        assert_eq!(n1,21);
    }
}
