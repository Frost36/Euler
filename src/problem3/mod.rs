
pub fn solve() -> u64 {
  largest_prime_factor(600851475143)
}

fn largest_prime_factor(x : u64) -> u64 {
  for y in 2..x {
    if x % y == 0 {
      return largest_prime_factor(x / y);
    }
  }
  x
}
