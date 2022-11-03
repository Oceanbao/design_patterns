/*
Visitor allows adding new behaviours to existing class hierarchy sans altering any existing code.

Deserialization

serde is Visitor pattern
- Visitor should be implemented for a Deserializable type.
- Visitor is passed to a Deserializer (Element) which accepts and drives
Visitor in order to construct a desired type.
*/

mod visitor {
    use crate::{TwoValuesArray, TwoValuesStruct};

    /// Visitor can visit one type, do conversions, and output another type.
    ///
    /// It's not like all visitors must return a new type, it's just an example
    /// that demonstrates the technique.
    pub trait Visitor {
        type Value;

        /// Visits a vector of integers and outputs a desired type.
        fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
    }

    /// Visitor implementation for a struct of two values.
    impl Visitor for TwoValuesStruct {
        type Value = TwoValuesStruct;

        fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
            TwoValuesStruct { a: v[0], b: v[1] }
        }
    }

    /// Visitor implementation for a struct of values array.
    impl Visitor for TwoValuesArray {
        type Value = TwoValuesArray;

        fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
            let mut ab = [0i32; 2];

            ab[0] = v[0];
            ab[1] = v[1];

            TwoValuesArray { ab }
        }
    }
}

use visitor::Visitor;

/// A struct of two integer values.
///
/// It's going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`.
#[derive(Default, Debug)]
pub struct TwoValuesStruct {
    a: i32,
    b: i32,
}

/// A struct of values array.
///
/// It's going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`.
#[derive(Default, Debug)]
pub struct TwoValuesArray {
    ab: [i32; 2],
}

/// `Deserializer` trait defines methods that can parse either a string or
/// a vector, it accepts a visitor which knows how to construct a new object
/// of a desired type (in our case, `TwoValuesArray` and `TwoValuesStruct`).
trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;
    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str is unimplemented")
    }
    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}

struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        // In this case, in order to apply a visitor, a deserializer should do
        // some preparation. The visitor does its stuff, but it doesn't do everything.
        let input_vec = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

struct VecDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}

fn main() {
    let deserializer = StringDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}
