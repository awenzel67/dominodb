use deno_core::convert::SmallInt;
use deno_core::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Index;
use std::{fs, i64};
use lazy_static::lazy_static;
use std::sync::Mutex;
use deno_core::v8::{self, Object};
use deno_core::ToV8;

lazy_static! {
  static ref DB_CACHE_ROWS:Mutex<Vec<String>> = Mutex::new(Vec::new());
  static ref DB_CACHE_BUCKETS:Mutex<HashMap<String,Vec<serde_json::Value>>> = Mutex::new(HashMap::new());
  static ref DB_BUCKETS_SCHEMA:Mutex<serde_json::Value> = Mutex::new(serde_json::Value::Null);
}

pub fn do_test34(){
   let file_path="C:/NHKI/data/talktodata/500 KB_V2.json".to_string();
   let schema_path="C:/NHKI/data/talktodata/employeeSchemaDescription_V2.json".to_string();
   load_data_vector(&file_path,&schema_path);

   let bucks=buckets();
   for buck in bucks{
      println!("{}",buck);
      let nbucks=len_bucket(&buck);
      println!("{}",nbucks);
      println!("--------");
      for ind in 0..nbucks{
         let row=object_bucket(&buck, ind);
         if ind < 1{
          println!("{}",row);
         }
      }
   }

  let mut runtime=init_dominodb2();
  runtime.execute_script(
  "<usage>",
  r#"
// Print helper function, calling Deno.core.print()
function print(value) {
  Deno.core.print(value.toString()+"\n");
}

let buckets=Deno.core.ops.op_get_buckets();
print(buckets);
let nbucket=Deno.core.ops.op_get_bucket_count(buckets[0]);
print(nbucket);
let jentity=Deno.core.ops.op_get_object_from_bucket(buckets[0],10);
let se=JSON.stringify(jentity);
print(se);
let jschema=Deno.core.ops.op_get_schema();
let sch=JSON.stringify(jschema);
print(sch);
"#,
    )
    .unwrap();


let sc2=r#"
function generate_root_classes_from_json_schema(json_schema,get_entity_data_function_name)
{
    var ret={};
    if(json_schema["type"]!=='object'){
        return null;
    }
    for (const key in json_schema['properties']){
    
        let arraydesc=json_schema['properties'][key];
        if(arraydesc["type"]!=='array')
            return null;
        let entity_name="ED"+key;
        var source_lines=[];
        source_lines.push(`
 class ${entity_name} { 
   constructor(eindex)
    {
        this.eindex=eindex;
    }
`
        )
        let array_object_desc=arraydesc["items"]["properties"];
        //console.log(array_object_desc)
        for (const name_prop in array_object_desc){
            source_lines.push(
`
get ${name_prop}()
    {
        let jdata=${get_entity_data_function_name}("${key}",this.eindex);
        //let jdata=JSON.parse(sData);
        return jdata.${name_prop};
    } 
`);              
        }
         source_lines.push(`
};
         ${entity_name}
`
        )
        ret[key]=source_lines.join('\n');
    }
    return ret;
} 
"#;

runtime.execute_script("<compilerfunction>",sc2).unwrap();

let sc3=r#"
print("++++++++++++++++++++++++++++++++++++++++")

//print(JSON.stringify(sch));
print(typeof jschema);
let root_class_scripts=generate_root_classes_from_json_schema(jschema,"get_entity_data_function");
"#;
runtime.execute_script("<testcompiler>",sc3).unwrap();


let sc4=r#"
print("-----------------------------------")
function get_entity_data_function(rootBucket,index)
{
    return Deno.core.ops.op_get_object_from_bucket(rootBucket,index);
}
let ep0=get_entity_data_function("employees",0);
print(JSON.stringify(ep0.profile));
"#;
runtime.execute_script("<testquery>",sc4).unwrap();


let sc5=r#"
print("*******************************")

data={};
for (const property in root_class_scripts) {
    let ClassEntity=eval(root_class_scripts[property]);
    let nentities=Deno.core.ops.op_get_bucket_count(property);
    print(nentities)
    var employees8=[];
    for (var i=0;i<nentities;i++)
    {
       let ei=new ClassEntity(i);
       employees8.push(ei);
    }
    data[property]=employees8;
}
"#;
runtime.execute_script("<testindex>",sc5).unwrap();

let sc6=r#"
print("*******************************")

let olo=data.employees[0].profile.contact.email;
print(olo);
"#;
runtime.execute_script("<testindex>",sc6).unwrap();

}


pub fn do_test33(){

  DB_CACHE_ROWS.lock().unwrap().push(r#"
    {
      "id":"E0001",
      "name":"Test 1"
    }
"#.to_string()
  );
  DB_CACHE_ROWS.lock().unwrap().push(r#"
    {
      "id":"E0002",
      "name":"Test 2"
    }
"#.to_string()
  );

 let mut runtime=init_dominodb2();
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


Deno.core.ops.op_sum2(0);

"#,
    )
    .unwrap();
}

#[op2]
fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_error::JsErrorBox> {
  // Sum inputs
  let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, OpError>
  Ok(sum)
}

