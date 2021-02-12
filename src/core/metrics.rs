extern crate num;
use crate::core::node::FloatElement;

#[derive(Debug, Clone, Copy)]
pub enum MetricType {
    Unknown,
    Manhattan,
    Dot,
    Euclidean,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::Unknown
    }
}

fn same_dimension<T>(vec1: &[T], vec2: &[T]) -> Result<(), &'static str>
where
    T: FloatElement,
{
    if vec1.len() != vec2.len() {
        return Result::Err("different dimensions");
    }
    Result::Ok(())
}

pub fn metric<T>(vec1: &[T], vec2: &[T], mt: MetricType) -> Result<T, &'static str>
where
    T: FloatElement,
{
    let x = match mt {
        MetricType::Euclidean => euclidean_distance(vec1, vec2),
        MetricType::Manhattan => manhattan_distance(vec1, vec2),
        MetricType::Dot => dot(vec1, vec2),
        MetricType::Unknown => Result::Err("unknown method"),
    };
    return x;
}

// TODO: SIMD support
// TODO: make these func private
pub fn dot<T>(vec1: &[T], vec2: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    same_dimension(vec1, vec2)?;
    let mut res = T::default();
    for i in 0..vec1.len() {
        res += vec1[i] * vec2[i];
    }

    // in this situation ,dot is more big, more related, so use the negative value to transform the target.
    return Result::Ok(res);
}

pub fn manhattan_distance<T>(vec1: &[T], vec2: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    same_dimension(vec1, vec2)?;
    let mut res = T::default();
    for i in 0..vec1.len() {
        res += vec1[i].abs() - vec2[i].abs();
    }
    return Result::Ok(res.abs());
}

pub fn euclidean_distance<T>(vec1: &[T], vec2: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    same_dimension(vec1, vec2)?;
    let mut res = T::default();
    for i in 0..vec1.len() {
        let diff = vec1[i] - vec2[i];
        res += diff * diff;
    }
    return Result::Ok(res.sqrt());
}

pub fn euclidean_distance_range<T>(
    vec1: &[T],
    vec2: &[T],
    begin: usize,
    end: usize,
) -> Result<T, &'static str>
where
    T: FloatElement,
{
    same_dimension(vec1, vec2)?;
    let mut res = T::default();
    for i in begin..end {
        let diff = vec1[i] - vec2[i];
        res += diff * diff;
    }
    return Result::Ok(res.sqrt());
}

pub fn get_norm<T>(vec1: &[T]) -> T
where
    T: FloatElement,
{
    match dot(&vec1, &vec1) {
        Ok(x) => x.sqrt(),
        _ => T::default(),
    }
}