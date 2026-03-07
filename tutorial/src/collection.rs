pub fn run_collection() {
    test_vector();
    test_string();
    test_hashmap();
    test_hashset();
    test_list();
    test_btreemap();
    test_btreeset();
}

fn test_btreeset() {
    use std::collections::BTreeSet;

    let mut set1 = BTreeSet::new();
    set1.insert(1);
    set1.insert(9);
    set1.insert(2);
    set1.insert(5);

    for k in set1.iter() {
        println!("btreeset: {}", k);
    }
    for k in set1.range(1..3) {
        println!("btreeset: {}", k);
    }
    println!("btreeset: min: {}", set1.first().unwrap());
    println!("btreeset: max: {}", set1.last().unwrap());
}

fn test_btreemap() {
    use std::collections::BTreeMap;

    let mut tree1 = BTreeMap::new();
    tree1.insert("aaa", 1);
    tree1.insert("zzz", 2);
    tree1.insert("ddd", 3);
    tree1.insert("jjj", 4);

    for (k, v) in tree1.iter() {
        println!("btreemap: {}-{}", k, v);
    }
    for (k, v) in tree1.range("aaa".."eee") {
        println!("btreemap: {}-{}", k, v);
    }

    println!("btreemap: get(aaa): {:?}", tree1.get("aaa"));
    println!("btreemap: get(qqq): {:?}", tree1.get("qqq"));
    println!("btreemap: min: {:?}", tree1.first_key_value());
    println!("btreemap: max: {:?}", tree1.last_key_value());
}

fn test_list() {
    use std::collections::LinkedList;

    let mut list1 = LinkedList::new();
    list1.push_back(1);
    list1.push_back(9);
    list1.push_back(5);
    list1.push_back(7);
    for v in &list1 {
        println!("list: {}", v);
    }

    list1.pop_front();
    println!("list: {:?}", &list1);
    list1.pop_back();
    println!("list: {:?}", &list1);

    let v = list1.front().unwrap();
    println!("list: get front {}", v);

    for v in list1.iter() {
        println!("list: {}", v);
    }
}

fn test_hashset() {
    use std::collections::HashSet;

    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    for v in &set1 {
        println!("hashset : {}", v);
    }
    println!("hashset has item 1: {}", set1.contains(&1));

    set1.insert(1);
    for v in &set1 {
        println!("hashset : {}", v);
    }
    println!("hashset has item 10: {}", set1.contains(&10));

    println!("hashset len: {}", set1.len());
    println!("hashset cap: {}", set1.capacity());
    println!("hashset is empty {}", set1.is_empty());
    set1.clear();
    println!("hashset len: {}", set1.len());
    println!("hashset cap: {}", set1.capacity());
    set1.shrink_to_fit();
    println!("hashset len: {}", set1.len());
    println!("hashset cap: {}", set1.capacity());
    println!("hashset is empty {}", set1.is_empty());
}

fn test_hashmap() {
    use std::collections::HashMap;

    let mut map1 = HashMap::new();
    map1.insert(1, "one");
    map1.insert(2, "two");
    println!("get(2) is: {}", map1.get(&2).unwrap());
    println!("get(3) is: {:?}", map1.get(&3));
    for (k, v) in &map1 {
        println!("in for: {}-{}", k, v);
    }

    let has_3 = map1.contains_key(&3);
    println!("cantain result is {:?}", has_3);

    *map1.entry(3).or_insert("three") = "three";
    println!("get(3) is: {:?}", map1.get(&3));
    *map1.entry(3).or_insert("three") = "four";
    println!("get(3) is: {:?}", map1.get(&3));

    let mut map2 = HashMap::new();
    map2.insert(1, "oneone");
    map2.insert(10, "ten");

    map1.extend(map2);
    println!("{:?}", map1);
    // map2 is moved
    // println!("{:?}", map2);
}

fn test_string() {
    let s1 = String::from("hello");
    println!("{}, len is {}", s1, s1.len());
    let s2 = "world".to_string();
    println!("{}, cap is {}", s2, s2.capacity());
    let s3 = s1 + " " + &s2;
    println!("s3 is {}", s3);
    // cannot use s1 again
    // println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    for s in s3.chars() {
        println!("in chars: {}", s);
    }

    let s4 = &s3[..5];
    println!("s4 is {}", s4);

    let mut s5 = "helloworld".to_string();
    s5.insert(5, ',');
    println!("s5 is {}", s5);
    s5.remove(5);
    println!("s5 is {}", s5);
}

fn test_vector() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(9);
    v1.push(5);
    println!("{:?}", v1);
    v1.sort();
    println!("{:?}", v1);

    let mut v2 = vec![1, 9, 5, 2, 4, 33, 77, 66];
    v2.push(1);
    println!("{:?}", v2);
    println!("{}", v2[0]);
    println!("{:?}", v2.get(5));
    println!("{:?}", v2.get(50));

    v2.remove(5);
    for v in &v2 {
        println!("in for: {}", v);
    }

    println!("vector len is {}", v2.len());
    println!("vector cap is {}", v2.capacity());
}
