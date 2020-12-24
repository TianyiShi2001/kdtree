//! A k-dimensional tree data structure

#![feature(min_const_generics)]
#![feature(binary_heap_into_iter_sorted)]

pub mod point;
pub use point::*;

use num_traits::Float;
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: PartialOrd + PartialEq + Clone, const DIM: usize> {
    pivot: Point<T, DIM>,
    left: Option<Box<Node<T, DIM>>>,
    right: Option<Box<Node<T, DIM>>>,
}

impl<T: PartialOrd + PartialEq + Clone, const DIM: usize> Node<T, DIM> {
    fn new<P: Into<Point<T, DIM>>>(point: P) -> Self {
        Self {
            pivot: point.into(),
            left: None,
            right: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct KdTree<T: PartialOrd + PartialEq + Clone, const DIM: usize> {
    root: Option<Box<Node<T, DIM>>>,
}

impl<T: PartialOrd + PartialEq + Clone + Debug, const DIM: usize> KdTree<T, DIM> {
    pub fn dim() -> usize {
        DIM
    }
    pub fn from_slice(points: &mut [Point<T, DIM>]) -> Self {
        fn build_node<T: PartialOrd + PartialEq + Clone, const DIM: usize>(
            points: &mut [Point<T, DIM>],
            depth: usize,
        ) -> Option<Box<Node<T, DIM>>> {
            let d = depth % DIM;
            points.sort_unstable_by(|a, b| a[d].partial_cmp(&b[d]).unwrap());
            let mut mid = points.len() / 2;
            let val = &points[mid][d];
            // ensure that points to the right of the pivot are strictly greater
            for i in mid + 1..points.len() {
                if points[i][d] != *val {
                    break;
                } else {
                    mid = i;
                }
            }
            let pivot = points[mid].clone();
            let (l, r) = points.split_at_mut(mid);

            Some(Box::new(Node {
                pivot,
                left: if l.is_empty() {
                    None
                } else {
                    build_node(l, depth + 1)
                },
                right: if r.len() == 1 {
                    None
                } else {
                    build_node(&mut r[1..], depth + 1)
                },
            }))
        }
        let root = build_node(points, 0);
        Self { root }
    }

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
    /// Insert a point into the tree.
    ///
    /// Inserting elements one by one is likely to cause the tree to become inbalanced.
    /// Prefer `from_slice` to construct the tree.
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

impl<T: PartialOrd + PartialEq + Clone + Clone + Float + Debug, const DIM: usize> KdTree<T, DIM> {
    pub fn k_nearest_neighbors(&self, query: &Point<T, DIM>, k: usize) -> Vec<(T, &Point<T, DIM>)> {
        let mut res_pq: BinaryHeap<(OrderedFloat<T>, *const Point<T, DIM>)> =
            BinaryHeap::with_capacity(k);
        fn knn<T: PartialOrd + PartialEq + Clone + Clone + Float + Debug, const DIM: usize>(
            node: Option<&Box<Node<T, DIM>>>,
            depth: usize,
            query: &Point<T, DIM>,
            min_bounds: &mut [T; DIM],
            max_bounds: &mut [T; DIM],
            result_pq: &mut BinaryHeap<(OrderedFloat<T>, *const Point<T, DIM>)>,
            k: usize,
        ) {
            if let Some(curr) = node {
                let d = depth % DIM;
                let val = &curr.pivot[d];
                let dist = curr.pivot.squared_eucledian(query);
                if result_pq.len() < k {
                    result_pq.push((OrderedFloat(dist), &curr.pivot as *const Point<T, DIM>));
                } else {
                    // Get the longest distance.
                    let mx = result_pq
                        .peek()
                        .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

                    if dist < mx {
                        result_pq.pop().unwrap();
                        result_pq.push((OrderedFloat(dist), &curr.pivot as *const Point<T, DIM>));
                    }
                }

                if &query[d] <= val {
                    let tmp = max_bounds[d];
                    max_bounds[d] = *val;
                    knn(
                        curr.left.as_ref(),
                        depth + 1,
                        query,
                        min_bounds,
                        max_bounds,
                        result_pq,
                        k,
                    );
                    max_bounds[d] = tmp;

                    // Get the longest distance.
                    let mx = result_pq
                        .peek()
                        .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

                    let tmp = min_bounds[d];
                    min_bounds[d] = *val;
                    if query.distance_to_space(min_bounds, max_bounds) <= mx {
                        knn(
                            curr.right.as_ref(),
                            depth + 1,
                            query,
                            min_bounds,
                            max_bounds,
                            result_pq,
                            k,
                        );
                    }
                    min_bounds[d] = tmp;
                } else {
                    let tmp = min_bounds[d];
                    min_bounds[d] = *val;
                    knn(
                        curr.right.as_ref(),
                        depth + 1,
                        query,
                        min_bounds,
                        max_bounds,
                        result_pq,
                        k,
                    );
                    min_bounds[d] = tmp;

                    let mx = result_pq
                        .peek()
                        .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

                    let tmp = max_bounds[d];
                    max_bounds[d] = *val;
                    if query.distance_to_space(min_bounds, max_bounds) <= mx {
                        knn(
                            curr.left.as_ref(),
                            depth + 1,
                            query,
                            min_bounds,
                            max_bounds,
                            result_pq,
                            k,
                        );
                    }
                    max_bounds[d] = tmp;
                }
            }
        }
        knn(
            self.root.as_ref(),
            0,
            query,
            &mut [T::neg_infinity(); DIM],
            &mut [T::infinity(); DIM],
            &mut res_pq,
            k,
        );

        res_pq
            .into_iter_sorted()
            .map(|(dist, point)| unsafe { (dist.into_inner(), point.as_ref().unwrap()) })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::{thread_rng, Rng};

    #[test]
    fn kdtree() {
        let mut points = {
            let mut rng = thread_rng();
            (0..10000)
                .map(|_| {
                    Point([
                        rng.gen_range(-50.0..50.0),
                        rng.gen_range(-50.0..50.0),
                        rng.gen_range(-50.0..50.0),
                    ])
                })
                .collect::<Vec<_>>()
        };
        let kdt = KdTree::from_slice(&mut points);
        let query = Point([0.0, 0.0, 0.0]);
        let nearest = kdt
            .k_nearest_neighbors(&query, 10)
            .into_iter()
            .map(|(dist, point)| (dist, point.clone()))
            .rev()
            .collect::<Vec<_>>();
        let mut expected = points
            .into_iter()
            .map(|p| (p.squared_eucledian(&query), p))
            .collect::<Vec<_>>();
        expected.sort_unstable_by_key(|p| OrderedFloat(p.0));
        assert_eq!(&nearest[..], &expected[..10]);
    }
}
