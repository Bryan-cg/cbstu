use std::cmp::Ordering;

#[derive(Default, Debug, Clone)]
pub struct Edge {
    either: usize,
    other: usize,
    weight: f64,
    upgraded_weight: f64,
    cost: f64,
    original_either: usize,
    original_other: usize,
    upgraded: bool,
}

impl Edge {
    pub fn new(either: usize, other: usize) -> Edge {
        Edge {
            either,
            other,
            weight: 0.0,
            upgraded_weight: 0.0,
            cost: 0.0,
            original_either: either,
            original_other: other,
            upgraded: false,
        }
    }
    pub fn endpoints(&self) -> (usize, usize) {
        (self.either, self.other)
    }
    pub fn original_nodes(&self) -> (usize, usize) {
        (self.original_either, self.original_other)
    }
    pub fn weight(mut self, weight: f64) -> Edge {
        self.weight = weight;
        self
    }
    pub fn upgraded(mut self, upgraded: bool) -> Edge {
        self.upgraded = upgraded;
        self
    }
    pub fn is_upgraded(&self) -> bool {
        self.upgraded
    }
    pub fn upgraded_weight(mut self, upgraded_weight: f64) -> Edge {
        self.upgraded_weight = upgraded_weight;
        self
    }
    pub fn cost(mut self, cost: f64) -> Edge {
        self.cost = cost;
        self
    }
    pub fn get_weight(&self) -> f64 {
        self.weight
    }
    pub fn get_upgraded_weight(&self) -> f64 {
        self.upgraded_weight
    }
    pub fn get_cost(&self) -> f64 {
        self.cost
    }
    pub fn original_either(mut self, original_either: usize) -> Edge {
        self.original_either = original_either;
        self
    }
    pub fn original_other(mut self, original_other: usize) -> Edge {
        self.original_other = original_other;
        self
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.either == other.either
            && self.other == other.other
            && self.weight == other.weight
    }
}
impl Eq for Edge {}

#[test]
fn builder_test() {
    let edge: Edge = Edge {
        either: 0,
        other: 1,
        weight: 10.0,
        upgraded_weight: 20.0,
        cost: 30.0,
        original_either: 2,
        original_other: 3,
        upgraded: false,
    };
    let edge_from_builder: Edge = Edge::new(0,1)
        .weight(10.0)
        .upgraded_weight(20.0)
        .cost(30.0)
        .original_either(2)
        .original_other(3);
    assert_eq!(edge, edge_from_builder);
}