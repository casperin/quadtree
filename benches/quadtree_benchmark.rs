use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use quadtree::{Boundary, Midpoint, Point, QuadTree};

struct Naive<T: PartialOrd + Copy + Midpoint> {
    boundary: Boundary<T>,
    points: Vec<Point<T>>,
}

impl<T: PartialOrd + Copy + Midpoint> Naive<T>
where
    T: PartialOrd + Copy + Midpoint,
{
    fn insert(&mut self, point: Point<T>) {
        if !QuadTree::contains(&self.boundary, &point) {
            return;
        }
        if !self.points.iter().any(|p| *p == point) {
            self.points.push(point);
        }
    }

    pub fn search(&self, boundary: &Boundary<T>) -> Vec<Point<T>> {
        self.points
            .iter()
            .filter(|point| QuadTree::contains(boundary, point))
            .copied()
            .collect()
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = get_rng(10000);
    let mut group = c.benchmark_group("QuadTree vs Naive");
    let mut at = 200;
    let to = 5_000;
    let inc = 200;

    let x1 = rng.next();
    let x2 = x1 + 50;
    let y1 = rng.next();
    let y2 = y1 + 50;
    let search_boundary = (x1, x2, y1, y2);

    while at <= to {
        let mut qt = QuadTree::new((0, 10000, 0, 10000));
        let mut naive = Naive {
            boundary: (0, 10000, 0, 10000),
            points: vec![],
        };
        for _ in 0..at {
            let p = (rng.next(), rng.next());
            qt.insert(p);
            naive.insert(p);
        }

        group.bench_with_input(BenchmarkId::new("QuadTree", at), &at, |b, _| {
            b.iter(|| qt.search(&search_boundary));
        });

        group.bench_with_input(BenchmarkId::new("Naive", at), &at, |b, _| {
            b.iter(|| naive.search(&search_boundary));
        });

        at += inc;
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

struct XorShift64 {
    a: u64,
    upper: u64,
}

impl XorShift64 {
    pub fn next(&mut self) -> u64 {
        let mut x = self.a;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.a = x;
        x % self.upper
    }
}

fn get_rng(upper: u64) -> XorShift64 {
    XorShift64 {
        a: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        upper,
    }
}
