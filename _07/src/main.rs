use std::fs::read;

fn parse_source(sc: &[u8]) -> (Vec<&[u8]>,usize) {
    let wd = sc.iter().position(|&i| i == b'\n').unwrap();
    let ar:Vec<_> = (0..(sc.len() + 1)/(wd + 1)).step_by(2).skip(1)
        .map(|i| &sc[i*(wd + 1)..(i*(wd + 1) + wd)]).collect();
    let st = ar[0].iter().position(|&i| i != b'.').unwrap();
    (ar,st)
}

fn part12(sc: &[&[u8]],st: usize) -> (usize,i64) {
    let mut ty = vec![0i64;sc[0].len()];
    ty[st] = 1;
    let mut n1 = 0;
    for l in sc.iter() {
        for i in (0..l.len()).filter(|&i| l[i] != b'.') {
            if ty[i] != 0 {
                n1 += 1;
                ty[i - 1] += ty[i];
                ty[i + 1] += ty[i];
                ty[i] = 0;
            }
        }
    }
    let n2 = ty.into_iter().sum::<i64>();
    (n1,n2)
}

fn main() {
    let sc = read("data").expect("file 'data'");
    let (ar,st) = parse_source(&sc);
    let (n1,n2) = part12(&ar,st);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
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
        let (n1,n2) = part12(&sc,st);
        assert_eq!(n1,21);
        assert_eq!(n2,40);
    }
}
