// P.146 コッホ曲線
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/5/ALDS1_5_C
//

// --------------------------------------------------------------------------

#[macro_use]
mod util {
    // https://zenn.dev/uesugi/articles/f3cf3c0a3f0a429496be
    macro_rules! input{
        (sc=$sc:expr,$($r:tt)*)=>{
            input_inner!{$sc,$($r)*}
        };
        ($($r:tt)*)=>{
            let mut sc=$crate::util::Scanner::new(std::io::stdin().lock());
            input_inner!{sc,$($r)*}
        };
    }

    macro_rules! input_inner{
        ($sc:expr)=>{};
        ($sc:expr,)=>{};
        ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{
            let $var=read_value!($sc,$t);
            input_inner!{$sc $($r)*}
        };
    }

    macro_rules! read_value{
        ($sc:expr,($($t:tt),*))=>{
            ($(read_value!($sc,$t)),*)
        };
        ($sc:expr,[$t:tt;$len:expr])=>{
            (0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()
        };
        ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
        ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
        ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
    }
    pub struct Scanner {
        s: Box<str>,
        input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,
    }
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R) -> Self {
            let s = {
                let mut s = String::new();
                reader.read_to_string(&mut s).unwrap();
                s.into_boxed_str()
            };
            let mut sc = Scanner {
                s,
                input: "".split_ascii_whitespace().peekable(),
            };
            use std::mem;
            let s: &'static str = unsafe { mem::transmute(&*sc.s) };
            sc.input = s.split_ascii_whitespace().peekable();
            sc
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T
        where
            T::Err: std::fmt::Debug,
        {
            self.input
                .next()
                .unwrap()
                .parse::<T>()
                .expect("Parse error")
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    solve(n);
}

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn koch(n: usize, a: &Point, b: &Point) {
    if n == 0 {
        return;
    }
    let th = std::f64::consts::PI * 60.0 / 180.0;

    let s = Point {
        x: (2.0 * a.x + 1.0 * b.x) / 3.0,
        y: (2.0 * a.y + 1.0 * b.y) / 3.0,
    };
    let t = Point {
        x: (1.0 * a.x + 2.0 * b.x) / 3.0,
        y: (1.0 * a.y + 2.0 * b.y) / 3.0,
    };
    let u = Point {
        x: (t.x - s.x) * f64::cos(th) - (t.y - s.y) * f64::sin(th) + s.x,
        y: (t.x - s.x) * f64::sin(th) + (t.y - s.y) * f64::cos(th) + s.y,
    };

    koch(n - 1, a, &s);
    println!("{:.8} {:.8}", &s.x, &s.y);
    koch(n - 1, &s, &u);
    println!("{:.8} {:.8}", &u.x, &u.y);
    koch(n - 1, &u, &t);
    println!("{:.8} {:.8}", &t.x, &t.y);
    koch(n - 1, &t, b);
}

fn solve(n: usize) {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 100.0, y: 0.0 };
    println!("{:.8} {:.8}", a.x, a.y);
    koch(n, &a, &b);
    println!("{:.8} {:.8}", b.x, b.y);
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    solve(1);
}

#[test]
fn test2() {
    solve(2);
}
