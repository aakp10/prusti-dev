/// An adaptation of the example from
/// https://rosettacode.org/wiki/Knuth_shuffle#Rust

extern crate prusti_contracts;

pub struct VecWrapperI32{
    v: Vec<i32>
}

impl VecWrapperI32 {
    // Encoded as body-less Viper function
    #[trusted]
    #[pure]
    #[ensures="result >= 0"]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    // Encoded as body-less Viper method
    #[trusted]
    #[ensures="result.len() == 0"]
    pub fn new() -> Self {
        VecWrapperI32{ v: Vec::new() }
    }

    // Encoded as body-less Viper function
    #[trusted]
    #[pure]
    #[requires="0 <= index && index < self.len()"]
    pub fn lookup(&self, index: usize) -> i32 {
        self.v[index]
    }

    // Encoded as body-less Viper method
    #[trusted]
    #[requires="0 <= index && index < self.len()"]
    #[ensures="self.len() == old(self.len())"]
    #[ensures="self.lookup(index) == value"]
    #[ensures="forall i: usize :: (0 <= i && i < self.len() && i != index) ==>
                    self.lookup(i) == old(self.lookup(i))"]
    pub fn store(&mut self, index: usize, value: i32) {
        self.v[index] = value;
    }

    #[trusted]
    #[ensures="self.len() == old(self.len()) + 1"]
    #[ensures="self.lookup(old(self.len())) == value"]
    #[ensures="forall i: usize :: (0 <= i && i < old(self.len())) ==>
                    self.lookup(i) == old(self.lookup(i))"]
    pub fn push(&mut self, value: i32) {
        self.v.push(value);
    }

    #[trusted]
    #[requires="0 <= index_a && index_a < self.len()"]
    #[requires="0 <= index_b && index_b < self.len()"]
    #[ensures="self.len() == old(self.len())"]
    #[ensures="self.lookup(index_a) == old(self.lookup(index_b))"]
    #[ensures="self.lookup(index_b) == old(self.lookup(index_a))"]
    #[ensures="forall i: usize :: (0 <= i && i < self.len() && i != index_a && i != index_b) ==>
                    self.lookup(i) == old(self.lookup(i))"]
    pub fn swap(&mut self, index_a: usize, index_b: usize) {
        self.v.swap(index_a, index_b);
    }
}

//extern crate rand;

//use rand::Rng;

struct ThreadRngWrapper {}

impl ThreadRngWrapper {
    #[trusted]
    #[requires="low < high"]
    #[ensures="low <= result && result < high"]
    fn gen_range(&mut self, low: usize, high: usize) -> usize {
        unimplemented!();
    }
}

#[trusted]
fn thread_rng() -> ThreadRngWrapper {
    unimplemented!();
}

fn knuth_shuffle(v: &mut VecWrapperI32) {
    let mut rng = thread_rng();
    let l = v.len();

    let mut n = 0;
    let bgn = 0;
    #[invariant="n >= 0"]
    #[invariant="bgn == 0"]
    #[invariant="l == v.len()"]
    while n < l {
        let i = rng.gen_range(bgn, l - n);
        v.swap(i, l - n - 1);
        n += 1;
    }
}

#[trusted]
fn print_vector_before(v: &mut VecWrapperI32) {
    println!("before: {:?}", v.v);
}

#[trusted]
fn print_vector_after(v: &mut VecWrapperI32) {
    println!("after:  {:?}", v.v);
}

fn main() {
    let mut v = VecWrapperI32::new();
    let mut i = 0;
    while i < 10 {
        v.push(i);
    }

    print_vector_before(&mut v);
    knuth_shuffle(&mut v);
    print_vector_after(&mut v);
}
