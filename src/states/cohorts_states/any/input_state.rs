#[derive(Debug, Default)]
pub struct InputState {
    pub count: f32,
    pub volume: f32,
}

impl InputState {
    pub fn iterate(&mut self, count: f32, volume: f32) {
        self.count += count;
        self.volume += volume;
    }
}
