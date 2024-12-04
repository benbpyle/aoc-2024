use std::fs;

fn is_descending(line: Vec<i64>) -> bool {
    let mut previous: i64 = 0;

    for n in 0..line.len() {
        if n == 0 {
            previous = line[0];
            continue;
        }

        let diff = line[n] - previous;

        if !(-3..0).contains(&diff) {
            return false;
        }

        previous = line[n];
    }

    true
}

fn is_ascending(line: Vec<i64>) -> bool {
    let mut previous: i64 = 0;

    for n in 0..line.len() {
        if n == 0 {
            previous = line[0];
            continue;
        }

        let diff = line[n] - previous;

        if diff <= 0 || diff > 3 {
            return false;
        }

        previous = line[n];
    }

    true
}

fn is_safe(line: Vec<i64>) -> bool {
    let descending = is_descending(line.clone());

    if descending {
        return true;
    }

    let ascending = is_ascending(line.clone());

    if ascending {
        return true;
    }

    false
}

fn main() {
    let mut count = 0;
    for line in fs::read_to_string("./02/src/input.txt").unwrap().lines() {
        let line_vals = line.split_whitespace();
        let mut input: Vec<i64> = Vec::new();
        for l in line_vals {
            input.push(l.parse().expect("Left side not an i64"));
        }

        let is = is_safe(input);
        if is {
            count += 1;
        }
    }

    println!("(Count)={}", count);
}

#[cfg(test)]
mod tests {
    use crate::{is_ascending, is_descending};

    #[test]
    fn descending_1() {
        let l: Vec<i64> = vec![12, 11, 10, 9, 8, 7];
        let t = is_descending(l);

        assert!(t);
    }

    #[test]
    fn descending_2() {
        let l: Vec<i64> = vec![12, 13, 10, 9, 8, 7];
        let t = is_descending(l);

        assert!(!t);
    }

    #[test]
    fn descending_3() {
        let l: Vec<i64> = vec![12, 11, 10, 9, 8, 7, 3];
        let t = is_descending(l);

        assert!(!t);
    }
    #[test]
    fn ascending_1() {
        let l: Vec<i64> = vec![7, 8, 9, 10, 11];
        let t = is_ascending(l);

        assert!(t);
    }

    #[test]
    fn ascending_2() {
        let l: Vec<i64> = vec![7, 8, 6, 9, 10, 11];
        let t = is_ascending(l);

        assert!(!t);
    }

    #[test]
    fn ascending_3() {
        let l: Vec<i64> = vec![7, 8, 9, 14, 15];
        let t = is_ascending(l);

        assert!(!t);
    }
}
