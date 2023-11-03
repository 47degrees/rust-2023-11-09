fn main() {
    let x = SomeStruct::default();
    drop(x);
    drop(x);
}

#[derive(Default)]
struct SomeStruct
{
    x: u64
}