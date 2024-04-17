mod qsort;
mod bst;
use bst::BST;

fn main() {
    let arr = vec![2, 6, 3, -8, 10];
    let mut bst = BST::default();
    for e in arr {
        bst.insert(e);
    }

    print!("{bst}");
    let x = 6;
    println!("Removing: {}", x);
    println!("{bst}, sucsess: {}", bst.try_delete(x));
}
