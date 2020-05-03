use decorator::decorator;

#[decorator(foo)]
fn bar(x: u64) -> String {
    println!("bar: {}", x);
    x.to_string()
}

#[decorator(foo)]
fn baz(v: u64) -> String {
    println!("baz: {}", v);
    v.to_string()
}

fn foo<F>(f: F, x: u64) -> String
where
    F: Fn(u64) -> String,
{
    println!("foo start");
    let ret = f(x);
    println!("foo end");
    ret
}

fn main() {
    println!("decorated-bar return: {}", bar(123));
    println!("decorated-baz return: {}", baz(456));
}
