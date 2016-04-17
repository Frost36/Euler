
pub fn solve() -> u32 {
  sum_multiples_3_and_5(1000)
}

fn sum_multiples_3_and_5(n: u32) -> u32 {
  let mut total_sum = 0;
  let mut count3 = 0;
  let mut count5 = 0;

  while count3 < n || count5 < n {
    if count3 < count5 {
      total_sum += count3;
      count3 += 3;
    } else if count3 > count5 {
      total_sum += count5;
      count5 += 5;
    } else { // count3 == count5
      total_sum += count5;
      count3 += 3;
      count5 += 5;
    }
  }
  total_sum
}
