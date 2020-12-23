// use crate::point::*;

// use num_traits::Float;
// use ordered_float::OrderedFloat;
// use std::collections::BinaryHeap;
// use std::fmt::Debug;

// #[derive(Debug)]
// struct Node<T: PartialOrd + PartialEq + Clone, const DIM: usize> {
//     pivot: Point<T, DIM>,
//     left: Option<Box<Node<T, DIM>>>,
//     right: Option<Box<Node<T, DIM>>>,
//     min_bounds: [T; DIM],
//     max_bounds: [T; DIM],
// }

// impl<T: PartialOrd + PartialEq + Clone, const DIM: usize> Node<T, DIM> {
//     fn new<P: Into<Point<T, DIM>>>(point: P, min_bounds: [T; DIM], max_bounds: [T; DIM]) -> Self {
//         Self {
//             pivot: point.into(),
//             left: None,
//             right: None,
//             min_bounds,
//             max_bounds,
//         }
//     }
// }

// impl<T: PartialOrd + PartialEq + Clone + Float, const DIM: usize> Node<T, DIM> {
//     fn distance_to_point(&self, point: &Point<T, DIM>) -> T {
//         point.distance_to_space(&self.min_bounds, &self.max_bounds)
//     }
// }

// #[derive(Default, Debug)]
// pub struct KdTree<T: PartialOrd + PartialEq + Clone, const DIM: usize> {
//     root: Option<Box<Node<T, DIM>>>,
// }

// impl<T: PartialOrd + PartialEq + Clone + Clone + Float, const DIM: usize> KdTree<T, DIM> {
//     pub fn dim() -> usize {
//         DIM
//     }
//     pub fn from_slice(points: &mut [Point<T, DIM>]) -> Self {
//         fn build_node<T: PartialOrd + PartialEq + Clone, const DIM: usize>(
//             points: &mut [Point<T, DIM>],
//             depth: usize,
//             mut min_bounds: [T; DIM],
//             mut max_bounds: [T; DIM],
//         ) -> Option<Box<Node<T, DIM>>> {
//             let d = depth / DIM;
//             points.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
//             let mut mid = points.len() / 2;
//             let val = points[mid][d].clone();
//             // ensure that points to the right of the pivot are strictly greater
//             for i in mid + 1..points.len() {
//                 if points[i][d] != val {
//                     break;
//                 } else {
//                     mid = i;
//                 }
//             }
//             let pivot = points[mid].clone();
//             let (l, r) = points.split_at_mut(mid);

//             Some(Box::new(Node {
//                 pivot,
//                 max_bounds: max_bounds.clone(),
//                 min_bounds: min_bounds.clone(),
//                 left: if l.is_empty() {
//                     None
//                 } else {
//                     max_bounds[d] = val.clone();
//                     build_node(l, depth + 1, min_bounds.clone(), max_bounds.clone())
//                 },
//                 right: if r.len() == 1 {
//                     None
//                 } else {
//                     min_bounds[d] = val;
//                     build_node(&mut r[1..], depth + 1, min_bounds, max_bounds)
//                 },
//             }))
//         }
//         let root = build_node(points, 0, [T::neg_infinity(); DIM], [T::infinity(); DIM]);
//         Self { root }
//     }

//     pub fn contains(&self, point: &Point<T, DIM>) -> bool {
//         let mut depth = 0;
//         let mut next = self.root.as_ref();
//         while let Some(curr) = next {
//             let curr_point = &curr.pivot;
//             if curr_point == point {
//                 return true;
//             }
//             let d = depth % DIM;
//             next = if point[d] <= curr_point[d] {
//                 curr.left.as_ref()
//             } else {
//                 curr.right.as_ref()
//             };
//             depth += 1;
//         }
//         false
//     }
// }

