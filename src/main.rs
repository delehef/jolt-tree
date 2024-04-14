pub fn main() {
    let serialized = trivial_tree::random_tree(10, 10);
    let (prove_tree, verify_tree) = guest::build_tree_hash();

    let (output, proof) = prove_tree(&serialized);
    let is_valid = verify_tree(proof);

    println!("output--: {:?}", output);
    println!("valid: {}", is_valid);
}
