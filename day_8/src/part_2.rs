use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    /// Creates a new Point from puzzle input line
    fn new(line: &str) -> Self {
        let mut coords: Vec<u32> = Vec::new();

        for c in line.split(',') {
            coords.push(c.parse().unwrap());
        }

        Self { x: coords[0], y: coords[1], z: coords[2] }
    }

    /// Calculates distance to `other`
    fn dist(&self, other: &Self) -> f64 {
        (
            (self.x as f64 - other.x as f64).powi(2) +
            (self.y as f64 - other.y as f64).powi(2) +
            (self.z as f64 - other.z as f64).powi(2)
        ).sqrt()
    }
}

/// Represents a couple of points
/// Specifically useful for the HashMap and vector in MatrixGraph
struct PointCouple {
    idx_1: usize,
    idx_2: usize,
    // p_1: Option<&'a Point>,
    // p_2: Option<&'a Point>,
    dist: f64,
}

impl PointCouple {
    /// Creates a new PointCouple ensuring that self.idx_1 <= self.idx_2
    // fn new(idx_1: usize, idx_2: usize, p_1: Option<&'a Point>, p_2: Option<&'a Point>) -> Self {
    fn new(idx_1: usize, idx_2: usize, dist: f64) -> Self {
        if idx_1 <= idx_2 {
            Self { idx_1, idx_2, dist }
        }
        else {
            Self { idx_1: idx_2, idx_2: idx_1, dist: dist }
        }
    }

    // /// Calculates the distance
    // fn dist(&self, points: Vec<Point>) -> Option<f64> {
    //     let p_1: Option<&Point> = points.get(self.idx_1);
    //     let p_2: Option<&Point> = points.get(self.idx_2);
    //
    //     match (p_1, p_2) {
    //         (None, _) | (_, None) => None,
    //         (Some(vp1), Some(vp2)) => Some(vp1.dist(&vp2)),
    //     }
    // }

    // /// Returns the index for the HashMap
    // fn map_idx(&self) -> (usize, usize) {
    //     (self.idx_1, self.idx_2)
    // }
}

/// Will represent the graph as a matrix m, with m[i][j] the distance between i and j
/// Used to sort points couples by distance
struct MatrixGraph {
    points: Vec<Point>, // associates index to Point
    m: Vec<PointCouple>,
}

impl MatrixGraph {
    /// Creates a new empty MatrixGraph
    fn new() -> Self {
        MatrixGraph { points: Vec::new(), m: Vec::new() }
    }

    /// Add a new Point to self
    fn add(&mut self, p: Point) {
        let p_idx: usize = self.points.len(); // Index of the new point

        // Calculates it distance to all the others
        for (o_idx, p_o) in self.points.iter().enumerate() {
            let dist: f64 = p.dist(p_o);
            let couple: PointCouple = PointCouple::new(o_idx, p_idx, dist);

            self.m.push(couple);
        }

        // Add point to the point list
        self.points.push(p);
    }

    /// Sorts `self.m` by distance
    fn sort(&mut self) {
        // self.m.sort_by_key(|couple| couple.dist);
        self.m.sort_by(|c1, c2| c1.dist.partial_cmp(&c2.dist).unwrap());
    }
}

/// Set for the union-find structure just below
#[derive(Debug)]
struct UnionFindSet<T> {
    s: Vec<T>,
}

impl<T: std::cmp::PartialEq + Clone> UnionFindSet<T> {
    /// Creates a new UnionFindSet containing the given element
    fn new(x: T) -> Self {
        let mut s: Vec<T> = Vec::new();
        s.push(x);

        UnionFindSet { s }
    }

    /// Checks if `x` is in this set
    fn has(&self, x: T) -> bool {
        self.s.contains(&x)
    }

    /// Extends the current set with the other
    fn extends(&mut self, other: &Self) {
        self.s.extend_from_slice(&other.s);
    }

    /// Calculates the number of elements in the current set
    fn size(&self) -> usize {
        self.s.len()
    }
}

/// Union-find structure to calculate the components of the graph.
/// It uses the indexes of the points from MatrixGraph.points and not the points themselves.
struct UnionFind<T> {
    sets: Vec<UnionFindSet<T>>,
}

