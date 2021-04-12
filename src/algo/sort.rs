// --------------------------------------------------------------------------

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
  for i in 1..arr.len() {
    let mut insert_index = i;

    while insert_index > 0 && arr[insert_index - 1] > arr[insert_index] {
      arr.swap(insert_index, insert_index - 1);
      insert_index -= 1;
    }
  }
}

// --------------------------------------------------------------------------

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
  for i in 0..arr.len() - 1 {
    for j in (i + 1..arr.len()).rev() {
      if arr[j] < arr[j - 1] {
        arr.swap(j, j - 1)
      }
    }
  }
}

// --------------------------------------------------------------------------

pub fn merge_sort<T>(parent_arr: &mut [T], low: usize, high: usize)
where
  T: Ord + Clone,
{
  let length = high - low + 1;
  if length > 1 {
    let mid = low + length / 2;
    merge_sort(parent_arr, low, mid - 1);
    merge_sort(parent_arr, mid, high);
    merge(parent_arr, low, mid, high);
  }
}

pub fn merge<T>(parent_arr: &mut [T], low: usize, mid: usize, high: usize)
where
  T: Ord + Clone,
{
  let left_arr = parent_arr[low..mid].to_owned();
  let right_arr = parent_arr[mid..high + 1].to_owned();

  let mut parent_index: usize = low;
  let mut left_index: usize = 0;
  let mut right_index: usize = 0;

  while left_index < left_arr.len() && right_index < right_arr.len() {
    if left_arr[left_index] <= right_arr[right_index] {
      parent_arr[parent_index] = left_arr[left_index].clone();
      left_index += 1;
    } else {
      parent_arr[parent_index] = right_arr[right_index].clone();
      right_index += 1;
    }
    parent_index += 1;
  }

  for i in left_index..left_arr.len() {
    parent_arr[parent_index] = left_arr[i].clone();
    parent_index += 1;
  }
  for i in right_index..right_arr.len() {
    parent_arr[parent_index] = right_arr[i].clone();
    parent_index += 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn bubble_sort_should_sort_elements() {
    let mut arr = [4, 2, 13, 6, 7, 12, 5, 49];

    bubble_sort(&mut arr);
    let expected = [2, 4, 5, 6, 7, 12, 13, 49];

    assert_eq!(arr, expected);
  }

  #[test]
  fn merge_should_work() {
    let mut arr = [1, 2, 3];

    merge(&mut arr, 0, 1, 1);
    let expected = [1, 2, 3];

    assert_eq!(arr, expected)
  }

  #[test]
  fn merge_sort_should_work() {
    let mut arr = [2, 1, 3, 6, 344, 22, 49, 123, 13, 66];
    let end = arr.len() - 1;

    merge_sort(&mut arr, 0, end);
    let expected = [1, 2, 3, 6, 13, 22, 49, 66, 123, 344];

    assert_eq!(arr, expected);
  }
}
