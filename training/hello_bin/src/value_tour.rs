use std::{collections::HashMap, thread};

pub fn value_tour() {
    // read
    let flag = true;
    if flag {
        println!("ok");
    } else {
        println!("no");
    }

    // modify value
    let mut total = 0;
    total += 1;
    println!("total={}", total);

    // pass to function
    let name = String::from("lxb");
    print_my_name(name);

    // pass by ref
    let mut map = HashMap::new();
    {
        print_map(&map);
    }
    let mmap_ref = &mut map;
    insert_map(mmap_ref);
    print_map(&map);
    map.insert("hello".into(), 1111);
    // insert_map(mmap_ref);
    print_map(&map);

    // multithreaded
    let mut data = vec![1, 2, 3, 4];

    let h = thread::spawn(move || {
        data.push(100);
        println!("thread data={:?}", data);
    });

    //data.push(5);

    let _ = h.join();
}

fn print_my_name(name: String) {
    println!("Hello {}", name);
}

fn print_map(map: &HashMap<String, u64>) {
    println!("{:?}", map);
}

fn insert_map(map: &mut HashMap<String, u64>) {
    map.insert("1".into(), 1);
}
