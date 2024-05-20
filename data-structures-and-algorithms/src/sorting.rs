use std::fmt::Debug;

// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    // comparison operator PartialOrd
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort left half,
    // sort the right half, O(n * log (n))
    // bring the sorted halfs together O(n)

    println!("MS:{:?}", v);
    if v.len() <= 1 {
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

fn pivot_random<T: PartialOrd>(v: &mut [T]) -> usize {
    if v.len() == 0 {
        return 0;
    }

    let mut p = crate::rand_gen::rand(v.len());
    v.swap(0, p);
    p = 0;

    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p); // guarantees to split it without overlapping
    quick_sort(a);
    quick_sort(&mut b[1..]); // middle already sorted
}

pub fn quick_sort_random<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot_random(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p); // guarantees to split it without overlapping
    quick_sort(a);
    quick_sort(&mut b[1..]); // middle already sorted
}

// // Deprecated!
// struct RawSend<T>(*mut [T]); // one element tuple

// unsafe impl<T> Send for RawSend<T> {}

// fn threadded_quick_sort<T: 'static + Debug + PartialOrd + Send>(v: &mut [T]) {
//     if v.len() <= 1 {
//         return;
//     }

//     let p = pivot_random(v);
//     println!("{:?}", v);

//     let (a, b) = v.split_at_mut(p);

//     let raw_a: *mut [T] = a as *mut [T];
//     let raw_s = RawSend(raw_a);

//     unsafe {
//         let handle = std::thread::spawn(move || {
//             threadded_quick_sort(&mut *raw_s.0);
//         });

//         threadded_quick_sort(b); // Sort the other half directly without slicing
//         handle.join().ok();
//     }
// }

// Thread pool handler for Quick Sort
pub fn quick_sort_rayon<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    // The rayon::join function ensures that both sorts are completed before quick_sort_rayon returns.
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot() {
        let mut v = vec![1, 3, 4, 19, 8, 11, 13, 6];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }

        // assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13])
    }
}
