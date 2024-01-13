use trait_example::InserttableContainer;

fn do_work_to_vec(container: &mut Vec<i32>) {
    container.push(42);
}

fn do_work_to_generic<Container>(container: &mut Container)
where
    Container: InserttableContainer<i32>,
{
    container.insert(42);
}


fn main() {
    let mut v = vec![];
    do_work_to_vec(&mut v);
    println!("v: {:?}", v);

    do_work_to_generic(&mut v);
    println!("v: {:?}", v);
}
