#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use buf_view::BufView;

#[jolt::provable]
fn tree_hash(serialized: &[u8]) -> Vec<u8> {
    let mut buf_view = BufView::wrap(&serialized);
    let db = trivial_tree::Node::parse(&mut buf_view).unwrap();
    // db.pretty();
    // println!("done");

    // println!("computing root hash...");
    let root_hash = db.hash();

    // println!("done: {:x?}.", root_hash);
    return root_hash;
}
