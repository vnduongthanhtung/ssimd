# ssimd

Simulated Simd on Rust stable channel. This is my effort to make the [simd crate](https://github.com/rust-lang-nursery/simd) work on stable channel. The work is based on the well-known method : auto-vectorization. However, in this crate, I try to provide an API that is as close as possible to the simd crate. While autovectorization seems to work as a luck, with a simple trick as follow, I have made autovectorization successful in most of the cases.

### Note

In order to make auto-vectorization successful in most of the cases, please turn on the BB optimizer :

[https://llvm.org/docs/Vectorizers.html\#the-slp-vectorizer](https://llvm.org/docs/Vectorizers.html#the-slp-vectorizer)

For Rust, you can turn on the BB optimizer with the following build command:

```
RUSTFLAGS="-C llvm-args=-vectorize-slp-aggressive" cargo build --release
```

### Examples

Let's start with a very simple example

```rust
extern crate ssimd;
use ssimd::f64x2;

#[inline(never)]
fn test_simd(a : f64x2, b: f64x2) {    
    let c = a + b;
    println!("{:?}", c);
}
fn main() {
    let a = f64x2::new(1.0, 1.0);
    let b = f64x2::new(2.0, 3.0);
    test_simd(a, b);
}
```

When compile to llvm-ir code you will see those instructions in the function "test\_simd":

```
      %7 = fadd <2 x double> %4, %6
      store <2 x double> %7, <2 x double>* %c, align 16
```

and the equivalent assembly code is :

```
    addpd    %xmm0, %xmm2
    movapd    %xmm2, 16(%rsp)
```

In this case, even with the default build command, LLVM can successfully vectorize the code.

Let's try a more tricky example :

```rust
extern crate ssimd;
use std::io::{self, BufRead};
use ssimd::f64x2;

fn test_simd(i : i32) {
    let a = f64x2::new((i + 1) as f64, (i + 2) as f64);
    let b = f64x2::new((i + 3) as f64, (i + 3) as f64);
    let c = a + b;
    println!("{:?}", c);
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let i : i32 = line.trim().parse().unwrap_or(0);

    test_simd(i);
}
```

The llvm-ir code in the function "test\_simd":

```
      %104 = bitcast %"ssimd::f64x2"* %c.i to i8*
      call void @llvm.lifetime.start(i64 16, i8* nonnull %104)
      %105 = fadd double %99, %103
      %106 = fadd double %101, %103
```

and the assembly code :

```
    cvtsi2sdl    %ecx, %xmm2
    addsd    %xmm2, %xmm0
    addsd    %xmm2, %xmm1
```

So llvm wasn't able to vectorize the code when an integer-to-float converstion is inserted inside the function. However if the BB optimizer is enable, you will see the llvm-ir code as follow:

```
        %109 = fadd <2 x double> %108, %106
        store <2 x double> %109, <2 x double>* %c.i, align 16
```

and the assembly code:

```
        addpd    %xmm2, %xmm0
        movapd    %xmm0, 96(%rsp)
```

So the code is successfully vectorized.

You can see more examples in the folder "examples". These examples are ported from the [simd crate](https://github.com/rust-lang-nursery/simd) to work on stable channel. Almost no modification from the original code is made. For these examples, some might not get autovectorization with default build command. However, when the BB optimizer is enabled, all examples are successfully vectorized. You can try more with your examples.

### AVX instructions

AVX instructions are not available in some machines. If you want to use AVX instructions with the intrinsic approach (like the one in the [simd crate](https://github.com/rust-lang-nursery/simd)), you will need to use the Rust attribute "target\_feature" to detect whether the machines support those instructions. You also need to provide a fallback function method in case AVX instructions are not available. However, if you use autovectorization, you only need to provide one function method, since LLVM will generate appropriate instructions for each machine configuration.

Consider the following example:

```rust
extern crate ssimd;
use ssimd::f64x4;

#[inline(never)]
fn test_simd(a : f64x4, b: f64x4) {
    let c = a + b;
    println!("{:?}", c);
}
fn main() {
    let a = f64x4::new(1.0, 1.0, 1.0, 1.0);
    let b = f64x4::new(2.0, 3.0, 4.0, 5.0);
    test_simd(a, b);
}

```

On machines that do not have AVX instructions, LLVM generates the following code:

```
	addpd	%xmm0, %xmm2
	addpd	%xmm1, %xmm3
	movapd	%xmm2, 64(%rsp)
	movapd	%xmm3, 80(%rsp)
 
```

On machines that support AVX instructions, LLVM will generate this:

```
	vaddpd	(%rsi), %ymm0, %ymm0
	vmovapd	%ymm0, 64(%rsp)

```

So everything will be done automatically for you.

