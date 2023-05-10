// P.136 割り当て
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_D
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
        k: usize,
        w: [usize; n],
    }
    solve(k, &w);
}

// 最大積載量 p の k 台のトラックで何個の荷物を詰めるかを返す。
fn check(p: usize, k: usize, w: &[usize]) -> usize {
    let n = w.len();
    let mut i = 0;
    for _ in 0..k {
        let mut s = 0;
        while s + w[i] <= p {
            s += w[i];
            i += 1;
            if i == n {
                return n;
            }
        }
    }
    i
}

fn solve(k: usize, w: &[usize]) {
    let mut left = 0;
    let n = w.len();
    let mut right = n * 10_000;
    let mut mid;
    while 1 < right - left {
        mid = (left + right) / 2;
        let v = check(mid, k, w);
        if n <= v {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let a = [8, 1, 7, 3, 9];
    solve(3, &a);
}

#[test]
fn test2() {
    let a = [1, 2, 2, 6];
    solve(2, &a);
}
