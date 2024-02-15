struct Change {
    total: usize,
    amounts: Vec<(i32, usize)>,
}

impl Change {
    fn new(denominations: &[i32]) -> Self {
        let mut amounts = Vec::with_capacity(denominations.len());

        for value in denominations {
            amounts.push((*value, 0));
        }

        Change {
            total: 0,
            amounts,
        }
    }
}

enum State {
    Unset,
    Invalid,
    Set(Change)
}

impl Default for State {
    fn default() -> Self {
        State::Unset
    }
}

fn parse_int_line_fill(line: &str, list: &mut Vec<i32>) -> bool {
    let split = line.split(' ');

    for value in split {
        let Ok(parsed) = value.parse() else {
            return false;
        };

        list.push(parsed);
    }

    true
}

fn parse_int_line(line: &str) -> Option<Vec<i32>> {
    let mut rtn = Vec::new();

    if parse_int_line_fill(line, &mut rtn) {
        Some(rtn)
    } else {
        None
    }
}

fn main() {
    if false {
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
            let result = calc_top_down(change, &denominations);

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

    let mut lines = std::io::stdin().lines();
    let mut denominations: Vec<i32> = Vec::new();
    let mut checks: Vec<i32> = Vec::new();
    let mut total_denominations: usize = 0;
    let mut total_checks: usize = 0;

    {
        let Some(check) = lines.next() else {
            panic!("no change line specified");
        };

        let change_line = check.expect("failed to read input from stdin");

        let Some(change_data) = parse_int_line(&change_line) else {
            panic!("invalid change line provided: \"{}\"", change_line);
        };

        if change_data.len() < 2 {
            panic!("too few change values: \"{}\"", change_line);
        }

        total_denominations = {
            let Ok(check): Result<usize, _> = change_data[0].try_into() else {
                panic!("amount of denominations specified is invalid: {}", change_data[0]);
            };

            if check == 0 {
                panic!("amount of denominations specified is invalid: {}", change_data[0]);
            }

            check
        };

        total_checks = {
            let Ok(check): Result<usize, _> = change_data[1].try_into() else {
                panic!("amount of checks specified is invalid: {}", change_data[1]);
            };

            check
        };
    }

    {
        let Some(check) = lines.next() else {
            panic!("no denominations specified");
        };

        let check_line = check.expect("failed to read input from stdin");

        if !parse_int_line_fill(&check_line, &mut denominations) {
            panic!("invalid denominations line provided: \"{}\"", check_line);
        }
    }

    let mut max_size: usize = 0;

    while let Some(line) = lines.next() {
        let line_check = line.expect("failed to read input from stdin");

        let Ok(value): Result<i32, _> = line_check.parse() else {
            panic!("invalid check value provided: \"{}\"", line_check);
        };

        if value < 0 {
            panic!("cannot calculate negative change: {}", value);
        }

        if (value as usize) > max_size {
            max_size = (value as usize);
        }

        checks.push(value);
    }

    let mut memorized = vec![0; max_size + 1];

    for value in &checks {
        std::thread::scope(|scope| {
            let result = std::thread::Builder::new()
                .name(format!("top_down_{}", value))
                .stack_size(512 * 1024 * 1024)
                .spawn_scoped(scope, || {
                    println!("calculating change for: {}", value);

                    let calc_result = calc_top_down_recurse(*value, &denominations, &mut memorized, 1);

                    println!("result: {}", calc_result);
                });

            if let Err(err) = result {
                println!("thread error: {:#?}", err);
            }
        });
    }
}

fn calc_top_down(change: i32, denominations: &[i32], memorized: &mut [isize], context: usize) -> isize {
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

        let result = calc_top_down_recurse(change - *v, denominations, memorized, context + 1);

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
