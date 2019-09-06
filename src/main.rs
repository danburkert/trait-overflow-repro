pub trait Frob {
    type Value: Clone;

    fn len(self) -> usize;

    fn wrap(self) -> Wrapper<Self>
    where
        Self: Sized,
    {
        Wrapper { inner: self }
    }
}

pub struct Unit;

impl Frob for Unit {
    type Value = ();
    fn len(self) -> usize {
        0
    }
}

impl<'a> Frob for &'a Unit {
    type Value = ();
    fn len(self) -> usize {
        0
    }
}

pub struct Wrapper<I>
{
    inner: I,
}

impl<I> Frob for Wrapper<I>
where
    I: Frob,
{
    type Value = I::Value;

    fn len(self) -> usize {
        self.inner.len()
    }
}

impl<'a, I> Frob for &'a Wrapper<I>
where
    &'a I: Frob,
{
    type Value = i32;

    fn len(self) -> usize {
        <&'a I as Frob>::len(&self.inner)
    }
}

fn main() {
    let unit: Unit = Unit;
    let wrapper: Wrapper<Unit> = unit.wrap();

    // Compiles fine
    assert_eq!(0, <&Wrapper<Unit> as Frob>::len(&wrapper));

    // Explode
    assert_eq!(0, <&Wrapper<_> as Frob>::len(&wrapper));
}
