// P.70 安定なソート
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/2/ALDS1_2_C

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
        a: [String; n],
    }
    solve(&analyze(&a));
}

// 入力を Card へデータ化する。
fn analyze(data: &[String]) -> Vec<Card> {
    let mut array = vec![];
    for v in data {
        let mut chars = v.chars();
        array.push(Card {
            suit: chars.next().unwrap(),
            value: chars.next().unwrap().to_digit(10).unwrap(),
        })
    }
    array
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    suit: char,
    value: u32,
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
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

// バブルソートした結果を返す。
fn bubble_sort(array: &[Card]) -> Vec<Card> {
    let mut a = Vec::from(array);
    let mut flag = true; // 今回交換したかどうかのフラグ
    let mut i = 0;

    // 前回、交換がなければ終了するループ
    while flag {
        flag = false;
        let mut j = a.len() - 1;

        while i < j {
            if a[j].value < a[j - 1].value {
                a.swap(j - 1, j);
                flag = true;
            }
            j -= 1;
        }
        i += 1;
    }

    a
}

// 選択ソートした結果を返す。
fn selection_sort(array: &[Card]) -> Vec<Card> {
    let mut a = Vec::from(array);

    for i in 0..a.len() {
        let mut min_j = i;
        for j in i..a.len() {
            if a[j].value < a[min_j].value {
                min_j = j;
            }
        }
        a.swap(i, min_j)
    }

    a
}

fn is_stable(a: &[Card], b: &[Card]) -> bool {
    for i in 0..a.len() {
        if a[i].suit != b[i].suit {
            return false;
        }
    }
    true
}

fn solve(a: &[Card]) {
    let b = bubble_sort(a);
    out(&b);
    println!("Stable");
    let s = selection_sort(a);
    out(&s);

    if is_stable(&b, &s) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}

// ---------------------------------------------------------------------

#[test]
fn test1() {
    let a = vec![
        "H4".to_string(),
        "C9".to_string(),
        "S4".to_string(),
        "D2".to_string(),
        "C3".to_string(),
    ];
    solve(&analyze(&a));
}

#[test]
fn test2() {
    let a = vec!["S1".to_string(), "H1".to_string()];
    solve(&analyze(&a));
}
