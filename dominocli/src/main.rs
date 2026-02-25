#[path = "./klo.rs"]
mod klo;
use dominodb::*;
use std::fs;

use crate::klo::do_test34;

fn main() {
  // let mut runtime=init_dominodb();
   //test_op(&mut runtime);
  //main1();
  //do_test34();
  main2();
}

fn main2() {
   let file_path="C:/NHKI/data/talktodata/500 KB_V2.json".to_string();
   let schema_path="C:/NHKI/data/talktodata/employeeSchemaDescription_V2.json".to_string();
   load_data(&file_path,&schema_path);
   // run_script();
   let mut runtime=init_dominodb();
   let file_path_script= "C:/NHKI/domy/query.txt".to_string();

   let code=fs::read_to_string(file_path_script).expect("Should have been able to read the file");
   //println!("{0}",code);
   let code2="var result = data.employees.map(x=>x.profile.contact.email);".to_string();
   let res=query(&mut runtime, code).expect("Error");
   println!("{0}",res);
}