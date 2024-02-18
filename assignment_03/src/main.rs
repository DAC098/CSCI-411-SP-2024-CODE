use std::str::FromStr;

#[derive(Debug, Clone)]
struct Change {
    total: usize,
    amounts: Vec<usize>,
}

impl Change {
    fn new(dnmn: usize) -> Self {
        let amounts = vec![0; dnmn];

        Change {
            total: 0,
            amounts,
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Set(Change),
    Unset,
    Invalid,
}

impl State {
    fn ref_change(&self) -> Option<&Change> {
        match self {
            State::Set(v) => Some(&v),
            _ => None
        }
    }
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
        if value.is_empty() {
            continue;
        }

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
    let mut method_to_use = 0;
    let mut run_checks = false;

    for arg in std::env::args() {
        match arg.as_str() {
            "--run-checks" => {
                run_checks = true;
            },
            "--bottom-up-2" => {
                method_to_use = 1;
            },
            "--top-down" => {
                method_to_use = 2;
            },
            _ => {}
        }
    }

    let mut total_denominations: usize = 0;
    let mut total_checks: usize = 0;
    let mut max_size: usize = 0;
    let mut lines = std::io::stdin().lines();
    let mut denominations: Vec<usize> = Vec::new();
    let mut checks: Vec<usize> = Vec::new();

    {
        let Some(check) = lines.next() else {
            panic!("no change line specified");
        };

        let change_line = check.expect("failed to read input from stdin");

        let Some(change_data) = parse_line::<usize>(&change_line) else {
            panic!("invalid change line provided: \"{}\"", change_line);
        };

        if change_data.len() < 2 {
            panic!("too few change values: \"{}\"", change_line);
        }

        if change_data[0] == 0 {
            panic!("amount of denominations specified is invalid: {}", change_data[0]);
        }

        total_denominations = change_data[0];
        total_checks = change_data[0];
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

    if denominations.len() != total_denominations {
        panic!("denominations provided nodes not match the specified amount");
    }

    while let Some(line) = lines.next() {
        let line_check = line.expect("failed to read input from stdin");

        let Ok(value): Result<usize, _> = line_check.parse() else {
            panic!("invalid check value provided: \"{}\"", line_check);
        };

        if value > max_size {
            max_size = value;
        }

        checks.push(value);
    }

    if checks.len() != total_checks {
        panic!("checks provided does not match the specified amount\n");
    }

    if run_checks {
        for dnmn in &denominations {
            print!("{} ", dnmn);
        }

        println!("");
    }

    match method_to_use {
        0 => {
            let mut running_count = vec![0; denominations.len()];
            let mut memorized_bottom_up = vec![(None, 0); max_size + 1];

            calc_bottom_up(&denominations, &mut memorized_bottom_up);

            for value in &checks {
                let mut index = *value;

                loop {
                    let (Some(v), lu) = &memorized_bottom_up[index] else {
                        break;
                    };

                    if *v == 0 {
                        break;
                    }

                    running_count[*lu] += 1;
                    index = index - denominations[*lu];
                }

                for count in &running_count {
                    print!("{} ", count);
                }

                println!("");

                running_count.fill(0);
            }
        },
        1 => {
            let mut memorized_bottom_up = vec![State::Unset; max_size + 1];

            calc_bottom_up_alt(&denominations, &mut memorized_bottom_up);

            for value in &checks {
                match &memorized_bottom_up[*value] {
                    State::Set(change) => {
                        if run_checks {
                            print!("{} => {}: ", value, change.total);

                            let mut count = 0;

                            for dnmn_index in 0..denominations.len() {
                                print!("{} ", change.amounts[dnmn_index]);

                                count += denominations[dnmn_index] * change.amounts[dnmn_index];
                            }

                            print!("= {}", count);

                            if count != *value {
                                println!(" invalid");
                            } else {
                                println!(" valid");
                            }
                        } else {
                            for dnmn in &change.amounts {
                                print!("{} ", dnmn);
                            }

                            println!("");
                        }
                    },
                    State::Invalid => {
                        for _ in &denominations {
                            print!("0 ");
                        }

                        println!("");
                    },
                    State::Unset => unreachable!(),
                }
            }
        },
        2 => {
            let mut memorized_top_down = vec![State::Unset; max_size + 1];

            denominations.reverse();

            for value in &checks {
                std::thread::scope(|scope| {
                    let result = std::thread::Builder::new()
                        .name(format!("top_down_{}", value))
                        .stack_size(512 * 1024 * 1024)
                        .spawn_scoped(scope, || {
                            calc_top_down(*value, &denominations, &mut memorized_top_down, 1);
                        });

                    if let Err(err) = result {
                        println!("thread error: {:#?}", err);
                    }
                });

                match &memorized_top_down[*value] {
                    State::Set(change) => {
                        if run_checks {
                            print!("{} => {}: ", value, change.total);
                            let mut count = 0;

                            for dnmn_index in (denominations.len() - 1)..=0 {
                                print!("{} ", change.amounts[dnmn_index]);

                                count += denominations[dnmn_index] * change.amounts[dnmn_index];
                            }

                            println!("= {}", count);
                        } else {
                            for dnmn in change.amounts.iter().rev() {
                                print!("{} ", dnmn);
                            }

                            println!("");
                        }

                        for dnmn in change.amounts.iter().rev() {
                            print!("{} ", dnmn);
                        }

                        println!("");
                    },
                    State::Invalid => {
                        for _ in &denominations {
                            print!("0 ");
                        }

                        println!("");
                    },
                    State::Unset => unreachable!(),
                }
            }
        },
        _ => unreachable!(),
    }
}

fn calc_top_down(change: usize, denominations: &[usize], memorized: &mut [State], context: usize) -> State {
    if change == 0 {
        #[cfg(debug_assertions)]
        println!("{} {} base case", context, change);

        return State::Set(Change::new(denominations.len()));
    }

    if let Some(found) = memorized[change].ref_change() {
        #[cfg(debug_assertions)]
        println!("{} {} memorized: {:?}", context, change, found);

        return State::Set(found.clone());
    }

    let mut lowest = State::Invalid;

    for dnmn in 0..denominations.len() {
        if denominations[dnmn] > change {
            #[cfg(debug_assertions)]
            println!("{} {} < dnmn {}", context, change, denominations[dnmn]);

            continue;
        }

        #[cfg(debug_assertions)]
        println!("{} {} sub: {}", context, change, denominations[dnmn]);

        let mut result = match calc_top_down(change - denominations[dnmn], denominations, memorized, context + 1) {
            State::Set(change) => change,
            State::Unset => unreachable!(),
            State::Invalid => continue,
        };

        lowest = match lowest {
            State::Set(curr) => {
                if curr.total > result.total {
                    result.total += 1;
                    result.amounts[dnmn] += 1;

                    #[cfg(debug_assertions)]
                    println!("{} {} updating lowest: {:?} prev: {:?}", context, change, result, curr);

                    State::Set(result)
                } else {
                    State::Set(curr)
                }
            },
            State::Unset => unreachable!(),
            State::Invalid => {
                result.total += 1;
                result.amounts[dnmn] += 1;

                #[cfg(debug_assertions)]
                println!("{} {} setting lowest: {:?}", context, change, result);

                State::Set(result)
            }
        };
    }

    memorized[change] = lowest;

    #[cfg(debug_assertions)]
    println!("{} {} result: {:?}", context, change, memorized[change]);

    memorized[change].clone()
}

fn calc_bottom_up_alt(denominations: &[usize], memorized: &mut [State]) {
    let dnmn_len = denominations.len();

    memorized[0] = State::Set(Change::new(dnmn_len));

    for index in 1..memorized.len() {
        let mut min = State::Invalid;

        #[cfg(debug_assertions)]
        println!("{}", index);

        for dnmn in 0..dnmn_len {
            #[cfg(debug_assertions)]
            print!("├─── {}", denominations[dnmn]);

            if denominations[dnmn] > index {
                #[cfg(debug_assertions)]
                println!(" skip");

                break;
            }

            let Some(cmp) = memorized[index - denominations[dnmn]].ref_change() else {
                #[cfg(debug_assertions)]
                println!(" no change for previous");

                continue;
            };

            #[cfg(debug_assertions)]
            print!(" {:?}", cmp);

            min = match min {
                State::Set(curr) => {
                    let mut rtn = cmp.clone();
                    rtn.total += 1;
                    rtn.amounts[dnmn] += 1;

                    #[cfg(debug_assertions)]
                    print!(" {} < {}", curr.total, rtn.total);

                    if curr.total <= rtn.total {
                        #[cfg(debug_assertions)]
                        println!(" no update");

                        State::Set(curr)
                    } else {
                        #[cfg(debug_assertions)]
                        println!(" updating to {:?}", rtn);

                        State::Set(rtn)
                    }
                },
                State::Unset => unreachable!(),
                State::Invalid => {
                    let mut rtn = cmp.clone();
                    rtn.total += 1;
                    rtn.amounts[dnmn] += 1;

                    #[cfg(debug_assertions)]
                    println!(" setting to {:?}", rtn);

                    State::Set(rtn)
                }
            };
        }

        #[cfg(debug_assertions)]
        println!("└─── memorizing {:?}", min);

        memorized[index] = min;
    }
}

fn calc_bottom_up(dnmn: &[usize], mem: &mut [(Option<usize>, usize)]) {
    mem[0] = (Some(0), 0);

    let mut min = None;
    let mut last_used = 0;

    for i in 1..mem.len() {
        min = None;
        last_used = 0;

        for d in 0..dnmn.len() {
            if dnmn[d] > i {
                continue;
            }

            let (Some(check), _) = &mem[i - dnmn[d]] else {
                continue;
            };

            let cmp = check + 1;

            min = Some(if let Some(min) = min {
                if min <= cmp {
                    min
                } else {
                    last_used = d;
                    cmp
                }
            } else {
                last_used = d;
                cmp
            });
        }

        mem[i] = (min, last_used);
    }
}
