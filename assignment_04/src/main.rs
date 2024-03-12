use std::str::FromStr;

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

fn longest_increasing_subsequence(list: &[i32]) -> (usize, Vec<usize>) {
    if list.is_empty() {
        return (0, vec![]);
    }

    let mut max_len = 1;
    let mut lengths = vec![1usize; list.len()];

    for index in 1..list.len() {
        let mut max = 0;

        for subindex in 0..index {
            if list[subindex] < list[index] && lengths[subindex] > max {
                max = lengths[subindex];
            }
        }

        max += 1;

        if max > 1 {
            if max > max_len {
                max_len = max;
            }

            lengths[index] = max;
        }
    }

    (max_len, lengths)
}

fn longest_decreasing_subsequence(list: &[i32]) -> (usize, Vec<usize>) {
    if list.is_empty() {
        return (0, vec![]);
    }

    let mut max_len = 1;
    let mut lengths = vec![1usize; list.len()];

    for index in 1..list.len() {
        let mut max = 0;

        for subindex in 0..index {
            if list[subindex] > list[index] && lengths[subindex] > max {
                max = lengths[subindex];
            }
        }

        max += 1;

        if max > 1 {
            if max > max_len {
                max_len = max;
            }

            lengths[index] = max;
        }
    }

    (max_len, lengths)
}

struct Calced {
    max_len: usize,
    calced: Vec<usize>,
}

fn longest_inc_dsc_subsequence(list: &[i32]) -> (Calced, Calced) {
    let mut lis_max_len = 1;
    let mut lds_max_len = 1;
    let mut lis_lengths = vec![1usize; list.len()];
    let mut lds_lengths = vec![1usize; list.len()];

    for index in 1..list.len() {
        let mut lis_max = 0;
        let mut lds_max = 0;

        for subindex in 0..index {
            if list[subindex] < list[index] && lis_lengths[subindex] > lis_max {
                lis_max = lis_lengths[subindex];
            }

            if list[subindex] > list[index] && lds_lengths[subindex] > lds_max {
                lds_max = lds_lengths[subindex];
            }
        }

        lis_max += 1;
        lds_max += 1;

        if lis_max > 1 {
            if lis_max > lis_max_len {
                lis_max_len = lis_max;
            }

            lis_lengths[index] = lis_max;
        }

        if lds_max > 1 {
            if lds_max > lds_max_len {
                lds_max_len = lds_max;
            }

            lds_lengths[index] = lds_max;
        }
    }

    (Calced {
        max_len: lis_max_len,
        calced: lis_lengths,
    },
    Calced {
        max_len: lds_max_len,
        calced: lds_lengths,
    })
}

