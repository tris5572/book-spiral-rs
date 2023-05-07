// P.127 ハッシュ
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_C
// HashSet を使って解いている。

// コメントアウトしている input! マクロを使った解き方は、
// データを一気に読んでメモリに溜め込むことにより(?)メモリ制限に引っかかるため、
// 自前で標準入力から値を読み取っている。
// ただこれだとユニットテストを書けないので面倒ではある。

// --------------------------------------------------------------------------

// #[macro_use]
// mod util {
//     // https://mirrorless-camera.info/column/14417.html
//     macro_rules! input {
//         ($($r:tt)*) => {
//         let mut bytes = std::io::Read::bytes(std::io::BufReader::new(std::io::stdin()));
//         let mut next = move |is_word: bool| -> String{
//             if is_word {
//                 bytes
//                     .by_ref()
//                     .map(|r|r.unwrap() as char)
//                     .skip_while(|c|c.is_whitespace())
//                     .take_while(|c|!c.is_whitespace())
//                     .collect()
//             } else {
//                 bytes
//                     .by_ref()
//                     .map(|r|r.unwrap() as char)
//                     .skip_while(|c| c == &'\n')
//                     .take_while(|c| c != &'\n')
//                     .collect()
//             }
//         };
//         input_inner!{next, $($r)*};
//         };
//     }
//     macro_rules! input_inner {
//         ($next:expr) => {};
//         ($next:expr, ) => {};

//         ($next:expr, static $var:ident : $t:tt $($rest:tt)*) => {
//             $var = read_value!($next, $t);
//             input_inner!{$next $($rest)*}
//         };

//         ($next:expr, mut $var:ident : $t:tt $($rest:tt)*) => {
//             let mut $var = read_value!($next, $t);
//             input_inner!{$next $($rest)*}
//         };

//         ($next:expr, $var:ident : $t:tt $($rest:tt)*) => {
//             let $var = read_value!($next, $t);
//             input_inner!{$next $($rest)*}
//         };
//     }
//     macro_rules! read_value {
//         ($next:expr, ( $($t:tt),* )) => {
//             ( $(read_value!($next, $t)),* )
//         };

//         ($next:expr, [ $t:tt ; all ]) => { {
//                 let mut str = $next(false);
//                 str.split_whitespace().map(|it| it.parse::<$t>().unwrap()).collect::<Vec<_>>()
//             }
//         };

//         ($next:expr, [ $t:tt ; $len:expr ]) => {
//             (0..$len as usize).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
//         };

//         ($next:expr, chars) => {
//             read_value!($next, String).chars().collect::<Vec<char>>()
//         };

//         ($next:expr, lines) => {
//             {
//                 let mut vec = Vec::new();
//                 let mut str = $next(false);
//                 while str != "" {
//                     vec.push(str);
//                     str = $next(false);
//                 }
//                 vec
//             }
//         };

//         ($next:expr, line) => {
//             $next(false)
//         };

//         ($next:expr, usize1) => {
//             read_value!($next, usize) - 1
//         };

//         ($next:expr, $t:ty) => {
//             $next(true).parse::<$t>().expect("Parse error")
//         };
//     }
// }

// fn main() {
//     input! {
//         n: usize,
//         s: [(String, String); n],
//     }
//     solve(&s);
// }

// fn solve(s: &[(String, String)]) {
//     let mut data = std::collections::HashSet::new();

//     for x in s {
//         if x.0 == "insert" {
//             data.insert(x.1.clone());
//         } else if data.contains(&x.1) {
//             println!("yes");
//         } else {
//             println!("no");
//         }
//     }
// }

fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

// fn read_pair<T: FromStr, S: FromStr>() -> (T, S) {
//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();
//     let mut buf = buf.trim().split_whitespace();

//     let t: T = buf.next().unwrap().parse().ok().unwrap();
//     let s: S = buf.next().unwrap().parse().ok().unwrap();

//     (t, s)
// }

fn main() {
    let n = read();
    let mut data = std::collections::HashSet::new();

    for _ in 0..n {
        let line = read::<String>();
        let mut buf = line.split_whitespace();
        let ope = buf.next().unwrap().to_owned();
        let key = buf.next().unwrap().to_owned();

        if ope == "insert" {
            data.insert(key);
        } else if data.contains(&key) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

// --------------------------------------------------------------------------

// #[test]
// fn test1() {
//     let s = [
//         ("insert".to_string(), "AAA".to_string()),
//         ("insert".to_string(), "AAC".to_string()),
//         ("find".to_string(), "AAA".to_string()),
//         ("find".to_string(), "CCC".to_string()),
//         ("insert".to_string(), "CCC".to_string()),
//         ("find".to_string(), "CCC".to_string()),
//     ];
//     solve(&s)
// }

// #[test]
// fn test2() {
//     let s = [
//         ("insert".to_string(), "AAA".to_string()),
//         ("insert".to_string(), "AAC".to_string()),
//         ("insert".to_string(), "AGA".to_string()),
//         ("insert".to_string(), "AGG".to_string()),
//         ("insert".to_string(), "TTT".to_string()),
//         ("find".to_string(), "AAA".to_string()),
//         ("find".to_string(), "CCC".to_string()),
//         ("find".to_string(), "CCC".to_string()),
//         ("insert".to_string(), "CCC".to_string()),
//         ("find".to_string(), "CCC".to_string()),
//         ("insert".to_string(), "T".to_string()),
//         ("find".to_string(), "TTT".to_string()),
//         ("find".to_string(), "T".to_string()),
//     ];
//     solve(&s)
// }
