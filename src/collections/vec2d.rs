use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Clone)]
pub struct Vec2D<T> {
    data: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl <T: Default + Clone + fmt::Display> Vec2D<T> {
    pub fn new(n_rows: usize, n_cols: usize) -> Vec2D<T> {
        let data = vec![T::default(); n_rows * n_cols];
        Vec2D { data, n_rows, n_cols }
    }
}

impl <T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        assert!(index.0 < self.n_rows);
        assert!(index.1 < self.n_cols);

        &self.data[index.0 * self.n_cols + index.1]
    }
}

impl <T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        assert!(index.0 < self.n_rows);
        assert!(index.1 < self.n_cols);

        &mut self.data[index.0 * self.n_cols + index.1]
    }
}

impl fmt::Display for Vec2D<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                match write!(f, "{} ", self[(i, j)] as usize) {
                    Err(e) => return Err(e),
                    _ => {}
                }
            }
            match write!(f, "\n") {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let mut test = Vec2D::<usize>::new(3, 3);

        test[(0, 0)] = 1;
        test[(0, 1)] = 2;
        test[(0, 2)] = 3;
        test[(1, 0)] = 4;
        test[(1, 1)] = 5;
        test[(1, 2)] = 6;
        test[(2, 0)] = 7;
        test[(2, 1)] = 8;
        test[(2, 2)] = 9;

        assert!(test.data == vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}