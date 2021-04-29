use std::cmp::{Ord, Ordering};

pub trait LowBound<'a, T: Ord> {
    fn size(&self) -> usize;
    fn get(&self, pos: usize) -> &T;

    #[inline]
    fn partition(&self, start: usize, half: usize, value: &'a T) -> usize {
        let mid = start + half;
        match self.get(mid).cmp(value) {
            Ordering::Greater => start,
            _ => mid,
        }
    }
}

pub fn lower_bound<'a, T: LowBound<'a, V>, V: Ord>(
    container: &'a T,
    value: &'a V,
) -> Result<usize, usize> {
    let mut size = container.size();
    if size == 0 {
        return Err(0);
    }

    let mut pos = 0usize;
    while size > 1 {
        let half = size / 2;
        pos = container.partition(pos, half, value);
        size -= half;
    }

    match container.get(pos).cmp(value) {
        Ordering::Equal => Ok(pos),
        v => Err(pos + (v == Ordering::Less) as usize),
    }
}

impl<'a, T: Ord> LowBound<'a, T> for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, pos: usize) -> &T {
        &self[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        assert_eq!(Ok(6), lower_bound(&arr, &13));
        assert_eq!(Err(9), lower_bound(&arr, &49));
        assert_eq!(Err(0), lower_bound(&arr, &0));
        assert_eq!(Err(10), lower_bound(&arr, &100));
        let arr = vec!['a', 'b', 'c', 'd', 'e', 'm'];
        assert_eq!(Ok(5), lower_bound(&arr, &'m'));
    }
}
