
pub fn solve() -> u32 {
  sum_square_difference(100)
}

fn sum_square_difference(x: u32) -> u32 {
  let mut sum = 0;
  let mut sum_square = 0;
  for i in 1..(x+1) {
    sum += i;
    sum_square += i.pow(2);
  }
  let square_sum = sum.pow(2);
  square_sum - sum_square
}
