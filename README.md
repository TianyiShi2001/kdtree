# kdtree

k-dimensional tree data structure implemented with const generics, used for finding k-nearest neighbours (KNN).

## Example Usage

```rust
use kdt::*;
use ordered_float::OrderedFloat;
use rand::{thread_rng, Rng};

fn main() {
    let mut points = {
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
    let kdt = KdTree::from_slice(&mut points);
    let query = Point([0.0, 0.0, 0.0]);
    let nearest = kdt
        .k_nearest_neighbors(&query, 10)
        .into_iter()
        // each point is returned as a reference. In most use cases you don't need to `clone`
        .map(|(dist, point)| (dist, point.clone()))
        // by default results are sorted in descending order of squared Eucledian distance to the query point
        .rev()
        .collect::<Vec<_>>();
    // compute by brutal force
    let mut expected = points
        .into_iter()
        .map(|p| (p.squared_eucledian(&query), p))
        .collect::<Vec<_>>();
    expected.sort_unstable_by_key(|p| OrderedFloat(p.0));
    assert_eq!(&nearest[..], &expected[..10]);
}

```