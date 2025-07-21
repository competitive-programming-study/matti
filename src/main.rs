use code::optional::set15::woodcutters::woodcutters;

fn main() {
    let tree = [(1, 2), (2, 1), (5, 10), (10, 9), (20, 1)];
    assert_eq!(woodcutters(&tree), 4);
}
