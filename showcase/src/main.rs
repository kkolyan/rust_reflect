use std::any::{Any, TypeId};
use std::collections::HashMap;
use rust_reflect::Reflected;
use rust_reflect_api::{ConstructorError, ConstructorFieldError, ConstructorFieldErrorResolution, Field, GetError, Reflected, SetError, Struct};

#[derive(Reflected)]
pub struct A {
    pub x: i32,
    pub y: f32,
}

fn main() {
    let meta = A::create_meta();
    let result = meta.builder()
        .field("x", 42)
        .field("y", 7.2f32)
        .new_instance()
        .unwrap();
    let a1 = *result.downcast::<A>().ok().unwrap();
    assert_eq!(a1.x, 42);

    let mut a2 = A { x: 17, y: 3.1 };

    let x = meta.fields.get("x").unwrap();
    let y = meta.fields.get("y").unwrap();

    assert_eq!(17, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());
    assert_eq!(3.1, *y.get_ref(&a2).unwrap().downcast_ref::<f32>().unwrap());

    x.set(&mut a2, 12).unwrap();
    y.set(&mut a2, 0.6f32).unwrap();
    assert_eq!(12, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());
    assert_eq!(0.6, *y.get_ref(&a2).unwrap().downcast_ref::<f32>().unwrap());

    println!("OK");
}