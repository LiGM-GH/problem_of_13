# Problem of 13

> Let's count all the numbers that are in range of 0..N and also which digits sum up to 13 exactly.

That's how the problem is stated.


Right now the benches are promising.
Obviously, there is at least one solution that involves using String.
Other solutions are only using integers. Some are counting digit sum each time, others do not.

Most solutions are generic over the digit sum number, meaning, they probably work for any M - sum of digits.
Most rely on iterations number instead of N.

> Tested for M in `1..50`

So the task statement can be changed to this:

> Let's count first K numbers for which digits sum up to M exactly.

There is also plan to add combinatorical solution, but it's unimplemented as of today.

The bench results look like this (K = 1_000_000, M = 13):

| Name                | Resulting value | Execution time |
| ----                | ---------       | --------:      |
| integer_static      | 30611101000     | 4623.3618 ms   |
| integer_dynamic     | 30611101000     | 4583.9194 ms   |
| integer_advanced    | 30611101000     | 356.1967 ms    |
| future_looking      | 30611101000     | 357.9142 ms    |
| fully_par full      | 30611101000     | 456.1541 ms    |
| fully_par (iters)   | 30611101000     | 112.3055 ms    |
| fully_par (preproc) | 30611101000     | 346.3530 ms    |
| slow_sequential     | 30611101000     | 23720.9696 ms  |

> Benches are not performed on a public server, just locally.
> This is not perfectly fair (meaning you'll probably get different execution times).
> Though it's probably enough for this type of task.

# Contributing

There are some methods that are hidden behind feature flag `unstable_deprecated` (mostly integers.rs).
That's for a reason they suddenly didn't pass some of the tests.

Any contribution on that topic is welcome.

There is also combinatorics.rs which has a struct that should implement SumSequencer, but it doesn't as of today.

Any contribution on that topic is welcome as well.
