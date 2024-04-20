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

type Cost = i64;

#[derive(Clone)]
enum EditKind {
    Sub,
    Ins,
    Del,
    Mat,
}

impl std::fmt::Display for EditKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditKind::Sub => write!(f, "{}", 's'),
            EditKind::Ins => write!(f, "{}", 'i'),
            EditKind::Del => write!(f, "{}", 'd'),
            EditKind::Mat => write!(f, "{}", 'm'),
        }
    }
}

#[derive(Clone)]
struct Edit {
    value: Cost,
    kind: EditKind,
}

impl Edit {
    fn sub(value: Cost) -> Self {
        Edit { value, kind: EditKind::Sub }
    }

    fn ins(value: Cost) -> Self {
        Edit { value, kind: EditKind::Ins }
    }

    fn del(value: Cost) -> Self {
        Edit { value, kind: EditKind::Del }
    }

    fn mat(value: Cost) -> Self {
        Edit { value, kind: EditKind::Mat }
    }
}

impl std::fmt::Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.kind)
    }
}

struct EditResult {
    result: Vec<Vec<Edit>>,
    longest: usize,
}

/// finds the minimum edit distance between the two slices
///
/// grid layout
///     =   f   r   o   m
///   +---+---+---+---+---+
/// = |   |   |   |   |   |
///   +---+---+---+---+---+
/// t |   |   |   |   |   |
///   +---+---+---+---+---+
/// o |   |   |   |   |   |
///   +---+---+---+---+---+
fn edit_distance(from: &[u8], to: &[u8], ins: Cost, del: Cost, sub: Cost, verbose: bool) -> EditResult {
    let mut longest = 0;
    let from_len = from.len() + 1;
    let to_len = to.len() + 1;

    let mut memory = vec![vec![Edit::mat(0); from_len]; to_len];

    for to_index in 1..to_len {
        memory[to_index][0] = Edit::ins(memory[to_index - 1][0].value + ins);
    }

    for from_index in 1..from_len {
        memory[0][from_index] = Edit::del(memory[0][from_index - 1].value + del);
    }

    for from_index in 1..from_len {
        if verbose {
            println!("==============================");
        }

        for to_index in 1..to_len {
            if verbose {
                print!("{from_index}:{to_index}");
            }

            let curr_sub = memory[to_index - 1][from_index - 1].value + sub;
            let curr_del = memory[to_index][from_index - 1].value + del;
            let curr_ins = memory[to_index - 1][from_index].value + ins;

            memory[to_index][from_index] = if from[from_index - 1] == to[to_index - 1] {
                let mut min = Edit::mat(memory[to_index - 1][from_index - 1].value);

                if curr_sub < min.value {
                    min = Edit::sub(curr_sub);
                }

                if curr_del < min.value {
                    min = Edit::del(curr_del);
                }

                if curr_ins < min.value {
                    min = Edit::ins(curr_ins);
                }

                // for printing
                if verbose {
                    if min.value == 0 {
                        if 1 > longest {
                            longest = 1;
                        }
                    } else if min.value < 0 {
                        let check = min.value.abs().ilog10() + 2;

                        if check > longest {
                            longest = check;
                        }
                    } else {
                        let check = min.value.ilog10() + 1;

                        if check > longest {
                            longest = check;
                        }
                    }

                    println!(" -> {min}");
                }

                min
            } else {
                let mut min = Edit::sub(curr_sub);

                if curr_del < min.value {
                    min = Edit::del(curr_del);
                }

                if curr_ins < min.value {
                    min = Edit::ins(curr_ins);
                }

                // for printing
                if verbose {
                    if min.value == 0 {
                        if 1 > longest {
                            longest = 1;
                        }
                    } else if min.value < 0 {
                        let check = min.value.abs().ilog10() + 2;

                        if check > longest {
                            longest = check;
                        }
                    } else {
                        let check = min.value.ilog10() + 1;

                        if check > longest {
                            longest = check;
                        }
                    }

                    println!(" -> {min}");
                }

                min
            };
        }
    }

    if verbose {
        println!("==============================");
    }

    EditResult {
        result: memory,
        longest: longest as usize,
    }
}

