use std::collections::HashMap;

#[allow(dead_code)]
pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    
    for &color in ar {
        *counts.entry(color).or_insert(0) += 1;
    }
    
    counts.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let ar: [i32; 9] = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_all_pairs() {
        let ar: [i32; 6] = [1, 1, 2, 2, 3, 3];
        assert_eq!(sock_merchant(6, &ar), 3);
    }

    #[test]
    fn test_no_pairs() {
        let ar: [i32; 4] = [1, 2, 3, 4];
        assert_eq!(sock_merchant(4, &ar), 0);
    }

    #[test]
    fn test_empty_array() {
        let ar: [i32; 0] = [];
        assert_eq!(sock_merchant(0, &ar), 0);
    }
}