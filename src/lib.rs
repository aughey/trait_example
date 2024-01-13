use rand::Rng;

pub mod priority_queue;
pub mod std_containers;

pub trait Priority {
    // Return the priority of the item
    fn priority(&self) -> u32;
}

impl Priority for i32 {
    fn priority(&self) -> u32 {
        *self as u32
    }
}

pub trait InserttableContainer<Item> {
    fn insert(&mut self, item: Item);
}

pub trait RemovableContainer<Item> {
    fn remove(&mut self) -> Option<Item>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn do_work(container: &mut impl InserttableContainer<i32>) {
    // Generate random number in the range [0, 99]
    let num = rand::thread_rng().gen_range(0..100);
    container.insert(num);
}

pub fn consume_priority<C, Item>(container: &mut C) -> bool
where
    C: RemovableContainer<Item>,
    Item: Priority,
{
    container.remove().is_some()
}

pub fn consume_work(container: &mut impl RemovableContainer<i32>) -> bool {
    // Generate random number in the range [0, 99]
    let data = container.remove();
    data.is_some()
}

pub fn outer_req_levy() {
    let mut container = Vec::new();

    container.push(ComplexData { a: 1, b: 2});

    _ = consume_priority(&mut container);
}

pub struct ComplexData {
    a: i32,
    b: i32,
}
impl Priority for ComplexData {
    fn priority(&self) -> u32 {
        (self.a  + self.b).clamp(0,100) as u32
    }
}

#[test]
fn test_regular_vector() {
    let mut container = Vec::new();

    assert!(container.is_empty());
    do_work(&mut container);
    assert!(!container.is_empty());
    assert_eq!(container.len(), 1);
}

#[test]
fn test_set_vector() {
    use std::collections::HashSet;
    let mut container = HashSet::new();

    assert!(container.is_empty());
    do_work(&mut container);
    assert!(!container.is_empty());
    assert_eq!(container.len(), 1);
}

#[cfg(test)]
fn test_generic_container<Container>()
where
    Container: InserttableContainer<i32> + RemovableContainer<i32> + Default,
{
    let mut container = Container::default();

    assert!(container.is_empty());
    do_work(&mut container);
    assert!(!container.is_empty());
    assert_eq!(container.len(), 1);

    let did_work = consume_work(&mut container);
    assert!(did_work);
    assert_eq!(container.len(), 0);
    let did_work = consume_work(&mut container);
    assert!(!did_work);
    assert_eq!(container.len(), 0);
}

#[test]
fn test_containers() {
    test_generic_container::<Vec<i32>>();
    test_generic_container::<std::collections::HashSet<i32>>();
}
