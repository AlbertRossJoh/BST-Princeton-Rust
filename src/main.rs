use bst::*;
fn main() {
    // test_ting();
    let mut huh:BST<i32, i32> = BST::new();
    for i in 5..10 {
        huh.put(i, i+10);
    }
    huh.put(2, 3);
    huh.put(1, 5);

    // if let Some(e) = huh.get(4) {
    //     println!("{}", e);
    // }

    // if let Some(e) = huh.get(0) {
    //     println!("{}", e);
    // }
    if let Some(e) = huh.delete_min() {
        println!("{}",e);
    }
    if let Some(e) = huh.get(1) {
        println!("{}", e);
    }
}
