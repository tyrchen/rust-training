use std::{collections::HashMap, thread};

pub fn value_tour() {
    // read
    let authenticated = true;
    // if authenticated {
    //     todo!()
    // } else {
    //     todo!()
    // }

    // modify value
    let mut total = 0usize;
    total += 1;

    // pass to function
    let name = "Tyr".to_string();
    print_my_name(name);

    // pass by ref
    let mut map: HashMap<String, String> = HashMap::new();
    let my_map = &mut map;
    // print_map(&map);

    // map.insert("hello".into(), "world".into());
    // insert_map(my_map);

    // multithreaded
    let mut data = vec![1, 2, 3];

    thread::spawn(move || {
        data.push(5);
    });

    // data.push(4);
}

fn print_my_name(name: String) {
    println!("{}", name);
}

fn print_map(map: &HashMap<String, String>) {
    todo!();
}

fn insert_map(map: &mut HashMap<String, String>) {
    todo!();
}
