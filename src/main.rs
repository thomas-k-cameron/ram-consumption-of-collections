use std::{collections::{HashMap, BTreeMap, VecDeque, LinkedList}, alloc::{GlobalAlloc, Layout}};

use jemallocator::Jemalloc;
#[global_allocator]
static ALLOC: CountAlloc<Jemalloc> = CountAlloc { inner: jemallocator::Jemalloc};
static mut RAM_COUNT: usize = 0;
struct CountAlloc<A: GlobalAlloc> {
    inner: A
}
unsafe impl<A: GlobalAlloc> GlobalAlloc for CountAlloc<A> {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        RAM_COUNT += layout.size();
        self.inner.alloc(layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        RAM_COUNT += layout.size();
        self.inner.alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        RAM_COUNT -= layout.size();
        self.inner.dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        RAM_COUNT = RAM_COUNT - layout.size() + new_size;
        self.inner.realloc(ptr, layout, new_size)
    }
}

impl<A: GlobalAlloc> CountAlloc<A> {
    pub unsafe fn show(&self, s: &str) {
        println!("{s}\t: {RAM_COUNT}");
    }
}

fn main() {
    unsafe {
        ALLOC.show("without collections");
        let a = hashmap::<EmptyStruct>();
        ALLOC.show("hashmap with EmptyStruct");
        drop(a);
        let b = hashmap::<BigStruct>();
        ALLOC.show("hashmap with BigStruct");
        drop(b);
        let c = btreemap::<EmptyStruct>();
        ALLOC.show("btreemap with EmptyStruct");
        drop(c);
        let d = btreemap::<BigStruct>();
        ALLOC.show("btreemap with BigStruct");
        drop(d);
        let e = vector::<EmptyStruct>();
        ALLOC.show("vector with EmptyStruct");
        drop(e);
        let f = vector::<BigStruct>();
        ALLOC.show("vector with BigStruct");
        drop(f);
        let f = vector_deque::<EmptyStruct>();
        ALLOC.show("vector deque with EmptyStruct");
        drop(f);
        let f = vector_deque::<BigStruct>();
        ALLOC.show("vector deque with BigStruct");
        drop(f);
        let f = linked_list::<EmptyStruct>();
        ALLOC.show("linked list with EmptyStruct");
        drop(f);
        let f = linked_list::<BigStruct>();
        ALLOC.show("linked list with BigStruct");
        drop(f);
    }
}

#[derive(Default)]
struct EmptyStruct;

#[allow(dead_code)]
#[derive(Default)]
struct BigStruct {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
    i: i64,
    j: i64,
    k: i64,
    l: i64,
    m: i64,
    n: i64,
    o: i64,
    p: i64,
    q: i64,
    r: i64,
    s: i64,
    t: i64,
    u: i64,
    v: i64,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}


type Key = i64;
fn btreemap<T: Default>() -> BTreeMap<Key,T> {
    let mut map = BTreeMap::new();
    for i in 0..1_000_000 {
        map.insert(i, T::default());
    }
    map
}

fn hashmap<T: Default>() -> HashMap<Key,T>{
    let mut map = HashMap::new();
    for i in 0..1_000_000 {
        map.insert(i, T::default());
    }
    map
}

fn vector<T: Default>() -> Vec<(Key,T)> {
    let mut stack = vec![];
    for i in 0..1_000_000 {
        stack.push((i, T::default()));
    }
    stack
}


fn vector_deque<T: Default>() -> VecDeque<(Key,T)> {
    let mut stack = VecDeque::new();
    for i in 0..1_000_000 {
        stack.push_back((i, T::default()));
    }
    stack
}


fn linked_list<T: Default>() -> LinkedList<(Key,T)> {
    let mut stack = LinkedList::new();
    for i in 0..1_000_000 {
        stack.push_back((i, T::default()));
    }
    stack
}
