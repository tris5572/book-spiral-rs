// P.142 全検索
// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/all/ALDS1_5_A
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
        a: [isize; n],
        q: usize,
        m: [isize; q],
    }
    solve(&a, &m);
}

// a の i 以降から m の値を作れるかどうかをチェックする。
fn check(i: usize, m: isize, a: &[isize]) -> bool {
    if m == 0 {
        return true;
    }
    if a.len() <= i {
        return false;
    }

    check(i + 1, m, a) || check(i + 1, m - a[i], a)
}

fn solve(a: &[isize], m: &[isize]) {
    for v in m {
        if check(0, *v, a) {
            println!("yes");
        } else {
            println!("no")
        }
    }
}

// --------------------------------------------------------------------------

#[test]
fn test1() {
    let a = [1, 5, 7, 10, 21];
    let b = [2, 4, 17, 8];
    solve(&a, &b);
}
