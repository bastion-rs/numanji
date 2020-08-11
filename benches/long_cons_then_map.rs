#![feature(test)]

// https://github.com/bastion-rs/numanji/issues/1

use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;

enum ArcList<T> {
    Cons(T, Arc<Self>),
    Nil,
}

impl<T> ArcList<T> {
    fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }
}

impl<T: Debug> Debug for ArcList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Nil => Ok(()),
            Self::Cons(e, t) if t.is_nil() => write!(f, "{:?}", e),
            Self::Cons(e, t) => write!(f, "{:?}, {:?}", e, *(*t)),
        }
    }
}
type Ptr<T> = Arc<ArcList<T>>;

fn cons<T>(t: T, list: Ptr<T>) -> Ptr<T> {
    Arc::new(ArcList::Cons(t, list.clone()))
}

fn count_inner<T>(acc: usize, list: Ptr<T>) -> usize {
    match &*list {
        ArcList::Nil => acc,
        ArcList::Cons(_, t) => count_inner(acc + 1, t.clone()),
    }
}

fn count<T>(list: Ptr<T>) -> usize {
    count_inner(0, list)
}

fn map<T, U>(f: fn(&T) -> U, list: Ptr<T>) -> Ptr<U> {
    match &*list {
        ArcList::Nil => Arc::new(ArcList::Nil),
        ArcList::Cons(x, t) => Arc::new(ArcList::Cons(f(x), map(f, t.clone()))),
    }
}

#[cfg(test)]
mod list_bench {
    extern crate test;
    use super::*;
    use rand::Rng;
    use std::sync::Arc;
    use test::Bencher;
    const SCALE: usize = 10000;
    #[bench]
    fn long_cons_then_count(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut rng = rand::thread_rng();
            let mut a = Arc::new(ArcList::Nil);
            for _ in 0..SCALE {
                a = cons(rng.gen::<usize>(), a);
            }
            assert_eq!(count(a), SCALE)
        });
    }

    #[bench]
    fn long_cons_then_map(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut rng = rand::thread_rng();
            let mut a = Arc::new(ArcList::Nil);
            for _ in 0..SCALE {
                a = cons(rng.gen::<usize>(), a);
            }
            map(|x| x + 1, a);
        });
    }

    #[bench]
    fn long_cons_then_count_in_multi_threads(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut handles = Vec::new();
            for _ in 0..6 {
                handles.push(
                    std::thread::Builder::new()
                        .stack_size(512 * 1024 * 1024)
                        .spawn(|| {
                            let mut rng = rand::thread_rng();
                            let mut a = Arc::new(ArcList::Nil);
                            for _ in 0..SCALE {
                                a = cons(rng.gen::<usize>(), a);
                            }
                            assert_eq!(count(a), SCALE)
                        })
                        .unwrap(),
                );
            }
            for i in handles {
                i.join().unwrap();
            }
        });
    }

    #[bench]
    fn long_cons_then_map_across_multi_threads(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut rng = rand::thread_rng();
            let mut handles = Vec::new();
            let mut a = Arc::new(ArcList::Nil);
            for _ in 0..SCALE {
                a = cons(rng.gen::<usize>(), a);
            }
            for _ in 0..6 {
                let a = a.clone();
                handles.push(
                    std::thread::Builder::new()
                        .stack_size(512 * 1024 * 1024)
                        .spawn(move || {
                            map(|x| x + 1, a);
                        })
                        .unwrap(),
                );
            }
            for i in handles {
                i.join().unwrap();
            }
        });
    }
}
