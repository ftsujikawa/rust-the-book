use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's vector: {:?}", v);
    });

    drop(v);

    handle.join().unwrap();
}
