fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn() {
    let a = 10;
    let b = 10;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c)
}

fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined String is '{}'", combined);
}

fn update_string() {
    let mut s = String::from("Initial String");
    println!("Before Update: {}", s);
    println!(
        "capacity: {} Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    for _ in 0..10 {
        s.push_str("some additional text");
    }

    s.push_str("and some extra text");
    println!("After update {}", s);
    println!(
        "capacity: {} Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}
