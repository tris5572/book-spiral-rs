// P.65 選択ソート
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/2/ALDS1_2_B

#[macro_use]
mod util {
    // https://mirrorless-camera.info/column/14417.html
    macro_rules! input {
        ($($r:tt)*) => {
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(std::io::stdin()));
        let mut next = move |is_word: bool| -> String{
            if is_word {
                bytes
                    .by_ref()
                    .map(|r|r.unwrap() as char)
                    .skip_while(|c|c.is_whitespace())
                    .take_while(|c|!c.is_whitespace())
                    .collect()
            } else {
                bytes
                    .by_ref()
                    .map(|r|r.unwrap() as char)
                    .skip_while(|c| c == &'\n')
                    .take_while(|c| c != &'\n')
                    .collect()
            }
        };
        input_inner!{next, $($r)*};
        };
    }
    macro_rules! input_inner {
        ($next:expr) => {};
        ($next:expr, ) => {};

        ($next:expr, static $var:ident : $t:tt $($rest:tt)*) => {
            $var = read_value!($next, $t);
            input_inner!{$next $($rest)*}
        };

        ($next:expr, mut $var:ident : $t:tt $($rest:tt)*) => {
            let mut $var = read_value!($next, $t);
            input_inner!{$next $($rest)*}
        };

        ($next:expr, $var:ident : $t:tt $($rest:tt)*) => {
            let $var = read_value!($next, $t);
            input_inner!{$next $($rest)*}
        };
    }
    macro_rules! read_value {
        ($next:expr, ( $($t:tt),* )) => {
            ( $(read_value!($next, $t)),* )
        };

        ($next:expr, [ $t:tt ; all ]) => { {
                let mut str = $next(false);
                str.split_whitespace().map(|it| it.parse::<$t>().unwrap()).collect::<Vec<_>>()
            }
        };

        ($next:expr, [ $t:tt ; $len:expr ]) => {
            (0..$len as usize).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        };

        ($next:expr, chars) => {
            read_value!($next, String).chars().collect::<Vec<char>>()
        };

        ($next:expr, lines) => {
            {
                let mut vec = Vec::new();
                let mut str = $next(false);
                while str != "" {
                    vec.push(str);
                    str = $next(false);
                }
                vec
            }
        };

        ($next:expr, line) => {
            $next(false)
        };

        ($next:expr, usize1) => {
            read_value!($next, usize) - 1
        };

        ($next:expr, $t:ty) => {
            $next(true).parse::<$t>().expect("Parse error")
        };
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    solve(n, &mut a);
}

fn out<T: std::fmt::Display>(a: &[T]) {
    (0..a.len()).for_each(|i| {
        if 0 < i {
            print!(" ");
        }
        print!("{}", a[i]);
    });
    println!();
}

fn solve(n: usize, a: &mut [i32]) {
    let mut count = 0;
    let mut min_j;

    for i in 0..n {
        min_j = i;
        for j in i..n {
            if a[j] < a[min_j] {
                min_j = j;
            }
        }
        if i != min_j {
            a.swap(i, min_j);
            count += 1;
        }
    }

    out(a);
    println!("{}", count);
}

// ---------------------------------------------------------------------

#[test]
fn test1() {
    let mut a = vec![5, 6, 4, 2, 1, 3];
    solve(a.len(), &mut a);
}

#[test]
fn test2() {
    let mut a = vec![5, 2, 4, 6, 1, 3];
    solve(a.len(), &mut a);
}

#[test]
fn test3() {
    let mut a = vec![1, 2, 3, 4];
    solve(a.len(), &mut a);
}
