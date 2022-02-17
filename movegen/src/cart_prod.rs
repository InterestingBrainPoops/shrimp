use board::small::SNAKE_MAX;
use tinyvec::{array_vec, ArrayVec};

use crate::makeunmake::Move;

// excerpted from : https://gist.github.com/kylewlacy/115965b40e02a3325558
/// Given a vector containing a partial Cartesian product, and a list of items,
/// return a vector adding the list of items to the partial Cartesian product.
///
/// # Example
///
/// ```
/// let partial_product = vec![vec![1, 4], vec![1, 5], vec![2, 4], vec![2, 5]];
/// let items = &[6, 7];
/// let next_product = partial_cartesian(partial_product, items);
/// assert_eq!(next_product, vec![vec![1, 4, 6],
///                               vec![1, 4, 7],
///                               vec![1, 5, 6],
///                               vec![1, 5, 7],
///                               vec![2, 4, 6],
///                               vec![2, 4, 7],
///                               vec![2, 5, 6],
///                               vec![2, 5, 7]]);
/// ```
pub fn partial_cartesian(
    a: ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; (4 as usize).pow(SNAKE_MAX as u32)]>,
    b: tinyvec::ArrayVec<[Move; 4]>,
) -> ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; (4 as usize).pow(SNAKE_MAX as u32)]> {
    a.into_iter()
        .flat_map(|xs| {
            b.iter()
                .cloned()
                .map(|y| {
                    let mut vec = xs;
                    vec.push(y);
                    vec
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Computes the Cartesian product of lists[0] * lists[1] * ... * lists[n].
///
/// # Example
///
/// ```
/// let lists: &[&[_]] = &[&[1, 2], &[4, 5], &[6, 7]];
/// let product = cartesian_product(lists);
/// assert_eq!(product, vec![vec![1, 4, 6],
///                          vec![1, 4, 7],
///                          vec![1, 5, 6],
///                          vec![1, 5, 7],
///                          vec![2, 4, 6],
///                          vec![2, 4, 7],
///                          vec![2, 5, 6],
///                          vec![2, 5, 7]]);
/// ```
pub fn cartesian_product(
    lists: ArrayVec<[ArrayVec<[Move; 4]>; SNAKE_MAX]>,
) -> ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; (4 as usize).pow(SNAKE_MAX as u32)]> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; (4 as usize).pow(SNAKE_MAX as u32)]> =
                first
                    .iter()
                    .cloned()
                    .map(|n| array_vec!([Move; SNAKE_MAX] => n))
                    .collect();

            rest.iter().cloned().fold(init, partial_cartesian)
        }
        None => {
            array_vec!()
        }
    }
}
