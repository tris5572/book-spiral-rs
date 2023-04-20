// P.87 キュー
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_B
// 自分でキューを実現するわけではなく、使い方のコードになっている。

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
        n:usize,
        q:i64,
        a: [(String, i64); n],
    }
    // let array = analyze(&a);
    solve(q, &a);
}

fn solve(q: i64, a: &[(String, i64)]) {
    let mut time = 0; // 経過時間
    let mut queue = std::collections::VecDeque::new();
    for v in a.iter() {
        queue.push_back(v.clone());
    }

    while !queue.is_empty() {
        let mut v = queue.pop_front().unwrap();
        if q < v.1 {
            // そのクオンタムで終了しないとき。
            v.1 -= q;
            queue.push_back(v);
            time += q;
        } else {
            // そのクオンタムで終了するとき。
            time += v.1;
            println!("{} {}", v.0, time);
        }
    }
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let q = 100;
    let a = [
        ("p1".to_string(), 150),
        ("p2".to_string(), 80),
        ("p3".to_string(), 200),
        ("p4".to_string(), 350),
        ("p5".to_string(), 20),
    ];
    solve(q, &a)
}
