fn main() {
    lol();
}

macro_rules! bf_ttm {
    ($t:tt) => {{
        stringify!($t)
    }};
    ($t:tt, $($t2:tt),+) => {{
        concat!(stringify!($t), $(stringify!($t2)),+)
    }};
}

macro_rules! bf_exec {
    ($cells:ident, $i:ident, ++++$($t:tt)*) => {{
        $cells[$i] += 4;
        bf_exec!($cells, $i, $($t)*)
    }};

    ($cells:ident, $i:ident, +) => {
        $cells[$i] += 1
    };

    ($cells:ident, $i:ident, ----$($t:tt)*) => {
        $cells[$i] -= 4;
        bf_exec!($cells, $i, $($t)*);
    };

    ($cells:ident, $i:ident, -) => {
        $cells[$i] -= 1
    };

    ($cells:ident, $i:ident,<) => {
        $i -= 1
    };

    ($cells:ident, $i:ident, >) => {{
        $i += 1;
    }};

    ($cells:ident, $i:ident, .) => {
        print!("{}", $cells[$i]);
    };

    ($cells:ident, $i:ident, ,) => {};

    ($cells:ident, $i:ident, [$($t:tt)+]) => {
        while $cells[$i] > 0 {
            bf_exec!($cells, $i, $($t)+)
        }
    };

    ($cells:ident, $i:ident, $op:tt $($t:tt)*) => {{
        bf_exec!($cells, $i, $op); bf_exec!($cells, $i, $($t)+);
    }};
}

macro_rules! bf {
    ($name:ident, $($t:tt)+) => {
        #[allow(unused_mut)]
        fn $name() {
            let mut cells = [0u8; 10000];
            let mut i: usize = 0;
            println!("{}: {}", stringify!($name), bf_ttm!($($t),+));
            bf_exec!(cells, i, $($t)+)
        }
    };
}

bf! {lol,
    +++++ [ - . ]
}
