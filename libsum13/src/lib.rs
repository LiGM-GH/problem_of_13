mod combinatorics;
pub mod integer;
pub mod string;
pub mod traits;
mod either_iterator;
mod utils;
mod macros;

trait DigitSum {
    fn digits_sum(&self) -> u64;
}

struct DigitIter(u64, u64);

impl Iterator for DigitIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % self.1;
            self.0 /= self.1;
            Some(ret)
        }
    }
}

impl DigitSum for u64 {
    fn digits_sum(&self) -> u64 {
        DigitIter(*self, 10).sum()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, num::NonZeroU8};

    use crate::{
        integer, string, traits::SumSequencerOnce,
    };

    #[test]
    fn test_int_variant() {
        let iterations = 10000;
        let strval = string::WithDigitSum13 {}.get_ints(iterations);
        let intval = integer::WithDigitSum13 {}.get_ints(iterations);

        strval
            .zip(intval)
            .for_each(|(left, right)| assert_eq!(left, right));
    }

    #[test]
    fn test_int_general() {
        let spawn_thing = |number: NonZeroU8| {
            let iterations = 2 * number.get() as u32;

            println!("iterations: {iterations}");

            string::WithDigitSum(number)
                .get_ints(iterations)
                .zip(integer::WithDigitSum(number).get_ints(iterations))
                .enumerate()
                .for_each(|(i, (left, right))| {
                    assert_eq!(
                        left, right,
                        "{i} didn't match: (string) {left} != (int) {right}"
                    )
                });
        };

        for i in 1..9 {
            spawn_thing(NonZeroU8::new(i).unwrap());
        }
    }

    #[test]
    fn test_max_nonerroring_sum() {
        let mut should_panic = false;

        let mut test_range = |sum: NonZeroU8, iterations: u32| {
            fn fails_check(
                ints: impl Iterator<Item = u64>,
                sum: NonZeroU8,
                iterations: u32,
                label: &str,
            ) -> bool {
                let strs = string::WithDigitSum(sum).get_ints(iterations);
                let mut should_panic = false;

                let mut iter = ints.zip(strs).enumerate().peekable();
                while let Some((i, (ints, strs))) = iter.next() {
                    if let Some((_i_next, (ints_next, strs_next))) = iter.peek()
                        && ints_next != strs_next
                    {
                        println!("{i:>3} | {ints} | {strs}");
                    }
                    if ints != strs {
                        should_panic = true;
                        println!(
                            "{label} mismatched for sum {sum} on iteration {i}: got {ints:?}, expected {strs:?}"
                        );
                        break;
                    }
                }

                should_panic
            }

            if fails_check(
                integer::WithDigitSumAdvanced(sum).get_ints(iterations),
                sum,
                iterations,
                "advanced",
            ) {
                should_panic = true;
            }

            if fails_check(
                integer::WithDigitSum(sum).get_ints(iterations),
                sum,
                iterations,
                "standard",
            ) {
                should_panic = true;
            }

            if fails_check(
                integer::FullyPar(sum).get_ints(iterations),
                sum,
                iterations,
                "fully_par",
            ) {
                should_panic = true;
            }

            if fails_check(
                integer::FutureLooking(sum).get_ints(iterations),
                sum,
                iterations,
                "future_looking",
            ) {
                should_panic = true;
            }

            if fails_check(
                integer::SlowSequential(sum).get_ints(iterations),
                sum,
                iterations,
                "slow",
            ) {
                should_panic = true;
            }
        };

        test_range(NonZeroU8::new(1).unwrap(), 2);
        test_range(NonZeroU8::new(2).unwrap(), 5);
        test_range(NonZeroU8::new(3).unwrap(), 8);
        test_range(NonZeroU8::new(4).unwrap(), 20);
        test_range(NonZeroU8::new(5).unwrap(), 50);
        test_range(NonZeroU8::new(6).unwrap(), 100);
        test_range(NonZeroU8::new(7).unwrap(), 100);
        test_range(NonZeroU8::new(8).unwrap(), 1000);
        test_range(NonZeroU8::new(9).unwrap(), 1000);
        test_range(NonZeroU8::new(10).unwrap(), 1000);
        test_range(NonZeroU8::new(11).unwrap(), 1000);
        test_range(NonZeroU8::new(12).unwrap(), 1000);
        test_range(NonZeroU8::new(13).unwrap(), 1000);
        test_range(NonZeroU8::new(14).unwrap(), 1000);
        test_range(NonZeroU8::new(15).unwrap(), 1000);
        test_range(NonZeroU8::new(16).unwrap(), 1000);
        test_range(NonZeroU8::new(17).unwrap(), 1000);
        test_range(NonZeroU8::new(18).unwrap(), 1000);
        test_range(NonZeroU8::new(19).unwrap(), 1000);
        test_range(NonZeroU8::new(20).unwrap(), 1000);
        test_range(NonZeroU8::new(21).unwrap(), 1000);
        test_range(NonZeroU8::new(22).unwrap(), 1000);
        test_range(NonZeroU8::new(23).unwrap(), 1000);
        test_range(NonZeroU8::new(24).unwrap(), 1000);
        test_range(NonZeroU8::new(25).unwrap(), 1000);
        test_range(NonZeroU8::new(26).unwrap(), 1000);
        test_range(NonZeroU8::new(27).unwrap(), 1000);
        test_range(NonZeroU8::new(28).unwrap(), 1000);
        test_range(NonZeroU8::new(29).unwrap(), 1000);
        test_range(NonZeroU8::new(30).unwrap(), 1000);
        test_range(NonZeroU8::new(31).unwrap(), 1000);
        test_range(NonZeroU8::new(32).unwrap(), 1000);
        test_range(NonZeroU8::new(33).unwrap(), 1000);
        test_range(NonZeroU8::new(34).unwrap(), 1000);
        test_range(NonZeroU8::new(35).unwrap(), 1000);
        test_range(NonZeroU8::new(36).unwrap(), 1000);
        test_range(NonZeroU8::new(37).unwrap(), 1000);
        test_range(NonZeroU8::new(38).unwrap(), 1000);
        test_range(NonZeroU8::new(39).unwrap(), 1000);
        test_range(NonZeroU8::new(40).unwrap(), 1000);
        test_range(NonZeroU8::new(41).unwrap(), 1000);
        test_range(NonZeroU8::new(42).unwrap(), 1000);
        test_range(NonZeroU8::new(43).unwrap(), 1000);
        test_range(NonZeroU8::new(44).unwrap(), 1000);
        test_range(NonZeroU8::new(45).unwrap(), 1000);
        test_range(NonZeroU8::new(46).unwrap(), 1000);
        test_range(NonZeroU8::new(47).unwrap(), 1000);
        test_range(NonZeroU8::new(48).unwrap(), 1000);
        test_range(NonZeroU8::new(49).unwrap(), 1000);
        test_range(NonZeroU8::new(50).unwrap(), 1000);

        if should_panic {
            panic!()
        }
    }

