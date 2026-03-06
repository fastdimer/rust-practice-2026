#[allow(dead_code)]
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }

    fn lcm(x: i32, y: i32) -> i32 {
        if x == 0 || y == 0 {
            return 0;
        }
        (x / gcd(x, y)) * y
    }

    let l = a.iter().cloned().reduce(lcm).unwrap_or(1);
    let g = b.iter().cloned().reduce(gcd).unwrap_or(0);

    let mut count = 0;
    let mut multiple = l;
    
    if l > 0 && g > 0 {
        while multiple <= g {
            if g % multiple == 0 {
                count += 1;
            }
            multiple += l;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x_basic() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_get_total_x_example() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2);
    }
    
    #[test]
    fn test_get_total_x_single_elements() {
        let a = vec![3];
        let b = vec![9];
        assert_eq!(get_total_x(&a, &b), 2);
    }
}