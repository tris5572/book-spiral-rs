// P.119 線形探索
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_A
//

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
        a: [usize; n],
        m: usize,
        b: [usize; m],
    }
    solve(&a, &b);
}

// a の中から key を線形探索する。
// 見付かったときは Some(index) 、見付からなかったら None を返す。
fn linear_search(a: &[usize], key: usize) -> Option<usize> {
    let mut i = 0;
    let mut a = Vec::from(a); // 番兵追加のために複製
    a.push(key); // 番兵を追加
    while a[i] != key {
        i += 1;
    }
    if i == a.len() - 1 {
        // 番兵に達しているときは見付かっていないとき
        None
    } else {
        Some(i)
    }
}

fn solve(a: &[usize], b: &[usize]) {
    let mut count = 0;
    for k in b {
        if linear_search(a, *k).is_some() {
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
    let a = [3, 1, 2];
    let b = [5];
    solve(&a, &b)
}

#[test]
fn test3() {
    let a = [1, 1, 2, 2, 3];
    let b = [1, 2];
    solve(&a, &b)
}
