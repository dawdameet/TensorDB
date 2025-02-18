use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use jni::objects::{JString, JObject};
use jni::sys::jstring;
use jni::JNIEnv;
lazy_static! {
    static ref TENSOR_DB: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "system" fn Java_com_meet_tensordb_TensorDB_store(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
    data: JString,
) {
    let key: String = env.get_string(&key).expect("Invalid key").into();
    let data: String = env.get_string(&data).expect("Invalid tensor data").into();

    let mut db = TENSOR_DB.lock().unwrap();
    db.insert(key, data);
}
#[no_mangle]
pub extern "system" fn Java_com_meet_tensordb_TensorDB_get(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
) -> jstring {
    let key: String = env.get_string(&key).expect("Invalid key").into();
    
    let db = TENSOR_DB.lock().unwrap();
    let result_str: String = match db.get(&key) {
        Some(tensor) => tensor.clone(), // Move result_str out of inner scope
        None => "null".to_string(),
    };

    env.new_string(result_str)
        .expect("Failed to create Java string")
        .into_raw()
}
