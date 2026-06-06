/// Tries to translate to microseconds to use the stall boot service correctly
#[allow(unused)]
pub struct StallDuration(pub usize);
#[allow(unused)]
impl StallDuration{
    pub fn from_secs(secs: usize) -> Self{
        Self(secs * 1_000_000)
    }
    pub fn from_milisecs(milisecs: usize) -> Self {
        Self(milisecs * 1_000)
    }
    pub fn as_microsecs(&self) -> usize {
        self.0 // Ts references the usize parameter
    }
}
