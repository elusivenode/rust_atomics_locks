use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    let numbers = (1..=20).collect::<Vec<i32>>();
    let mut handles = vec![];

    for n in numbers {
        let handle = thread::spawn(move || {
            let id = thread::current().id();
            println!("In thread {id:?} running printing {n}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
