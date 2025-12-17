use crate::structures::{MatrixGraph, UnionFind, self};

/// Creates the links until there is only one set
///
/// In:
///     - matrix_graph: the matrix representation of the graph
/// Out:
///     The puzzle result: product of the x coordinate from the points of the last couple added.
fn create_links(matrix_graph: &MatrixGraph) -> u128 {
    let nb_points: usize = matrix_graph.nb_points();
    let mut union_find: UnionFind<usize> = UnionFind::new(&(0 .. nb_points).collect());

    let mut couple_idx: usize = 0;

    while union_find.nb_components() > 1 {
        let (idx_1, idx_2) = matrix_graph.get_couple(couple_idx).get_indexes();
        union_find.union(idx_1, idx_2);
        // println!("couple_idx: {couple_idx}, nb_components: {}", union_find.nb_components());

        couple_idx += 1;
    }

    couple_idx -= 1;

    let (idx_1, idx_2) = matrix_graph.get_couple(couple_idx).get_indexes();

    let x_p1 = matrix_graph.get_pt_x(idx_1).unwrap();
    let x_p2 = matrix_graph.get_pt_x(idx_2).unwrap();

    // println!("p1: {:?}, p2: {:?}", p1, p2);

    (x_p1 as u128) * (x_p2 as u128)
}

pub fn sol(filename: &str) -> u128 {
    let matrix_graph = structures::create_matrix_graph(filename);
    create_links(&matrix_graph)
}

