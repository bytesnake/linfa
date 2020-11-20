use std::marker::PhantomData;
use linfa::{Float, Label};

/// The possible impurity measures for training.
#[derive(Clone, Copy, Debug)]
pub enum SplitQuality {
    Gini,
    Entropy,
}

/// The set of hyperparameters that can be specified for fitting a
/// [decision tree](struct.DecisionTree.html).
#[derive(Clone, Copy, Debug)]
pub struct DecisionTreeParams<F, L> {
    pub n_classes: usize,
    pub split_quality: SplitQuality,
    pub max_depth: Option<usize>,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub min_impurity_decrease: F,
    pub phantom: PhantomData<L>
}

impl<F: Float, L: Label> DecisionTreeParams<F, L> {
    pub fn n_classes(mut self, n_classes: usize) -> Self {
        self.n_classes = n_classes;
        self
    }

    pub fn split_quality(mut self, split_quality: SplitQuality) -> Self {
        self.split_quality = split_quality;
        self
    }

    pub fn max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn min_samples_split(mut self, min_samples_split: usize) -> Self {
        self.min_samples_split = min_samples_split;
        self
    }

    pub fn min_samples_leaf(mut self, min_samples_leaf: usize) -> Self {
        self.min_samples_leaf = min_samples_leaf;
        self
    }

    pub fn min_impurity_decrease(mut self, min_impurity_decrease: F) -> Self {
        self.min_impurity_decrease = min_impurity_decrease;
        self
    }
}
