use crate::value_object::ValueObject;
use std::hash::Hash;

pub trait Identifier: ValueObject + Hash {}
