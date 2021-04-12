pub struct Heap {
  tree: Vec<i32>,
}

impl std::fmt::Display for Heap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.tree)
  }
}

impl Heap {
  pub fn new(tree: &[i32]) -> Heap {
    let tree = tree.to_owned();
    Heap { tree }
  }

  pub fn parent(&self, i: usize) -> usize { i / 2 }
  pub fn left(&self, i: usize) -> usize { 2 * i + 1 }
  pub fn right(&self, i: usize) -> usize { 2 * i + 2 }
  pub fn heap_size(&self) -> usize { self.tree.len() }
  pub fn get(&self, i: usize) -> i32 { self.tree[i] }

  pub fn exchange(&mut self, src: usize, dest: usize) { self.tree.swap(src, dest) }

  pub fn max_heapify(&mut self, i: usize) {
    let mut largest = i;

    loop {
      let l = self.left(largest);
      let r = self.right(largest);

      if l < self.heap_size() && self.get(l) > self.get(largest) {
        self.exchange(l, largest);
        largest = l;
      } else if r < self.heap_size() && self.get(r) > self.get(largest) {
        self.exchange(r, largest);
        largest = r;
      } else {
        break;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_string_test() {
    let heap = Heap::new(&vec![3, 2, 1]);

    let result = heap.to_string();
    let expected = String::from("[3, 2, 1]");

    assert_eq!(result, expected);
  }

  #[test]
  fn max_heapify_test() {
    let mut heap = Heap::new(&vec![2, 1, 3]);

    heap.max_heapify(0);
    let result = heap.to_string();
    let expected = String::from("[3, 1, 2]");

    assert_eq!(result, expected);
  }
}
