use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::HashMap;
use rust_reflect_api::{ConstructorError, ConstructorFieldError, ConstructorFieldErrorResolution, Field, GetError, ReflectedStruct, SetError, Struct};

#[derive(rust_reflect::MyDerive)]
struct A {
    x: i32,
}

thread_local! {

}

impl ReflectedStruct for A {
    fn create_meta() -> Struct {
        Struct {
            name: "showcase.A",
            type_id: TypeId::of::<A>().type_id(),
            fields: HashMap::from([
                Field {
                    name: "x",
                    type_id: TypeId::of::<i32>().type_id(),
                    get_ref_delegate: |instance| match instance.downcast_ref::<Self>() {
                        None => Err(GetError::InvalidTarget),
                        Some(instance) => Ok(&instance.x)
                    },
                    set_delegate: |instance, value| {
                        match instance.downcast_mut::<Self>() {
                            None => Err(SetError::InvalidTarget),
                            Some(instance) => {
                                match value.downcast_ref::<i32>() {
                                    None => Err(SetError::InvalidValue),
                                    Some(value) => {
                                        instance.x = *value;
                                        Ok(())
                                    }
                                }
                            }
                        }
                    },
                }
            ].map(|it| (it.name, it))),
            constructor: |mut values| {
                let mut field_errors = vec![];
                let x = match values.remove("x") {
                    None => {
                        field_errors.push(ConstructorFieldError {
                            field: "x",
                            resolution: ConstructorFieldErrorResolution::MissingField,
                        });
                        None
                    }
                    Some(value) => {
                        match value.downcast::<i32>() {
                            Ok(value) => {
                                Some(*value)
                            }
                            Err(_) => None
                        }
                    }
                };
                if field_errors.is_empty() {
                    Ok(Box::from(A {
                        x: x.unwrap()
                    }))
                } else {
                    Err(ConstructorError { field_errors })
                }
            },
        }
    }
}

fn main() {
    let meta = A::create_meta();
    let meta: &Struct = meta.borrow();
    let result = meta.builder()
        .field("x", 42)
        .new_instance()
        .unwrap();
    let a1 = *result.downcast::<A>().ok().unwrap();
    assert_eq!(a1.x, 42);

    let mut a2 = A { x: 17 };

    let x = meta.fields.get("x").unwrap();

    assert_eq!(17, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());

    x.set(&mut a2, 12).unwrap();
    assert_eq!(12, *x.get_ref(&a2).unwrap().downcast_ref::<i32>().unwrap());

    println!("OK");
}