use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn distance(a:&[f32;3], b:&[f32;3]) -> f32 {
    (a[0] - b[0]).abs().hypot((a[1] - b[1]).abs()).hypot((a[2] - b[2]).abs())
}

struct Pair {
    di: f32,
    pr: f32,
    i: usize,
    j: usize,
}

impl Pair {
    fn new(i:usize, j:usize, ar:&[[f32;3]]) -> Self {
        Pair { di:distance(&ar[i],&ar[j]), pr:ar[i][0]*ar[j][0], i, j }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other:&Self) -> bool {
        self.di == other.di
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other:&Self) -> Ordering {
        other.di.total_cmp(&self.di)
    }
}

struct Playground {
    nb: usize,
    hp: BinaryHeap<Pair>,
    gp: Vec<HashSet<usize>>,
}

impl Playground {
    fn new(sc: &str) -> Self {
        let sc:Vec<[f32;3]> = sc.lines().map(|s| s.split(',').map(|j| j.parse().expect("can't parse number"))
            .collect::<Vec<_>>().try_into().expect("expected 3 values per line")).collect();
        let nb = sc.len();
        let hp:BinaryHeap<_> = (1..sc.len()).flat_map(|i| (0..i).map(move |j| (i,j)))
            .map(|(i,j)| Pair::new(i,j,&sc)).collect();
        let gp = Vec::new();
        Playground { nb,hp,gp }
    }

    fn connect(&mut self) -> Option<f32> {
        if let Some(Pair{ i,j,pr,.. }) = self.hp.pop() {
            let u:Vec<_> = (0..self.gp.len()).filter(|&k| self.gp[k].contains(&i) || self.gp[k].contains(&j)).collect();
            match u[..] {
                [a,b] => {
                    let (x,y) = self.gp.split_at_mut(b);
                    x[a].extend(y[0].drain());
                    self.gp.remove(b);
                }
                [a] => {
                    self.gp[a].insert(i);
                    self.gp[a].insert(j);
                }
                _ => {
                    let mut x = HashSet::new();
                    x.insert(i);
                    x.insert(j);
                    self.gp.push(x);
                }
            }
            Some(pr)
        } else {
            None
        }
    }

    fn part12(&mut self, p1:usize) -> (usize,f32) {
        for _ in 0..p1 {
            let _ = self.connect().expect("not enough boxes");
        }
        let mut a:Vec<_> = self.gp.iter().map(|s| s.len()).collect();
        a.sort();
        let n1 = a[(a.len() - 3)..].into_iter().product::<usize>();
        let n2 = loop {
            let pr = self.connect().expect("not connected?");
            if self.gp.len() == 1 && self.gp[0].len() == self.nb { break pr; }
        };
        (n1,n2)
    }
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let mut pg = Playground::new(&sc);
    let (n1,n2) = pg.part12(1000);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
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
        let sc = DATA;
        let mut pg = Playground::new(sc);
        let (n1,n2) = pg.part12(10);
        assert_eq!(n1,40);
        assert_eq!(n2,25272.0);
    }
}