fn longest_bitonic_subsequence(list: &[i32]) -> usize {
    let start = std::time::Instant::now();

    let mut bitonic_max = 0;

    //println!("calculating increasing bitonic");

    {
        let (_lis_max, lis_calced) = longest_increasing_subsequence(list);

        for index in 0..list.len() {
            let (lds_max, _lds_calced) = longest_decreasing_subsequence(&list[index..list.len()]);

            let summed = lds_max + lis_calced[index] - 1;

            //println!("index: {}\n{:?} {} | {:?} {}", index, &list[0..=index], lis_calced[index], &list[index..list.len()], lds_max);
            //println!("{:?} | {:?}", &lis_calced[0..=index], lds_calced);
            //println!("{} + {} - 1 = {}", lds_max, lis_calced[index], summed);

            if summed > bitonic_max {
                bitonic_max = summed;
            }
        }
    }

    //println!("calculating decreasing bitonic");

    {
        let (_lds_max, lds_calced) = longest_decreasing_subsequence(list);

        for index in 0..list.len() {
            let (lis_max, _lis_calced) = longest_increasing_subsequence(&list[index..list.len()]);

            let summed = lis_max + lds_calced[index] - 1;

            //println!("index: {}\n{:?} {} | {:?} {}", index, &list[0..=index], lds_calced[index], &list[index..list.len()], lis_max);
            //println!("{:?} | {:?}", &lds_calced[0..=index], lis_calced);
            //println!("{} + {} - 1 = {}", lis_max, lds_calced[index], summed);

            if summed > bitonic_max {
                bitonic_max = summed;
            }
        }
    }

    let duration = start.elapsed();

    println!("1 duration: {:?}", duration);

    let start = std::time::Instant::now();

    let mut test_bitonic_max = 0;

    {
        let (lis_base, lds_base) = longest_inc_dsc_subsequence(list);

        for index in 0..list.len() {
            let (sub_lis, sub_lds) = longest_inc_dsc_subsequence(&list[index..list.len()]);

            let lis_sum = sub_lds.max_len + lis_base.calced[index] - 1;
            let lds_sum = sub_lis.max_len + lds_base.calced[index] - 1;

            if lis_sum > lds_sum {
                if lis_sum > test_bitonic_max {
                    test_bitonic_max = lis_sum;
                }
            } else {
                if lds_sum > test_bitonic_max {
                    test_bitonic_max = lds_sum;
                }
            }
        }
    }

    let duration = start.elapsed();

    println!("2 duration: {:?}", duration);

    let start = std::time::Instant::now();

    let mut bitonic_max = 0;
    let rev_len = list.len() - 1;

    let mut ins_bit_lens = vec![(0usize, 0usize); list.len()];
    let mut dsc_bit_lens = vec![(0usize, 0usize); list.len()];
    ins_bit_lens[0].0 = 1;
    ins_bit_lens[rev_len].1 = 1;
    dsc_bit_lens[0].0 = 1;
    dsc_bit_lens[rev_len].1 = 1;

    for index in 1..list.len() {
        let rev_index = rev_len - index;

        for subindex in 0..index {
            let rev_subindex = rev_len - subindex;

            // increasing bitonic
            if list[subindex] < list[index] && ins_bit_lens[subindex].0 > ins_bit_lens[index].0 {
                ins_bit_lens[index].0 = ins_bit_lens[subindex].0;
            }

            if list[rev_subindex] < list[rev_index] && ins_bit_lens[rev_subindex].1 > ins_bit_lens[rev_index].1 {
                ins_bit_lens[rev_index].1 = ins_bit_lens[rev_subindex].1;
            }

            // decreasing bitonic
            if list[subindex] > list[index] && dsc_bit_lens[subindex].0 > dsc_bit_lens[index].0 {
                dsc_bit_lens[index].0 = dsc_bit_lens[subindex].0;
            }

            if list[rev_subindex] > list[rev_index] && dsc_bit_lens[rev_subindex].1 > dsc_bit_lens[rev_index].1 {
                dsc_bit_lens[rev_index].1 = dsc_bit_lens[rev_subindex].1;
            }
        }

        ins_bit_lens[index].0 += 1;
        ins_bit_lens[rev_index].1 += 1;
        dsc_bit_lens[index].0 += 1;
        dsc_bit_lens[rev_index].1 += 1;

        if index == rev_index {
            let lis_value = ins_bit_lens[index].0 + ins_bit_lens[index].1 - 1;
            let lds_value = dsc_bit_lens[index].0 + dsc_bit_lens[index].1 - 1;

            if lis_value > lds_value {
                if lis_value > bitonic_max {
                    bitonic_max = lis_value;
                }
            } else {
                if lds_value > bitonic_max {
                    bitonic_max = lds_value;
                }
            }
        } else if index > rev_index {
            bitonic_max = [
                ins_bit_lens[index].0 + ins_bit_lens[index].1 - 1,
                ins_bit_lens[rev_index].0 + ins_bit_lens[rev_index].1 - 1,
                dsc_bit_lens[index].0 + dsc_bit_lens[index].1 - 1,
                dsc_bit_lens[rev_index].0 + dsc_bit_lens[rev_index].1 - 1,
                bitonic_max
            ].into_iter().max().unwrap();
        }
    }

    let duration = start.elapsed();

    println!("3 duration: {:?}", duration);

    bitonic_max
}

