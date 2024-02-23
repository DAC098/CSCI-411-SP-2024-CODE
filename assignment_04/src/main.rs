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
    /*
    let mut bitonic_max = 0;

    println!("calculating increasing bitonic");

    {
        let (lis_max, lis_calced) = longest_increasing_subsequence(list);

        for index in 0..list.len() {
            let (lds_max, lds_calced) = longest_decreasing_subsequence(&list[index..list.len()]);

            let summed = lds_max + lis_calced[index] - 1;

            println!("index: {}\n{:?} {} | {:?} {}", index, &list[0..=index], lis_calced[index], &list[index..list.len()], lds_max);
            println!("{:?} | {:?}", &lis_calced[0..=index], lds_calced);
            println!("{} + {} - 1 = {}", lds_max, lis_calced[index], summed);

            if summed > bitonic_max {
                bitonic_max = summed;
            }
        }
    }

    println!("calculating decreasing bitonic");

    {
        let (lds_max, lds_calced) = longest_decreasing_subsequence(list);

        for index in 0..list.len() {
            let (lis_max, lis_calced) = longest_increasing_subsequence(&list[index..list.len()]);

            let summed = lis_max + lds_calced[index] - 1;

            println!("index: {}\n{:?} {} | {:?} {}", index, &list[0..=index], lds_calced[index], &list[index..list.len()], lis_max);
            println!("{:?} | {:?}", &lds_calced[0..=index], lis_calced);
            println!("{} + {} - 1 = {}", lis_max, lds_calced[index], summed);

            if summed > bitonic_max {
                bitonic_max = summed;
            }
        }
    }
    */
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

    println!("test_bitonic_max: {}", test_bitonic_max);

    let mut bitonic_max = 0;
    let rev_len = list.len() - 1;

    let mut ins_bit_lens = vec![(1usize, 1usize); list.len()];
    let mut dsc_bit_lens = vec![(1usize, 1usize); list.len()];

    for index in 1..list.len() {
        for subindex in 0..index {
            // increasing bitonic
            if list[subindex] < list[index] && ins_bit_lens[subindex].0 > ins_bit_lens[index].0 {
                ins_bit_lens[index].0 = ins_bit_lens[index].0;
            }

            if list[rev_len - subindex] < list[rev_len - index] && ins_bit_lens[rev_len - subindex].1 > ins_bit_lens[rev_len - index].1 {
                ins_bit_lens[rev_len - index].1 = ins_bit_lens[rev_len - subindex].1;
            }

            // decreasing bitonic
            if list[subindex] > list[index] && dsc_bit_lens[subindex].0 > dsc_bit_lens[index].0 {
                dsc_bit_lens[index].0 = dsc_bit_lens[index].0;
            }

            if list[rev_len - subindex] > list[rev_len - index] && dsc_bit_lens[rev_len - subindex].1 > dsc_bit_lens[rev_len - index].1 {
                dsc_bit_lens[rev_len - index].1 = dsc_bit_lens[rev_len - subindex].1;
            }
        }

        ins_bit_lens[index].0 += 1;
        ins_bit_lens[rev_len - index].1 += 1;
        dsc_bit_lens[index].0 += 1;
        dsc_bit_lens[rev_len - index].1 += 1;
    }

    println!("increasing: {:?}", ins_bit_lens);
    println!("decreasing: {:?}", dsc_bit_lens);

    for index in 0..list.len() {
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
    }

    bitonic_max
}

fn main() {
    let list = [
        ((3, 3), vec![2,7,4,3,8]),
        ((4, 2), vec![2,4,3,7,4,5]),
    ];

    for test in list {
        let lens = test.0;

        println!("test: ({}, {}) {:?}", lens.0, lens.1, test.1);

        let (lis_max, lis_calced) = longest_increasing_subsequence(&test.1);

        println!("LIS: {} {:?}", lis_max, lis_calced);

        let (lds_max, lds_calced) = longest_decreasing_subsequence(&test.1);

        println!("LDS: {} {:?}", lds_max, lds_calced);

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
}
