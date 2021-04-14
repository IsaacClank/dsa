use std::collections::HashMap;

// -------------------- Add two binary numbers of n bits --------------------

pub fn add_binary(l: &[bool], r: &[bool]) -> Vec<bool> {
  let mut result = vec![false; l.len() + 1];

  for i in (0..l.len()).rev() {
    let carry_in = result[i + 1];

    let mut bit_result = false;
    let mut carry_out = false;
    let mut on_count: u8 = 0;

    for bit in [carry_in, l[i], r[i]].iter() {
      if *bit {
        on_count += 1
      }
    }

    if on_count >= 2 {
      carry_out = true;
    }
    if on_count == 1 || on_count == 3 {
      bit_result = true;
    }

    result[i + 1] = bit_result;
    result[i] = carry_out;
  }

  result
}

// ------- https://www.hackerrank.com/challenges/sock-merchant/problem ------

pub fn sock_merchant(ar: &[u32]) -> u32 {
  let mut sock_hash: HashMap<&u32, u32> = HashMap::new();
  let mut pair_count = 0u32;

  ar.iter().for_each(|color| {
    if sock_hash.contains_key(color) {
      if let Some(_) = sock_hash.get_mut(color) {
        pair_count += 1;
        sock_hash.remove(color);
      }
    } else {
      sock_hash.insert(color, 1);
    }
  });

  return pair_count;
}

// -------------------------- Find maximum subarray -------------------------

pub fn find_max_subarray_linear_time(arr: &[i32]) -> (i32, usize, usize) {
  let mut max_value = arr[0];
  let mut low = 0;
  let mut high = 1;

  let mut chain_sum = arr[0];
  let mut chain_low = 0;
  let mut chain_high = 0;

  for (i, value) in arr.iter().enumerate() {
    chain_high += 1;
    if i == 0 {
      continue;
    }

    if chain_sum <= 0 {
      chain_sum = *value;
      chain_low = i;
    } else {
      chain_sum += value
    }

    println!("\n{} - {}", i, value);
    println!("chain: {} - {} - {}", chain_low, chain_high, chain_sum);
    println!("max: {} - {} - {}", low, high, max_value);

    if chain_sum >= max_value {
      max_value = chain_sum;
      low = chain_low;
      high = chain_high;
    }
  }

  (max_value, low, high)
}

// --- https://www.hackerrank.com/challenges/jumping-on-the-clouds/problem --

pub fn jumping_on_clouds(arr: &[bool]) -> u16 {
  let mut min_jumps = 0;
  let mut position = 0usize;

  while position != arr.len() - 1 {
    if let Some(true) = arr.get(position + 2) {
      position += 2;
    } else {
      position += 1;
    }

    min_jumps += 1;
  }

  min_jumps
}

// ----- https://www.hackerrank.com/challenges/counting-valleys/problem -----

pub fn counting_valeys(path: &str) -> u32 {
  let mut count = 0u32;
  let mut level = 0;

  for step in path.chars() {
    if step == 'u' {
      level += 1;
      if level == 0 {
        count += 1
      }
    } else {
      level -= 1
    }
  }

  count
}

// ------ https://www.hackerrank.com/challenges/repeated-string/problem -----

pub fn repeated_string(s: &str, n: usize) -> usize {
  if s.len() < 1 {
    return 0;
  }

  let offset = n % s.len();
  let repetition = (n - offset) / s.len();
  let mut count = 0;

  for c in s.chars() {
    if c == 'a' {
      count += 1
    }
  }

  count * repetition + repeated_string(&s[0..offset], s[0..offset].len())
}

// --------- https://www.hackerrank.com/challenges/2d-array/problem ---------

pub fn hourglass_sum(arr: &[Vec<i32>], i: usize, j: usize, width: usize, height: usize) -> isize {
  if width == 1 && height == 1 {
    return find_hourglass_with_center(&arr, i, j);
  }

  let mut _max_1 = isize::MIN;
  let mut _max_2 = isize::MIN;

  if width > height {
    _max_1 = hourglass_sum(&arr, i, j, width / 2, height);
    _max_2 = hourglass_sum(&arr, i, j + width / 2, width / 2, height);
  } else {
    _max_1 = hourglass_sum(&arr, i, j, width, height / 2);
    _max_2 = hourglass_sum(&arr, i + height / 2, j, width, height / 2);
  }

  if _max_1 > _max_2 {
    return _max_1;
  } else {
    return _max_2;
  }
}

pub fn find_hourglass_with_center(arr: &[Vec<i32>], i: usize, j: usize) -> isize {
  return (arr[i - 1][j - 1]
    + arr[i - 1][j]
    + arr[i - 1][j + 1]
    + arr[i][j]
    + arr[i + 1][j - 1]
    + arr[i + 1][j]
    + arr[i + 1][j + 1]) as isize;
}

// ------------- https://leetcode.com/problems/contiguous-array/ ------------

