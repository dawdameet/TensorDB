use std::vec;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;
use ndarray::{Array2,Axis};
#[no_mangle]
pub extern "system" fn processTensor(){
    println!("Processing Tensors...");
    let mat: Array2<f32>=Array2::from_shape_vec((2,2), vec![1.0,2.0,3.0])
            .expect("Tensor process failed.");
    let res=mat.dot(&mat);
    println!("Multiplied Matrix: \n{}",res);
    0
}