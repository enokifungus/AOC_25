use std::fs::read_to_string;
use std::collections::HashMap;
use std::fmt;

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
struct Node {
    x: u16,
}

impl Node {
    fn new(s: &str) -> Self {
        Node { x: s.chars().map(|c| (c as u8) - b'a').fold(0,|a,b| a*26 + b as u16) }
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
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u16)>>;
}

#[derive(Debug)]
struct Device {
    nb: HashMap<Node,Vec<Node>>,
}

impl Network for Device {
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u16)>> {
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
    pt: HashMap<Node,Vec<(Node,u16)>>,
}

impl Network for Station {
    fn get_paths(&self, st:Node) -> Option<impl Iterator<Item = (Node,u16)>> {
        self.pt.get(&st).map(|a| a.iter().copied())
    }
}

impl Station {
    fn new(nw:&impl Network, sn:&[Node]) -> Self {
        let mut pt = HashMap::new();
        let mut ds = sn.iter().map(|&x| (x,0)).collect();
        for x in sn {
            Station::search(nw,x,0,ds);
            pt[x] = ds.drain().collect();
        }
    }

    fn search(nw:&impl Network, cu:Node, n:u16, ds:&mut HashMap<Node,u16>) {
        if n > 0 {
            if let Some(x) = ds.entry(cu) { *x += 1; return; }
        }
        if Some(a) in nw.get_path(cu) {
            for (x,m) in a {
                search(nw,x,m + n,ds);
            }
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn _1() {
        let sc = DATA;
        let dv = Devices::new(sc);
        println!("{dv:?}");
    }
}