// impl<T: PartialOrd + PartialEq + Clone + Clone + Float + Debug, const DIM: usize> KdTree<T, DIM> {
//     //     pub fn knearestneighbors_iterative(
//     //         &self,
//     //         query: &Point<T, DIM>,
//     //         k: usize,
//     //     ) -> Vec<(T, &Point<T, DIM>)> {
//     //         let mut res_pq: BinaryHeap<(OrderedFloat<T>, *const Point<T, DIM>)> =
//     //             BinaryHeap::with_capacity(k);
//     //         let mut search_pq: BinaryHeap<(OrderedFloat<T>, *const Option<Box<Node<T, DIM>>>)> =
//     //             BinaryHeap::new();
//     //         search_pq.push((
//     //             OrderedFloat(T::zero()),
//     //             &self.root as *const Option<Box<Node<T, DIM>>>,
//     //         ));
//     //         while let Some((dist, node)) = search_pq.pop() {
//     //             let node = unsafe { &*node };
//     // let p =
//     //             let mx = res_pq.pop().map_or(T::infinity(), |x|x.0.into_inner());
//     //             if res_pq.len()<k  {

//     //             }
//     //         }
//     //         0
//     //     }
//     pub fn _knearestneighbors(&self, query: &Point<T, DIM>, k: usize) -> Vec<(T, &Point<T, DIM>)> {
//         let mut res_pq: BinaryHeap<(OrderedFloat<T>, *const Point<T, DIM>)> =
//             BinaryHeap::with_capacity(k);
//         fn knn<T: PartialOrd + PartialEq + Clone + Clone + Float + Debug, const DIM: usize>(
//             curr: &Box<Node<T, DIM>>,
//             depth: usize,
//             query: &Point<T, DIM>,
//             result_pq: &mut BinaryHeap<(OrderedFloat<T>, *const Point<T, DIM>)>,
//             k: usize,
//         ) {
//             let d = depth % DIM;
//             let val = &curr.pivot[d];
//             let dist = curr.pivot.squared_eucledian(query);
//             if result_pq.len() < k {
//                 result_pq.push((OrderedFloat(dist), &curr.pivot as *const Point<T, DIM>));
//             } else {
//                 // Get the longest distance.
//                 let mx = result_pq
//                     .peek()
//                     .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

//                 if dist < mx {
//                     println!("{:?}", dist);
//                     result_pq.pop().unwrap();
//                     result_pq.push((OrderedFloat(dist), &curr.pivot as *const Point<T, DIM>));
//                 }
//             }
//             if &query[d] <= val {
//                 if let Some(left) = curr.left.as_ref() {
//                     knn(left, depth + 1, query, result_pq, k);

//                     // Get the longest distance.
//                     let mx = result_pq
//                         .peek()
//                         .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

//                     if left.distance_to_point(query) < mx {
//                         if let Some(right) = curr.right.as_ref() {
//                             knn(right, depth + 1, query, result_pq, k);
//                         }
//                     }
//                 }
//             } else {
//                 if let Some(right) = curr.left.as_ref() {
//                     knn(right, depth + 1, query, result_pq, k);

//                     let mx = result_pq
//                         .peek()
//                         .map_or(T::infinity(), |(dist, _p)| dist.into_inner());

//                     if right.distance_to_point(query) < mx {
//                         if let Some(left) = curr.left.as_ref() {
//                             knn(left, depth + 1, query, result_pq, k);
//                         }
//                     }
//                 }
//             }
//         }
//         if let Some(root) = self.root.as_ref() {
//             knn(root, 0, query, &mut res_pq, k);
//         }

//         res_pq
//             .into_iter_sorted()
//             .map(|(dist, point)| unsafe { (dist.into_inner(), point.as_ref().unwrap()) })
//             .collect()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use lazy_static::lazy_static;
//     use rand::{thread_rng, Rng};

//     #[test]
//     fn kdtree() {
//         let mut points = {
//             let mut rng = thread_rng();
//             (0..100)
//                 .map(|_| {
//                     Point([
//                         rng.gen_range(-50.0..50.0),
//                         rng.gen_range(-50.0..50.0),
//                         rng.gen_range(-50.0..50.0),
//                     ])
//                 })
//                 .collect::<Vec<_>>()
//         };
//         let kdt = KdTree::from_slice(&mut points);
//         // println!("{:?}", kdt);
//         let query = Point([0.0, 0.0, 0.0]);
//         let mut nearest = kdt._knearestneighbors(&query, 10);
//         nearest.reverse();
//         println!("{:?}", nearest);
//         let mut points = points
//             .into_iter()
//             .map(|p| (p.squared_eucledian(&query), p))
//             .collect::<Vec<_>>();
//         points.sort_unstable_by_key(|p| OrderedFloat(p.0));
//         println!("{:?}", &points[..10]);
//     }
// }
