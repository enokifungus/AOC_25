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
        self.wd = Some(self.va.take().expect("data error (x)"));
    }

    fn colon(&mut self) {
        if let Some(x) = self.bn.take() {
            self.bk.push(x);
        }
        let v = self.va.take().expect("data error (:)");
        if let Some(x) = self.wd.take() {
            println!("{x} {v},");
            self.ar = x*v;
            self.fx = Some(0);
        }
    }

    fn space(&mut self) {
        if let Some(v) = self.va.take() {
            let i = self.fx.as_mut().expect("data error (space)");
            self.fi += self.bk[*i] * v;
            *i += 1;
        }
    }

    fn endline(&mut self) {
        self.space();
        if self.fx.is_some() {
            self.fx = None;
            println!("{}",self.fi);
            if self.fi <= self.ar { self.n1 += 1; }
        }
    }
}

fn main() {
    let fi = File::open("data").expect("can't open");
    let sc = BufReader::new(fi);
    let mut pr = Presents::new();
    for i in sc.bytes() {
        let i = i.expect("read error");
        //print!("{i}{}",i as char);
        match i {
            b'.' => { pr.period(); }
            b'#' => { pr.pound(); }
            b'0'..=b'9' => { pr.digit(i); }
            b'x' => { pr.letter_x(); }
            b':' => { pr.colon(); }
            b' ' => { pr.space(); }
            b'\n'|b'\r' => { pr.endline(); }
            _ => { panic!("data error"); }
        }
    }
    pr.endline();
    println!("PART1: {}",pr.n1);
}
