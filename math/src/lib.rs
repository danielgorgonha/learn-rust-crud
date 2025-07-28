//! Math library for WebAssembly
//! 
//! This library provides basic mathematical operations that can be compiled
//! to WebAssembly and executed by the CRUD API.

/// Adds two integers
/// 
/// # Arguments
/// * `x` - First integer
/// * `y` - Second integer
/// 
/// # Returns
/// The sum of x and y
#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Multiplies two integers
/// 
/// # Arguments
/// * `x` - First integer
/// * `y` - Second integer
/// 
/// # Returns
/// The product of x and y
#[no_mangle]
pub extern "C" fn mul(x: i32, y: i32) -> i32 {
    x * y
}

/// Subtracts y from x
/// 
/// # Arguments
/// * `x` - First integer
/// * `y` - Second integer
/// 
/// # Returns
/// The difference (x - y), or 0 if x < y
#[no_mangle]
pub extern "C" fn sub(x: i32, y: i32) -> i32 {
    if x < y {
        return 0;
    }
    x - y
}

/// Divides x by y
/// 
/// # Arguments
/// * `x` - Dividend
/// * `y` - Divisor
/// 
/// # Returns
/// The quotient (x / y), or 0 if y == 0
#[no_mangle]
pub extern "C" fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0;
    }
    x / y
}

/// Calculates the remainder of x divided by y
/// 
/// # Arguments
/// * `x` - Dividend
/// * `y` - Divisor
/// 
/// # Returns
/// The remainder (x % y), or 0 if y == 0
#[no_mangle]
pub extern "C" fn rem(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0;
    }
    x % y
}

/// Returns the absolute value of x
/// 
/// # Arguments
/// * `x` - Integer value
/// 
/// # Returns
/// The absolute value of x
#[no_mangle]
pub extern "C" fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Returns the maximum of two integers
/// 
/// # Arguments
/// * `x` - First integer
/// * `y` - Second integer
/// 
/// # Returns
/// The larger of x and y
#[no_mangle]
pub extern "C" fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

/// Returns the minimum of two integers
/// 
/// # Arguments
/// * `x` - First integer
/// * `y` - Second integer
/// 
/// # Returns
/// The smaller of x and y
#[no_mangle]
pub extern "C" fn min(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

/// Calculates x raised to the power of y
/// 
/// # Arguments
/// * `x` - Base
/// * `y` - Exponent
/// 
/// # Returns
/// x^y, or 0 if y < 0 (to avoid floating point)
#[no_mangle]
pub extern "C" fn pow(x: i32, y: i32) -> i32 {
    if y < 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }
    if y == 1 {
        return x;
    }
    
    let mut result = x;
    for _ in 1..y {
        result = result * x;
    }
    result
}