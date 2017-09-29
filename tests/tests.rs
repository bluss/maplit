#[macro_use]
extern crate maplit;

use std::collections::{ HashMap, HashSet };

#[test]
fn test_parse() {
    let mut m = hashmap!{};
    m.insert(1, 1);
    let _: HashMap<i32, i32> = hashmap!{1 => 1};
    let _: HashMap<i32, i32> = hashmap!{1 => 1,};
    let _: HashMap<i32, i32> = hashmap!{1 + 1 => 1, 2 + 1 => 2};
    let _: HashMap<i32, i32> = hashmap!{1 + 1 => 1, 2 + 1 => 2,};
    let _: HashMap<i32, i32> = hashmap!{{1 + 2} => 1, (1 + 3) => {0 + 2}};
    let m: HashMap<String, i32> = hashmap!{"a" => 1 + 2, "b" => 1 + 3};
    assert_eq!(m["a"], 3);
    assert_eq!(m["b"], 4);
    let m: HashMap<String, i32> = hashmap!{"a" => 1 + 2, "b" => 1 + 3, };
    assert_eq!(m["a"], 3);
    assert_eq!(m["b"], 4);

    let mut s = hashset!{};
    s.insert(1);
    let _: HashSet<i32> = hashset!{1};
    let _: HashSet<i32> = hashset!{1,};
    let _: HashSet<i32> = hashset!{1, 2};
    let _: HashSet<i32> = hashset!{1, 2,};
    let _: HashSet<i32> = hashset!{1 + 1, 2 + 1};
    let _: HashSet<i32> = hashset!{1 + 1, 2 + 1,};
    let _: HashSet<i32> = hashset!{{1 + 1}, (2 + 1)};
}

#[test]
fn hashset() {
    let mut set = hashset!{};
    assert!(set.is_empty());
    set.insert(2);
    let set: HashSet<i32> = hashset!{1};
    assert_eq!(set.len(), 1);
    let set: HashSet<i32> = hashset!{2, 3};
    assert_eq!(set.len(), 2);
    // Test that we can use many elements without hitting the macro recursion limit
    let set: HashSet<i64> = hashset!{1,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,
    };
    assert_eq!(set.len(), 10);
}
