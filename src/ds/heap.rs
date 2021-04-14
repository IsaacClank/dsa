pub struct MaxHeap {
  arr: Vec<i32>,
  size: usize,
}

impl std::fmt::Display for MaxHeap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.arr)
  }
}

impl MaxHeap {
  pub fn new(tree: &[i32]) -> MaxHeap {
    let arr = tree.to_owned();
    let size = arr.len();
    let mut heap = MaxHeap { arr, size };

    for i in (0..heap.size / 2).rev() {
      heap.heapify(i);
    }

    heap
  }

  pub fn heapify(&mut self, i: usize) {
    let mut largest = i;

    loop {
      let parent = largest;
      let left = self.left(parent);
      let right = self.right(parent);

      if let Some(value) = self.key_at(left) {
        if value > self.key_at(largest).unwrap() {
          largest = left;
        }
      }

      if let Some(value) = self.key_at(right) {
        if value > self.key_at(largest).unwrap() {
          largest = right;
        }
      }

      if largest != parent {
        self.arr.swap(parent, largest);
      } else {
        break;
      }
    }
  }

  pub fn left(&self, i: usize) -> usize {
    2 * i + 1
  }

  pub fn right(&self, i: usize) -> usize {
    2 * i + 2
  }

  pub fn key_at(&self, i: usize) -> Option<i32> {
    if i < self.size {
      Some(self.arr[i])
    } else {
      None
    }
  }

  pub fn sort(&mut self) -> &[i32] {
    while self.size > 1 {
      self.extract_root();
    }

    &self.arr
  }

  pub fn extract_root(&mut self) -> i32 {
    let max = self.key_at(0).unwrap();

    self.arr.swap(0, self.size - 1);
    self.size -= 1;
    self.heapify(0);

    max
  }

  pub fn increase_key(&mut self, i: usize, key: i32) {
    if key < self.arr[i] {
      eprintln!("New key is smaller than current key");
      std::process::exit(1);
    }

    self.arr[i] = key;
    let mut curr_key = i;

    while curr_key > 0 {
      let parent = self.parent(curr_key);

      if self.arr[curr_key] > self.arr[parent] {
        self.heapify(parent);
        curr_key = parent;
      } else {
        break;
      }
    }
  }

  pub fn parent(&self, i: usize) -> usize {
    (i - 1) / 2
  }

  pub fn insert(&mut self, key: i32) {
    self.increment_size();
    self.increase_key(self.size - 1, key);
  }

  pub fn increment_size(&mut self) {
    if self.size == self.arr.len() {
      self.arr.push(i32::MIN);
    }
    self.size += 1;
  }
}

// i_th_child = parent_pos * 2 + i (1<=i<=d)

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_string_test() {
    let heap = MaxHeap::new(&vec![3, 2, 1]);

    let result = heap.to_string();
    let expected = String::from("[3, 2, 1]");

    assert_eq!(result, expected);
  }

  #[test]
  fn build_heap_test() {
    let arr = vec![1, 2, 3, 5, 6];

    let heap = MaxHeap::new(&arr);
    let expected = MaxHeap::new(&vec![6, 5, 3, 1, 2]).to_string();

    assert_eq!(heap.to_string(), expected);
  }

  #[test]
  fn heap_sort_test() {
    let arr = vec![6, 5, 3, 1, 2];
    let mut heap = MaxHeap::new(&arr);

    let arr = heap.sort();
    let expected = vec![1, 2, 3, 5, 6];

    assert_eq!(arr, expected);
  }

  #[test]
  fn extract_max_test() {
    let arr = vec![6, 6, 3, 1, 2];
    let mut heap = MaxHeap::new(&arr);

    let max = heap.extract_root();
    let expected = 6;

    assert_eq!(max, expected);
  }

  #[test]
  fn increase_key_at_test() {
    let mut heap = MaxHeap::new(&vec![6, 5, 4, 3, 2, 1]);

    heap.increase_key(4, 10);
    let expected = MaxHeap::new(&vec![10, 6, 4, 3, 5, 1]);

    assert_eq!(heap.to_string(), expected.to_string());
  }

  #[test]
  fn heap_insert_test() {
    let mut heap = MaxHeap::new(&vec![6, 5, 4, 3, 2, 1]);

    heap.insert(10);
    let expected = MaxHeap::new(&vec![10, 5, 6, 3, 2, 1, 4]);

    assert_eq!(heap.to_string(), expected.to_string());
  }
}

// 2  3  5   8
// 4  9  12  14
// 16 +  +   +
// +  +  +   +
