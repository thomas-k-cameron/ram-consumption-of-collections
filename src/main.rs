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

fn main() {
    for i in (0..16).step_by(2) {
        let n = 2i64.pow(i);
        calculate(n);
    }
}

#[derive(serde::Serialize)]
struct Log {
    entry_count: i64,
    value_type: &'static str,
    collection: &'static str,
    capacity: Option<usize>,
    ram_usage: usize,
    shrink_type: ShrinkType
}

#[derive(serde::Serialize, Clone, Copy)]
enum ShrinkType {
    AfterShrink,
    BeforeShrink,
    None
}

macro_rules! this {
    ($i:ident, $func:ident) => {
        this!($i, $func, EmptyStruct);
        this!($i, $func, BigStruct);
    };
    (@ linked_list $_:ident) => {
        None
    };
    (@ btreemap $_:ident) => {
        None
    };
    (@ nothing $_:ident) => {
        None
    };
    (@ $_:ident $cap:ident) => {
        Some($cap.capacity())
    };
    (@ shrink vector $a:ident $t:ident) => {
        $a.shrink_to_fit();
        $t = ShrinkType::AfterShrink;
    };
    (@ shrink vector_deque $a:ident $t:ident) => {
        $a.shrink_to_fit();
        $t = ShrinkType::AfterShrink;
    };
    (@ shrink hashmap $a:ident $t:ident) => {
        $a.shrink_to_fit();
        $t = ShrinkType::AfterShrink;
    };
    (@ shrink $_:ident $__:ident $t:ident) => {
        $t = ShrinkType::None;
    };
    ($i:ident, $func:ident, $gen:ident) => {
        #[allow(unused_mut)]
        let mut a = $func::<$gen>($i);
        let capacity = this!(@ $func a);
        let f = |shrink_type, capacity| Log {
            entry_count: $i,
            value_type: stringify!($gen),
            collection: stringify!($func),
            capacity,
            ram_usage: RAM_COUNT,
            shrink_type
        };
        
        let mut shrink_type = ShrinkType::BeforeShrink;
        let s = serde_json::to_string(&f(shrink_type, capacity)).unwrap();
        println!("{s}");
        drop(s);
        this!(@ shrink $func a shrink_type);
        let capacity = this!(@ $func a);
        let s = serde_json::to_string(&f(shrink_type, capacity)).unwrap();
        println!("{s}");
        drop(a);
        drop(s);
    };
}

fn calculate(i: i64) {
    unsafe {
        this!(i, nothing);
        this!(i, hashmap);
        this!(i, btreemap);
        this!(i, vector);
        this!(i, vector_deque);
        this!(i, linked_list);
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

struct Nothing;
fn nothing<T: Default>(_: i64) -> Nothing {
    Nothing
}

fn btreemap<T: Default>(a: i64) -> BTreeMap<Key,T> {
    let mut map = BTreeMap::new();
    for i in 0..a {
        map.insert(i, T::default());
    }
    map
}

fn hashmap<T: Default>(a: i64) -> HashMap<Key,T>{
    let mut map = HashMap::new();
    for i in 0..a {
        map.insert(i, T::default());
    }
    map
}

fn vector<T: Default>(a: i64) -> Vec<(Key,T)> {
    let mut stack = vec![];
    for i in 0..a {
        stack.push((i, T::default()));
    }
    stack
}


fn vector_deque<T: Default>(a: i64) -> VecDeque<(Key,T)> {
    let mut stack = VecDeque::new();
    for i in 0..a {
        stack.push_back((i, T::default()));
    }
    stack
}


fn linked_list<T: Default>(a: i64) -> LinkedList<(Key,T)> {
    let mut stack = LinkedList::new();
    for i in 0..a {
        stack.push_back((i, T::default()));
    }
    stack
}
