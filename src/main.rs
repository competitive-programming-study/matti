use code::data_structs::segment_tree::SegmentTree;

fn main() {
    let s = SegmentTree::build(&[5,1,4,3,2]);
    println!("{:?}",s.tree);
}