    #[test]
    fn test_fully_par_with_zip() {
        let iterations = 100_000;
        let intval = integer::WithDigitSum::new(13).get_ints(iterations);

        let super_val = integer::FullyPar::new(13).get_ints(iterations);

        let mut iter = intval.zip(super_val).peekable();
        let mut need_panic = false;

        while let Some((left, right)) = iter.next() {
            if left == right
                && let Some((left_peek, right_peek)) = iter.peek()
                && left_peek != right_peek
            {
                println!("{left} | {right} | {left_peek} | {right_peek}");
                need_panic = true;
            }
        }

        if need_panic {
            panic!();
        }
    }

    #[test]
    fn test_fully_par_with_hashsets() {
        let iterations = 100_000;
        let intval = integer::WithDigitSum::new(13)
            .get_ints(iterations)
            .take_while(|val| *val < iterations as u64);

        let super_val = integer::FullyPar::new(13)
            .get_ints(iterations)
            .take_while(|val| *val < iterations as u64);

        let int_result = intval.collect::<HashSet<_>>();
        let super_result = super_val.collect::<HashSet<_>>();

        assert_eq!(
            super_result
                .difference(&int_result)
                .copied()
                .collect::<Vec<u64>>(),
            vec![],
        );

        let mut diff = int_result
            .difference(&super_result)
            .copied()
            .collect::<Vec<u64>>();

        diff.sort();

        assert_eq!(diff, vec![]);
    }

    #[allow(unused, dead_code, deprecated)]
    #[cfg(feature = "unstable_deprecated")]
    mod deprecated {
        use std::collections::HashSet;

        use rayon::iter::{ParallelBridge, ParallelExtend};

        use crate::{bench_it, integer, traits::SumSequencer};

        #[test]
        fn test_naive_par_against_integers_static() {
            let iterations = 10000;

            let cool = integer::NaivePar::new(13).get_ints(iterations);

            let intval = integer::WithDigitSum13 {}.get_ints(iterations);

            let mut cool_set = HashSet::new();
            cool_set.par_extend(cool.par_bridge());

            let mut int_set = HashSet::new();
            int_set.par_extend(intval.par_bridge());

            assert_eq!(int_set, cool_set);
        }

        #[test]
        fn test_naive_par_against_integers_standard() {
            let iterations = 100_000;

            let intval = integer::NaivePar::new(13).get_ints(iterations);
            let super_int = integer::WithDigitSum::new(13).get_ints(iterations);

            println!(
                "The last number of super integers is {:?}",
                bench_it(|| { intval.last() })
            );
            println!(
                "The last number of super integers is {:?}",
                bench_it(|| { super_int.last() })
            );
        }
    }
}
