#![allow(unused_imports)]
use code::data_structs::tree::TreeNode;
use code::set4::max_path::max_path;
use code::test_util::TestCase;
use code::{tree, tree_leaf, tree_left, tree_right};

type TestC<'a> = TestCase<&'a TreeNode<i32>, Option<i32>>;

#[test]
fn test_one_node() -> () {
    let t = tree_leaf!(10);
    TestC::new(&t, None).test(max_path);
}

#[test]
fn test_only_one() -> () {
    let t = tree!(1, tree_leaf!(2), tree_leaf!(3));
    TestC::new(&t, Some(6)).test(max_path);
}

#[test]
fn test_generic_1() -> () {
    let t = tree!(3, tree!(4, tree_leaf!(-10), tree_leaf!(4)), tree_leaf!(5));
    TestC::new(&t, Some(16)).test(max_path);
}

#[test]
fn test_generic_2() -> () {
    let t = tree!(
        -15,
        tree!(5, tree!(-8, tree_leaf!(2), tree_leaf!(-3)), tree_leaf!(1)),
        tree!(
            6,
            tree_leaf!(3),
            tree_right!(9, tree!(0, tree_leaf!(4), tree_left!(-1, tree_leaf!(10))))
        )
    );
    println!("{}", t.to_string());
    TestC::new(&t, Some(27)).test(max_path);
}
#[test]
fn test_generic_3() -> () {
    let t = tree!(3, tree!(4, tree_leaf!(-10), tree_leaf!(4)), tree_leaf!(1));
    TestC::new(&t, Some(12)).test(max_path);
}
#[test]
fn test_leaf_to_root() -> () {
    let t = tree_right!(1, tree_leaf!(2));
    TestC::new(&t, Some(3)).test(max_path);
}
