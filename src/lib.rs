#![no_std]

use core::iter::Take;

pub fn try_take<T: ExactSizeIterator>(iter: &mut T, length: usize) -> Result<Take<&mut T>, usize> {
    let iter = iter.take(length);
    let actual_length = iter.len();
    if actual_length == length {
        Ok(iter)
    } else {
        Err(actual_length)
    }
}

#[cfg(test)]
mod tests {
    use crate::try_take;

    #[test]
    fn test_success() {
        let _ = try_take(&mut (0..5), 5).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_failure() {
        match try_take(&mut (0..5), 6) {
            Err(4) => {},
            _ => panic!()
        }
    }
}
