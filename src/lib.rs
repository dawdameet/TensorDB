use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;
use ndarray::Array2;

#[no_mangle]
pub extern "system" fn Java_com_meet_tensordb_TensorDB_processTensor(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let tensor: Array2<f32> = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0])
        .expect("Tensor creation failed.");

    let result = tensor.dot(&tensor); // Matrix multiplication
    let result_str = format!("Processed Tensor:\n{}", result);

    let java_str = env.new_string(result_str)
        .expect("Failed to create Java String");

    java_str.as_raw() // Corrected: Use `as_raw()` instead of `into_inner()`
}
