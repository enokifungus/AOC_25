use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
struct Node {
    x: u16,
}

impl Node {
    fn new(s: &str) -> Self {
        Node { x: s.chars().map(|c| (c as u8) - b'a').fold(0,|a,b| a*26 + b as u16) }
    }

    fn new_multi(s: &str) -> Vec<Node> {
        s.split(' ').map(Node::new).collect()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut x = self.x;
        for _ in 0..3 {
            s.insert(0,((x%26) as u8 + b'a') as char);
            x /= 26;
        }
        write!(f,"{s}")
    }
}

trait Network {
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u64)>>;

    fn get_weight(&self, bg:Node, ed:Node) -> Option<u64> {
        self.get_paths(bg)
            .and_then(|it| it.filter_map(|(n,w)| (n == ed).then(|| w)).next())
    }
}

#[derive(Debug)]
struct Device {
    nb: HashMap<Node,Vec<Node>>,
}

impl Network for Device {
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u64)>> {
        self.nb.get(&st).map(|a| a.iter().map(|&x| (x,1)))
    }
}

impl Device {
    fn new(sc: &str) -> Self {
        let nb = sc.lines().map(|s| {
            let (t,s) = s.split_once(": ").unwrap();
            let a = Node::new(t);
            let b:Vec<_> = s.split(' ').map(Node::new).collect();
            (a,b)
        }).collect();
        Self { nb }
    }
}

#[derive(Debug)]
struct Station {
    pt: HashMap<Node,Vec<(Node,u64)>>,
}

impl Network for Station {
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u64)>> {
        self.pt.get(&st).map(|a| a.iter().copied())
    }
}

impl Station {
    fn new(nw:&impl Network, sn:&[Node]) -> Self {
        let mut pt = HashMap::new();
        let mut ds = sn.iter().map(|&x| (x,0)).collect();
        for &x in sn {
            Station::search(nw,x,1,&mut ds);
            let v:Vec<_> = ds.iter_mut()
                .filter_map(|(k,v)|
                    if *v != 0 { let o = Some((*k,*v)); *v = 0; o }
                    else { None })
                .collect();
            if !v.is_empty() { pt.insert(x,v); }
        }
        Station { pt }
    }

    fn search(nw:&impl Network, cu:Node, n:u64, ds:&mut HashMap<Node,u64>) {
        if let Some(a) = nw.get_paths(cu) {
            for (x,m) in a {
                if let Entry::Occupied(mut y) = ds.entry(x) { *y.get_mut() += m*n; }
                else { Station::search(nw,x,m*n,ds); }
            }
        }
    }
}

fn part1(dv: &impl Network) -> u64 {
    let sn = Node::new_multi("you out");
    let st = Station::new(dv,&sn);
    st.get_weight(sn[0],sn[1]).unwrap()
}

fn part2(dv: &impl Network,sn: &[Node]) -> u64 {
    let mut st = Station::new(dv,sn);
    if sn.len() > 4 {
        st = Station::new(&st,&sn[..4]);
    }
    (0..3).map(|i| st.get_weight(sn[i],sn[i+1]).unwrap()).product::<u64>()
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let dv = Device::new(&sc);
    let n1 = part1(&dv);
    let sn = "svr fft dac out dcv azy gvl mkx ihi thu hlx yur ljo uvc vqv vzm mbe lau cvo esm wwx wxj ikn nhi pzo you ltb wkd xty";
    let sn = Node::new_multi(sn);
    let n2 = part2(&dv,&sn);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1:&str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const DATA2:&str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn _1() {
        let sc = DATA1;
        let dv = Device::new(sc);
        let n1 = part1(&dv);
        assert_eq!(n1,5);
    }

    #[test]
    fn _2() {
        let sc = DATA2;
        let dv = Device::new(sc);
        let sn = Node::new_multi("svr fft dac out");
        let n2 = part2(&dv,&sn);
        assert_eq!(n2,2);
    }
}
