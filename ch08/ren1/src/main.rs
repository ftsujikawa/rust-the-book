use std::collections::HashMap;

fn main() {
    let mut v = vec![23, 53, 12, 62, 35, 98, 71, 31, 12];
    let mut t = 0;
    for i in &v {
        t += i;
    }
    println!("mean is {}", t / v.len());
    v.sort();
    println!("v is {:?}", v);
    println!("median is {}", v[v.len() / 2]);

    let mut map = HashMap::new();
    for val in &v {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    let mut max = 0;
    let mut k: &&usize = &&0;
    for (key, val) in &map {
        if max < *val {
            k = key;
            max = *val;
        }
    }
    println!("mode is {}", k);
}
