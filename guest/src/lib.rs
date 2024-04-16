#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable(stack_size = 8_000_000, memory_size = 100_000_000)]
fn tree_hash(serialized: &[u8]) -> [u8; 32] {
    let mut buf_view = trivial_tree::buf_view::BufView::wrap(&serialized);
    let db = trivial_tree::Node::parse(&mut buf_view).unwrap();
    let root_hash = db.hash();
    return root_hash.try_into().unwrap();
}
