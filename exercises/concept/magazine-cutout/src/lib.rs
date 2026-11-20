// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // todo!()
    let mut magazine_hash: HashMap<&str, usize> = HashMap::new();
    for w in magazine {
        *magazine_hash.entry(w).or_insert(0) += 1;
    }
    for w in note {
        let word = magazine_hash.entry(w).or_insert(0);
        if *word == 0 {
            return false;
        }
        *word -= 1;
    }
    true
}
