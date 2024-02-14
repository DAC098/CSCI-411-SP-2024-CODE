fn main() {
    let checks = [
        (3, 27, vec![1, 5, 10, 25]),
        (3, 27, vec![25, 10, 5, 1]),
        (4, 20, vec![3,8,11]),
        (3, 11, vec![2,3,5]),
        (3, 17, vec![2,3,5,7]),
        (4, 15, vec![2,3,7]),
        (-1, 7, vec![3,5]),
        (-1, 1, vec![2,3,5])
    ];

    for (expected, change, denominations) in checks {
        let result = calc(change, &denominations);

        assert!(
            result == expected,
            "result: {} expected: {} change: {}\ndenominations: {:?}",
            result,
            expected,
            change,
            denominations
        );
    }
}

fn calc_recursive(change: i32, denominations: &[i32], memorized: &mut [isize], context: usize) -> isize {
    if change < 0 {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {} not found", context, change, width=(context + 1));

        return -1;
    } else if change == 0 {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {}", context, change, width=(context + 1));

        return 0;
    }

    let index = (change - 1) as usize;

    if memorized[index] != 0 {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {} memorized: {}", context, change, memorized[index], width=(context + 1));

        return memorized[index];
    }

    let mut did_update = false;
    let mut lowest = isize::MAX;

    for v in denominations {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {} sub: {}", context, change, v, width=(context + 1));

        let result = calc_recursive(change - *v, denominations, memorized, context + 1);

        if result != -1 && result < lowest {
            did_update = true;
            lowest = 1 + result;
        }
    }

    if did_update {
        memorized[index] = lowest;
    } else {
        memorized[index] = -1;
    }

    #[cfg(debug_assertions)]
    println!("{:-<width$} result: {}", context, memorized[index], width=(context + 1));

    memorized[index]
}

pub fn calc(change: i32, denominations: &[i32]) -> isize {
    #[cfg(debug_assertions)]
    println!("calc_change(change: {}, denominations: {:?})", change, denominations);

    if change <= 0 {
        0
    } else {
        let mut memorized = vec![0; change as usize];

        let result = calc_recursive(change, denominations, &mut memorized, 1);

        if cfg!(debug_assertions) {
            print!("results: | ");

            for index in 0..memorized.len() {
                print!("{}: {} | ", index + 1, memorized[index]);
            }

            println!("");
        }

        result
    }
}
