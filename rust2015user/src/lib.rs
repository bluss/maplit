#[macro_use(hashset)]
extern crate maplit;

pub mod test2 {
    use std::collections::HashSet;

    pub fn make_map() -> HashSet<String> {
        let s = String::from;
        hashset! {
            s("a1"),
            s("a2"),
        }
    }

}
