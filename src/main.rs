pub fn main() {
    let serialized = trivial_tree::random_tree(1, 2);
    let program_summary = guest::analyze_tree_hash(&serialized);
    program_summary
        .write_to_file("failing.txt".into())
        .expect("should write");

    // let (prove_tree, verify_tree) = guest::build_tree_hash();
    // let (output, proof) = prove_tree(&serialized);
    // let is_valid = verify_tree(proof);
    // println!("output--: {:?}", output);
    // println!("valid: {}", is_valid);
}
