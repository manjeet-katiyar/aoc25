fn main() {
    let lines = utils::read_input("day03/input");

    let res1: i32 = lines
        .iter()
        .map(|line| {
            let mut a = -1;
            let mut b = -1;

            for c in line.chars() {
                let curr = c.to_digit(10).unwrap() as i32;
                if a == -1 {
                    a = curr;
                } else if b == -1 {
                    b = curr;
                } else if a < b {
                    a = b;
                    b = curr;
                } else if b < curr {
                    b = curr;
                }
            }

            a * 10 + b
        })
        .sum();

    dbg!(res1);

    let res2: i64 = lines
        .iter()
        .map(|line| {
            let n = line.len();
            let mut mem = vec![vec![-1; 12]; n];

            let mut res = 0;
            for (idx, c) in line.chars().enumerate() {
                let curr = c.to_digit(10).unwrap() as i64;
                mem[idx][0] = curr;

                for j in 0..idx {
                    for k in 0..11 {
                        mem[idx][k + 1] = std::cmp::max(mem[idx][k + 1], mem[j][k] * 10 + curr);
                    }
                }

                res = std::cmp::max(res, mem[idx][11]);
            }

            res
        })
        .sum();

    dbg!(res2);
}
