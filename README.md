# TensorDB-Java

TensorDB-Java is a Java wrapper for the TensorDB Rust library using JNI (Java Native Interface). It enables Java applications to interact with Rust-based tensor processing functions.

## Project Structure

```
├── tensordb               # Rust library (TensorDB)
│   ├── src                # Rust source code
│   │   ├── lib.rs         # Rust implementation for tensor operations
│   ├── target/release     # Compiled Rust library (DLL/.rlib/.d files)
│   └── Cargo.toml         # Rust project configuration
│
├── tensordb-java          # Java wrapper ----X you are here
│   ├── src/main/java/com/meet/tensordb
│   │   ├── TensorDB.java  # Java class interacting with Rust
│   ├── target             # Compiled Java classes
│   └── tensorlib.dll      # Compiled Rust library placed in root
└── README.md
```

## Current Implementation Status

- ✅ **Rust Library (`tensordb`)**
  - Implemented basic tensor processing in Rust.
  - Exposed Rust functions via JNI.
  - Successfully compiled `.dll` (Windows) for JNI.

- ✅ **Java Wrapper (`tensordb-java`)**
  - Created `TensorDB.java` with native method signatures.
  - Loaded `tensorlib.dll` in Java.
  - Successfully called Rust functions from Java.

- ✅ **Successful JNI Execution**
  - Java calls Rust via JNI and retrieves processed tensor results.
  - Sample output:
    ```
    Rust says: Processed Tensor:
    [[7, 10],
     [15, 22]]
    ```

## Build & Run Instructions

### 1. Build Rust Library
Run the following command inside `tensordb`:
```sh
cargo build --release
```
This generates `tensorlib.dll` inside `target/release`. Copy it to the root of `tensordb-java`.

### 2. Compile Java Code
Run inside `tensordb-java`:
```sh
javac -h src/main/java com/meet/tensordb/TensorDB.java
```

### 3. Run Java Application
```sh
java -cp src/main/java com.meet.tensordb.TensorDB
```

## Next Steps
- Add more tensor operations.
- Optimize JNI interaction for performance.
- Package the library as a JAR for distribution.
- Add multi-platform support (Linux/macOS shared libraries).

---
**Author:** Meet (TensorDB Creator)

