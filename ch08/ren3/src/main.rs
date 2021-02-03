use std::collections::HashMap;

fn main() {
    let texts = vec![
        "Add Sally to Engineering",
        "Add tsu to Sales",
        "Add Amir to Sales",
    ];

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for text in texts {
        let mut i = 0;
        let mut key = "";
        let mut val = "";
        for x in text.split_whitespace() {
            if i == 1 {
                val = x;
            }
            else if i == 3 {
                key = x;
            }
            i += 1;
        }

        let names = map.entry(&key).or_insert(Vec::new());
        names.push(val);
        names.sort();
    }

    for (section, names) in &map {
        print!("{}: ", section);
        for name in names {
            print!("{} ", name);
        }
        println!("");
    }
}
