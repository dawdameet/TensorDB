use rocksdb::{DB, Options};
use jni::objects::{JString, JObject};
use jni::sys::{jstring, jint};
use jni::JNIEnv;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_INSTANCE: Mutex<DB> = Mutex::new(init_db());
}

// Initialize RocksDB
fn init_db() -> DB {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, "./tensordb").expect("Failed to open database")
}

#[no_mangle]
pub extern "system" fn Java_com_meet_tensordb_TensorDB_store(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
    data: JString,
) -> jint {
    let key = match env.get_string(&key) {
        Ok(k) => k.to_string_lossy().into_owned(),
        Err(_) => return -1,
    };

    let data = match env.get_string(&data) {
        Ok(d) => d.to_string_lossy().into_owned(),
        Err(_) => return -1,
    };

    let db = DB_INSTANCE.lock().unwrap();
    match db.put(key, data) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_meet_tensordb_TensorDB_get(
    mut env: JNIEnv,
    _class: JObject,
    key: JString,
) -> jstring {
    let key = match env.get_string(&key) {
        Ok(k) => k.to_string_lossy().into_owned(),
        Err(_) => return std::ptr::null_mut(),
    };

    let db = DB_INSTANCE.lock().unwrap();
    let result = db.get(key).ok().flatten();

    match result {
        Some(value) => {
            let value_str = String::from_utf8(value).expect("Invalid UTF-8 data");
            env.new_string(value_str).expect("Failed to create Java string").into_raw()
        }
        None => std::ptr::null_mut(),
    }
}
