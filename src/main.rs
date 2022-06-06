use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
struct Atan<T, I> {
    angle: T,
    period: I,
}

impl<T, U, I, J> Add<&Atan<U, J>> for &Atan<T, I>
where
    for<'a, 'b> &'a T: Mul<&'b U>,
{
    type Output = Atan<T, I>;

    fn add(self, rhs: &Atan<U, J>) -> Self::Output {
        let prod = &self.angle * &rhs.angle;
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
