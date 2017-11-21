// Simulated SIMD on Rust Stable Channel
// Inspired by crate `simd` developed for nightly Channel
// Reference link : https://github.com/rust-lang-nursery/simd

#![allow(non_camel_case_types)]
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor, Not, Shl, Shr};

/// 2x32-bit vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u32x2(u32, u32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i32x2(i32, i32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct f32x2(f32, f32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool32x2(i32, i32);

/// 4x32-bit vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u32x4(u32, u32, u32, u32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i32x4(i32, i32, i32, i32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct f32x4(f32, f32, f32, f32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool32x4(i32, i32, i32, i32);

/// 16x8-bit integer vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u16x8(u16, u16, u16, u16,
                 u16, u16, u16, u16);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i16x8(i16, i16, i16, i16,
                 i16, i16, i16, i16);
                 
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool16x8(i16, i16, i16, i16,
                     i16, i16, i16, i16);

/// 8x16-bit integer vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u8x16(u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8);
                 
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool8x16(i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8);


/// 2x64-bit vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u64x2(u64, u64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i64x2(i64, i64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct f64x2(pub f64, pub f64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool64x2(i64, i64);


/// 4x64-bit vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u64x4(u64, u64, u64, u64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i64x4(i64, i64, i64, i64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct f64x4(f64, f64, f64, f64);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool64x4(i64, i64, i64, i64);


/// 8x32-bit vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u32x8(u32, u32, u32, u32,
                 u32, u32, u32, u32);
                 
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i32x8(i32, i32, i32, i32,
                 i32, i32, i32, i32);
                 
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct f32x8(f32, f32, f32, f32,
                 f32, f32, f32, f32);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool32x8(i32, i32, i32, i32,
                     i32, i32, i32, i32);
                  
/// 16x16-bit integer vectors
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u16x16(u16, u16, u16, u16, u16, u16, u16, u16,
                  u16, u16, u16, u16, u16, u16, u16, u16);
                  
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i16x16(i16, i16, i16, i16, i16, i16, i16, i16,
                  i16, i16, i16, i16, i16, i16, i16, i16);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool16x16(i16, i16, i16, i16, i16, i16, i16, i16,
                      i16, i16, i16, i16, i16, i16, i16, i16);

/// 32x8-bit integer vector
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct u8x32(u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8);
                 
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct i8x32(i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8);

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct bool8x32(i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8);

macro_rules! basic_impls {
    ($(
        $name: ident : $elem: ident, 
        $bool_name: ident : $bool_elem: ident, 
        $length: expr,
        $($index:tt : $field:ident),*;
    )*) => {
        
        $(impl $name {
            /// Create new instance
            #[inline(always)]
            pub fn new($($field: $elem),*) -> Self {
                $name($($field),*)
            }
            
            /// Create new instance with all lanes set to a value
            #[inline(always)]
            pub fn splat(x: $elem) -> Self {
                Self {$($index : x),*}
            }
            
            /// Get the `idx`th lane value
            #[inline(always)]
            pub fn extract(self, idx: u32) -> $elem {
                assert!(idx < $length);
                unsafe {
                    let p = (&self) as *const _ as *const $elem;
                    *(p.offset(idx as isize))
                }
            }
            
            /// Replace the `idx`th lane with new value
            #[inline(always)]
            pub fn replace(self, idx: u32, elem: $elem) -> Self {
                assert!(idx < $length);
                unsafe {
                    let mut ret = self;
                    let p = (&mut ret) as *mut _ as *mut $elem;
                    *(p.offset(idx as isize)) = elem;
                    ret
                }
            }
            
            /// Load instance from an array
            #[inline(always)]
            pub fn load(array: &[$elem], idx: usize) -> Self {
                $name($(array[idx + $index]),*)
            }
            
            /// Store self to an array
            #[inline(always)]
            pub fn store(self, array: &mut [$elem], idx: usize) {
                $(array[idx + $index] = self.$index);*
            }
            
            /// Compare if equal
            #[inline(always)]
            pub fn eq(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index == rhs.$index) as $bool_elem),*)
            }
            
            /// Compare if not equal
            #[inline(always)]
            pub fn ne(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index != rhs.$index) as $bool_elem),*)
            }
            
            /// Compare if less than
            #[inline(always)]
            pub fn lt(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index < rhs.$index) as $bool_elem),*)
            }
            
            /// Compare if less than or equal
            #[inline(always)]
            pub fn le(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index <= rhs.$index) as $bool_elem),*)
            }
            
            /// Compare if greater than
            #[inline(always)]
            pub fn gt(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index > rhs.$index) as $bool_elem),*)
            }
            
            /// Compare if greater than or equal
            #[inline(always)]
            pub fn ge(self, rhs: Self) -> $bool_name {
                $bool_name($((self.$index >= rhs.$index) as $bool_elem),*)
            }
            
            /// Get max values by lane
            #[inline(always)]
            pub fn max(self, rhs: Self) -> Self {
                $name($(if self.$index > rhs.$index { self.$index } else {rhs.$index}),*)
            }
            
            /// Get min values by lane            
            #[inline(always)]
            pub fn min(self, rhs: Self) -> Self {
                $name($(if self.$index < rhs.$index { self.$index } else {rhs.$index}),*)
            }
        })*
        
        /// Add trait (+)
        $(impl Add for $name {            
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                $name($(self.$index + rhs.$index),*)
            }
        })*
        
        /// Sub trait (-)
        $(impl Sub for $name {            
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                $name($(self.$index - rhs.$index),*)
            }
        })*
                
        /// Mul trait (*)
        $(impl Mul for $name {            
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                $name($(self.$index * rhs.$index),*)
            }
        })*
        
        /// Div trait (/)
        $(impl Div for $name {            
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                $name($(self.$index / rhs.$index),*)
            }
        })*
    }
}

