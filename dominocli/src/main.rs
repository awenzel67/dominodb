#[path = "./klo.rs"]
mod klo;
use dominodb::*;
use std::fs;

use crate::klo::do_test34;

fn main() {
  // let mut runtime=init_dominodb();
   //test_op(&mut runtime);
  //main1();
  do_test34();
}

fn main1() {

   // run_script();
   let mut runtime=init_dominodb();
   let sto:String="C:/NHKI/domy/data/500_KB_V2.json".to_string();
   load_data_op(&mut runtime, &sto);
   let file_path_script= "C:/NHKI/domy/query.txt".to_string();
   let code=fs::read_to_string(file_path_script).expect("Should have been able to read the file");
   let res=query(&mut runtime, code).expect("Error");
   println!("{0}",res);
   let mut runtime=init_dominodb();
   reload_data_op(&mut runtime);
   let file_path_script= "C:/NHKI/domy/query.txt".to_string();
   let code=fs::read_to_string(file_path_script).expect("Should have been able to read the file");
   let res=query(&mut runtime, code).expect("Error");
   println!("{0}",res);
}

fn main2() {

   // run_script();
   let mut runtime=init_dominodb();
   let sto:String="C:/NHKI/domy/data/500_KB_V2.json".to_string();
   load_data_op(&mut runtime, &sto);
   let file_path_script= "C:/NHKI/domy/testquery.txt".to_string();
   let code=fs::read_to_string(file_path_script).expect("Should have been able to read the file");
   //println!("{0}",code);
   let res=query(&mut runtime, code).expect("Error");
   println!("{0}",res);
}