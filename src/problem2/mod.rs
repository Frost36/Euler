
pub fn solve() -> u32 {
  even_fibonacci_sum(1, 2, 4000000)
}

fn even_fibonacci_sum(f1 : u32, f2: u32, max: u32) -> u32 {
  if f1 >= max {
    0
  } else {
    even_fibonacci_sum(f2, f1 + f2, max) +
    if f1 % 2 == 0 {
      f1
    } else {
      0
    }
  }
}
