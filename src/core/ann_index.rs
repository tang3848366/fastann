use crate::core::metrics;
use crate::core::node;

pub trait AnnIndex<E: node::FloatElement, T: node::IdxType> {
    fn construct(&self); // construct algorithm structure
    fn add(&mut self, item: &node::Node<E, T>);
    fn once_constructed(&self) -> bool; // has already been constructed?
    fn reconstruct(&mut self);
    fn search_node(
        &self,
        item: &node::Node<E, T>,
        k: usize,
        mt: metrics::MetricType,
    ) -> Vec<(node::Node<E, T>, E)>;

    fn search(&self, item: &[E], k: usize, mt: metrics::MetricType) -> Vec<(node::Node<E, T>, E)>;

    fn load(&self, path: &str) -> Result<(), &'static str>;

    fn dump(&self, path: &str) -> Result<(), &'static str>;
}
