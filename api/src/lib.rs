use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Reflected {
    fn create_meta() -> Struct;
}

#[derive(Debug)]
pub struct ConstructorError {
    pub field_errors: Vec<ConstructorFieldError>,
}

#[derive(Debug)]
pub struct ConstructorFieldError {
    pub field: &'static str,
    pub resolution: ConstructorFieldErrorResolution,
}

#[derive(Debug)]
pub enum ConstructorFieldErrorResolution {
    MissingField,
    UnknownField,
    InvalidType,
}

pub type Constructor = fn(HashMap<&'static str, Box<dyn Any>>) -> Result<Box<dyn Any>, ConstructorError>;

pub struct Struct {
    pub name: &'static str,
    pub type_id: TypeId,
    pub fields: HashMap<&'static str, Field>,
    pub constructor: Constructor,
}

#[derive(Debug)]
pub enum GetError {
    InvalidTarget,
}

#[derive(Debug)]
pub enum SetError {
    InvalidTarget,
    InvalidValueType,
}

pub type GetRef = fn(&dyn Any) -> Result<&dyn Any, GetError>;
pub type Set = fn(&mut dyn Any, Box<dyn Any>) -> Result<(), SetError>;

pub struct Field {
    pub name: &'static str,
    pub type_id: TypeId,
    pub type_name: &'static str,
    pub get_ref_delegate: GetRef,
    pub set_delegate: Set,
}

impl Field {
    pub fn get_ref<'a>(&self, t: &'a dyn Any) -> Result<&'a dyn Any, GetError> {
        (self.get_ref_delegate)(t)
    }
    pub fn set<'a, T: 'static>(&self, t: &'a mut dyn Any, value: T) -> Result<(), SetError> {
        (self.set_delegate)(t, Box::from(value) as Box<dyn Any>)
    }
}

pub struct StructBuilder {
    constructor: Constructor,
    values: HashMap<&'static str, Box<dyn Any>>,
}

impl StructBuilder {
    pub fn field<T: 'static>(mut self, name: &'static str, value: T) -> Self {
        self.values.insert(name, Box::from(value) as Box<dyn Any>);
        self
    }

    pub fn new_instance(self) -> Result<Box<dyn Any>, ConstructorError> {
        (self.constructor)(self.values)
    }
}

impl Struct {
    pub fn builder(&self) -> StructBuilder {
        StructBuilder { constructor: self.constructor, values: Default::default() }
    }
}
