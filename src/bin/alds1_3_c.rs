// P.95 連結リスト
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_C
// このコードでは連結リストを使用していない。
// 途中の要素の削除には O(n) かかるので、Vec 等を使うのと変わりない。
// 先頭への追加/削除は Vec では O(n) でそこそこ負担だが、キューを使えば良い。

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
        a: [line; n],
    }
    solve(&a);
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

fn solve(a: &[String]) {
    let mut queue = std::collections::VecDeque::new();

    for v in a {
        if v == "deleteFirst" {
            queue.pop_front();
        } else if v == "deleteLast" {
            queue.pop_back();
        } else if v.starts_with("insert ") {
            queue.push_front(v.replace("insert ", "").trim().parse::<i64>().unwrap());
        } else {
            let n = v.replace("delete ", "").trim().parse::<i64>().unwrap();
            if let Some(i) = queue.iter().position(|x| *x == n) {
                queue.remove(i);
            }
        }
    }

    out(&Vec::from(queue));
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let a = [
        "insert 5".to_string(),
        "insert 2".to_string(),
        "insert 3".to_string(),
        "insert 1".to_string(),
        "delete 3".to_string(),
        "insert 6".to_string(),
        "delete 5".to_string(),
    ];
    solve(&a)
}

#[test]
fn test2() {
    let a = [
        "insert 5".to_string(),
        "insert 2".to_string(),
        "insert 3".to_string(),
        "insert 1".to_string(),
        "delete 3".to_string(),
        "insert 6".to_string(),
        "delete 5".to_string(),
        "deleteFirst".to_string(),
        "deleteLast".to_string(),
    ];
    solve(&a)
}
