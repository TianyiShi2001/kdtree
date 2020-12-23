#![feature(min_const_generics)]

use num_traits::{Float, PrimInt, ToPrimitive};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point<T: PartialOrd + PartialEq, const DIM: usize>([T; DIM]);

impl<T: PartialOrd + PartialEq, const DIM: usize> std::ops::Index<usize> for Point<T, DIM> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T: PartialOrd + PartialEq, const DIM: usize> std::ops::IndexMut<usize> for Point<T, DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub trait FloatPoint<F> {
    fn squared_eucledian(&self, other: Self) -> F;
}

impl<F: Float, const DIM: usize> FloatPoint<F> for Point<F, DIM> {
    /// Returns the squared euclidean distance between two points.
    fn squared_eucledian(&self, other: Self) -> F {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&x, &y)| (x - y) * (x - y))
            .fold(F::zero(), ::std::ops::Add::add)
    }
}

pub trait IntPoint {
    fn squared_eucledian(&self, other: Self) -> f64;
}

impl<I: PrimInt + ToPrimitive, const DIM: usize> IntPoint for Point<I, DIM> {
    fn squared_eucledian(&self, other: Self) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&x, &y)| (x - y).to_f64().unwrap() * (x - y).to_f64().unwrap())
            .fold(0., ::std::ops::Add::add)
    }
}

impl<T: PartialOrd + PartialEq, const DIM: usize> From<[T; DIM]> for Point<T, DIM> {
    fn from(input: [T; DIM]) -> Self {
        Self(input)
    }
}
// TODO: implement from Vec<T>?

struct Node<T: PartialOrd + PartialEq, const DIM: usize> {
    pivot: Point<T, DIM>,
    left: Option<Box<Node<T, DIM>>>,
    right: Option<Box<Node<T, DIM>>>,
}

impl<T: PartialOrd + PartialEq, const DIM: usize> Node<T, DIM> {
    fn new<P: Into<Point<T, DIM>>>(point: P) -> Self {
        Self {
            pivot: point.into(),
            left: None,
            right: None,
        }
    }
}

#[derive(Default)]
pub struct KdTree<T: PartialOrd + PartialEq, const DIM: usize> {
    root: Option<Box<Node<T, DIM>>>,
}

impl<T: PartialOrd + PartialEq, const DIM: usize> KdTree<T, DIM> {
    pub fn dim() -> usize {
        DIM
    }
}
impl<T: PartialOrd + PartialEq, const DIM: usize> KdTree<T, DIM> {
    pub fn contains(&self, point: &Point<T, DIM>) -> bool {
        let mut depth = 0;
        let mut next = self.root.as_ref();
        while let Some(curr) = next {
            let curr_point = &curr.pivot;
            if curr_point == point {
                return true;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                curr.left.as_ref()
            } else {
                curr.right.as_ref()
            };
            depth += 1;
        }
        false
    }
    pub fn insert(&mut self, point: Point<T, DIM>) -> bool {
        let mut depth = 0;
        let mut next = &mut self.root;
        while let Some(curr) = next {
            let curr_point = &curr.pivot;
            if *curr_point == point {
                return false;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                &mut curr.left
            } else {
                &mut curr.right
            };
            depth += 1;
        }
        *next = Some(Box::new(Node::new(point)));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::KdTree;

    #[test]
    fn kdtree() {
        // let mut kdt = KdTree::<i32, 3>::default();
        // kdt.insert([1, 2, 3]);
        // kdt.insert([5, 1, -6]);
        // kdt.insert([5, 1, -4]);
        // kdt.insert([3, 6, 8]);
        // kdt.insert([2, 4, 6]);
        // kdt.insert([8, 0, 1]);
        // assert!(kdt.contains(&[1, 2, 3]));
        // assert!(kdt.contains(&[5, 1, -6]));
        // assert!(kdt.contains(&[5, 1, -4]));
        // assert!(kdt.contains(&[3, 6, 8]));
        // assert!(kdt.contains(&[2, 4, 6]));
        // assert!(kdt.contains(&[8, 0, 1]));
        // assert!(!kdt.contains(&[1, 2, 4]));
        // assert!(!kdt.contains(&[0, 3, 1]));
    }
}
