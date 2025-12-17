use crate::structures::{MatrixGraph, UnionFind, self};

/// Creates the first `n` links
fn create_links(matrix_graph: &MatrixGraph, n: usize) -> UnionFind<usize> {
    let nb_points: usize = matrix_graph.nb_points();
    let mut union_find: UnionFind<usize> = UnionFind::new(&(0 .. nb_points).collect());

    for couple_idx in 0 .. n {
        let (idx_1, idx_2) = matrix_graph.get_couple(couple_idx).get_indexes();

        union_find.union(idx_1, idx_2);
    }

    union_find
}

pub fn sol(filename: &str, nb_pairs: usize) -> u32 {
    let matrix_graph = structures::create_matrix_graph(filename);
    let mut union_find = create_links(&matrix_graph, nb_pairs);

    // for (idx, k) in matrix_graph.m.iter().enumerate() {
    //     if idx < 1000 {
    //         println!("d: {:.2},    p_1, p_2:  {:02}, {:02},    {:?}, {:?}", k.dist, k.idx_1, k.idx_2, matrix_graph.points[k.idx_1], matrix_graph.points[k.idx_2]);
    //     }
    // }

    // union_find._sort();
    // println!("sets: {:#?}", union_find.sets);

    union_find.calc_result(3)
}