impl<T: PartialEq + Clone + Copy> UnionFind<T> {
    /// Creates a new UnionFind with a distincts set for each value of `items`
    fn new(items: &Vec<T>) -> Self {
        let mut sets: Vec<UnionFindSet<T>> = Vec::new();

        for item in items {
            sets.push(UnionFindSet::new(*item));
        }

        UnionFind { sets }
    }

    /// Calculates the index of the set containing `x` in `self.sets`.
    fn find(&self, x: T) -> Option<usize> {
        for (idx, set) in self.sets.iter().enumerate() {
            if set.has(x) {
                return Some(idx);
            }
        }

        None
    }

    /// Merges the set containing `x` with the one containing `y`.
    /// If they are already in the same set, do not perform any action.
    fn union(&mut self, x: T, y: T) {
        match (self.find(x), self.find(y)) {
            (Some(idx_x), Some(idx_y)) if idx_x != idx_y => {
                let (i, j) = 
                    if idx_x <= idx_y { (idx_x, idx_y) }
                    else { (idx_y, idx_x) };

                // First delete the set containing y
                let set_y: UnionFindSet<T> = self.sets.remove(j);

                // Second extends the set containing x with the one containing y
                self.sets[i].extends(&set_y);
            },
            _ => (),
        }
    }

    /// Calculates the number of components
    fn nb_components(&self) -> usize {
        self.sets.len()
    }

    /// Sorts the sets by element count.
    fn _sort(&mut self) {
        self.sets.sort_by_key(|s| -(s.size() as isize))
    }

    // /// Calculates the product of the size of the `n` largests sets
    // /// This is the expected result of the challenge (with n = 3)
    // ///
    // /// Internally calls self._sort.
    // ///
    // fn calc_result(&mut self, n: usize) -> u32 {
    //     self._sort();
    //
    //     // print!("calc_result: ");
    //
    //     let mut res: usize = 1;
    //     for set_idx in 0 .. n {
    //         res *= self.sets[set_idx].size();
    //         
    //         // print!("{}, ", self.sets[set_idx].size());
    //     }
    //
    //     // println!(" res: {res}");
    //
    //     res as u32
    // }
}

/// Reads the points, stores them and sorts them by distance
fn create_matrix_graph(filename: &str) -> MatrixGraph {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the points and store them
    let mut matrix_graph: MatrixGraph = MatrixGraph::new();

    for line in reader.lines() {
        let l = line.unwrap();

        let p: Point = Point::new(&l);
        // println!("Added p: {:?}", &p);
        matrix_graph.add(p);
    }

    // Sort the points
    matrix_graph.sort();

    matrix_graph
}

/// Creates the links until there is only one set
fn create_links(matrix_graph: &MatrixGraph) -> u128 {
    let nb_points: usize = matrix_graph.points.len();
    let mut union_find: UnionFind<usize> = UnionFind::new(&(0 .. nb_points).collect());

    let mut couple_idx: usize = 0;

    while union_find.nb_components() > 1 {
        let c: &PointCouple = &matrix_graph.m[couple_idx];
        
        // println!("couple_idx: {couple_idx}, nb_components: {}", union_find.nb_components());

        union_find.union(c.idx_1, c.idx_2);

        couple_idx += 1;
    }

    couple_idx -= 1;

    let idx_p1 = matrix_graph.m[couple_idx].idx_1;
    let idx_p2 = matrix_graph.m[couple_idx].idx_2;

    let p1 = &matrix_graph.points[idx_p1];
    let p2 = &matrix_graph.points[idx_p2];

    // println!("p1: {:?}, p2: {:?}", p1, p2);

    (p1.x as u128) * (p2.x as u128)
}

pub fn sol_2(filename: &str) -> u128 {
    let matrix_graph = create_matrix_graph(filename);
    create_links(&matrix_graph)

    // for (idx, k) in matrix_graph.m.iter().enumerate() {
    //     if idx < 1000 {
    //         println!("d: {:.2},    p_1, p_2:  {:02}, {:02},    {:?}, {:?}", k.dist, k.idx_1, k.idx_2, matrix_graph.points[k.idx_1], matrix_graph.points[k.idx_2]);
    //     }
    // }

    // union_find._sort();
    // println!("sets: {:#?}", union_find.sets);
}