fn main() {
    let mut verbose = false;
    let mut args = std::env::args();
    args.next();

    loop {
        let Some(arg) = args.next() else {
            break;
        };

        if arg == "--verbose" {
            verbose = true;
        }
    }

    let mut lines = std::io::stdin().lines();
    let ins: Cost;
    let del: Cost;
    let sub: Cost;

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

        let costs_line = lines.next()
            .expect("missing edit weights")
            .expect("failed to read input from stdin");

        let costs = parse_line::<Cost>(&costs_line)
            .expect("failed to parse costs line. invalid intager characters providied");

        if costs.len() != 3 {
            panic!("invalid number of costs provided. expected 3");
        }

        ins = costs[0];
        del = costs[1];
        sub = costs[2];
    }

    for line in lines {
        let valid = line.expect("failed to read input from stdin");

        let Some((from, to)) = valid.split_once(' ') else {
            panic!("invalid test string provided: \"{}\"", valid);
        };

        if !from.is_ascii() {
            panic!("string a contains non ascii characters");
        }

        if !to.is_ascii() {
            panic!("string b contains non ascii characters");
        }

        let from_bytes = from.as_bytes();
        let to_bytes = to.as_bytes();

        let rtn = edit_distance(from_bytes, to_bytes, ins, del, sub, verbose);
        let result = rtn.result;

        if verbose {
            let longest = rtn.longest;
            let leading_width = (to_bytes.len().ilog10() + 1) as usize;

            let dash_spacer = "-".repeat(longest);
            let spacer = " ".repeat(longest);
            let leading_dash_spacer = "-".repeat(leading_width);
            let leading_spacer = " ".repeat(leading_width);

            print!(" {leading_spacer}    |");

            for col in 0..=from_bytes.len() {
                print!(" {col:longest$} ");
            }

            println!("");
            print!(" {leading_spacer}    |");

            for index in 0..=from_bytes.len() {
                if index == 0 {
                    print!(" {spacer} ");
                } else {
                    print!(" {:>longest$} ", char::from(from_bytes[index - 1]));
                }
            }

            println!("");
            print!("-{leading_dash_spacer}----+");

            for _ in 0..=from_bytes.len() {
                print!("-{dash_spacer}-");
            }

            println!("");

            for (index, row) in result.iter().enumerate() {
                if index == 0 {
                    print!(" {index:leading_width$}    |");
                } else {
                    print!(" {index:leading_width$}  {} |", char::from(to_bytes[index - 1]));
                }

                for pair in row {
                    print!(" {:longest$}{}", pair.value, pair.kind);
                }

                println!("");
            }
        }

        let mut from_index = from_bytes.len();
        let mut to_index = to_bytes.len();
        let edit_value = result[to_bytes.len()][from_bytes.len()].value;

        let mut from_output = Vec::new();
        let mut to_output = Vec::new();

        while from_index != 0 && to_index != 0 {
            if verbose {
                print!("indexs: {from_index}:{to_index}");
            }

            match result[to_index][from_index].kind {
                EditKind::Mat => {
                    if verbose {
                        print!(" mat");
                    }

                    from_output.push(from_bytes[from_index - 1]);
                    to_output.push(to_bytes[to_index - 1]);
                    from_index -= 1;
                    to_index -= 1;
                }
                EditKind::Sub => {
                    if verbose {
                        print!(" sub");
                    }

                    from_output.push(from_bytes[from_index - 1]);
                    to_output.push(to_bytes[to_index - 1]);
                    from_index -= 1;
                    to_index -= 1;
                }
                EditKind::Ins => {
                    if verbose {
                        print!(" ins");
                    }

                    from_output.push(b'_');
                    to_output.push(to_bytes[to_index - 1]);
                    to_index -= 1;
                }
                EditKind::Del => {
                    if verbose {
                        print!(" del");
                    }

                    from_output.push(from_bytes[from_index - 1]);
                    to_output.push(b'_');
                    from_index -= 1;
                },
            }

            if verbose {
                println!(" -> {from_index}:{to_index}");
            }
        }

        while from_index != 0 {
            from_output.push(from_bytes[from_index - 1]);
            to_output.push(b'_');
            from_index -= 1;
        }

        while to_index != 0 {
            from_output.push(b'_');
            to_output.push(to_bytes[to_index - 1]);
            to_index -= 1;
        }

        from_output.reverse();
        to_output.reverse();

        println!(
            "{}\n{}\n{edit_value}",
            std::str::from_utf8(&from_output).unwrap(),
            std::str::from_utf8(&to_output).unwrap()
        );
    }
}
