use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 16], 9));
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = HashMap::new();
    for (idx, n) in nums.iter().enumerate() {
        if let Some(&x) = result.get(n) {
            return vec![x as i32, idx as i32];
        }
        result.insert(target - n, idx);
    }
    vec![]
}
