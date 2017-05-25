
/// Wraps a function into a box to be compatible with pipeline
pub fn p0<R>(f: fn(&R) -> R)
             -> Box<Fn(&R) -> R> where
    R: 'static
{
    Box::new(move |r_input| {
        f(r_input)
    })
}


/// Partial Application
pub fn p1<T1, R>(f: fn(T1, &R) -> R, x: T1)
                 -> Box<Fn(&R) -> R> where
    T1: 'static + Copy,
    R: 'static
{
    Box::new(move |r_input| {
        f(x, r_input)
    })
}


pub fn pipeline<T>(start_value: T, funcs: &[Box<Fn(&T) -> T>]) -> T {
    funcs.iter().fold(start_value, |current_value, f| f(&current_value))
}
