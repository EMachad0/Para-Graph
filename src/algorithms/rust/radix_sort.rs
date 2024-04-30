use itertools::Itertools;
use rayon::prelude::*;

pub fn radix_sort_serial(arr: &mut [usize]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x,
        None => return,
    };
    let radix = arr.len().next_power_of_two();
    let mut base = 1;
    while base <= max {
        let digit_of = |x| x / base % radix;
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        base *= radix;
    }
}

pub fn radix_sort_par(arr: &mut [usize]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x,
        None => return,
    };
    let radix = arr.len().next_power_of_two();
    let mut base = 1;
    while base <= max {
        let digit_of = |x| x / base % radix;
        let digits = arr.iter().map(|&x| digit_of(x)).collect_vec();
        let mut counter = vec![0; radix];
        digits.iter().for_each(|&x| {
            counter[x] += 1;
        });
        let mut counter = counter
            .iter()
            .scan(0, |s, &e| {
                *s += e;
                Some(*s)
            })
            .collect_vec();
        let idxs = digits
            .iter()
            .rev()
            .map(|&x| {
                counter[x] -= 1;
                counter[x]
            })
            .collect_vec();
        let idxs = idxs.into_iter().rev().collect_vec();
        arr.to_owned().iter().enumerate().for_each(|(i, &x)| {
            arr[idxs[i]] = x;
        });
        base *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_ARR: [usize; 40] = [
        963, 482, 145, 973, 281, 856, 724, 329, 920, 198, 29, 735, 503, 920, 74, 621, 415, 877,
        266, 253, 499, 782, 720, 481, 444, 96, 762, 901, 864, 679, 503, 3, 650, 718, 644, 380, 66,
        368, 192, 370,
    ];

    #[test]
    fn ascending_serial() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_serial(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn descending_serial() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_serial(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn large_random_serial() {
        let mut v = LARGE_ARR.to_vec();
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_serial(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn ascending_cpu() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_par(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn descending_cpu() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_par(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn large_random_cpu() {
        let mut v = LARGE_ARR.to_vec();
        let expected = v.iter().cloned().sorted().collect_vec();
        radix_sort_par(&mut v);
        assert_eq!(v, expected);
    }
}
