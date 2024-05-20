use crate::*;

/// Alternative implementation of `Iterator::sum::<KahanBabuska<_>>().total()`.
pub fn kahan_babuska_sum<T, I>(iter: I) -> T
where
    T: Float,
    I: IntoIterator<Item = T>,
{
    let mut s = T::zero();
    let mut c = T::zero();
    for x in iter {
        let y = x + c;
        let t = s + y;
        c = y - (t - s);
        s = t;
    }
    s + c
}

/// Alternative implementation of `Iterator::sum::<KahanBabuskaNeumaier<_>>().total()`.
pub fn kahan_babuska_neumaier_sum<T, I>(iter: I) -> T
where
    T: Float + AddAssign,
    I: IntoIterator<Item = T>,
{
    let mut s = T::zero();
    let mut c = T::zero();
    for x in iter {
        let (t, d) = two_sum(s, x);
        s = t;
        c += d;
    }
    s + c
}

/// Alternative implementation of `Iterator::sum::<KahanBabuskaNeumaier<_>>().total()`
/// using and absolute value comparison.
pub fn kahan_babuska_neumaier_abs_sum<T, I>(iter: I) -> T
where
    T: Float + AddAssign,
    I: IntoIterator<Item = T>,
{
    let mut s = T::zero();
    let mut c = T::zero();
    for x in iter {
        let t = s + x;
        if s.abs() >= x.abs() {
            // c += (s - t) + x;
            c += x - (t - s);
        } else {
            // c += (x - t) + s;
            c += s - (t - x);
        }
        s = t;
    }
    s + c
}

/// Alternative implementation of [`two_sum`] using an absolute value comparison.
pub fn abs_two_sum<T: Float>(a: T, b: T) -> (T, T) {
    let s = a + b;
    let t = if a.abs() >= b.abs() {
        b - (s - a)
    } else {
        a - (s - b)
    };
    (s, t)
}

/// Alternative implementation of `Iterator::sum::<KahanBabuskaNeumaier<_>>().total()`
/// using `abs_two_sum`.
pub fn kahan_babuska_neumaier_abs_two_sum<T, I>(iter: I) -> T
where
    T: Float + AddAssign,
    I: IntoIterator<Item = T>,
{
    let mut s = T::zero();
    let mut c = T::zero();
    for x in iter {
        let (t, d) = abs_two_sum(s, x);
        s = t;
        c += d;
    }
    s + c
}
