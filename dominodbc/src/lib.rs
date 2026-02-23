use std::ffi::{CStr, c_char};
use dominodb::*;
use deno_core::JsRuntime;
use global_var::*;
use std::cell::RefCell;

//new_global_var!(JsRuntime, init_dominodb());



#[unsafe(no_mangle)]
pub extern "C" fn domino_load_data(filepath_datao: *const c_char) -> i32 {
        let mut runtime=init_dominodb(); 
        let filepath_data =  unsafe{ CStr::from_ptr(filepath_datao)};
        let filepath_data = filepath_data.to_str().expect("Unicode conversion failed.");
        let filepath_data=filepath_data.to_string();
        load_data_op(&mut runtime, &filepath_data);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn domino_query(queryo: *const c_char,buffer: *mut c_char, nbuffer: u32 ) -> i32 {
    let mut ret=0; 
    let query_string:String;

    unsafe{
        let querys = CStr::from_ptr(queryo) ;
        let querys = querys.to_str().expect("Unicode conversion failed.");
        query_string = querys.to_string();

    }

    let mut runtime=init_dominodb(); 
    reload_data_op(&mut runtime);
       
    let res= query(&mut runtime, query_string);
    match res {
            Ok(json) => {
              copy_string_to_buffer(json, buffer, nbuffer);
            },
            Err(err) => {
               copy_string_to_buffer(err, buffer, nbuffer);
               ret=-1;
            },
    }
    ;
    ret
}

fn copy_string_to_buffer(ret: String,buffer: *mut c_char, nbuffer: u32 ){
   let mut i:usize=0;
        let imax:usize=nbuffer as usize;
        let bytes2=ret.as_bytes();
        for icha in bytes2 {
            let fg: i8= *icha as i8;
            if i < imax{
                unsafe{
            (*buffer.add(i))=fg;
                }
            i=i+1;
            }
        }
        unsafe{
        if (*buffer.wrapping_add(i)) != 0 {
            if  i+1<imax {
                (*buffer.add(i+1))=0;
            }
            else {
                 (*buffer.add(i))=0;
            }
        }
    }
}
