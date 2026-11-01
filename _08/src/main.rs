fn distance(a:[f32;3], b:[f32;3]) -> f32 {
    (a[0] - b[0]).abs().hypot((a[1] - b[1]).abs()).hypot((a[2] - b[2]).abs())
}

fn parse_source(sc: &str) -> Vec<[f32;3]> {
    sc.lines().map(|i| i.split(',')
        .map(|j| j.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into().unwrap()).collect()
}

fn main() {
    println!("Hello, world!");
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
        let mut di:Vec<_> = (1..ar.len()).flat_map(|i| (0..i).map(move |j| (i,j))).map(|(i,j)| distance(ar[i],ar[j])).collect();
        di.sort_unstable_by(f32::total_cmp);
        let n1 = di[di.len() - 10..].into_iter().product::<f32>();
        println!("{n1}");
    }
}
