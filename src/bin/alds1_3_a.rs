// P.82 スタック
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_A
// 自分でスタックを実現するわけではなく、使い方のコードになっている。

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Plus,
    Minus,
    Multi,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Number(i64),
    Operator(Op),
}

fn main() {
    input! {
        a: [String; all],
    }
    let array = analyze(&a);
    solve(&array);
}

// 入力された文字列の配列を解析して返す。
fn analyze(a: &[String]) -> Vec<Symbol> {
    let mut array = vec![];

    a.iter().for_each(|v| match v.parse() {
        Ok(n) => array.push(Symbol::Number(n)),
        Err(_) => match v.chars().next().unwrap() {
            '+' => array.push(Symbol::Operator(Op::Plus)),
            '-' => array.push(Symbol::Operator(Op::Minus)),
            '*' => array.push(Symbol::Operator(Op::Multi)),
            _ => unreachable!(),
        },
    });

    array
}

fn solve(a: &[Symbol]) {
    let mut stack = vec![];
    for v in a.iter() {
        match v {
            Symbol::Number(n) => stack.push(*n),
            Symbol::Operator(op) => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(match op {
                    Op::Plus => n1 + n2,
                    Op::Minus => n1 - n2,
                    Op::Multi => n1 * n2,
                })
            }
        }
    }
    println!("{}", stack.pop().unwrap());
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let array = ["1".to_string(), "2".to_string(), "+".to_string()];
    let array = analyze(&array);
    solve(&array)
}

#[test]
fn test2() {
    let array = [
        "1".to_string(),
        "2".to_string(),
        "+".to_string(),
        "3".to_string(),
        "4".to_string(),
        "-".to_string(),
        "*".to_string(),
    ];
    let array = analyze(&array);
    solve(&array)
}
