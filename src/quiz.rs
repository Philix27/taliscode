// Question 18
// fn maind() {
//      let Some(3) = None;
// }

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn say() {
    say_gobe(&mut || 4);
}

fn say_gobe<T: FnOnce() -> i32>(f: T) {
    let _ = f;

    let integer = Option::Some(5);
}
