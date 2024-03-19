// Uncomment this file!

/// Returns the sum of given integer vector
///
/// # Arguments
///
/// * `v` - A vector containing integers
///
pub fn sumvec(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::b_sumvec::sumvec;

    #[test]
    fn test_sumvec() {
        let result = sumvec(&vec![2, 3, 4]);
        assert_eq!(9, result);
    }
}
