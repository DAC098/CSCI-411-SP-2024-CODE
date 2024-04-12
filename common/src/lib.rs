use std::str::FromStr;

pub fn parse_line_fill<T>(line: &str, list: &mut Vec<T>) -> bool
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

pub fn parse_line<T>(line: &str) -> Option<Vec<T>>
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

pub fn get_int_len(v: i64) -> usize {
    if v == 0 {
        1
    } else if v < 0 {
        (v.abs().ilog10() + 2) as usize
    } else {
        (v.ilog10() + 1) as usize
    }
}

pub fn get_usize_len(v: usize) -> usize {
    if v == 0 {
        1
    } else {
        (v.ilog10() + 1) as usize
    }
}
