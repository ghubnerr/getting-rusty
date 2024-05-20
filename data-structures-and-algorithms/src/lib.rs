#![allow(unused, dead_code)]

mod bst;
mod dp;
mod encoding;
mod graphs;
mod hasher;
mod hashmaps;
mod linkedlists;
mod rand_gen;
mod sorting;
mod stacks;

pub use bst::*;
pub use encoding::*;
pub use graphs::*;
pub use hasher::*;
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
    fn test_linkedlist() {
        let mut ll = LinkedList::new();
        ll.push_front(3);
        ll.push_back(12);
        ll.push_front(1);

        println!("ll = {:?}", ll);
    }

    #[test]
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
    }

    #[test]
    fn test_bin_tree_balancing() {
        let mut t = BinTree::new();
        for i in 0..1e3 as i32 {
            t.add_sorted(i);
        }
        t.print_lfirst(0);
    }

    #[test]
    fn test_huffman_encoding() {
        let s = "at an apple app";
        println!("{}", s);
        let t = build_tree(s);
        t.print_lfirst(0, '<');
        println!("n = {:?}", t.encode_char('n'));
        println!("ec = {:?}", t.encode_str(s));
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_graph_map() -> Result<(), GraphErr> {
        let mut g = Graph::new();
        for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
            g.add_node(x, ());
        }
        g.add_edge('a', 'H', 'D', 6)?;
        g.add_edge('b', 'D', 'C', 18)?;
        g.add_edge('c', 'C', 'B', 10)?;
        g.add_edge('d', 'H', 'A', 7)?;
        g.add_edge('e', 'A', 'C', 4)?;
        g.add_edge('f', 'H', 'G', 5)?;
        g.add_edge('g', 'G', 'A', 8)?;
        g.add_edge('h', 'A', 'F', 3)?;
        g.add_edge('i', 'F', 'E', 15)?;
        g.add_edge('j', 'C', 'E', 12)?;

        println!("{:?}", g);

        println!("shortest path A-D = {:?},", g.shortest_path('A', 'D'));
        println!("shortest path H-B = {:?},", g.shortest_path('H', 'B'));
        Ok(())
    }

    #[test]
    fn test_greedy_salesman() -> Result<(), GraphErr> {
        let mut g = Graph::new();
        for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
            g.add_node(x, ());
        }
        g.add_edge('a', 'H', 'D', 6)?;
        g.add_edge('b', 'D', 'C', 18)?;
        g.add_edge('c', 'C', 'B', 10)?;
        g.add_edge('d', 'H', 'A', 7)?;
        g.add_edge('e', 'A', 'C', 4)?;
        g.add_edge('f', 'H', 'G', 5)?;
        g.add_edge('g', 'G', 'A', 8)?;
        g.add_edge('h', 'A', 'F', 3)?;
        g.add_edge('i', 'F', 'E', 15)?;
        g.add_edge('j', 'C', 'E', 12)?;

        println!("{:?}", g);
        println!("greedy salesman = {:?},", g.greedy_salesman('A'));
        Ok(())
    }

    #[test]
    fn test_get_right_values() {
        let mut hm = HMap::new();
        hm.insert("james".to_string(), 64);
        hm.insert("gabe".to_string(), 53);
        hm.insert("david".to_string(), 23);
        hm.insert("laura".to_string(), 234);
        hm.insert("danny".to_string(), 53);
        hm.insert("pete".to_string(), 12);
        hm.insert("andy".to_string(), 23);
        hm.insert("mridul".to_string(), 42);
        hm.insert("cole".to_string(), 11);
        hm.insert("sarah".to_string(), 22);
        hm.insert("abdul".to_string(), 33);
        hm.insert("cymone".to_string(), 44);
        hm.insert("geralt".to_string(), 77);
        hm.insert("hello".to_string(), 88);
        hm.insert("aaaa".to_string(), 99);

        // same
        hm.insert("david".to_string(), 23);

        assert_eq!(hm.get("gabe"), Some(&53));
        assert_eq!(hm.len(), 15);

        println!("hm = {:?}", hm);
    }

    fn test_lots_of_numbers() {
        let mut hm = HMap::new();
        for x in 0..10000 {
            hm.insert(x, x + 250);
        }
        assert_eq!(hm.len(), 10000);

        assert_eq!(hm.get(&500), Some(&750));
        for (n, x) in hm.main.buckets.iter().enumerate() {
            assert!(x.len() < 10, "main bucket too big");
        }

        for (n, x) in hm.grow.buckets.iter().enumerate() {
            assert!(x.len() < 10, "grow bucket too big");
        }
    }
}
