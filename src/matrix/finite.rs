use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};
use std::iter::{Sum, zip};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Shape {
    pub height: usize,
    pub width: usize,
}

impl Shape {
    pub fn empty() -> Shape { Shape { height: 0, width: 0 } }
    pub fn is_empty(&self) -> bool { *self == Shape::empty() }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct FiniteMatrix<T, > {
    storage: Vec<T>,
    shape: Shape,
}

impl<T> FiniteMatrix<T> {
    pub fn height(&self) -> usize { self.shape.height }
    pub fn width(&self) -> usize { self.shape.width }
    pub fn shape(&self) -> Shape { self.shape }

    pub fn from_generator(shape: Shape, mut gen: impl FnMut() -> T) -> FiniteMatrix<T> {
        let length = shape.height * shape.width;
        let mut storage = Vec::with_capacity(length);
        for _ in 0..length {
            storage.push(gen())
        }
        let shape = if shape.height == 0 || shape.width == 0 { Shape::empty() } else { shape };
        FiniteMatrix { storage, shape }
    }

    pub fn from_indexed_generator(shape: Shape, mut gen: impl FnMut(usize, usize) -> T) -> FiniteMatrix<T> {
        let iter = (0..shape.height)
            .flat_map(|row| (0..shape.width).map(move |column| (row, column)))
            .map(|(row, column)| gen(row, column));
        Self::from_iter(shape, iter)
    }

    pub fn from_iter(shape: Shape, mut iter: impl Iterator<Item=T>) -> FiniteMatrix<T> {
        Self::from_generator(shape, || iter.next().expect("Not enough elements to make a matrix"))
    }

    fn flat_index(&self, index: (usize, usize)) -> usize {
        assert!(index.0 < self.height());
        let width = self.width();
        assert!(index.1 < width);
        let flat_index = width * index.0 + index.1;
        flat_index
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> { self.storage.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> { self.storage.iter_mut() }

    fn indexes_impl(height: usize, width: usize) -> impl Iterator<Item=(usize, usize)> {
        (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)))
    }

    pub fn indexes(&self) -> impl Iterator<Item=(usize, usize)> {
        Self::indexes_impl(self.height(), self.width())
    }

    pub fn iter_with_indexes(&self) -> impl Iterator<Item=((usize, usize), &T)> {
        zip(self.indexes(), self.iter())
    }

    pub fn iter_mut_with_indexes(&mut self) -> impl Iterator<Item=((usize, usize), &mut T)> {
        zip(self.indexes(), self.iter_mut())
    }

    pub fn into_iter_with_indexes(self) -> impl Iterator<Item=((usize, usize), T)> {
        zip(self.indexes(), self.into_iter())
    }

    pub fn map<R>(self, f: impl Fn(T) -> R) -> FiniteMatrix<R> {
        FiniteMatrix::<R>::from_iter(self.shape, self.into_iter().map(f))
    }

    pub fn map_with_indexes<R>(self, f: impl Fn((usize, usize), T) -> R) -> FiniteMatrix<R> {
        FiniteMatrix::<R>::from_iter(
            self.shape, self.into_iter_with_indexes().map(|(x, y)| f(x, y)),
        )
    }
}


impl<T> Index<usize> for FiniteMatrix<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.storage.index(index)
    }
}

impl<T> IndexMut<usize> for FiniteMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.storage.index_mut(index)
    }
}

impl<T> Index<(usize, usize)> for FiniteMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.storage.index(self.flat_index(index))
    }
}

impl<T> IndexMut<(usize, usize)> for FiniteMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.storage.index_mut(self.flat_index(index))
    }
}


impl<T: Clone> FiniteMatrix<T> {
    pub fn with_element(shape: Shape, element: T) -> FiniteMatrix<T> {
        Self::from_generator(shape, || element.clone())
    }
}

impl<T: Default> FiniteMatrix<T> {
    pub fn with_default(shape: Shape) -> FiniteMatrix<T> {
        Self::from_generator(shape, T::default)
    }
}


impl<T: Display> Display for FiniteMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut column = 0;
        for element in self.iter() {
            write!(f, "{}", element)?;
            column += 1;
            if column == self.width() {
                write!(f, "\n")?;
                column = 0;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}


impl<T> IntoIterator for FiniteMatrix<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}


impl<T: Add<T, Output=T>> Add for FiniteMatrix<T> {
    type Output = FiniteMatrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let shape = self.shape;
        assert_eq!(shape, rhs.shape);
        let element_wise_sum = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a + b);
        FiniteMatrix::<T>::from_iter(shape, element_wise_sum)
    }
}

impl<T: Sub<T, Output=T>> Sub for FiniteMatrix<T> {
    type Output = FiniteMatrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let shape = self.shape;
        assert_eq!(shape, rhs.shape);
        let element_wise_sum = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a - b);
        FiniteMatrix::<T>::from_iter(shape, element_wise_sum)
    }
}

impl<T: Neg<Output=T>> Neg for FiniteMatrix<T> {
    type Output = FiniteMatrix<T>;

    fn neg(self) -> Self::Output {
        self.map(T::neg)
    }
}

impl<T: Mul<Output=T> + Sum + Clone> Mul for FiniteMatrix<T> {
    type Output = FiniteMatrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width(), rhs.height());
        let k_max = self.width();
        let shape = Shape { height: self.height(), width: rhs.width() };
        let result = |x, y| (0..k_max).map(|k| self[(x, k)].clone() * rhs[(k, y)].clone()).sum();
        FiniteMatrix::<T>::from_indexed_generator(shape, result)
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::finite::{FiniteMatrix, Shape};

    #[test]
    fn print() {
        let shape = Shape { height: 2, width: 3 };
        assert_eq!(
            format!("{}", FiniteMatrix::<i32>::with_default(shape)),
            "0 0 0\n0 0 0\n"
        );
        assert_eq!(
            format!("{}", FiniteMatrix::<i32>::with_element(shape, 4)),
            "4 4 4\n4 4 4\n"
        );
        let generator = |x, y| format!("({x}, {y})");
        assert_eq!(
            format!("{}", FiniteMatrix::<String>::from_indexed_generator(shape, generator)),
            "(0, 0) (0, 1) (0, 2)\n(1, 0) (1, 1) (1, 2)\n"
        );
        assert_eq!(
            format!("{}", FiniteMatrix::<i32>::from_iter(shape, 0..6)),
            "0 1 2\n3 4 5\n"
        );
    }
}
