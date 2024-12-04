use std::{
    fs,
    io::{BufRead, BufReader},
};

fn report_is_valid(report: &Vec<i64>) -> bool {
    if report.len() == 1 {
        return true;
    }
    // check ascending or descending order
    let is_ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = report.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    // check diff is within range [1,3]
    let is_valid_range = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .all(|x| x.abs() >= 1 && x.abs() <= 3);

    is_valid_range
}

fn report_is_tolerable(report: &Vec<i64>) -> bool {
    if report_is_valid(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);
        if report_is_valid(&report_copy) {
            return true;
        }
    }

    false
}

fn is_safe(line: Vec<i64>) -> bool {
    let v = report_is_valid(&line);
    if v {
        return true;
    }

    let t = report_is_tolerable(&line);

    if t {
        return true;
    }

    false
}

// 465
fn main() {
    let mut count = 0;
    for line in fs::read_to_string("./02/src/input.txt").unwrap().lines() {
        let line_vals = line.split_whitespace();
        let mut input: Vec<i64> = Vec::new();
        for l in line_vals {
            input.push(l.parse().expect("Left side not an i64"));
        }

        let is = is_safe(input.clone());
        println!("(Line)={:?}, (Safety)={}", input.clone(), is);
        if is {
            count += 1;
        }
    }

    //    let count = part2();
    println!("(Count)={}", count);
}

#[cfg(test)]
mod tests {
    use crate::is_safe;

    #[test]
    fn descending_1() {
        let l: Vec<i64> = vec![12, 11, 10, 9, 8, 7];
        let t = is_safe(l);

        assert!(t);
    }

    #[test]
    fn descending_2() {
        let l: Vec<i64> = vec![12, 13, 10, 9, 8, 7];
        let t = is_safe(l);

        assert!(t);
    }

    #[test]
    fn descending_3() {
        let l: Vec<i64> = vec![12, 11, 10, 9, 8, 7, 3];
        let t = is_safe(l);

        assert!(t);
    }
    #[test]
    fn ascending_1() {
        let l: Vec<i64> = vec![7, 8, 9, 10, 11];
        let t = is_safe(l);

        assert!(t);
    }

    #[test]
    fn ascending_2() {
        let l: Vec<i64> = vec![7, 8, 6, 9, 10, 11];
        let t = is_safe(l);

        assert!(t);
    }

    #[test]
    fn ascending_3() {
        let l: Vec<i64> = vec![7, 8, 9, 14, 15];
        let t = is_safe(l);

        assert!(!t);
    }

    #[test]
    fn is_safe_1() {
        let l: Vec<i64> = vec![2, 2, 3, 4, 2, 8];
        let t = is_safe(l);
        assert!(!t);
    }

    #[test]
    fn is_safe_2() {
        let l: Vec<i64> = vec![1, 1, 2, 3, 4];
        let t = is_safe(l);
        assert!(t);
    }

    #[test]
    fn is_safe_odd_1() {
        let l = vec![3, 8, 6, 8, 10, 12, 15];
        let t = is_safe(l);
        assert!(t);
    }
}
