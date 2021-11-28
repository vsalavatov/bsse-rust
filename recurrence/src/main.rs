macro_rules! recurrence {
    ($fun:ident[$idx:ident] = $($vals:literal),+ , ..., $calc:expr) => {{
        let mut $fun = vec![$($vals),+];
        let mut $idx = 0;
        std::iter::from_fn(move || {
            if $idx < $fun.len() {
                $idx += 1;
                return Some($fun[$idx-1]);
            }
            let value = $calc;
            $fun.push(value.clone());
            $idx += 1;
            if $fun.len() >= 64 { // drop first half
                let len = $fun.len();
                let to_drop = $fun.len() / 2;
                $fun.rotate_right(len - to_drop);
                $fun.truncate(len - to_drop);
                $idx -= to_drop;
            }
            Some(value)
        })
    }};
}

fn main() {
    let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];

    for n in fib.take(10) {
        print!("{} ", n);
    }
    println!();
    // 0 1 1 2 3 5 8 13 21 34

    let other = recurrence![f[i] = 1.0, ..., f[i-1] * i as f64];
    for n in other.take(10) {
        print!("{} ", n);
    }
    println!();
    // 1 1 2 6 24 120 720 5040 40320 362880

    let f = recurrence![a[n] = 0, 1, 2, 3, 4, ..., a[n-1] + 1];
    println!("{:?}", f.take(100).collect::<Vec<i32>>());
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99]
}
