use num::PrimInt;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::ops::Range;

fn binary_search<T, F, Q>(eval: F, range: Range<T>, query: &Q) -> Option<T>
where
    T: PrimInt,
    F: Fn(&T, &Q) -> Ordering,
{
    let mut low = range.start;
    let mut high = range.end;
    let mut result: Option<T> = None;
    while low <= high {
        let mid = low + (high - low) / T::from(2).unwrap();
        match eval(&mid, query) {
            Ordering::Less => low = mid + T::one(),
            Ordering::Greater | Ordering::Equal => {
                high = mid - T::one();
                result = Some(mid);
            }
        }
    }
    result
}

pub fn binary_search_serial<T, F, Q>(eval: F, range: Range<T>, queries: &[Q]) -> Vec<Option<T>>
where
    T: PrimInt,
    F: Copy + Fn(&T, &Q) -> Ordering,
{
    queries
        .iter()
        .map(|query| binary_search(eval, range.clone(), query))
        .collect()
}

pub fn binary_search_par_cpu<T, F, Q>(eval: F, range: Range<T>, queries: &[Q]) -> Vec<Option<T>>
where
    T: PrimInt + Send + Sync,
    F: Copy + Fn(&T, &Q) -> Ordering + Send + Sync,
    Q: Sync,
{
    queries
        .par_iter()
        .map(|query| binary_search(eval, range.clone(), query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use num::BigUint;
    use std::str::FromStr;

    #[test]
    fn test_serial_search_find_equal() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let queries = [3, 4, 7];
        let results =
            binary_search_serial(|mid, query| data[*mid].cmp(query), 0..data.len(), &queries);
        assert_eq!(results, vec![Some(2), Some(3), Some(6)]);
    }

    #[test]
    fn test_serial_search_partial_greater_equal() {
        let data = [1, 3, 5, 7, 9];
        let queries = [2, 3, 6, 7];
        let results =
            binary_search_serial(|mid, query| data[*mid].cmp(query), 0..data.len(), &queries);
        assert_eq!(results, vec![Some(1), Some(1), Some(3), Some(3)]);
    }

    #[test]
    fn test_serial_search_factorial() {
        let factorial = |n: usize| (1..=n).product::<BigUint>();
        let queries= ["2", "6", "71569457046263802294811533723186532165584657342365752577109445058227039255480148842668944867280814080000000000000000000", "71569457046263802294811533723186532165584657342365752577109445058227039255480148842668944867280814080000000000000000001"]
            .iter()
            .map(|s| BigUint::from_str(s).unwrap())
            .collect_vec();
        let results = binary_search_serial(
            |mid, query| factorial(*mid).cmp(query),
            0..80usize,
            &queries,
        );
        assert_eq!(results, vec![Some(2), Some(3), Some(80), None]);
    }

    #[test]
    fn test_par_search_find_equal() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let queries = [3, 4, 7];
        let results =
            binary_search_par_cpu(|mid, query| data[*mid].cmp(query), 0..data.len(), &queries);
        assert_eq!(results, vec![Some(2), Some(3), Some(6)]);
    }

    #[test]
    fn test_par_search_partial_greater_equal() {
        let data = [1, 3, 5, 7, 9];
        let queries = [2, 3, 6, 7];
        let results =
            binary_search_par_cpu(|mid, query| data[*mid].cmp(query), 0..data.len(), &queries);
        assert_eq!(results, vec![Some(1), Some(1), Some(3), Some(3)]);
    }

    #[test]
    fn test_par_search_factorial() {
        let factorial = |n: usize| (1..=n).product::<BigUint>();
        let queries= ["2", "6", "71569457046263802294811533723186532165584657342365752577109445058227039255480148842668944867280814080000000000000000000", "71569457046263802294811533723186532165584657342365752577109445058227039255480148842668944867280814080000000000000000001"]
            .iter()
            .map(|s| BigUint::from_str(s).unwrap())
            .collect_vec();
        let results = binary_search_par_cpu(
            |mid, query| factorial(*mid).cmp(query),
            0..80usize,
            &queries,
        );
        assert_eq!(results, vec![Some(2), Some(3), Some(80), None]);
    }
}
