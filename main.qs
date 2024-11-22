fib = |n| map n {
  0 | 1 => n
  _ => fib(n-1) + fib(n-2)
}

fib(25)?
