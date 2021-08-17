use wasmer::{Store, Module, Instance, Value, imports, Val, Function};
use std::ops::Deref;

fn main() -> anyhow::Result<()> {
    let store = Store::default();
    let connector = std::fs::read("../connector/target/wasm32-unknown-unknown/release/connector.wasm")?;
    let module = Module::new(&store, connector)?;

    let import_object = imports! {
        "env" => {
            "daje" => Function::new_native(&store, daje),
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    let sum = instance.exports.get_function("sum")?;
    let counter = instance.exports.get_function("counter")?;

    let result = sum.call(&[Value::I32(41), Value::I32(1)])?;
    if let &[Val::I32(num)] = result.deref() {
        println!("Result: {}", num);
    } else {
        panic!("wtf");
    }

    let count1 = counter.call(&[])?;
    if let &[Val::I32(num)] = count1.deref() {
        println!("Counter: {}", num); // 1
    }

    let count2 = counter.call(&[])?;
    if let &[Val::I32(num)] = count2.deref() {
        println!("Counter: {}", num); // 2
    }

    let instance2 = Instance::new(&module.clone(), &import_object)?;
    let counter2 = instance2.exports.get_function("counter")?;
    let count3 = counter2.call(&[])?;
    if let &[Val::I32(num)] = count3.deref() {
        println!("Counter (instance 2): {}", num); // 1
    }

    Ok(())
}

pub fn daje(num: i32) {
    println!("Daje {}", num);
}
