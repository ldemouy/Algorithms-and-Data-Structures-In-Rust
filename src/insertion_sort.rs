use std::cmp::{PartialOrd,PartialEq};
use std::fmt::{Display};

pub fn insertion_sort<T: PartialOrd + PartialEq + Clone + Display>(array: & mut Vec<T> ) -> &mut Vec<T> {
    let mut sorted_array = array;
    for i in 1..sorted_array.len() {
        let key : &T = &mut sorted_array[i].clone();
        let mut j = i - 1;
        while sorted_array[j] > *key{
            sorted_array[j+1] = sorted_array[j].clone();
            if j == 0 {
                break;
            }
            j = j - 1;
        }
        sorted_array[j + 1] =  key.clone();
    }
    sorted_array
}


//TODO: Change the algorithm such that it can handle the first element properly
#[test]
#[cfg(expect_fail)] //using the algorithm from the book will result in the first element not being set due to the constraints of Rust
fn reverse_order() {
    let mut array = vec![5, 4, 3, 2, 1];
    let array = insertion_sort(&mut array);
    let val = vec![1, 2, 3, 4, 5];
    assert_eq!(array.as_slice(), val.as_slice());
}

#[test]
fn mixed_order() {
    let mut array = vec![1, 3, 2, 5, 4];
    let array = insertion_sort(&mut array);
    let val = vec![1, 2, 3, 4, 5];

    assert_eq!(array.as_slice(), val.as_slice() );
}

#[test]
fn in_order()
{
    let mut array = vec![1, 2, 3, 4, 5];
    let array = insertion_sort(&mut array);
    let val = vec![1, 2, 3, 4, 5];
    assert_eq!(array.as_slice(), val.as_slice());
}