basic_impls! {
    u32x2:u32, bool32x2:i32, 2, 0:x0, 1:x1;
    i32x2:i32, bool32x2:i32, 2, 0:x0, 1:x1;
    f32x2:f32, bool32x2:i32, 2, 0:x0, 1:x1;
    
    u32x4:u32, bool32x4:i32, 4, 0:x0, 1:x1, 2:x2, 3:x3;
    i32x4:i32, bool32x4:i32, 4, 0:x0, 1:x1, 2:x2, 3:x3;
    f32x4:f32, bool32x4:i32, 4, 0:x0, 1:x1, 2:x2, 3:x3;
    
    u16x8:u16, bool16x8:i16, 8, 0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    i16x8:i16, bool16x8:i16, 8, 0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
   
    u64x2:u64, bool64x2:i64, 2, 0:x0, 1:x1;
    i64x2:i64, bool64x2:i64, 2, 0:x0, 1:x1;
    f64x2:f64, bool64x2:i64, 2, 0:x0, 1:x1;
    
    u64x4:u64, bool64x4:i64, 4, 0:x0, 1:x1 ,2:x2, 3:x3;
    i64x4:i64, bool64x4:i64, 4, 0:x0, 1:x1 ,2:x2, 3:x3;
    f64x4:f64, bool64x4:i64, 4, 0:x0, 1:x1 ,2:x2, 3:x3;
    
    u32x8:u32, bool32x8:i32, 8, 0:x0, 1:x1 ,2:x2, 3:x3, 4:x4, 5:x5 ,6:x6, 7:x7;
    i32x8:i32, bool32x8:i32, 8, 0:x0, 1:x1 ,2:x2, 3:x3, 4:x4, 5:x5 ,6:x6, 7:x7;
    f32x8:f32, bool32x8:i32, 8, 0:x0, 1:x1 ,2:x2, 3:x3, 4:x4, 5:x5 ,6:x6, 7:x7;
    
    u16x16:u16, bool16x16:i16, 16,  0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                                    8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                               
    i16x16:i16, bool16x16:i16, 16,  0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                                    8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                               
    u8x32:u8, bool8x32:i8, 32,      0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                                    8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15,
                                    16:x16, 17:x17 , 18:x18, 19:x19, 20:x20, 21:x21 ,22:x22, 23:x23,
                                    24:x24, 25:x25 , 26:x26, 27:x27, 28:x28, 29:x29 ,30:x30, 31:x31;
                            
    i8x32:i8, bool8x32:i8, 32,      0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                                    8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15,
                                    16:x16, 17:x17 , 18:x18, 19:x19, 20:x20, 21:x21 ,22:x22, 23:x23,
                                    24:x24, 25:x25 , 26:x26, 27:x27, 28:x28, 29:x29 ,30:x30, 31:x31;
}

