# Problem of 13
> Let's count all the numbers that are in range of 0..N and also which digits sum up to 13 exactly.

That's how the problem is stated.
Right now the benches are promising.
Obviously, there is at least one solution that involves using String.
Other solutions are only using integers. Some are counting digit sum each time, others do not.

Most rely on iterations number instead of N.
So the task statement can be changed to this:

> Let's count first K numbers for which digits sum up to 13 exactly.

There is also plan to add combinatorical solution, but it's unimplemented as of today.

The bench results look like this (iterations number is 1000000):

| Name                   | Resulting value | Execution time |
| ----                   | ---------       | --------:      |
| integer_static         | 30611101000     | 4607.9751 ms   |
| integer_dynamic        | 30611101000     | 4770.9358 ms   |
| integer_advanced       | 30611101000     | 318.4746 ms    |
| future_looking         | 30611101000     | 3676.6814 ms   |
| fully_par full`*`      | 30611101000     | 598.4634 ms    |
| fully_par iters`**`    | 30611101000     | 165.4490 ms    |
| fully_par preproc`***` | 30611101000     | 390.4602 ms    |
| slow_sequential        | 30611101000     | 24200.0586 ms  |

- `*` - fully_par where the end number is unknown
- `**` - fully_par where the end number is known
- `***` - fully_par where the only time measured is the time of finding the end number
