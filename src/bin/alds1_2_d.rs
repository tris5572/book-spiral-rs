// P.74 シェルソート
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/2/ALDS1_2_D

// --------------------------------------------------------------------------

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
    solve(&mut a);
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

// 間隔g を指定した挿入ソート。交換した回数を返す。
fn insertion_sort(a: &mut [i32], g: usize) -> usize {
    let mut cnt = 0;
    for i in g..a.len() {
        let v = a[i];
        let mut j = i - g;
        let mut done = false;

        while !done {
            if v < a[j] {
                a[j + g] = a[j];
                cnt += 1;
                if j < g {
                    a[j] = v;
                    done = true;
                } else {
                    j -= g;
                }
            } else {
                a[j + g] = v;
                done = true;
            }
        }
    }
    cnt
}

fn create_g(n: usize) -> Vec<usize> {
    let mut h = 1;
    let mut g = vec![];
    loop {
        if n < h {
            break;
        }
        g.push(h);
        h = 3 * h + 1;
    }
    g
}

fn solve(a: &mut [i32]) {
    let mut cnt = 0;
    let g = create_g(a.len());
    let g: Vec<usize> = g.into_iter().rev().collect();
    g.iter().for_each(|v| cnt += insertion_sort(a, *v));
    println!("{}", g.len());
    out(&g);
    println!("{}", cnt);
    a.iter().for_each(|v| println!("{}", v));
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let mut a = vec![5, 1, 4, 3, 2];
    solve(&mut a);
}

#[test]
fn test2() {
    let mut a = vec![3, 2, 1];
    solve(&mut a);
}
