pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for usize {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
