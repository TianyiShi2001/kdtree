#![feature(test)]

extern crate test;
use test::Bencher;

use kdt::*;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref POINTS_3D: Vec<Point<f64, 3>> = {
        let mut rng = thread_rng();
        (0..100000)
            .map(|_| {
                Point([
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                ])
            })
            .collect::<Vec<_>>()
    };
    static ref POINTS_5D: Vec<Point<f64, 5>> = {
        let mut rng = thread_rng();
        (0..100000)
            .map(|_| {
                Point([
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                ])
            })
            .collect::<Vec<_>>()
    };
    static ref POINTS_10D: Vec<Point<f64, 10>> = {
        let mut rng = thread_rng();
        (0..100000)
            .map(|_| {
                Point([
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
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
    fn kdtree_3d(b: &mut Bencher) {
        let mut kdtree = ::kdtree::KdTree::with_capacity(3, 100);
        for point in POINTS_3D.iter() {
            kdtree.add(point.0, ()).unwrap();
        }
        b.iter(|| kdtree.nearest(&[0., 0., 0.], 100, &::kdtree::distance::squared_euclidean));
    }

    #[bench]
    fn kdt_3d(b: &mut Bencher) {
        let mut points = POINTS_3D.clone();
        let kdt = KdTree::from_slice(&mut points);
        b.iter(|| kdt.k_nearest_neighbors(&Point([0., 0., 0.]), 100));
    }

    #[bench]
    fn brutal_force_3d(b: &mut Bencher) {
        let points = POINTS_3D.clone();
        let query = &Point([0., 0., 0.]);
        b.iter(|| {
            let mut dists = points
                .iter()
                .map(|p| (p.squared_eucledian(query), p))
                .collect::<Vec<_>>();
            dists.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        });
    }

    #[bench]
    fn kdtree_5d(b: &mut Bencher) {
        let mut kdtree = ::kdtree::KdTree::with_capacity(5, 100);
        for point in POINTS_5D.iter() {
            kdtree.add(point.0, ()).unwrap();
        }
        b.iter(|| {
            kdtree.nearest(
                &[0., 0., 0., 0., 0.],
                100,
                &::kdtree::distance::squared_euclidean,
            )
        });
    }

    #[bench]
    fn kdt_5d(b: &mut Bencher) {
        let mut points = POINTS_5D.clone();
        let kdt = KdTree::from_slice(&mut points);
        b.iter(|| kdt.k_nearest_neighbors(&Point([0., 0., 0., 0., 0.]), 100));
    }

    #[bench]
    fn brutal_force_5d(b: &mut Bencher) {
        let points = POINTS_5D.clone();
        let query = &Point([0., 0., 0., 0., 0.]);
        b.iter(|| {
            let mut dists = points
                .iter()
                .map(|p| (p.squared_eucledian(query), p))
                .collect::<Vec<_>>();
            dists.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        });
    }

    #[bench]
    fn kdtree_10d(b: &mut Bencher) {
        let mut kdtree = ::kdtree::KdTree::with_capacity(10, 100);
        for point in POINTS_10D.iter() {
            kdtree.add(point.0, ()).unwrap();
        }
        b.iter(|| {
            kdtree.nearest(
                &[0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
                100,
                &::kdtree::distance::squared_euclidean,
            )
        });
    }

    #[bench]
    fn kdt_10d(b: &mut Bencher) {
        let mut points = POINTS_10D.clone();
        let kdt = KdTree::from_slice(&mut points);
        b.iter(|| kdt.k_nearest_neighbors(&Point([0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]), 100));
    }

    #[bench]
    fn brutal_force_10d(b: &mut Bencher) {
        let points = POINTS_10D.clone();
        let query = &Point([0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        b.iter(|| {
            let mut dists = points
                .iter()
                .map(|p| (p.squared_eucledian(query), p))
                .collect::<Vec<_>>();
            dists.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        });
    }
}
