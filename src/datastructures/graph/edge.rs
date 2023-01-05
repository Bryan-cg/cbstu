use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Default, Debug, Clone)]
pub struct Edge {
    either: usize,
    other: usize,
    weight: f64,
    upgraded_weight: f64,
    cost: f64,
    tmp_either: usize,
    temp_other: usize,
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
            tmp_either: either,
            temp_other: other,
            upgraded: false,
        }
    }
    pub fn endpoints(&self) -> (usize, usize) {
        (self.either, self.other)
    }
    pub fn original_endpoints(&self) -> (usize, usize) {
        (self.tmp_either, self.temp_other)
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
    pub fn set_original_endpoints(mut self, either: usize, other: usize) -> Edge {
        self.tmp_either = either;
        self.temp_other = other;
        self
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.either == other.either
            && self.other == other.other
            && self.weight == other.weight
            && self.upgraded_weight == other.upgraded_weight
            && self.cost == other.cost
    }
}
impl Eq for Edge {}

//todo: check if hasing is correct
impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.either.hash(state);
        self.other.hash(state);
        self.weight.to_bits().hash(state);
        self.upgraded_weight.to_bits().hash(state);
        self.cost.to_bits().hash(state);
    }
}

#[test]
fn builder_test() {
    let edge: Edge = Edge {
        either: 0,
        other: 1,
        weight: 10.0,
        upgraded_weight: 20.0,
        cost: 30.0,
        tmp_either: 2,
        temp_other: 3,
        upgraded: false,
    };
    let edge_from_builder: Edge = Edge::new(0,1)
        .weight(10.0)
        .upgraded_weight(20.0)
        .cost(30.0)
        .set_original_endpoints(2, 3);
    assert_eq!(edge, edge_from_builder);
}