fn lis_lds_main() {
    /*
    let list = [
        ((3, 3), vec![2,7,4,3,8]),
        ((4, 2), vec![2,4,3,7,4,5]),
    ];

    for test in list {
        let lens = test.0;

        let (lis_max, _lis_calced) = longest_increasing_subsequence(&test.1);

        let (lds_max, _lds_calced) = longest_decreasing_subsequence(&test.1);

        assert_eq!(lis_max, lens.0);
        assert_eq!(lds_max, lens.1);
    }

    let bitonic = [
        (7, vec![1,2,3,4,3,2,1]),
        (5, vec![5,7,3,2,8,5,5,2]),
        (5, vec![3,1,7,8,8,4,10,23]),
        (0, vec![5,8,8,3,4,1,7,-3,2,9,12]),
        (0, vec![1,11,2,10,4,5,2,1]),
        (0, vec![1,2,5,3,2]),
        (0, vec![4,5,9,7,6,3,1]),
    ];

    for test in bitonic {
        println!("bit: {} {:?}", test.0, test.1);

        let max = longest_bitonic_subsequence(&test.1);

        println!("max: {}", max);
    }
    */

    let check =     [5,8,8,3,4,1,7,-3,2,9,12];
    let check_rev = [12,9,2,-3,7,1,4,3,8,8,5];
    let mut lis_sum = [0,0,0,0,0,0,0,0,0,0,0];
    let mut lds_sum = [0,0,0,0,0,0,0,0,0,0,0];

    print!("   base:");

    for v in check {
        print!(" {:2}", v);
    }

    print!("\n\n    lis:");

    let (lis_max, lis_calced) = longest_increasing_subsequence(&check);

    for index in 0..lis_calced.len() {
        print!(" {:2}", lis_calced[index]);
        lis_sum[index] += lis_calced[index];
    }

    print!("\nrev lis:");

    let (lis_max, mut lis_calced) = longest_increasing_subsequence(&check_rev);
    lis_calced.reverse();

    for index in 0..lis_calced.len() {
        print!(" {:2}", lis_calced[index]);
        lis_sum[index] += lis_calced[index];
    }

    print!("\n    sum:");

    for v in lis_sum {
        print!(" {:2}", v - 1);
    }

    print!("\n\n    lds:");

    let (lds_max, lds_calced) = longest_decreasing_subsequence(&check);

    for index in 0..lds_calced.len() {
        print!(" {:2}", lds_calced[index]);
        lds_sum[index] += lds_calced[index];
    }

    print!("\nrev lds:");

    let (lds_max, mut lds_calced) = longest_decreasing_subsequence(&check_rev);
    lds_calced.reverse();

    for index in 0..lds_calced.len() {
        print!(" {:2}", lds_calced[index]);
        lds_sum[index] += lds_calced[index];
    }

    print!("\n    sum:");

    for v in lds_sum {
        print!(" {:2}", v - 1);
    }

    println!("");
}

fn min_index(list: &[usize]) -> usize {
    if list.is_empty() {
        return 0;
    }

    let mut curr = 0;

    for index in 1..list.len() {
        if list[curr] > list[index] {
            curr = index;
        }
    }

    curr
}

fn edit_distance(a: &[u8], b: &[u8], ins: usize, del: usize, sub: usize) -> Vec<Vec<usize>> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    let a_len = a.len() + 1;
    let b_len = b.len() + 1;

    let mut p = vec![vec![0usize; b_len]; a_len];
    p[0][0] = 0;

    for index in 1..a_len {
        p[index][0] = p[index - 1][0] + del;
    }

    for index in 1..b_len {
        p[0][index] = p[0][index - 1] + ins;
    }

    for a_index in 1..a_len {
        for b_index in 1..b_len {
            if a[a_index - 1] == b[b_index - 1] {
                p[a_index][b_index] = p[a_index - 1][b_index - 1];
            } else {
                let check = [
                    p[a_index - 1][b_index - 1] + sub,
                    p[a_index][b_index - 1] + ins,
                    p[a_index - 1][b_index] + del
                ];

                let min_index = min_index(&check);

                p[a_index][b_index] = check[min_index];
            }
        }
    }

    p
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let ins: usize;
    let del: usize;
    let sub: usize;

    {
        let total: usize = {
            let check = lines.next()
                .expect("no edit distance data specified")
                .expect("failed to read input from stdin");

            let Ok(rtn) = check.parse() else {
                panic!("failed to parse total strings line: \"{}\"", check);
            };

            rtn
        };

        let weights_line = lines.next()
            .expect("missing edit weights")
            .expect("failed to read input from stdin");

        let weights = parse_line::<usize>(&weights_line)
            .expect("failed to parse weights line. invalid integer characters providied");

        if weights.len() != 3 {
            panic!("invalid number of weights provided. expected 3");
        }

        ins = weights[0];
        del = weights[1];
        sub = weights[2];
    }

    for line in lines {
        let valid = line.expect("failed to read input from stdin");

        let Some((a, b)) = valid.split_once(' ') else {
            panic!("invalid test string provided: \"{}\"", valid);
        };

        if !a.is_ascii() {
            panic!("string a contains non ascii characters");
        }

        if !b.is_ascii() {
            panic!("string b contains non ascii characters");
        }

        let result = edit_distance(a.as_bytes(), b.as_bytes(), ins, del, sub);

        let edit_value = result[a.len()][b.len()];

        eprintln!("{} {}", a, b);

        for list in result {
            eprintln!("{:?}", list);
        }

        eprintln!("result: {}", edit_value);
    }
}
