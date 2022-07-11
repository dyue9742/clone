pub struct ContentionMeasure(usize);
impl ContentionMeasure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }
}

pub trait NormalizedLockFree {
    type Input;
    type Output;
    type CasDescriptor;

    fn prepare(&self, op: &Self::Input) -> [Self::CasDescriptor; 1];
    fn execute(&self, cases: &[Self::CasDescriptor], i: usize, contention: ContentionMeasure);
    fn cleanup(&self);
}

struct WaitFreeSimulator<LF: NormalizedLockFree> {
    _lf: LF,
}

struct WaitFreeLinkedList<T> {
    simulator: WaitFreeSimulator<LockFreeLinkedList<T>>,
}

struct LockFreeLinkedList<T> {
    t: T,
}

impl<T> WaitFreeLinkedList<T> {
    pub fn insert(&self, t: T) {
    }
}
