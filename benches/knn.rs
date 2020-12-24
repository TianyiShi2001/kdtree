#![feature(test)]

extern crate test;
use test::Bencher;

use kd_tree::*;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref POINTS_3D: Vec<Point<f64, 3>> = {
        let mut rng = thread_rng();
        (0..100)
            .map(|_| {
                Point([
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                ])
            })
            .collect::<Vec<_>>()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn kdtree(b: &mut Bencher) {
        let mut kdtree = ::kdtree::KdTree::with_capacity(3, 100);
        for point in POINTS_3D.iter() {
            kdtree.add(point.0, ()).unwrap();
        }
        b.iter(|| kdtree.nearest(&[0., 0., 0.], 10, &::kdtree::distance::squared_euclidean));
    }

    #[bench]
    fn kd_tree(b: &mut Bencher) {
        let mut points = POINTS_3D.clone();
        let kdt = KdTree::from_slice(&mut points);
        b.iter(|| kdt.knearestneighbors(&Point([0., 0., 0.]), 10));
    }
}
