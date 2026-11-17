use crate::adapter::Adapter;
use crate::port::Port;

pub trait DrivenPort: Port + Adapter {}
