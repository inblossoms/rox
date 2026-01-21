use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

thread_local! {
    static RNG_STATE: Cell<u32> = Cell::new(initial_seed());
}

// 种子生成
fn initial_seed() -> u32 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_nanos() as u32 // 使用纳秒时间作为种子
}

pub fn random() -> f64 {
    RNG_STATE.with(|cell| {
        let mut state = cell.get();
        // 线性同余生成器 (LCG) 参数 (Numerical Recipes)
        const A: u32 = 1664525;
        const C: u32 = 1013904223;
        // state = (state * 1664525 + 1013904223) % 2^32
        state = state.wrapping_mul(A).wrapping_add(C); // 避免溢出
        cell.set(state);
        (state as f64) / (u32::MAX as f64 + 1.0) // 转换为 [0.0, 1.0)
    })
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    if min > max {
        println!(
            "Warning: received invalid arguments, must be min <= max, will get the result of the swap."
        );
        return rand_range(max, min);
    }
    min + (max - min) * random()
}
pub fn rand_int(min: i32, max: i32) -> i32 {
    if min > max {
        println!(
            "Warning: received invalid arguments, must be min <= max, will get the result of the swap."
        );
        return rand_int(max, min);
    }

    min + (max - min) * random() as i32
}
