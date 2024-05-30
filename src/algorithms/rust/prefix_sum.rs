use crate::bridge::ffi;
use num::Num;
use rayon::prelude::*;

pub fn pref_sum_serial<T>(arr: &mut [T])
where
    T: Num + Clone,
{
    for i in 1..arr.len() {
        arr[i] = arr[i].clone() + arr[i - 1].clone();
    }
}

pub fn pref_sum_par_cpu<T>(arr: &mut [T])
where
    T: Num + Clone + Send + Sync,
{
    let threads = rayon::current_num_threads();
    let chunks = threads;
    let chunk_size = arr.len().div_ceil(chunks);
    arr.par_chunks_mut(chunk_size).for_each(|chunk| {
        pref_sum_serial(chunk);
    });
    let mut offsets: Vec<T> = arr
        .par_chunks(chunk_size)
        .take(chunks - 1)
        .map(|chunk| chunk.last().unwrap())
        .cloned()
        .collect();
    pref_sum_serial(&mut offsets);
    arr.par_chunks_mut(chunk_size)
        .skip(1)
        .zip(offsets.par_iter())
        .for_each(|(chunk, offset)| {
            for i in 0..chunk.len() {
                chunk[i] = chunk[i].clone() + offset.clone();
            }
        });
}

pub fn pref_sum_par_gpu(arr: &mut [f64]) {
    unsafe {
        ffi::prefix_sum(arr.len(), arr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const ARR: [u64; 60] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    ];

    fn expected() -> Vec<u64> {
        ARR.iter()
            .scan(0, |s, &e| {
                *s += e;
                Some(*s)
            })
            .collect()
    }

    #[test]
    fn test_pref_sum_serial() {
        let mut arr = ARR.into_iter().collect_vec();
        pref_sum_serial(&mut arr);
        assert_eq!(arr, expected());
    }

    #[test]
    fn test_pref_sum_par_cpu() {
        let mut arr = ARR.into_iter().collect_vec();
        pref_sum_par_cpu(&mut arr);
        assert_eq!(arr, expected());
    }

    #[test]
    fn test_pref_sum_par_gpu() {
        let mut arr = ARR.into_iter().map(|x| x as f64).collect_vec();
        pref_sum_par_gpu(&mut arr);
        assert_eq!(arr, expected().into_iter().map(|x| x as f64).collect_vec());
    }

    #[test]
    fn test_pref_sum_par_gpu_pot2() {
        let mut arr = [1., 2.];
        pref_sum_par_gpu(&mut arr);
        assert_eq!(arr, [1., 3.]);
    }
}