#[op2]
#[string]
fn op_sum2(irow:u32) -> Result<String, deno_error::JsErrorBox> {
  // Sum inputs
  //let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, OpError>
  let ih=irow as usize;
  let len=DB_CACHE_ROWS.lock().unwrap().len();
  if ih>=len{
    return Err(deno_error::JsErrorBox::type_error("XX"))
  }
  let stro=DB_CACHE_ROWS.lock().unwrap()[ih].clone();
  Ok(stro)
}

struct MJObject{
   mjobject: serde_json::Value
}

impl<'a> ToV8<'a> for MJObject {
  type Error = std::convert::Infallible;
  fn to_v8(
    self,
    scope: &mut v8::PinScope<'a, '_>,
  ) -> Result<v8::Local<'a, v8::Value>, Self::Error> {
    let zu=deno_core::serde_v8::to_v8(scope,self.mjobject).expect("msg");
    
      Ok(zu)
    
  }
}

#[op2]
pub fn op_get_object_from_bucket(#[string]bucket:String, index:i32) -> MJObject {
  let jdata=object_bucket(&bucket, index as usize);
  MJObject { mjobject: jdata }
}

#[op2]
pub fn op_get_buckets() -> Vec<String> {
  buckets()
}

#[op2(fast)]
pub fn op_get_bucket_count(#[string] bucket:String) -> u32 {
  len_bucket(&bucket) as u32
}

#[op2]
pub fn op_get_schema() -> MJObject {
  let jdata=DB_BUCKETS_SCHEMA.lock().unwrap().clone();
  MJObject { mjobject: jdata }
}


fn init_dominodb2() -> JsRuntime {
  // Build a deno_core::Extension providing custom ops
  const DECL: OpDecl = op_get_bucket_count();
  let ext = Extension {
    name: "my_bucket_count",
    ops: std::borrow::Cow::Borrowed(&[DECL]),
    ..Default::default()
  };

  const DECL2: OpDecl = op_get_buckets();
  let ext2 = Extension {
    name: "my_buckets",
    ops: std::borrow::Cow::Borrowed(&[DECL2]),
    ..Default::default()
  };

  const DECL3: OpDecl = op_get_object_from_bucket();
  let ext3 = Extension {
    name: "my_object_from_bucket",
    ops: std::borrow::Cow::Borrowed(&[DECL3]),
    ..Default::default()
  };

  const DECL4: OpDecl = op_get_schema();
  let ext4 = Extension {
    name: "my_schema",
    ops: std::borrow::Cow::Borrowed(&[DECL4]),
    ..Default::default()
  };

  // Initialize a runtime instance
  JsRuntime::new(RuntimeOptions {
    extensions: vec![ext,ext2,ext3,ext4],
    ..Default::default()
  })
}


pub fn load_data_vector(file_path: &String,schema_path: &String) {
  let file = File::open(file_path).expect("msg");
  let reader = BufReader::new(file);
  let value: serde_json::Value = serde_json::from_reader(reader).expect("msg");
  if(value.is_object())
  {
      let objo=value.as_object().unwrap();
      for key in objo.keys() {
           
            
            let oarray=objo.get(key);
            if let Some(jarray)=oarray {
               
               let marray = jarray.as_array();
               if let Some(m2array)=marray{
                let mut jva: Vec<serde_json::Value>=Vec::new();
                for element in m2array.into_iter(){
                  let ko: &serde_json::Value=element;
                  jva.push(ko.to_owned());
                }
                 DB_CACHE_BUCKETS.lock().unwrap().insert(key.to_string(),jva);
              }
            }
      }
  }
  let schema = File::open(schema_path).expect("msg");
  let reader_schema = BufReader::new(schema);
  let value_schema: serde_json::Value = serde_json::from_reader(reader_schema).expect("msg");
  {
    let db_schema=DB_BUCKETS_SCHEMA.lock();
    let mut schemav=db_schema.unwrap();
    *schemav=value_schema;
  }
}

pub fn buckets() -> Vec<String> {
  let mut mana:Vec<String>=Vec::new();
  for kv in DB_CACHE_BUCKETS.lock().unwrap().keys(){
     mana.push(kv.to_string());  
  }
  mana
}

pub fn len_bucket(bucket:&String) -> usize {
  let mut ij:usize=0;
  let db=DB_CACHE_BUCKETS.lock();
  let cfs=db.unwrap();
  let bu=cfs.get(bucket);
  if let Some(hu)=bu{
     ij=hu.len();
  }
  //for key in xyz{
  //  mana.push(key.to_string());
  //}
  ij
}

pub fn object_bucket(bucket:&String,i:usize) -> serde_json::Value {
  let db=DB_CACHE_BUCKETS.lock();
  let cfs = db.unwrap();
  let mut ij:usize=0;
  //let res:serde_json::Value=serde_json::Value::Null;
  let bu=cfs.get(bucket);
  if let Some(cu)=bu{
      let resa=cu.get(i);
      if let Some(resb)=resa{
           return resb.clone();    
      }
  }
  //for key in xyz{
  //  mana.push(key.to_string());
  //}
  serde_json::Value::Null
}