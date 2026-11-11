use std::fs::File;
use std::io::{BufReader,Read};

struct Presents {
    bk: Vec<i32>,
    n1: i32,
    bn: Option<i32>,
    va: Option<i32>,
    fx: Option<usize>,
    fi: i32,
    ar: i32,
    wd: Option<i32>,
}

impl Presents {
    fn new() -> Self {
        Self { bk:Vec::new(), n1:0, bn:None, va:None, fx:None, fi:0, ar:0, wd: None }
    }

    fn period(&mut self) {
        self.bn.get_or_insert_default();
    }

    fn pound(&mut self) {
        *self.bn.get_or_insert_default() += 1;
    }

    fn digit(&mut self, i:u8) {
        let x = self.va.get_or_insert_default();
        *x = *x*10 + (i - b'0') as i32;
    }

    fn letter_x(&mut self) {
        let v = self.va.take().unwrap();
        self.wd = Some(v);
    }

    fn colon(&mut self) {
        let v = self.va.take().unwrap();
        if let Some(x) = self.bn.take() {
            self.bk.push(x);
        }
        if let Some(x) = self.wd.take() {
            self.ar = x*v;
            self.fx = Some(0);
        }
    }

    fn space(&mut self) {
        if let Some(v) = self.va.take() {
            let i = self.fx.as_mut().unwrap();
            self.fi += self.bk[*i] * v;
            *i += 1;
        }
    }

    fn endline(&mut self) {
        if let Some(v) = self.va.take() {
            let i = self.fx.take().unwrap();
            self.fi += self.bk[i] * v;
            if self.fi <= self.ar { self.n1 += 1; }
            self.fi = 0;
            self.ar = 0;
        }
    }
}

fn part1(sc: impl Iterator<Item = Option<u8>>) -> i32 {
    let mut pr = Presents::new();
    for i in sc {
        let i = i.unwrap();
        match i {
            b'.' => { pr.period(); }
            b'#' => { pr.pound(); }
            b'0'..=b'9' => { pr.digit(i); }
            b'x' => { pr.letter_x(); }
            b':' => { pr.colon(); }
            b' ' => { pr.space(); }
            b'\n'|b'\r' => { pr.endline(); }
            _ => { panic!(); }
        }
    }
    pr.endline();
    pr.n1
}

fn main() {
    let fi = File::open("data").expect("can't open");
    let sc = BufReader::new(fi).bytes().map(Result::ok);
    let n1 = part1(sc);
    println!("PART1: {n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&[u8] = b"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn _1() {
        let sc = DATA.into_iter().map(|&i| Some(i));
        let n1 = part1(sc);
        assert_eq!(n1,2); //this will fail
    }
}
