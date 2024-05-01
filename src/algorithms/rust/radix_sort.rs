use crate::algorithms::prefix_sum::pref_sum_par_cpu;
use itertools::Itertools;
use rayon::prelude::*;
use std::cell::Cell;

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

    let chunks = rayon::current_num_threads();
    let chunk_size = arr.len().div_ceil(chunks);
    let mut digits = Vec::new();

    let mut counters = vec![0usize; radix * chunks]
        .chunks_exact(radix)
        .map(|x| x.to_vec())
        .collect_vec();
    while base <= max {
        counters.iter_mut().for_each(|x| x.fill(0));

        let digit_of = |x| x / base % radix;
        arr.par_iter()
            .map(|&x| digit_of(x))
            .collect_into_vec(&mut digits);

        digits
            .par_chunks(chunk_size)
            .zip(counters.par_iter_mut())
            .for_each(|(chunk, counter)| {
                chunk.iter().for_each(|&x| {
                    counter[x] += 1;
                });
            });

        let slice = &mut counters[..];
        let slice_of_cells: &[Cell<_>] = Cell::from_mut(slice).as_slice_of_cells();
        slice_of_cells.windows(2).for_each(|window| {
            let prev = window[0].take();
            let mut curr = window[1].take();
            curr.par_iter_mut()
                .zip(prev.par_iter())
                .for_each(|(c, p)| *c += *p);
            window[1].set(curr);
            window[0].set(prev);
        });
        pref_sum_par_cpu(counters.last_mut().unwrap());
        let (front, end) = counters.split_at_mut(chunks - 1);
        let end = &mut end[0];
        front.par_iter_mut().for_each(|counter| {
            counter
                .par_iter_mut()
                .skip(1)
                .zip(end.par_iter())
                .for_each(|(c, e)| *c += *e);
        });

        let idxs = digits
            .par_chunks(chunk_size)
            .zip(counters.par_iter_mut())
            .flat_map(|(chunk, counter)| {
                let aux = chunk
                    .iter()
                    .rev()
                    .map(|&x| {
                        counter[x] -= 1;
                        counter[x]
                    })
                    .collect_vec();
                aux.into_iter().rev().collect_vec()
            })
            .collect::<Vec<_>>();
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
