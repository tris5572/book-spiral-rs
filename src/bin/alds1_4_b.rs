// P.122 二分探索
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_B
// 二分探索を自前で実装している。解くだけなら Vec の binary_search を使えば良い。

// --------------------------------------------------------------------------

#[macro_use]
mod util {
    // https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
    macro_rules! input {
        (source = $s:expr, $($r:tt)*) => {
            let mut iter = $s.split_whitespace();
            input_inner!{iter, $($r)*}
        };
        ($($r:tt)*) => {
            let s = {
                use std::io::Read;
                let mut s = String::new();
                std::io::stdin().read_to_string(&mut s).unwrap();
                s
            };
            let mut iter = s.split_whitespace();
            input_inner!{iter, $($r)*}
        };
    }

    macro_rules! input_inner {
        ($iter:expr) => {};
        ($iter:expr, ) => {};

        ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
            let $var = read_value!($iter, $t);
            input_inner!{$iter $($r)*}
        };
    }

    macro_rules! read_value {
        ($iter:expr, ( $($t:tt),* )) => {
            ( $(read_value!($iter, $t)),* )
        };

        ($iter:expr, [ $t:tt ; $len:expr ]) => {
            (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        };

        ($iter:expr, chars) => {
            read_value!($iter, String).chars().collect::<Vec<char>>()
        };

        ($iter:expr, usize1) => {
            read_value!($iter, usize) - 1
        };

        ($iter:expr, $t:ty) => {
            $iter.next().unwrap().parse::<$t>().expect("Parse error")
        };
    }
}

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        q: usize,
        t: [usize; q],
    }
    solve(&s, &t);
}

// a から key を二分探索した結果を返す。
fn binary_search(a: &[usize], key: usize) -> Option<()> {
    let mut left = 0;
    let mut right = a.len();
    let mut mid;
    while left < right {
        mid = (left + right) / 2;
        if key == a[mid] {
            return Some(());
        }
        if key < a[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    None
}

fn solve(a: &[usize], b: &[usize]) {
    let mut count = 0;
    for k in b {
        if binary_search(a, *k).is_some() {
            count += 1;
        }
    }
    println!("{}", count);
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let a = [1, 2, 3, 4, 5];
    let b = [3, 4, 1];
    solve(&a, &b)
}

#[test]
fn test2() {
    let a = [1, 2, 3];
    let b = [5];
    solve(&a, &b)
}

#[test]
fn test3() {
    let a = [1, 1, 2, 2, 3];
    let b = [1, 2];
    solve(&a, &b)
}