macro_rules! int_impls {
    ($(
        $name: ident : $elem: ident, 
        $($index:tt : $field:ident),*;
        )*) => {
        
        $(impl $name {
            /// Select between elements of `then` and `else_`, based on
            /// the corresponding element of `self`.
            /// Equivalent to:
            /// T::new(if self.0 { then.0 } else { else_.0 },
            ///        if self.1 { then.1 } else { else_.1 },
            ///        ...)
            #[inline(always)]
            pub fn select(&self, then: Self, else_ : Self) -> Self {
                $name($((self.$index & then.$index) | (!self.$index & else_.$index)),*)
            }
        })*
        
        /// BitAnd trait (&)
        $(impl BitAnd for $name {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                $name($(self.$index & rhs.$index),*)
            }
        })*
            
        /// BitOr trait (|)
        $(impl BitOr for $name {            
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                $name($(self.$index | rhs.$index),*)
            }
        })*
        
        /// BitXor trait (^)        
        $(impl BitXor for $name {            
            type Output = Self;
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                $name($(self.$index ^ rhs.$index),*)
            }
        })*
        
        /// Not trait (!)
        $(impl Not for $name {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
              $name($(!self.$index),*)
            }
        })*
        
        /// Shift left trait (<<)
        $(impl Shl<usize> for $name {
            type Output = Self;
            #[inline(always)]
            fn shl(self, sz: usize) -> Self {
                $name($(self.$index << (sz as $elem)),*)
            }
        })*
        
        /// Shift right trait (>>)
        $(impl Shr<usize> for $name {
            type Output = Self;
            #[inline(always)]
            fn shr(self, sz: usize) -> Self {
                $name($(self.$index >> (sz as $elem)),*)
            }
        })*
    }
}

int_impls! {
    u32x2:u32,  0:x0, 1:x1;
    i32x2:i32,  0:x0, 1:x1;
    
    u32x4:u32,  0:x0, 1:x1, 2:x2, 3:x3;
    i32x4:i32,  0:x0, 1:x1, 2:x2, 3:x3;
    
    u16x8:u16,  0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    i16x8:i16,  0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
   
    u64x2:u64,  0:x0, 1:x1;
    i64x2:i64,  0:x0, 1:x1;
    
    u64x4:u64,  0:x0, 1:x1 ,2:x2, 3:x3;
    i64x4:i64,  0:x0, 1:x1 ,2:x2, 3:x3;
    
    u32x8:u32,  0:x0, 1:x1 ,2:x2, 3:x3, 4:x4, 5:x5 ,6:x6, 7:x7;
    i32x8:i32,  0:x0, 1:x1 ,2:x2, 3:x3, 4:x4, 5:x5 ,6:x6, 7:x7;
    
    u16x16:u16, 0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                
    i16x16:i16, 0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                               
    u8x32:u8,   0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15,
                16:x16, 17:x17 , 18:x18, 19:x19, 20:x20, 21:x21 ,22:x22, 23:x23,
                24:x24, 25:x25 , 26:x26, 27:x27, 28:x28, 29:x29 ,30:x30, 31:x31;
                
    i8x32:i8,   0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15,
                16:x16, 17:x17 , 18:x18, 19:x19, 20:x20, 21:x21 ,22:x22, 23:x23,
                24:x24, 25:x25 , 26:x26, 27:x27, 28:x28, 29:x29 ,30:x30, 31:x31;
}

macro_rules! bool_impls {
    ($(
        $name: ident : $elem: ident, 
        $length: expr,
        $($index:tt : $field:ident),*;
        )*) => {
        
        $(impl $name {
            /// Create new instance
            #[inline(always)]
            pub fn new($($field: $elem),*) -> Self {
                $name($($field),*)
            }

            /// Create new instance with all lanes set to a vale
            #[inline(always)]
            pub fn splat(x: $elem) -> Self {
                Self {$($index : x),*}
            }
            
            /// Get the `idx`th lane value
            #[inline(always)]
            pub fn extract(self, idx: u32) -> $elem {
                assert!(idx < $length);
                unsafe {
                    let p = (&self) as *const _ as *const $elem;
                    *(p.offset(idx as isize))
                }
            }
            
            /// Replace the `idx`th lane with new value
            #[inline(always)]
            pub fn replace(self, idx: u32, elem: $elem) -> Self {
                assert!(idx < $length);
                unsafe {
                    let mut ret = self;
                    let p = (&mut ret) as *mut _ as *mut $elem;
                    *(p.offset(idx as isize)) = elem;
                    ret
                }
            }
            
            /// Load instance from an array
            #[inline(always)]
            pub fn load(array: &[$elem], idx: usize) -> Self {
                $name($(array[idx + $index]),*)
            }
            
            /// Store self to an array
            #[inline(always)]
            pub fn store(self, array: &mut [$elem], idx: usize) {
                $(array[idx + $index] = self.$index);*
            }
            
            /// Check if all lanes are true
            #[inline(always)]
            pub fn all(self) -> bool {
                $((self.$index != 0)) && *
            }
            
            /// Check if at least one lane is true
            #[inline(always)]
            pub fn any(self) -> bool {
                $((self.$index != 0)) || *
            }            
       })*
       
       /// Not trait (!)
       $(impl Not for $name {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
              $name($(!self.$index),*)
            }
       })*
    }
}

