use dashmap::DashMap;
use lazy_static::lazy_static;
use jni::objects::{JString, JObject};
use jni::sys::{jstring, jint};
use jni::JNIEnv;

lazy_static! {
    static ref TENSOR_DB: DashMap<String, String> = DashMap::new();
}

#[no_mangle]
pub extern "system" fn Java_com_example_TensorDB_store(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
    data: JString,
) -> jint {
    let key = match env.get_string(&key) {
        Ok(k) => k.to_string_lossy().into_owned(),
        Err(_) => return -1, // Return error code
    };
    
    let data = match env.get_string(&data) {
        Ok(d) => d.to_string_lossy().into_owned(),
        Err(_) => return -1,
    };
    
    TENSOR_DB.insert(key, data);
    0 
}


#[no_mangle]
pub extern "system" fn Java_com_example_TensorDB_get(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
) -> jstring {
    let key = match env.get_string(&key) {
        Ok(k) => k.to_string_lossy().into_owned(),
        Err(_) => return std::ptr::null_mut(), // Return null if key is invalid
    };
    
    let result = TENSOR_DB.get(&*key).map(|entry| entry.value().clone());
    
    match result {
        Some(value) => env.new_string(value).expect("Failed to create Java string").into_raw(),
        None => std::ptr::null_mut(),
    }
}
