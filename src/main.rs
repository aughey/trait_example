use std::hash::Hash;

pub trait InserttableContainer<Item> {
    fn insert(&mut self, item: Item);
}

impl<T> InserttableContainer<T> for Vec<T> {
    fn insert(&mut self, item: T) {
        self.push(item);
    }
}

impl<T> InserttableContainer<T> for std::collections::HashSet<T> 
where T: Eq + Hash
{
    fn insert(&mut self, item: T) {
        std::collections::HashSet::insert(self, item);
    }
}

fn do_work_to_vec(container: &mut Vec<i32>, val: i32) {
    container.push(val);
}

fn do_work_to_generic<Container>(container: &mut Container, val: i32)
where
    Container: InserttableContainer<i32>,
{
    container.insert(val);
}

fn do_work_generic_loop<Container>(container: &mut Container, items: impl IntoIterator<Item = i32>)
where
    Container: InserttableContainer<i32>,
{
    for i in items {
        container.insert(i);
    }
}

#[inline(never)]
fn only_vec() -> usize {
    let mut v = vec![];
    do_work_to_generic(&mut v, 70_000_000);
    v.len()
}

fn main() {
    _ = only_vec();

    let mut v = vec![];
    do_work_to_vec(&mut v, 40_000_000);
    println!("v: {:?}", v);

    let mut v = vec![];
    do_work_to_vec(&mut v, 50_000_000);
    println!("v: {:?}", v);

    let mut v = vec![];
    do_work_to_generic(&mut v, 60_000_000);
    println!("v: {:?}", v);

    let mut v = vec![];
    do_work_to_generic(&mut v, 70_000_000);
    println!("v: {:?}", v);

    let mut v = std::collections::HashSet::new();
    do_work_to_generic(&mut v, 70_000_000);
    println!("v: {:?}", v);

    let mut v = vec![];
    do_work_generic_loop(&mut v, [1, 2, 3, 4, 5]);
    println!("v: {:?}", v);

    let mut v = vec![];
    do_work_generic_loop(&mut v, 1..=5);
    println!("v: {:?}", v);
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn callmain() {
    main()
}
