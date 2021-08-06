```
❯ cd /tmp
❯ git clone https://github.com/haraldh/rust_llvm_error.git
❯ cd rust_llvm_error/
❯ cargo clean; cargo build
   Compiling scopeguard v1.1.0
   Compiling primordial v0.1.0
   Compiling lock_api v0.4.4
   Compiling spinning v0.1.0
   Compiling llvm_error v0.1.0 (/home/haraldh/llvm_error)
    Finished dev [unoptimized + debuginfo] target(s) in 1.20s

❯ cd ..
❯ mkdir -p /tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/
❯ mv rust_llvm_error /tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/
❯ cd /tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/rust_llvm_error/
❯ cargo clean; cargo build
   Compiling scopeguard v1.1.0
   Compiling primordial v0.1.0
   Compiling lock_api v0.4.4
   Compiling spinning v0.1.0
   Compiling llvm_error v0.1.0 (/home/haraldh/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/llvm_error)
LLVM ERROR: Invalid LLVMRustLinkage value!
error: could not compile `llvm_error`
```
