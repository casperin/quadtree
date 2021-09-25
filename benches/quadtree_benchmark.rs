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
    let points_counts = [100, 1000, 5000, 10000, 30000, 75000, 100000];

    for points_count in points_counts {
        let points: Vec<(u64, u64)> = (0..points_count)
            .map(|_| (rng.next(), rng.next()))
            .collect();
        let search_boundary = (rng.next(), rng.next(), rng.next(), rng.next());

        group.bench_with_input(
            BenchmarkId::new("QuadTree", points_count),
            &points_count,
            |b, _| {
                let mut qt = QuadTree::new((0, 10000, 0, 10000));
                for point in points.iter() {
                    qt.insert(*point);
                }
                b.iter(|| qt.search(&search_boundary));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Naive", points_count),
            &points_count,
            |b, _| {
                let mut naive = Naive {
                    boundary: (0, 10000, 0, 10000),
                    points: vec![],
                };
                for point in points.iter() {
                    naive.insert(*point);
                }
                b.iter(|| naive.search(&search_boundary));
            },
        );
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
