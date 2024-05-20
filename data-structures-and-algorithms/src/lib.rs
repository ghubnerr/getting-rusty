mod bst;
mod dp;
mod graphs;
mod hashmaps;
mod linkedlists;
mod rand_gen;
mod sorting;
mod stacks;

pub use bst::*;
pub use graphs::*;
pub use hashmaps::*;
pub use linkedlists::*;
pub use sorting::*;
pub use stacks::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 5, 7, 473, 312];
        bubble_sort(&mut v);
        assert_eq!(v, vec![4, 5, 7, 312, 473]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 5, 7, 473, 312];
        let v = merge_sort(v);
        assert_eq!(v, vec![4, 5, 7, 312, 473]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 5, 7, 473, 312];
        quick_sort(&mut v);
        assert_eq!(v, vec![4, 5, 7, 312, 473]);
    }

    #[test]
    fn test_quick_sort_random() {
        let mut v = vec![4, 5, 7, 473, 312];
        quick_sort_random(&mut v);
        assert_eq!(v, vec![4, 5, 7, 312, 473]);
    }

    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![4, 5, 7, 473, 312];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![4, 5, 7, 312, 473]);
    }

    #[test]
    #[should_panic]
    fn test_linkedlist() {
        let mut ll = LinkedList::new();
        ll.push_front(3);
        ll.push_back(12);
        ll.push_front(1);

        println!("ll = {:?}", ll);
        panic!();
    }

    #[test]
    #[should_panic]
    fn test_bin_tree() {
        let mut t = BinTree::new();
        t.add_sorted(4);
        t.add_sorted(44);
        t.add_sorted(24);
        t.add_sorted(6);
        t.add_sorted(5);
        t.add_sorted(47);
        t.add_sorted(24);
        t.print_lfirst(0);
        panic!();
    }

    #[test]
    #[should_panic]
    fn test_bin_tree_balancing() {
        let mut t = BinTree::new();
        for i in 0..1e5 as i32 {
            t.add_sorted(i);
        }
        t.print_lfirst(0);
        panic!();
    }
}
