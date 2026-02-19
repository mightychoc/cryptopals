use super::traits::{Alphabet, Scorer};
use std::sync::OnceLock;

pub struct LogLikelihood<A: Alphabet> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> LogLikelihood<A> {
    pub fn get_unigrams() -> &'static [f64] {
        static CACHE: OnceLock<Vec<f64>> = OnceLock::new();
        CACHE.get_or_init(|| A::unigram_probs().iter().map(|&p| -p.ln()).collect())
    }
    pub fn get_bigrams() -> Option<&'static [f64]> {
        static CACHE: OnceLock<Option<Vec<f64>>> = OnceLock::new();
        let opt_vec = CACHE
            .get_or_init(|| A::bigram_probs().map(|table| table.iter().map(|&p| p.ln()).collect()));
        opt_vec.as_ref().map(|v| v.as_slice())
    }
}

// Negative log-likelihood for unigrams
pub struct UnigramLogScorer<A: Alphabet> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> Scorer for UnigramLogScorer<A> {
    fn score(&self, input: &[u8]) -> f64 {
        let logs = LogLikelihood::<A>::get_unigrams();

        let (sum, letter_count) = input.iter().fold((0.0, 0), |(acc_sum, acc_count), &b| {
            if let Some(idx) = A::index(b) {
                (acc_sum + logs[idx], acc_count + 1)
            } else {
                (acc_sum, acc_count)
            }
        });

        if letter_count == 0 {
            0.0
        } else {
            // Normalize log-likelihood
            // We can naturally bound the value to be between 0 and 1
            // taking the geometric mean probability
            (sum / letter_count as f64).exp()
        }
    }
}

// Chi-squared scorer for unigrams
pub struct UnigramChiScorer<A: Alphabet> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> Scorer for UnigramChiScorer<A> {
    fn score(&self, input: &[u8]) -> f64 {
        let mut observed = vec![0usize; <A>::SIZE];
        let mut letter_count = 0;

        input.iter().for_each(|&b| {
            if let Some(idx) = <A>::index(b) {
                observed[idx] += 1;
                letter_count += 1;
            }
        });

        if letter_count == 0 {
            return 0.0;
        }

        let n = letter_count as f64;

        let chi: f64 = (0..A::SIZE)
            .map(|i| {
                let expected = <A>::unigram_probs()[i] * n;
                if expected > 0.0 {
                    let diff = observed[i] as f64 - expected;
                    diff * diff / expected
                } else {
                    0.0
                }
            })
            .sum();

        // Chi-squared measures "distance" of the
        // observed from the expected distribution.
        // Hence, we can map this to [0,1] by using
        // an exponential decay. A perfect match (chi = 0)
        // then gets mapped to 1 and a big chi near 0.
        (-chi / n).exp()
    }
}

// Negative log-likelihood for bigrams
pub struct BigramLogScorer<A: Alphabet> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> Scorer for BigramLogScorer<A> {
    fn score(&self, input: &[u8]) -> f64 {
        let Some(logs) = LogLikelihood::<A>::get_bigrams() else {
            return 0.0;
        };

        let mut sum = 0.0;
        let mut count = 0;
        let mut last_idx = None;

        for &b in input {
            if let Some(curr) = A::index(b) {
                if let Some(prev) = last_idx {
                    // Row-major access
                    sum += logs[prev * A::SIZE + curr];
                    count += 1;
                }
                last_idx = Some(curr);
            } else {
                last_idx = None;
            }
        }
        // Normalize log-likelihood
        // We can naturally bound the value to be between 0 and 1
        // taking the geometric mean probability
        if count == 0 {
            0.0
        } else {
            (sum / count as f64).exp()
        }
    }
}

// Chi-squared scorer for bigrams
pub struct BigramChiScorer<A: Alphabet> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> Scorer for BigramChiScorer<A> {
    fn score(&self, input: &[u8]) -> f64 {
        let Some(probs) = A::bigram_probs() else {
            return 0.0;
        };

        let mut observed = vec![0usize; A::SIZE * A::SIZE];
        let mut bigram_count = 0;
        let mut last_idx = None;
        for &b in input {
            if let Some(curr) = A::index(b) {
                if let Some(prev) = last_idx {
                    observed[prev * A::SIZE + curr] += 1;
                    bigram_count += 1;
                }
                last_idx = Some(curr);
            } else {
                last_idx = None;
            }
        }

        if bigram_count == 0 {
            return 0.0;
        }

        let n = bigram_count as f64;
        let chi: f64 = (0..(A::SIZE * A::SIZE))
            .map(|i| {
                let expected = probs[i] * n;
                if expected > 0.0 {
                    let diff = observed[i] as f64 - expected;
                    diff * diff / expected
                } else {
                    0.0
                }
            })
            .sum();

        // Chi-squared measures "distance" of the
        // observed from the expected distribution.
        // Hence, we can map this to [0,1] by using
        // an exponential decay. A perfect match (chi = 0)
        // then gets mapped to 1 and a big chi near 0.
        (-chi / n).exp()
    }
}
