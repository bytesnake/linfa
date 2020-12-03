use linfa::{Float, Label};
use std::marker::PhantomData;

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
    pub min_weight_split: f32,
    pub min_weight_leaf: f32,
    pub min_impurity_decrease: F,
    pub phantom: PhantomData<L>,
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

    pub fn min_weight_split(mut self, min_weight_split: f32) -> Self {
        self.min_weight_split = min_weight_split;
        self
    }

    pub fn min_weight_leaf(mut self, min_weight_leaf: f32) -> Self {
        self.min_weight_leaf = min_weight_leaf;
        self
    }

    pub fn min_impurity_decrease(mut self, min_impurity_decrease: F) -> Self {
        self.min_impurity_decrease = min_impurity_decrease;
        self
    }
}