bool_impls! {
    bool32x2:i32, 2,    0:x0, 1:x1;
    bool32x4:i32, 4,    0:x0, 1:x1 , 2:x2, 3:x3;    
    bool16x8:i16, 8,    0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
                    
    bool8x16:i8, 16,    0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                        8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                    
    bool64x2:i64, 2,    0:x0, 1:x1;
    bool64x4:i64, 4,    0:x0, 1:x1 , 2:x2, 3:x3;
    bool32x8:i32, 8,    0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    
    bool16x16:i16, 16,  0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                        8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15;
                    
    bool8x32:i8, 32,    0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7,
                        8:x8, 9:x9 , 10:x10, 11:x11, 12:x12, 13:x13 ,14:x14, 15:x15,
                        16:x16, 17:x17 , 18:x18, 19:x19, 20:x20, 21:x21 ,22:x22, 23:x23,
                        24:x24, 25:x25 , 26:x26, 27:x27, 28:x28, 29:x29 ,30:x30, 31:x31;
}

macro_rules! float_impls {
    ($(
        $name: ident, 
        $($index:tt : $field:ident),*;
        )*) => {
        
        $(impl $name {
            /// Get square root
            #[inline]
            pub fn sqrt(self) -> Self {
                $name($(self.$index.sqrt()),*)
            }

            /// Get reciprocal of square root
            #[inline]
            pub fn approx_rsqrt(self) -> Self {
                $name($(1.0 / self.$index.sqrt()),*)
            }

            /// Get reciprocal
            #[inline]
            pub fn approx_reciprocal(self) -> Self {
                $name($(1.0 / self.$index),*)
            }                
       })*
    }
}

float_impls! {
    f32x2, 0:x0, 1:x1;
    f32x4, 0:x0, 1:x1 , 2:x2, 3:x3;
    f32x8, 0:x0, 1:x1 , 2:x2, 3:x3, 4:x4, 5:x5 , 6:x6, 7:x7;    
    f64x2, 0:x0, 1:x1;
    f64x4, 0:x0, 1:x1 , 2:x2, 3:x3;
}

macro_rules! conv_impls {
    ($(
        $cvt: ident,
        $from_name: ident : $from_elem: ident -> $to_name : ident : $to_elem : ident,
        $($index:tt : $field:ident),*;
        )*) => {
            
        $(impl $from_name {
            pub fn $cvt(self) -> $to_name {
                $to_name($(self.$index as $to_elem), *)
            }
        })*
    }
}

