use std::{
    ops::Index,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering}, marker::PhantomData,
};

const THRESHOLD: usize = 2;
const RETRY_THRESHOLD: usize = 2;

pub struct ContentionMesure(usize);
impl ContentionMesure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }

    pub fn use_slow_path(&self) -> bool {
        self.0 > THRESHOLD
    }
}

pub trait CasDescriptor {
    fn execute(&self) -> Result<(), ()>;
}

pub trait CasDescriptors<D>: Index<usize, Output = D>
where
    D: CasDescriptor,
{
    fn len(&self) -> usize;
}

pub trait NormalizedLockFree {
    type Input; // it can be an associated type then make it be an associated type and promote it to a generic type
                //of the trait if that ends up being necessary
    type Output;
    type Cas: CasDescriptor;
    type Cases: CasDescriptors<Self::Cas>;

    fn generator(&self, op: &Self::Input, contention: &mut ContentionMesure) -> Self::Cases;
    fn wrap_up(
        &self,
        res: Result<(), usize>,
        performed: &Self::Cases,
        contention: &mut ContentionMesure,
    ) -> Result<Self::Output, ()>;
}

pub struct OperationRecordBox<LF: NormalizedLockFree> {
    val: AtomicPtr<OperationRecord<LF>>,
}

enum OperationState<T> {
    PreCas,
    ExecuteCas,
    PostCas,
    Completed(T),
}

impl<T> OperationState<T> {
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed(..))
    }
}

// rcu
pub struct OperationRecord<LF: NormalizedLockFree> {
    owner: std::thread::ThreadId,
    input: LF::Input,
    state: OperationState<LF::Output>,
    cas_list: LF::Cases,
}

pub struct HelpQueue<LF> {
    _o: PhantomData<LF>,
}

impl<LF:NormalizedLockFree> HelpQueue<LF> {
    pub fn enqueue(&self, help: *const OperationRecordBox<LF>) {}

    pub fn peek(&self) -> Option<*const OperationRecordBox<LF>> {}

    pub fn try_remove_front(&self, completed: *const OperationRecordBox<LF>) -> Result<(), ()> {
        Ok(())
    }
}

struct WaitFreeSimulator<LF: NormalizedLockFree> {
    algorithm: LF,
    help: HelpQueue<LF>,
}

impl<LF: NormalizedLockFree> WaitFreeSimulator<LF> {
    fn cas_executor(
        &self,
        descriptors: &LF::Cases,
        contention: &mut ContentionMesure,
    ) -> Result<(), usize> {
        let len = descriptors.len();
        for i in 0..len {
            if descriptors[i].execute().is_err() {
                // TODO: record contention
                contention.detected();
                return Err(i);
            }
        }
        Ok(())
    }

    // Guarantees that on return orb is not in help queue,
    fn help_op(&self, orb: &OperationRecordBox<LF>) {
        let or = unsafe {&*orb.val.load(Ordering::SeqCst)};
        match or.state {
            OperationState::Completed(..) => {
                let _ = self.help.try_remove_front(orb);
            }
        }
    }

    fn help_first(&self) {
        if let Some(help) = self.help.peek() {
            // help make progess
        }
    }
    pub fn run(&self, op: LF::Input) -> LF::Output {
        // fast path
        for retry in 0.. {
            let help = true;
            if help {
                self.help_first();
            }

            let contention = ContentionMesure(0);
            let cases = self.algorithm.generator(&op, &mut contention);
            if contention.use_slow_path() {
                break;
            }
            let result = self.cas_executor(&cases, &mut contention);
            if contention.use_slow_path() {
                break;
            }
            match self.algorithm.wrap_up(result, &cases, &mut contention) {
                Ok(outcome) => return outcome,
                Err(()) => {}
            }
            if contention.use_slow_path() {
                break;
            }
            if retry > RETRY_THRESHOLD {
                break;
            }
        }

        // slow path
        let i = 0;
        let orb = OperationRecordBox {
            val: AtomicPtr::new(Box::into_raw(Box::new(OperationRecord {
                owner: std::thread::current().id(),
                input: op,
                stat: OperationState::PreCas,
                cas_list: (),

            }))),
        };
        self.help.enqueue(&orb);
        while !unsafe { &*orb.val.load(Ordering::SeqCst) }.completed {
            self.help_first();
        }
        todo!()
    }
}

// struct WaitFreeLinkedList<T> {
//     simulator: WaitFreeSimulator<LockFreeLinkedList<T>>,
// }

// struct LockFreeLinkedList<T> {
//     t: T,
// }

// impl<T> NormalizedLockFree for LockFreeLinkedList<T> {}

// impl<T> WaitFreeLinkedList<T> {
//     pub fn push_front(&mut self, t: T) {
//         // let i = self.simulator.enqueue(Insert(t));
//         // self.simulator.wait_fir(i)
//     }
// }

#[cfg(test)]
mod tests {}
