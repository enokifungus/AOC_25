use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use float_ord::FloatOrd;

fn distance(a:[f32;3], b:[f32;3]) -> f32 {
    (a[0] - b[0]).abs().hypot((a[1] - b[1]).abs()).hypot((a[2] - b[2]).abs())
}

fn parse_source(sc: &str) -> Vec<[f32;3]> {
    sc.lines().map(|i| i.split(',')
        .map(|j| j.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into().unwrap()).collect()
}

fn part2(ar: &[[f32;3]]) -> f32 {
    let mut di:Vec<_> = (1..ar.len()).flat_map(|i| (0..i).map(move |j| (i,j)))
        .map(|(i,j)| (distance(ar[i],ar[j]),i,j)).collect();
    di.sort_unstable_by(|&i,&j| i.0.total_cmp(&j.0));
    let mut gp:Vec<HashSet<usize>> = Vec::new();
    for &(_,i,j) in di.iter() {
        let u:Vec<_> = (0..gp.len()).filter(|&x| gp[x].contains(&i) || gp[x].contains(&j)).collect();
        if let [v,w] = u[..] {
            let (s,t) = gp.split_at_mut(w);
            s[v].extend(t[0].drain());
            gp.remove(w);
        } else if let [v] = u[..] {
            gp[v].insert(i);
            gp[v].insert(j);
        } else {
            let mut s = HashSet::new();
            s.insert(i);
            s.insert(j);
            gp.push(s);
        }
        if gp.len() == 1 && gp[0].len() == ar.len() {
            return ar[i][0]*ar[j][0];
        }
    }
    panic!("can't connect all");
}

fn part1(ar: &[[f32;3]], m: usize) -> usize {
    let mut di:Vec<_> = (1..ar.len()).flat_map(|i| (0..i).map(move |j| (i,j)))
        .map(|(i,j)| (distance(ar[i],ar[j]),i,j)).collect();
    di.sort_unstable_by(|&i,&j| i.0.total_cmp(&j.0));
    let mut gp:Vec<HashSet<usize>> = Vec::new();
    for &(_,i,j) in di.iter().take(m) {
        let u:Vec<_> = (0..gp.len()).filter(|&x| gp[x].contains(&i) || gp[x].contains(&j)).collect();
        if let [v,w] = u[..] {
            let (s,t) = gp.split_at_mut(w);
            s[v].extend(t[0].drain());
            gp.remove(w);
        } else if let [v] = u[..] {
            gp[v].insert(i);
            gp[v].insert(j);
        } else {
            let mut s = HashSet::new();
            s.insert(i);
            s.insert(j);
            gp.push(s);
        }
    }
    let mut sz:Vec<_> = gp.iter().map(|s| s.len()).collect();
    sz.sort();
    sz[(sz.len() - 3)..].iter().product::<usize>()
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let ar = parse_source(&sc);
    let n1 = part1(&ar,1000);
    println!("PART1: {n1}");
    let n2 = part2(&ar);
    println!("PART2: {n2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA:&str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn _1() {
        let ar = parse_source(DATA);
        let n1 = part1(&ar,10);
        let n2 = part2(&ar);
        assert_eq!(n1,40);
        assert_eq!(n2,25272.0);
    }

    #[derive(PartialEq,Eq,PartialOrd,Ord,Debug)]
    struct Foo {
        a: FloatOrd<f32>,
        b: i32,
    }

    #[test]
    fn _2() {
        let a = b"bravo";
        let mut b:BinaryHeap<_> = a.iter().map(|x| Foo{a:FloatOrd((x - b'a') as f32),b:99}).collect();
        println!("{b:?}");
        while let Some(x) = b.pop() {
            println!("{x:?}");
        }
    }
}
