// https://www.codewars.com/kata/algebraic-lists/train/rust

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T>
    {
        fn make_cons<T>(iter: &mut dyn Iterator<Item=T>) -> Cons<T>
            where T: Clone
        {
            if let Some(x) = iter.next() {
                Cons::Cons(x, Box::new(make_cons(iter)))
            } else {
                Cons::Null
            }
        }

        let mut iter = it.into_iter();
        make_cons(&mut iter)
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool
    {
        match self {
            Cons::Cons(x, next) => if fun(x) {
                Cons::Cons(x.clone(), Box::new(next.filter(fun)))
            } else {
                next.filter(fun)
            },
            Cons::Null => Cons::Null,
        }
    }

    pub fn map<F,S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S, S: Clone
    {
        match self {
            Cons::Cons(x, next) => Cons::Cons(fun(x.clone()), Box::new(next.map(fun))),
            Cons::Null => Cons::Null,
        }
    }
}