pub fn contiguous_array(arr: &[bool]) -> usize {
  let mut max_length = 0;
  let mut state_to_index: HashMap<i32, usize> = HashMap::new();
  let mut count = 0;

  for (index, value) in arr.iter().enumerate() {
    count += if *value { 1 } else { -1 };

    if let Some(past_index) = state_to_index.get(&count) {
      max_length = std::cmp::max(max_length, index - past_index);
    } else {
      state_to_index.insert(count, index);
    }
  }

  max_length
}

// --------- https://leetcode.com/problems/valid-parenthesis-string/ --------

pub fn valid_parenthesis_string(s: &str) -> bool {
  let mut value = 0;
  let mut asterick_count = 0;
  let mut asterick_used = 0;

  for char in s.chars() {
    match char {
      '(' => value += 1,
      ')' => {
        value -= 1;
      }
      _ => {
        asterick_count += 1;
      }
    }

    if asterick_used < asterick_count {
      asterick_used += 1;
      if value < 0 {
        value += 1;
      } else if value > 0 {
        value -= 1;
      }
    }
  }

  return value == 0;
}

// ------ https://www.hackerrank.com/challenges/minimum-swaps-2/problem -----

pub fn minimum_swap(arr: &mut [i32]) -> u32 {
  let mut count = 0;

  for i in 0..arr.len() {
    let curr_val = arr[i];
    let intended_value = i as i32 + 1;

    if curr_val == intended_value {
      continue;
    }

    for j in i + 1..arr.len() {
      if arr[j] == intended_value {
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        count += 1;
        break;
      }
    }
  }

  return count;
}

// ----------------- https://leetcode.com/problems/two-sum/ -----------------

pub fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
  let mut result = vec![];
  let mut map: HashMap<i32, usize> = HashMap::new();

  // let nums = vec![2, 7, 11, 15];
  for (curr_index, num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(complement_index) = map.get(&num) {
      result = vec![*complement_index, curr_index];
      break;
    } else {
      map.insert(complement, curr_index);
    }
  }

  result
}

// ----------- https://www.hackerrank.com/challenges/crush/problem ----------

pub fn array_manipulation(length: usize, operations: &[[u32; 3]]) -> u32 {
  let mut max_value = 0;
  let mut arr = vec![0i32; length];

  for operation in operations {
    let start = operation[0] as usize - 1;
    let end = operation[1] as usize;
    let value = operation[2];

    arr[start] += value as i32;
    if end < arr.len() {
      arr[end] -= value as i32;
    }
  }

  arr.iter().fold(0, |peak, value| {
    max_value = std::cmp::max(max_value, peak + value);
    peak + value
  });

  max_value as u32
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sock_merchant_test() {
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let pair_count = sock_merchant(&ar);
    let expected = 3u32;

    assert_eq!(pair_count, expected);
  }

  #[test]
  fn jumping_on_cloud_test() {
    let arr = vec![true, true, true, true, false, true];

    let min_jumps = jumping_on_clouds(&arr);
    let expected = 3u16;

    assert_eq!(min_jumps, expected);
  }

  #[test]
  fn counting_valley_test() {
    let path = "uddduduu";

    let count = counting_valeys(&path);
    let expected = 1u32;

    assert_eq!(count, expected)
  }

  #[test]
  fn repeated_string_test() {
    let s = "epsxyyflvrrrxzvnoenvpegvuonodjoxfwdmcvwctmekpsnamchznsoxaklzjgrqruyzavshfbmuhdwwmpbkwcuomqhiyvuztwvq";
    let n = 549382313570;

    let count = repeated_string(&s, n);
    let expected = 16481469408;

    assert_eq!(count, expected);
  }

  #[test]
  fn find_max_subarray_test() {
    let arr = vec![
      13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let max_subarray = find_max_subarray_linear_time(&arr);
    let expected = (43, 7, 11);

    assert_eq!(max_subarray, expected);
  }

  #[test]
  fn hourglass_sum_test() {
    let arr = [
      vec![-9, -9, -9, 1, 1, 1],
      vec![0, -9, 0, 4, 3, 2],
      vec![-9, -9, -9, 1, 2, 3],
      vec![0, 0, 8, 6, 6, 0],
      vec![0, 0, 0, -2, 0, 0],
      vec![0, 0, 1, 2, 4, 0],
    ];

    let result = hourglass_sum(&arr, 1, 1, 4, 4);
    let expected = 28;

    assert_eq!(result, expected)
  }

  #[test]
  fn contiguous_array_test() {
    let arr = vec![true, true, true, false, false];

    let result = contiguous_array(&arr);
    let expected = 4;

    assert_eq!(result, expected);
  }

  #[test]
  fn valid_parenthesis_string_test() {
    let s = "(*)";

    let result = valid_parenthesis_string(s);
    let expected = true;

    assert_eq!(result, expected)
  }

  #[test]
  fn tow_sum_test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(&nums, target);
    let expected = vec![0, 1];

    assert_eq!(result, expected);
  }

  #[test]
  fn array_manipulation_test() {
    let n = 5;
    let operations = vec![[1, 2, 100], [2, 5, 100], [3, 4, 100]];

    let result = array_manipulation(n, &operations);
    let expected = 200;

    assert_eq!(result, expected);
  }
}
