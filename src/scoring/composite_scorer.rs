use super::traits::Scorer;

pub struct WeightedScorer {
    scorer: Box<dyn Scorer>,
    weight: f64,
}

// Compose multiple Scorers to one "big" scorer
pub struct CompositeScorer {
    scorers: Vec<WeightedScorer>,
}

impl CompositeScorer {
    pub fn new() -> Self {
        Self {
            scorers: Vec::new(),
        }
    }

    // The static lifetime is here required because we put the scorer
    // into a Box. Heap memory lives independently of local scopes, thus
    // the scorer cannot borrow anything from the stack memory (= 'static)
    pub fn add<S: Scorer + 'static>(mut self, scorer: S, weight: f64) -> Self {
        self.scorers.push(WeightedScorer {
            scorer: Box::new(scorer),
            weight,
        });
        self
    }
}

impl Scorer for CompositeScorer {
    fn score(&self, input: &[u8]) -> f64 {
        self.scorers
            .iter()
            .map(|ws| ws.scorer.score(input) * ws.weight)
            .sum()
    }
}
