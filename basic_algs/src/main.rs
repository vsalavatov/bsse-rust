fn max_out_of_10(arr: &[i64; 10]) -> i64 {
    let mut mx = arr[0];
    for val in arr {
        if mx < *val {
            mx = *val;
        }
    }
    return mx;
}

fn check_primality(x: u64) -> bool {
    if x <= 1 {
        return false;
    }
    let mut div = 2;
    while div * div <= x {
        if x % div == 0 {
            return false;
        }
        div += 1;
    }
    return true;
}

fn nth_prime(mut n: u32) -> u64 {
    if n == 0 {
        panic!("there is no 0th prime number")
    }
    let mut p: u64 = 1;
    while n > 0 {
        p += 1;
        if check_primality(p) {
            n -= 1;
        }
    }
    return p;
}

// performs a binary search in a sorted array
// returns an index of the first element that is greater or equal to x
fn lower_bound(arr: &[i64; 10], x: i64) -> usize {
    if arr[0] >= x {
        return 0;
    }

    let mut left: usize = 0;
    let mut right: usize = arr.len();
    while left + 1 < right {
        let mid = (left + right) / 2;
        if arr[mid] < x {
            left = mid;
        } else {
            right = mid;
        }
    }
    return right;
}

fn main() {
    let arr10: [i64; 10] = [3, 10, 0, -4, 18, 1, 2, 7, 2, -20];
    print!("Array: ");
    for i in arr10 {
        print!("{} ", i);
    }
    println!();
    println!("Max: {}", max_out_of_10(&arr10));

    print!("First 20 prime numbers: ");
    for i in 1..21 {
        print!("{} ", nth_prime(i));
    }
    println!();
    println!("10000th prime number: {}", nth_prime(10000));

    let mut sorted_arr = arr10;
    sorted_arr.sort();
    print!("Sorted array: ");
    for i in sorted_arr {
        print!("{} ", i);
    }
    println!();

    println!(
        "Lower-bound index of -123123: {}",
        lower_bound(&sorted_arr, -123123)
    );
    println!("Lower-bound index of 2: {}", lower_bound(&sorted_arr, 2));
    println!("Lower-bound index of 3: {}", lower_bound(&sorted_arr, 3));
    println!("Lower-bound index of 4: {}", lower_bound(&sorted_arr, 4));
    println!(
        "Lower-bound index of 4123123: {}",
        lower_bound(&sorted_arr, 4123123)
    );
}
