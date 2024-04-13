#[derive(Debug, Default)]
pub struct OutputState {
    pub count: f32,
    pub volume: f32,
}

impl OutputState {
    pub fn iterate(&mut self, count: f32, volume: f32) {
        self.count += count;
        self.volume += volume;
    }
}
