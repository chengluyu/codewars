// https://www.codewars.com/kata/isomorphism/train/rust

// I know a little about type theory. This question is really tough to me.
// Here is some (useless) postscript from the author.
// https://zhuanlan.zhihu.com/p/27828408

#![allow(dead_code)]

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this.
/// This is called ISO.
pub enum Void { }

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A: 'static, B: 'static> = (Box<Fn(A) -> B>, Box<Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
    where F1: 'static + Fn(A) -> B,
          F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<Fn(A) -> B> { iso.0 }

/// and vice versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<Fn(B) -> A> { iso.1 }

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    let id = |x: bool| x;
    (Box::new(id), Box::new(id))
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    let not = |x: bool| !x;
    (Box::new(not), Box::new(not))
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    let id = |x: A| x;
    (Box::new(id), Box::new(id))
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>((ab, ba): ISO<A, B>) -> ISO<B, A> {
    (Box::new(ba), Box::new(ab))
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>
((&ab, &ba): ISO<A, B>, (&bc, &cb): ISO<B, C>) -> ISO<A, C> {
    let ac = |a: A| bc(ab(a));
    let ca = |c: C| ba(cb(c));
    (Box::new(ac), Box::new(ca))
}

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>
((ab, ba): ISO<A, B>, (cd, dc): ISO<C, D>) -> ISO<(A, C), (B, D)> {
    let acbd = |(a, c): (A, C)| (ab(a), cd(c));
    let bdac = |(b, d): (B, D)| (ba(b), dc(d));
    (Box::new(acbd), Box::new(bdac))
}

pub fn iso_vec<A: 'static, B: 'static>((ab, ba): ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    let va2vb = |va: Vec<A>| -> Vec<B> { va.into_iter().map(&ab).collect() };
    let vb2va = |vb: Vec<B>| -> Vec<A> { vb.into_iter().map(&ba).collect() };
    (Box::new(va2vb), Box::new(vb2va))
}

pub fn iso_option<A: 'static, B: 'static>
((ab, ba): ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    let oa2ob = |oa: Option<A>|
        oa.and_then(|a| Some(ab(a))).or(None);
    let ob2oa = |ob: Option<B>|
        ob.and_then(|b| Some(ba(b))).or(None);
    (Box::new(oa2ob), Box::new(ob2oa))
}

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>
((ab, ba): ISO<A, B>, (cd, dc): ISO<C, D>) -> ISO<Result<A, C>, Result<B, D>> {
    let eac2ebd = |eac: Result<A, C>| // -> Result<B, D>
        eac.and_then(|a| Ok(ab(a))).or_else(|c| Err(cd(c)));
    let ebd2eac = |ebd: Result<B, D>| // -> Result<A, C>
        ebd.and_then(|b| Ok(ba(b))).or_else(|d| Err(dc(d)));
    (Box::new(eac2ebd), Box::new(ebd2eac))
}

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>
((oa2ob, ob2oa): ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    let ab = |a: A| match oa2ob(Some(a)) {
        Some(b) => b,
        None => match oa2ob(None) {
            None => panic!("impossible"),
            Some(b) => b,
        }
    };
    let ba = |b: B| match ob2oa(Some(b)) {
        Some(a) => a,
        None => match ob2oa(None) {
            None => panic!("impossible"),
            Some(a) => a,
        }
    };
    (Box::new(ab), Box::new(ba))
}

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    let f = |x: Result<Vec<()>, ()>| -> Result<Vec<()>, Void> {
        match x {
            Ok(y) => { let mut yy = y; yy.insert(0, ()); Ok(yy) },
            Err(y) => Ok(Vec::new()),
        }
    };
    let g = |x: Result<Vec<()>, Void>|
        match x {
            Ok(y) => if y.is_empty() { Err(()) } else { let mut yy = y; yy.remove(0); Ok(yy) },
            Err(y) => absurd(y),
        };
    (Box::new(f), Box::new(g))
}

pub type IsoFL<A, B, C, D> = Box<FnOnce(Box<Fn(A) -> C>) -> Box<FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<FnOnce(Box<Fn(B) -> D>) -> Box<FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// The type annotation is shown above. You need you return something like
/// (Box::new(...), Box::new(...))
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>
((ab, ba): ISO<A, B>, (cd, dc): ISO<C, D>) -> IsoF<A, B, C, D> {
    let p = |f: Box<Fn(A) -> C>| -> Box<FnOnce(B) -> D> {
        let r = |b: B| cd(f(ba(b)));
        Box::new(r)
    };
    let q = |f: Box<Fn(B) -> D>| -> Box<FnOnce(A) -> C> {
        let s = |a: A| dc(f((ab(a))));
        Box::new(s)
    };
    (Box::new(p), Box::new(q))
}

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    let f = |(ab, ba): ISO<A, B>| // -> ISO<B, A>
        (ba, ab);
    let g = |(ba, ab): ISO<B, A>| // -> ISO<A, B>
        (ab, ba);
    (Box::new(f), Box::new(g))
}


fn main() {
    assert!(sub_st_l(iso_bool())(true));
    assert!(!sub_st_l(iso_bool())(false));
    assert!(sub_st_l(iso_bool_not())(false));
}
