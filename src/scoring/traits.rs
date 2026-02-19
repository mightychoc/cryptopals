/// Defines an alphabet to use for scoring dependent on letter frequency.
/// The idea is to allow for a plug-and-play architecture where one
/// only needs to define the unigram (= single-letter) frequencies of the alphabet
/// in question in order to use [`super::letter_frequency::UnigramLogScorer`] and
/// [`super::letter_frequency::UnigramChiScorer`]. If one also supplies the bigram
/// (= letter-pair) frequencies, then the same applies for [`super::letter_frequency::BigramLogScorer`]
/// and [`super::letter_frequency::BigramChiScorer`].
pub trait Alphabet {
    const SIZE: usize;
    fn index(byte: u8) -> Option<usize>;

    fn unigram_probs() -> &'static [f64];
    fn bigram_probs() -> Option<&'static [f64]>;

    fn unigram_log_probs() -> &'static [f64];
    fn bigram_log_probs() -> Option<&'static [f64]>;
}

/// The basic trait for any scoring function. Must be implemented such that a
/// normalized score in the interval [0,1] is returned, where 1 means "very similar to
/// valid decrpyted text" and 0 means "almost certainly gibberish". Different [`Scorer`]
/// implementations can be combined using [`super::composite_scorer::CompositeScorer`].
pub trait Scorer {
    fn score(&self, input: &[u8]) -> f64;
}
