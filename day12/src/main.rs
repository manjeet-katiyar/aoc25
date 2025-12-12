fn main() {
    let lines = utils::read_input("day12/input");
    let lines: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|l| l.to_vec()).collect();

    match lines.as_slice() {
        [rest @ .., last] => {
            let sizes: Vec<usize> = rest
                .iter()
                .map(|shape| {
                    shape
                        .iter()
                        .map(|l| l.chars().filter(|&c| c == '#').count())
                        .sum()
                })
                .collect();

            let res: usize = last
                .iter()
                .map(|q| {
                    let (dim, ps) = q.split_once(": ").expect(": is expected");
                    let (x, y): (usize, usize) = dim
                        .split_once("x")
                        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                        .expect("x is expected");

                    let ps: Vec<usize> = ps.split(" ").map(|c| c.parse().unwrap()).collect();

                    let available = x * y;
                    let required: usize =
                        ps.iter().enumerate().map(|(idx, &p)| p * sizes[idx]).sum();

                    if available > required { 1 } else { 0 }
                })
                .sum();

            println!("RES: {}", res);
        }
        [] => panic!("empty"),
    }
}
