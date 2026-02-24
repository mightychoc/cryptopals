pub trait Scorer {
    fn score(&self, input: &[u8]) -> f64;
}

impl<A: Scorer, B: Scorer> std::ops::Add<WeightedScorer<B>> for WeightedScorer<A> {
    type Output = CompositeScorer;
    fn add(self, rhs: WeightedScorer<B>) -> CompositeScorer { 
        CompositeScorer::new()
        .add_weighted(self.scorer, self.weight)
        .add_weighted(rhs.scorer, rhs.weight)
     }
}

pub trait ScorerExtension : Scorer + Sized {
    fn weighted(self, weight: f64) -> WeightedScorer<Self> {
        WeightedScorer {scorer: self, weight}
    }

    fn default_weight<S: Scorer + 'static>(self, other: S) -> CompositeScorer
    where Self: 'static
    {
        CompositeScorer::new()
            .add(self, 1.0)
            .add(other, 1.0)
    }
}

impl<T: Scorer> ScorerExtension for T {}