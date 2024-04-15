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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let max: usize = match arr.iter().max() {
        Some(&x) => x,
        None => return,
    };
    let radix = arr.len().next_power_of_two();
    let mut base = 1;
    while base <= max {
        let digit_of = |x| x / base % radix;
        let counter = Arc::new((0..radix).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
        arr.par_iter().for_each(|&x| {
            counter[digit_of(x)].fetch_add(1, Ordering::Relaxed);
        });
        counter.iter().tuple_windows().for_each(|(a, b)| {
            b.store(
                b.load(Ordering::Relaxed) + a.load(Ordering::Relaxed),
                Ordering::Relaxed,
            );
        });

        let sorted = Arc::new(
            (0..arr.len())
                .map(|_| AtomicUsize::new(0))
                .collect::<Vec<_>>(),
        );
        arr.par_iter().rev().for_each(|&x| {
            let idx = counter[digit_of(x)].fetch_sub(1, Ordering::Relaxed);
            sorted[idx - 1].store(x, Ordering::Relaxed);
        });

        arr.par_iter_mut().enumerate().for_each(|(i, x)| {
            *x = sorted[i].load(Ordering::Relaxed);
        });
        base *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_serial() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        radix_sort_serial(&mut v);
        let expected = v.iter().sorted().cloned().collect_vec();
        assert_eq!(v, expected);
    }

    #[test]
    fn descending_serial() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        radix_sort_serial(&mut v);
        let expected = v.iter().sorted().cloned().collect_vec();
        assert_eq!(v, expected);
    }

    #[test]
    fn ascending_par() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        radix_sort_par(&mut v);
        let expected = v.iter().sorted().cloned().collect_vec();
        assert_eq!(v, expected);
    }

    #[test]
    fn descending_par() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        radix_sort_par(&mut v);
        let expected = v.iter().sorted().cloned().collect_vec();
        assert_eq!(v, expected);
    }
}
