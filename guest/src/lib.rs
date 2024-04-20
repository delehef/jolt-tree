#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use trivial_tree::{buf_view::BufView, Node};

#[jolt::provable]
fn tree_hash(serialized: &[u8]) -> [u8; 32] {
    let mut buf_view = BufView::wrap(&serialized);
    let db = Node::<32>::parse(&mut buf_view).unwrap();

    let root_hash = db.hash();
    return root_hash.try_into().unwrap();
}
