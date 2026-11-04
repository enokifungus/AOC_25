use std::fs::read_to_string;

fn print_graph() {
    let sc = read_to_string("data").expect("file 'data'");
    for s in sc.lines() {
        let mut a:Vec<_> = s.split(' ').collect();
        let b = a.remove(0).strip_suffix(':').unwrap();
        for s in a {
            println!("{b}->{s}");
        }
    }
}

fn main() {
    print_graph();
}
