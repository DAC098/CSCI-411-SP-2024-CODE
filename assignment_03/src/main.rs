use std::str::FromStr;

struct Change {
    total: usize,
    amounts: Vec<usize>,
}

impl Change {
    fn new(denominations: &[i32]) -> Self {
        let amounts = vec![0; denominations.len()];

        Change {
            total: 0,
            amounts,
        }
    }
}

enum State {
    Set(Change),
    Unset,
    Invalid,
}

impl Default for State {
    fn default() -> Self {
        State::Unset
    }
}

fn parse_line_fill<T>(line: &str, list: &mut Vec<T>) -> bool
where
    T: FromStr
{
    let split = line.split(' ');

    for value in split {
        let Ok(parsed) = value.parse() else {
            return false;
        };

        list.push(parsed);
    }

    true
}

fn parse_line<T>(line: &str) -> Option<Vec<T>>
where
    T: FromStr
{
    let mut rtn = Vec::new();

    if parse_line_fill(line, &mut rtn) {
        Some(rtn)
    } else {
        None
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut denominations: Vec<usize> = Vec::new();
    let mut checks: Vec<usize> = Vec::new();
    let mut total_denominations: usize = 0;
    let mut total_checks: usize = 0;

    {
        let Some(check) = lines.next() else {
            panic!("no change line specified");
        };

        let change_line = check.expect("failed to read input from stdin");

        let Some(change_data) = parse_line::<i32>(&change_line) else {
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

        if !parse_line_fill(&check_line, &mut denominations) {
            panic!("invalid denominations line provided: \"{}\"", check_line);
        }
    }

    let mut max_size: usize = 0;

    while let Some(line) = lines.next() {
        let line_check = line.expect("failed to read input from stdin");

        let Ok(value): Result<usize, _> = line_check.parse() else {
            panic!("invalid check value provided: \"{}\"", line_check);
        };

        if value < 0 {
            panic!("cannot calculate negative change: {}", value);
        }

        if value > max_size {
            max_size = value;
        }

        checks.push(value);
    }

    let mut memorized_top_down = vec![Some(0); max_size + 1];
    let mut memorized_bottom_up = vec![0; max_size + 1];

    for value in &checks {
        std::thread::scope(|scope| {
            let result = std::thread::Builder::new()
                .name(format!("top_down_{}", value))
                .stack_size(512 * 1024 * 1024)
                .spawn_scoped(scope, || {
                    println!("calculating change for: {}", value);

                    let calc_result = calc_top_down(*value, &denominations, &mut memorized_top_down, 1);

                    println!("result: {}", calc_result.unwrap_or(0));
                });

            if let Err(err) = result {
                println!("thread error: {:#?}", err);
            }
        });
    }

    calc_bottom_up(&denominations, &mut memorized_bottom_up);

    for value in &checks {
        println!("calculated change for: {} | {}",
            value,
            memorized_bottom_up[*value as usize]
        );
    }
}

fn calc_top_down(change: usize, denominations: &[usize], memorized: &mut [Option<usize>], context: usize) -> Option<usize> {
    if change == 0 {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {}", context, change, width=(context + 1));

        return Some(0);
    }

    if memorized[change].is_some() {
        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {} memorized: {}", context, change, memorized[change], width=(context + 1));

        return memorized[change].clone();
    }

    let mut lowest: Option<usize> = None;

    for v in denominations {
        if *v > change {
            continue;
        }

        #[cfg(debug_assertions)]
        println!("{:-<width$} change: {} sub: {}", context, change, v, width=(context + 1));

        let Some(result) = calc_top_down(change - *v, denominations, memorized, context + 1) else {
            continue;
        };

        lowest = Some(if let Some(l) = lowest {
            if result < l {
                result + 1
            } else {
                l
            }
        } else {
            result + 1
        });
    }

    memorized[change] = lowest;

    #[cfg(debug_assertions)]
    println!("{:-<width$} result: {}", context, memorized[change], width=(context + 1));

    memorized[change].clone()
}

fn calc_bottom_up(denominations: &[usize], memorized: &mut [usize]) {
    let mut min = None::<usize>;

    for index in 1..memorized.len() {
        for dnmn in denominations {
            if *dnmn > index {
                break;
            }

            min = Some(if let Some(m) = min {
                if m < memorized[index - *dnmn] {
                    m
                } else {
                    memorized[index - *dnmn]
                }
            } else {
                memorized[index - *dnmn]
            } + 1);
        }

        if let Some(min) = min.take() {
            memorized[index] = min;
        }
    }
}
