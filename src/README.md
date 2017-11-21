# ssimd
Simulated Simd on Rust stable channel

To make auto-vectorization enable in most of the cases, please turn on the BB vectorizer : https://llvm.org/docs/Vectorizers.html#the-slp-vectorizer
For example, use the following command to build your projects:
        
    RUSTFLAGS="-C llvm-args=-vectorize-slp-aggressive" cargo build --release
            
