// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;
use std::thread::JoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {

    if v.len() == 0 {
        return 0;
    }

    if v.len() == 1 {
        return v[0];
    }

    let mid = v.len()/2;

    let (front,back) = v.split_at(mid);
    let front = front.to_vec();
    let back = back.to_vec();

    let handle_1 = thread::spawn(move || front.iter().sum::<i32>());
    let handle_2 = thread::spawn(move || back.iter().sum::<i32>());

    let sum_front = handle_1.join().unwrap();
    let sum_back = handle_2.join().unwrap();

    sum_front + sum_back
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
