mod adj;
mod hash;
mod vegetable;

use crate::adj::ADJ;
use crate::hash::my_hash;
use crate::vegetable::VEGETABLE;
use once_cell::sync::Lazy;
use std::ops::Div;
use std::ops::Rem;

static ALL_NAMES: Lazy<(Vec<String>, Vec<String>)> = Lazy::new(|| {
    let adjs = ADJ.split(",").map(|item| item.trim().to_string()).collect();
    let vegetables = VEGETABLE
        .split(",")
        .map(|item| item.trim().to_string())
        .collect();
    (adjs, vegetables)
});

pub fn generate_nickname(peer_id: &str) -> String {
    let peer_id = my_hash(peer_id.as_bytes());
    let adjs = ALL_NAMES.0.len();
    let vegetables = ALL_NAMES.1.len();
    let res_name_index = (peer_id as usize).rem(adjs * vegetables);
    let left = res_name_index.div(vegetables);
    let right = res_name_index.rem(vegetables);
    format!(
        "{} {}",
        ALL_NAMES.0.get(left).unwrap(),
        ALL_NAMES.1.get(right).unwrap()
    )
}

#[test]
fn test_nickname() {
    let v = generate_nickname("16842655250600210769");
    println!("{v}");
}