/// Conversion among types
conv_impls! {
    to_i, u32x2 : u32 -> i32x2 : i32,       0:x0, 1:x1;
    to_i, f32x2 : f32 -> i32x2 : i32,       0:x0, 1:x1;    
    to_u, i32x2 : i32 -> u32x2 : u32,       0:x0, 1:x1;
    to_u, f32x2 : f32 -> u32x2 : u32,       0:x0, 1:x1;    
    to_f, u32x2 : u32 -> f32x2 : f32,       0:x0, 1:x1;
    to_f, i32x2 : i32 -> f32x2 : f32,       0:x0, 1:x1;
    to_i, bool32x2 : i32 -> i32x2 : i32,    0:x0, 1:x1;
    to_u, bool32x2 : i32 -> u32x2 : u32,    0:x0, 1:x1;
    
    to_i, u32x4 : u32 -> i32x4 : i32,       0:x0, 1:x1, 2:x2, 3:x3;
    to_i, f32x4 : f32 -> i32x4 : i32,       0:x0, 1:x1, 2:x2, 3:x3;    
    to_u, i32x4 : i32 -> u32x4 : u32,       0:x0, 1:x1, 2:x2, 3:x3;
    to_u, f32x4 : f32 -> u32x4 : u32,       0:x0, 1:x1, 2:x2, 3:x3;    
    to_f, u32x4 : u32 -> f32x4 : f32,       0:x0, 1:x1, 2:x2, 3:x3;
    to_f, i32x4 : i32 -> f32x4 : f32,       0:x0, 1:x1, 2:x2, 3:x3;
    to_i, bool32x4 : i32 -> i32x4 : i32,    0:x0, 1:x1, 2:x2, 3:x3;
    to_u, bool32x4 : i32 -> u32x4 : u32,    0:x0, 1:x1, 2:x2, 3:x3;
    
    to_i, u32x8 : u32 -> i32x8 : i32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    to_i, f32x8 : f32 -> i32x8 : i32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;    
    to_u, i32x8 : i32 -> u32x8 : u32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    to_u, f32x8 : f32 -> u32x8 : u32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;    
    to_f, u32x8 : u32 -> f32x8 : f32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    to_f, i32x8 : i32 -> f32x8 : f32,       0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    to_i, bool32x8 : i32 -> i32x8 : i32,    0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    to_u, bool32x8 : i32 -> u32x8 : u32,    0:x0, 1:x1, 2:x2, 3:x3, 4:x4, 5:x5, 6:x6, 7:x7;
    
    to_i, u64x2 : u64 -> i64x2 : i64,       0:x0, 1:x1;
    to_i, f64x2 : f64 -> i64x2 : i64,       0:x0, 1:x1;    
    to_u, i64x2 : i64 -> u64x2 : u64,       0:x0, 1:x1;
    to_u, f64x2 : f64 -> u64x2 : u64,       0:x0, 1:x1;
    to_f, u64x2 : u64 -> f64x2 : f64,       0:x0, 1:x1;
    to_f, i64x2 : i64 -> f64x2 : f64,       0:x0, 1:x1;
    to_i, bool64x2 : i64 -> i64x2 : i64,    0:x0, 1:x1;
    to_u, bool64x2 : i64 -> u64x2 : u64,    0:x0, 1:x1;
    
    to_i, u64x4 : u64 -> i64x4 : i64,       0:x0, 1:x1, 2:x2, 3:x3;
    to_i, f64x4 : f64 -> i64x4 : i64,       0:x0, 1:x1, 2:x2, 3:x3;    
    to_u, i64x4 : i64 -> u64x4 : u64,       0:x0, 1:x1, 2:x2, 3:x3;
    to_u, f64x4 : f64 -> u64x4 : u64,       0:x0, 1:x1, 2:x2, 3:x3;    
    to_f, u64x4 : u64 -> f64x4 : f64,       0:x0, 1:x1, 2:x2, 3:x3;
    to_f, i64x4 : i64 -> f64x4 : f64,       0:x0, 1:x1, 2:x2, 3:x3;
    to_i, bool64x4 : i64 -> i64x4 : i64,    0:x0, 1:x1, 2:x2, 3:x3;
    to_u, bool64x4 : i64 -> u64x4 : u64,    0:x0, 1:x1, 2:x2, 3:x3;
    
    to_i32, i64x2 : i64 -> i32x2 : i32,     0:x0, 1:x1;
    to_i64, i32x2 : i32 -> i64x2 : i64,     0:x0, 1:x1;    
    to_i32, i64x4 : i64 -> i32x4 : i32,     0:x0, 1:x1, 2:x2, 3:x3;
    to_i64, i32x4 : i32 -> i64x4 : i64,     0:x0, 1:x1, 2:x2, 3:x3;
    
    to_u32, u64x2 : u64 -> u32x2 : u32,     0:x0, 1:x1;
    to_u64, u32x2 : u32 -> u64x2 : u64,     0:x0, 1:x1;    
    to_u32, u64x4 : u64 -> u32x4 : u32,     0:x0, 1:x1, 2:x2, 3:x3;
    to_u64, u32x4 : u32 -> u64x4 : u64,     0:x0, 1:x1, 2:x2, 3:x3;
    
    to_f32, f64x2 : f64 -> f32x2 : f32,     0:x0, 1:x1;
    to_f64, f32x2 : f32 -> f64x2 : f64,     0:x0, 1:x1;    
    to_f32, f64x4 : f64 -> f32x4 : f32,     0:x0, 1:x1, 2:x2, 3:x3;
    to_f64, f32x4 : f32 -> f64x4 : f64,     0:x0, 1:x1, 2:x2, 3:x3;
}
