// P.54 挿入ソート

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

// 処理を工夫して、インデックスを楽に書けるようにしたもの。
fn solve(a: &mut [i32]) {
    out(a);

    for i in 1..a.len() {
        let v = a[i];
        let mut j = i - 1;
        let mut done = false;

        while !done {
            if v < a[j] {
                a[j + 1] = a[j];
                if j == 0 {
                    a[0] = v;
                    done = true;
                } else {
                    j -= 1;
                }
            } else {
                a[j + 1] = v;
                done = true;
            }
        }

        out(a);
    }
}

// 解答例の通り解く方法。インデックスが負の値になる可能性があるので面倒。
// fn solve(a: &mut [i32]) {
//     out(a);

//     for i in 1..a.len() {
//         let v = a[i];
//         let mut j = (i - 1) as isize;
//         while (0 <= j) && (v < a[j as usize]) {
//             a[j as usize + 1] = a[j as usize];
//             j -= 1;
//         }
//         a[(j + 1) as usize] = v;

//         out(a);
//     }
// }

// ---------------------------------------------------------------------

#[test]
fn test1() {
    let mut a = vec![5, 2, 4, 6, 1, 3];
    solve(&mut a);
}

#[test]
fn test2() {
    let mut a = vec![1, 2, 3];
    solve(&mut a);
}
