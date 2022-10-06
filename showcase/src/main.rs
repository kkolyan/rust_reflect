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
    let struct_a: Struct = A::create_meta();

    // create object dynamically
    let a1 = struct_a.builder()
        .field("x", 42)
        .field("y", 7.2f32)
        .new_instance()
        .unwrap().downcast::<A>().unwrap();
    assert_eq!(a1.x, 42);
    assert_eq!(a1.y, 7.2);

    let mut a2 = A { x: 17, y: 3.1 };

    let x = struct_a.fields.get("x").unwrap();
    let y = struct_a.fields.get("y").unwrap();

    // read dynamically
    assert_eq!(17, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());
    assert_eq!(3.1, *y.get_ref(&a2).unwrap().downcast_ref::<f32>().unwrap());

    // write dynamically
    x.set(&mut a2, 12).unwrap();
    y.set(&mut a2, 0.6f32).unwrap();
    assert_eq!(12, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());
    assert_eq!(0.6, *y.get_ref(&a2).unwrap().downcast_ref::<f32>().unwrap());

    // introspect struct description
    let mut fields_desc = struct_a.fields.iter().map(|(_, field)|
        format!("{}: {}", field.name, field.type_name)
    )
        .collect::<Vec<_>>();
    fields_desc.sort();
    let type_desc = format!("{} {{ {} }}", struct_a.name, fields_desc
        .join(", "));

    assert_eq!("A { x: i32, y: f32 }", type_desc);

    println!("OK");
}