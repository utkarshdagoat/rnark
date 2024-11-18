pub(crate) fn digit_to_str(num: u64) -> Option<char> {
    match num {
        0 => Some('0'),
        1 => Some('1'),
        2 => Some('2'),
        3 => Some('3'),
        4 => Some('4'),
        5 => Some('5'),
        6 => Some('6'),
        7 => Some('7'),
        8 => Some('8'),
        9 => Some('9'),
        10 => Some('a'),
        11 => Some('b'),
        12 => Some('c'),
        13 => Some('d'),
        14 => Some('e'),
        15 => Some('f'),
        _ => None,
    }
}

pub(crate) fn remove_trailing_zeroes(vec: &mut Vec<u64>) {
    match vec.iter().rposition(|&x| x != 0) {
        Some(index) => vec.truncate(index + 1),
        None => *vec = vec![0],
    }
}
