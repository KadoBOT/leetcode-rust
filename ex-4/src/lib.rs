#[cfg(test)]
mod tests {
    use crate::find_median_sorted_arrays;

    #[test]
    fn example_1() {
        assert_eq!(2.0, find_median_sorted_arrays(vec![1, 3], vec![2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2.5, find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    }

    #[test]
    fn has_empty() {
        assert_eq!(1.0, find_median_sorted_arrays(vec![], vec![1]));
    }

    #[test]
    fn with_negative() {
        assert_eq!(-1.0, find_median_sorted_arrays(vec![3], vec![-2, -1]));
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut sorted_merge = [nums1, nums2].concat();
    sorted_merge.sort();
    let mid = (sorted_merge.len() as f64 / 2.0).floor();

    if sorted_merge.len() % 2 == 0 {
        return (sorted_merge[mid as usize - 1] + sorted_merge[mid as usize]) as f64 / 2.0;
    }

    *sorted_merge.get(mid as usize).unwrap() as f64
}
