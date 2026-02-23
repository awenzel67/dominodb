// Copyright 2018-2025 the Deno authors. MIT license.
//!  This example shows you how to define ops in Rust and then call them from
//!  JavaScript.

use deno_core::*;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
   static ref DB_CACHE:Mutex<String> = Mutex::new("".to_string());
}

/// An op for summing an array of numbers. The op-layer automatically
/// deserializes inputs and serializes the returned Result & value.
#[op2]
fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_error::JsErrorBox> {
  // Sum inputs
  let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, OpError>
  Ok(sum)
}

#[op2]
#[string]
fn op_sum2() -> Result<String, deno_error::JsErrorBox> {
  // Sum inputs
  //let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, OpError>
  let stro=DB_CACHE.lock().unwrap().to_string();
  Ok(stro)
}

pub fn load_data_var(runtime: &mut JsRuntime, file_path: &String ) {
      let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
      deno_core::scope!(handle_scope, runtime);

      let var_context=handle_scope.get_current_context();

      let global=var_context.global(handle_scope);
 
    let myobject_value=v8::String::new(handle_scope,&contents).unwrap();
    let myobject_name=v8::String::new(handle_scope,"data").unwrap();
   //let context=scope.get_current_context();
    global.set(handle_scope,myobject_name.into(),myobject_value.into());
}

pub fn init_dominodb() -> JsRuntime {
  // Build a deno_core::Extension providing custom ops
  const DECL: OpDecl = op_sum();
  let ext = Extension {
    name: "my_ext",
    ops: std::borrow::Cow::Borrowed(&[DECL]),
    ..Default::default()
  };

  const DECL2: OpDecl = op_sum2();
  let ext2 = Extension {
    name: "my_ext2",
    ops: std::borrow::Cow::Borrowed(&[DECL2]),
    ..Default::default()
  };
  // Initialize a runtime instance
  JsRuntime::new(RuntimeOptions {
    extensions: vec![ext,ext2],
    ..Default::default()
  })
}

pub fn test_op(runtime: &mut JsRuntime) {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.

  let data_js_start=r#"
// Print helper function, calling Deno.core.print()
function print(value) {
  Deno.core.print(value.toString()+"\n");
}

const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Deno.core.ops.op_sum2());

// And incorrect usage
try {
  print(Deno.core.ops.op_sum2());
} catch(e) {
  print('Exception:');
  print(e);
}
  "#;  

  runtime
    .execute_script(
      "<usage2>",
      data_js_start,
    )
    .unwrap();
}

pub fn load_data_op(runtime: &mut JsRuntime,file_path: &String) {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.

  let isInCache=DB_CACHE.lock().unwrap().len()>0;
  if !isInCache {
      let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
      DB_CACHE.lock().unwrap().push_str(&contents);
  }

  let mut data_js_start="let data=JSON.parse(Deno.core.ops.op_sum2());".to_owned();

  runtime
    .execute_script(
      "<usage>",
      data_js_start,
    )
    .unwrap();
}

pub fn reload_data_op(runtime: &mut JsRuntime) {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  let mut data_js_start="let data=JSON.parse(Deno.core.ops.op_sum2());".to_owned();

  runtime
    .execute_script(
      "<usage>",
      data_js_start,
    )
    .unwrap();
}



pub fn load_data(runtime: &mut JsRuntime, file_path: &String ) {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_js_start="let data=".to_owned();
    data_js_start.push_str(&contents);


  runtime
    .execute_script(
      "<usage>",
      data_js_start,
    )
    .unwrap();
}

pub fn query(runtime: &mut JsRuntime, code: String ) -> Result<String, String> {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
    let mut data_js_start="".to_string();
    data_js_start.push_str(&code);
    data_js_start.push_str("\n");
    data_js_start.push_str("result");
    let res = runtime.execute_script("<anon>",  data_js_start);
    match res {
    Ok(global) => {
      deno_core::scope!(scope, runtime);
      let local = v8::Local::new(scope, global);
      // Deserialize a `v8` object into a Rust type using `serde_v8`,
      // in this case deserialize to a JSON `Value`.
      let deserialized_value =serde_v8::from_v8::<serde_json::Value>(scope, local);
      match deserialized_value {
        Ok(value) => Ok(value.to_string()),
        Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
      }
    }
    Err(err) => {
      let me=err.message.expect("Error");
      Err(me)
    }
  }
}


pub fn run_script_inrt(runtime: &mut JsRuntime ) {
  // Build a deno_core::Extension providing custom ops
  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  runtime
    .execute_script(
      "<usage>",
      r#"
// Print helper function, calling Deno.core.print()
function print(value) {
  Deno.core.print(value.toString()+"\n");
}

const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Deno.core.ops.op_sum(arr));

// And incorrect usage
try {
  print(Deno.core.ops.op_sum(0));
} catch(e) {
  print('Exception:');
  print(e);
}
"#,
    )
    .unwrap();
}

pub fn run_script() {
  // Build a deno_core::Extension providing custom ops
  const DECL: OpDecl = op_sum();
  let ext = Extension {
    name: "my_ext",
    ops: std::borrow::Cow::Borrowed(&[DECL]),
    ..Default::default()
  };

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    ..Default::default()
  });

  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  runtime
    .execute_script(
      "<usage>",
      r#"
// Print helper function, calling Deno.core.print()
function print(value) {
  Deno.core.print(value.toString()+"\n");
}

const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Deno.core.ops.op_sum(arr));

// And incorrect usage
try {
  print(Deno.core.ops.op_sum(0));
} catch(e) {
  print('Exception:');
  print(e);
}
"#,
    )
    .unwrap();
}

fn eval(
  context: &mut JsRuntime,
  code: String,
) -> Result<serde_json::Value, String> {
  let res = context.execute_script("<anon>", code);
  match res {
    Ok(global) => {
      deno_core::scope!(scope, context);
      let local = v8::Local::new(scope, global);
      // Deserialize a `v8` object into a Rust type using `serde_v8`,
      // in this case deserialize to a JSON `Value`.
      let deserialized_value =
        serde_v8::from_v8::<serde_json::Value>(scope, local);

      match deserialized_value {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
      }
    }
    Err(err) => Err(format!("Evaling error: {err:?}")),
  }
}