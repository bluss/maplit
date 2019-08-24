pub mod test1 {
    use std::collections::HashSet;

    pub fn make_map() -> HashSet<String> {
        let s = String::from;
        maplit::hashset! {
            s("a1"),
            s("a2"),
        }
    }
}

pub mod test2 {
    use maplit::hashset;
    use std::collections::HashSet;

    pub fn make_map() -> HashSet<String> {
        let s = String::from;
        hashset! {
            s("a1"),
            s("a2"),
        }
    }

    pub fn convert() -> HashSet<String> {
        maplit::convert_args!(hashset!("a", "b"))
    }
}
