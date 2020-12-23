use num_traits::{Float, PrimInt, ToPrimitive};

use std::fmt::Debug;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point<T: PartialOrd + PartialEq + Clone, const DIM: usize>(pub [T; DIM]);

impl<T: PartialOrd + PartialEq + Clone, const DIM: usize> std::ops::Index<usize> for Point<T, DIM> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T: PartialOrd + PartialEq + Clone, const DIM: usize> std::ops::IndexMut<usize>
    for Point<T, DIM>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub trait FloatPoint<F: Float, const DIM: usize> {
    fn squared_eucledian(&self, other: &Self) -> F;
    fn distance_to_space(&self, min_bounds: &[F; DIM], max_bounds: &[F; DIM]) -> F;
}

impl<F: Float, const DIM: usize> FloatPoint<F, DIM> for Point<F, DIM> {
    /// Returns the squared euclidean distance between two points.
    fn squared_eucledian(&self, other: &Self) -> F {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&x, &y)| (x - y) * (x - y))
            .fold(F::zero(), std::ops::Add::add)
    }
    fn distance_to_space(&self, min_bounds: &[F; DIM], max_bounds: &[F; DIM]) -> F {
        let mut other = [F::nan(); DIM];
        for i in 0..DIM {
            other[i] = if self[i] > max_bounds[i] {
                max_bounds[i]
            } else if self[i] < min_bounds[i] {
                min_bounds[i]
            } else {
                self[i]
            };
        }
        self.squared_eucledian(&Point(other))
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

impl<T: PartialOrd + PartialEq + Clone, const DIM: usize> From<[T; DIM]> for Point<T, DIM> {
    fn from(input: [T; DIM]) -> Self {
        Self(input)
    }
}
// TODO: implement from Vec<T>?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_distance_to_space() {
        let dis = Point([0.0, 0.0]).distance_to_space(&[1.0, 1.0], &[2.0, 2.0]);
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_outside_inf() {
        let dis = Point([0.0, 0.0]).distance_to_space(&[1.0, 1.0], &[f64::INFINITY, f64::INFINITY]);
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_inside_inf() {
        let dis = Point([2.0, 2.0]).distance_to_space(
            &[f64::NEG_INFINITY, f64::NEG_INFINITY],
            &[f64::INFINITY, f64::INFINITY],
        );
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn test_distance_inside_normal() {
        let dis = Point([2.0, 2.0]).distance_to_space(&[0.0, 0.0], &[3.0, 3.0]);
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn distance_to_half_space() {
        let dis = Point([-2.0, 0.0])
            .distance_to_space(&[0.0, f64::NEG_INFINITY], &[f64::INFINITY, f64::INFINITY]);
        assert_eq!(dis, 4.0);
    }
}
