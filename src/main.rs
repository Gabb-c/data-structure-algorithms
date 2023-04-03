use data_structure_algorithms::search;

fn main() {
    let vec: Vec<i32> = (0..=124).collect();

    search::binary(12, &vec);
}
