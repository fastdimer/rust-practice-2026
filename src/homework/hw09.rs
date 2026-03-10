#[allow(dead_code)]
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut min_id = 0;

    for id in 1..=5 {
        if counts[id] > max_count {
            max_count = counts[id];
            min_id = id as i32;
        }
    }

    min_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_basic() {
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3]), 1);
    }

    #[test]
    fn test_migratory_birds_highest_count() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }

    #[test]
    fn test_migratory_birds_tie() {
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }
}