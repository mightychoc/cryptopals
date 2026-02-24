pub struct WeightedScorer<S: Scorer> {
    scorer: S,
    weight: f64,
}

pub struct CompositeScorer {
    scorers: Vec<(Box<dyn Scorer>, f64)>,
}

impl CompositeScorer {
    pub fn new() -> Self {
        Self {
            scorers: Vec::new(),
        }
    }
    pub fn add_weighted<S: Scorer + 'static>(mut self, scorer: S, weight: f64) -> Self {
        self.scorers.push((Box::new(scorer), weight));
        self
    }
}

impl Scorer for CompositeScorer {
    fn score(&self, input: &[u8]) -> f64 {
        let total_weight: f64 = self.scorers.iter().map(|(_, w)| w).sum();
        self.scorers
            .iter()
            .map(|(s, w)| s.score(input) * w)
            .sum::<f64>()
            / total_weight.max(1.0)
    }
}
