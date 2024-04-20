use trivial_tree::Node;

pub fn main() {
    let tree = Node::<32>::random_tree(5, 10);
    let program_summary = guest::analyze_tree_hash(&tree.serialize());
    program_summary
        .write_to_file("failing.txt".into())
        .expect("should write");

    // let (prove_tree, verify_tree) = guest::build_tree_hash();
    // let (output, proof) = prove_tree(&tree.serialize());
    // let is_valid = verify_tree(proof);
    // println!("output: {:?}", output);
    // println!("valid: {}", is_valid);
}
