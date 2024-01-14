pub trait InserttableContainer<Item> {
    fn insert(&mut self, item: Item);
}

impl<T> InserttableContainer<T> for Vec<T> {
    fn insert(&mut self, item: T) {
        self.push(item);
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


fn main() {
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

}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn callmain() {
    main()
}