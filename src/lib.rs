pub type Point<T> = (T, T);
pub type Boundary<T> = (T, T, T, T);

#[derive(Debug)]
pub enum QuadTree<T: PartialOrd + Copy + Midpoint> {
    Leaf(usize, Boundary<T>, Vec<Point<T>>),
    Node(usize, Boundary<T>, [Box<QuadTree<T>>; 4]),
}

impl<T: PartialOrd + Copy + Midpoint> QuadTree<T>
where
    T: PartialOrd + Copy + Midpoint,
{
    pub fn new(boundary: Boundary<T>) -> Self {
        Self::with_node_capacity(64, boundary)
    }

    pub fn with_node_capacity(capacity: usize, boundary: Boundary<T>) -> Self {
        QuadTree::Leaf(capacity, boundary, vec![])
    }

    pub fn insert(&mut self, point: Point<T>) -> bool {
        if !Self::contains(&self.get_boundary(), &point) {
            return false;
        }

        if let QuadTree::Leaf(capacity, _, points) = self {
            if points.len() < *capacity {
                if !points.iter().any(|p| *p == point) {
                    points.push(point);
                }
                return true;
            }
        }

        if let QuadTree::Leaf(capacity, boundary, points) = self {
            let (x1, x2, y1, y2) = boundary;
            let mid_x = x1.midpoint(*x2);
            let mid_y = y1.midpoint(*y2);

            let mut top_lef = QuadTree::Leaf(*capacity, (*x1, mid_x, *y1, mid_y), vec![]);
            let mut bot_lef = QuadTree::Leaf(*capacity, (*x1, mid_x, mid_y, *y2), vec![]);
            let mut top_rig = QuadTree::Leaf(*capacity, (mid_x, *x2, *y1, mid_y), vec![]);
            let mut bot_rig = QuadTree::Leaf(*capacity, (mid_x, *x2, mid_y, *y2), vec![]);

            for point in points {
                if top_lef.insert(*point) {
                    continue;
                }
                if bot_lef.insert(*point) {
                    continue;
                }
                if top_rig.insert(*point) {
                    continue;
                }
                if bot_rig.insert(*point) {
                    continue;
                }
                panic!("Should never reach here");
            }

            let node = QuadTree::Node(
                *capacity,
                *boundary,
                [
                    Box::new(top_lef),
                    Box::new(bot_lef),
                    Box::new(top_rig),
                    Box::new(bot_rig),
                ],
            );
            *self = node;
        }

        match self {
            QuadTree::Leaf(_, _, _) => panic!("We should never be a leaf at this point"),
            QuadTree::Node(_, _, children) => {
                for child in children {
                    if child.insert(point) {
                        return true;
                    }
                }
                panic!("Should not get here!");
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            QuadTree::Leaf(_, _, points) => points.len(),
            QuadTree::Node(_, _, [a, b, c, d]) => a.size() + b.size() + c.size() + d.size(),
        }
    }

    pub fn search(&self, boundary: &Boundary<T>) -> Vec<Point<T>> {
        if !Self::intersects(&self.get_boundary(), boundary) {
            return vec![];
        }
        match self {
            QuadTree::Leaf(_, _, points) => points
                .iter()
                .copied()
                .filter(|point| Self::contains(&boundary, point))
                .collect(),
            QuadTree::Node(_, _, children) => children
                .iter()
                .flat_map(|child| child.search(boundary))
                .collect(),
        }
    }

    fn get_boundary(&self) -> Boundary<T> {
        match self {
            QuadTree::Leaf(_, boundary, _) => *boundary,
            QuadTree::Node(_, boundary, _) => *boundary,
        }
    }

    pub fn contains((x1, x2, y1, y2): &Boundary<T>, (x, y): &Point<T>) -> bool {
        *x1 <= *x && *x2 > *x && *y1 <= *y && *y2 > *y
    }

    fn intersects(
        (a_x1, a_x2, a_y1, a_y2): &Boundary<T>,
        (b_x1, b_x2, b_y1, b_y2): &Boundary<T>,
    ) -> bool {
        a_x1 < b_x2 && a_x2 > b_x1 && a_y1 < b_y2 && a_y2 > b_y1
    }
}

pub trait Midpoint {
    fn midpoint(&self, a: Self) -> Self;
}

impl Midpoint for f32 {
    fn midpoint(&self, a: f32) -> f32 {
        (*self + a) / 2.0
    }
}

impl Midpoint for f64 {
    fn midpoint(&self, a: f64) -> f64 {
        (*self + a) / 2.0
    }
}

impl Midpoint for i32 {
    fn midpoint(&self, a: i32) -> i32 {
        (*self + a) / 2
    }
}

impl Midpoint for i64 {
    fn midpoint(&self, a: i64) -> i64 {
        (*self + a) / 2
    }
}

impl Midpoint for u32 {
    fn midpoint(&self, a: u32) -> u32 {
        (*self + a) / 2
    }
}

impl Midpoint for u64 {
    fn midpoint(&self, a: u64) -> u64 {
        (*self + a) / 2
    }
}

impl Midpoint for usize {
    fn midpoint(&self, a: usize) -> usize {
        (*self + a) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::QuadTree as Q;

    #[test]
    fn types_work() {
        let _: Q<f32> = Q::new((0.0, 1.0, 0.0, 1.0));
        let _: Q<f64> = Q::new((0.0, 1.0, 0.0, 1.0));
        let _: Q<i32> = Q::new((0, 1, 0, 1));
        let _: Q<u32> = Q::new((0, 1, 0, 1));
        let _: Q<i64> = Q::new((0, 1, 0, 1));
        let _: Q<u64> = Q::new((0, 1, 0, 1));
        let _: Q<usize> = Q::new((0, 1, 0, 1));
    }

    #[test]
    fn contains() {
        let qt = Q::new((0, 10, 0, 10));
        let b = qt.get_boundary();
        assert!(Q::contains(&b, &(0, 0)));
        assert!(Q::contains(&b, &(1, 2)));
        assert!(!Q::contains(&b, &(0, 10)));
        assert!(!Q::contains(&b, &(10, 10)));
        assert!(!Q::contains(&b, &(5, 11)));
    }

    #[test]
    fn intersects() {
        let qt = Q::new((5, 10, 5, 10));
        let b = qt.get_boundary();
        assert!(Q::intersects(&b, &(6, 7, 6, 7)));
        assert!(Q::intersects(&b, &(9, 11, 9, 11)));
        assert!(!Q::intersects(&b, &(10, 11, 10, 11)));
        assert!(!Q::intersects(&b, &(4, 5, 4, 5)));
    }

    #[test]
    fn insert_and_size() {
        let mut qt = Q::new((0, 10, 0, 10));
        let mut count = 0;
        for i in 0..10 {
            for j in 0..10 {
                qt.insert((i, j));
                count += 1;
            }
        }
        assert_eq!(qt.size(), count);
    }

    #[test]
    fn simple_search() {
        let mut qt = Q::new((0, 5, 0, 5));
        qt.insert((1, 2));
        qt.insert((2, 2));
        qt.insert((2, 3)); // outside search area
        assert_eq!(qt.size(), 3);
        let points = qt.search(&(0, 3, 0, 3));
        assert_eq!(points.len(), 2);
    }

    #[test]
    fn larger_search() {
        let mut qt = Q::new((0, 100, 0, 100));

        // We go outside, but they should be ignored.
        for i in 0..102 {
            for j in 0..102 {
                qt.insert((i, j));
            }
        }
        assert_eq!(qt.size(), 100 * 100);

        let search_boundary = (60, 120, 80, 150);
        let points = qt.search(&search_boundary);
        assert_eq!(points.len(), 40 * 20); // effective overlap is 40*20
    }

    /// This will overflow the stack if duplicates are not ignored. This happens because it will
    /// keep trying to subdivide the node, but since 5 points are in the same place, it'll keep
    /// having to subdivide and never get anywhere.
    #[test]
    fn ignores_duplicates() {
        let mut qt = Q::new((0, 10, 0, 10));
        qt.insert((1, 1));
        qt.insert((1, 1));
        qt.insert((1, 1));
        qt.insert((1, 1));
        qt.insert((1, 1)); //   \(>_<)/
        assert_eq!(qt.size(), 1);
    }

    #[test]
    fn same_result_from_different_capacities() {
        let mut rng = get_rng();

        let mut qt1 = Q::with_node_capacity(4, (0, 1000, 0, 1000));
        let mut qt2 = Q::with_node_capacity(16, (0, 1000, 0, 1000));
        let mut qt3 = Q::with_node_capacity(64, (0, 1000, 0, 1000));
        let mut qt4 = Q::with_node_capacity(265, (0, 1000, 0, 1000));

        for _ in 0..100 {
            for _ in 0..100 {
                let p = (rng.next(), rng.next());
                qt1.insert(p);
                qt2.insert(p);
                qt3.insert(p);
                qt4.insert(p);
            }
        }

        // Define a search boundary
        let a = rng.next();
        let b = rng.next();
        let c = rng.next();
        let d = rng.next();
        let (x1, x2) = if a < b { (a, b) } else { (b, a) };
        let (y1, y2) = if c < d { (c, d) } else { (d, c) };
        let search_boundary = (x1, x2, y1, y2);

        let points1 = qt1.search(&search_boundary);
        let points2 = qt2.search(&search_boundary);
        let points3 = qt3.search(&search_boundary);
        let points4 = qt4.search(&search_boundary);

        assert_eq!(points1.len(), points2.len());
        assert_eq!(points1.len(), points3.len());
        assert_eq!(points1.len(), points4.len());

        // Go through each element in first result, and make sure each of them exists in the others
        for point in points1 {
            assert!(points2.iter().any(|p| *p == point));
            assert!(points3.iter().any(|p| *p == point));
            assert!(points4.iter().any(|p| *p == point));
        }
    }

    #[test]
    fn lots_of_numbers() {
        // This test should probably be written as a fuzzy thing when I know how that stuff works.
        let mut rng = get_rng();
        let mut qt = Q::new((0, 1000, 0, 1000));

        // Insert 10,000 points
        for _ in 0..100 {
            for _ in 0..100 {
                let p = (rng.next(), rng.next());
                qt.insert(p);
            }
        }

        // Define a boundary that we want to search in
        let a = rng.next();
        let b = rng.next();
        let c = rng.next();
        let d = rng.next();
        let (x1, x2) = if a < b { (a, b) } else { (b, a) };
        let (y1, y2) = if c < d { (c, d) } else { (d, c) };
        let search_boundary = (x1, x2, y1, y2);

        // Make sure that everything that comes out of `search` is within out defined boundary.
        for (x, y) in qt.search(&search_boundary) {
            assert!(x >= x1);
            assert!(x < x2);
            assert!(y >= y1);
            assert!(y < y2);
        }
    }

    struct XorShift64 {
        a: u64,
    }

    impl XorShift64 {
        pub fn next(&mut self) -> u64 {
            let mut x = self.a;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.a = x;
            x % 1000
        }
    }

    fn get_rng() -> XorShift64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Pseudo random generator
        XorShift64 {
            a: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }
}
