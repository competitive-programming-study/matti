#[macro_export]
macro_rules! test_case {
    ($func:expr, ($($arg:expr),*), $expected:expr) => {
        assert_eq!($func($($arg),*), $expected);
    };
}

#[allow(unused)]
pub struct TestCase<I, O> {
    input: I,
    output: O,
}

#[allow(unused)]
impl<I, O> TestCase<I, O>
where
    I: Clone,                       // for cloning input if needed
    O: PartialEq + std::fmt::Debug, // for assert_eq! and debugging output
{
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }

    pub fn test<F>(&self, fun: F)
    where
        F: Fn(I) -> O,
    {
        assert_eq!(fun(self.input.clone()), self.output);
    }

    /// Tests multiple functions against the expected output
    pub fn test_multiple<F>(&self, funcs: &[F])
    where
        F: Fn(I) -> O,
    {
        for (i, fun) in funcs.iter().enumerate() {
            let result = fun(self.input.clone());
            assert_eq!(
                result, self.output,
                "Function at index {} failed: got {:?}, expected {:?}",
                i, result, self.output
            );
        }
    }
}
