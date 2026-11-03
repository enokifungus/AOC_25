use std::fs::read_to_string;
use std::mem::swap;

const _DATA:&str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

fn rect_area(a:&[i32;2], b:&[i32;2]) -> i64 {
    ((a[0].abs_diff(b[0]) + 1) as i64)*((a[1].abs_diff(b[1]) + 1) as i64)
}

fn any_within(ar: &[[i32;2]], i:usize, j:usize) -> bool {
    let [mut a,mut b] = ar[i];
    let [mut c,mut d] = ar[j];
    if a == c || b == d { return false; }
    if a > c { swap(&mut a,&mut c); }
    if b > d { swap(&mut b,&mut d); }
    ar.iter().any(|&[x,y]| a < x && x < c && b < y && y < d)
}

fn main() {
    let sc = read_to_string("data").expect("file 'data'");
    let ar:Vec<[i32;2]> = sc.lines().map(|s| s.split(',').map(|t| t.parse().unwrap())
        .collect::<Vec<_>>().try_into().unwrap()).collect();
    let n1 = (1..ar.len()).flat_map(|i| (0..i).map(move |j| (i,j)))
        .map(|(i,j)| rect_area(&ar[i],&ar[j])).max().unwrap();
    let a = ar[0][1];
    let b = ar.iter().position(|[_,y]| *y < a).unwrap() + 1;
    let n2 = (0..b+1).filter(|&i| !any_within(&ar[0..b+1],248,i)).map(|i| rect_area(&ar[248],&ar[i])).max().unwrap();
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}
