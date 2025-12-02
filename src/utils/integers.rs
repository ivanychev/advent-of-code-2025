use num_traits::{PrimInt, ToPrimitive};

pub struct Divisors<T: PrimInt + ToPrimitive> {
    value: T,
    current: T,
    sqrt_value: T,
    should_return_next: T,
}

impl<T: PrimInt + ToPrimitive> Iterator for Divisors<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_return_next != T::zero() {
            let result = Some(self.should_return_next);
            self.should_return_next = T::zero();
            return result;
        }
        while self.current <= self.sqrt_value {
            let divisor = self.current;
            self.current = self.current + T::one();
            if self.value % divisor == T::zero() {
                let other = self.value / divisor;
                if other != divisor {
                    self.should_return_next = other;
                }
                return Some(divisor);
            }
        }
        None
    }
}

pub fn divisors<T: PrimInt + ToPrimitive>(n: T) -> Divisors<T> {
    Divisors {
        value: n,
        current: T::one(),
        sqrt_value: n.to_f64().map(|f| T::from(f.sqrt()).unwrap()).unwrap(),
        should_return_next: T::zero(),
    }
}

pub fn count_digits(mut n: i64) -> u32 {
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

#[test]
fn test_count_digits() {
    assert_eq!(count_digits(0), 0);
    assert_eq!(count_digits(5), 1);
    assert_eq!(count_digits(42), 2);
    assert_eq!(count_digits(123456), 6);
    assert_eq!(count_digits(1000000000), 10);
}

#[test]
fn test_divisors() {
    let mut divs: Vec<i64> = divisors(28).collect();
    divs.sort();
    assert_eq!(divs, vec![1, 2, 4, 7, 14, 28]);

    divs = divisors(36).collect();
    divs.sort();
    assert_eq!(divs, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);

    divs = divisors(13).collect();
    divs.sort();
    assert_eq!(divs, vec![1, 13]);

    divs = divisors(1).collect();
    divs.sort();
    assert_eq!(divs, vec![1]);
}
