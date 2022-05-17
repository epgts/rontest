#[derive(serde::Serialize)]
struct Foo(u64);

fn main() {
    let foo = Foo(0);
    println!("{}", ::ron::to_string(&foo).unwrap());
}
