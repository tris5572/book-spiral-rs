// P.114 面積計算
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_D

// --------------------------------------------------------------------------

fn main() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    solve(&a);
}

fn solve(a: &str) {
    let mut s1 = vec![];
    let mut s2: Vec<(usize, usize)> = vec![];
    let mut sum = 0;

    for (i, c) in a.chars().enumerate() {
        if c == '\\' {
            s1.push(i);
        } else if c == '/' && !s1.is_empty() {
            let j = s1.pop().unwrap();
            let mut a = i - j;
            sum += a;
            while !s2.is_empty() && j < s2.last().unwrap().0 {
                a += s2.pop().unwrap().1;
            }
            s2.push((j, a));
        }
    }
    println!("{}", sum);

    print!("{}", s2.len());
    for b in s2 {
        print!(" {}", b.1);
    }
    println!();
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    solve(r"\\//");
}

#[test]
fn test2() {
    solve(r"\\///\_/\/\\\\/_/\\///__\\\_\\/_\/_/\");
}
