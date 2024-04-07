use std::vec;

#[derive(Debug)]
struct RingBuffer<T> {
    storage: Vec<Option<T>>,
    read_idx: usize,
    write_idx: usize,
}

#[derive(Debug)]
struct Full;

impl<T: Clone> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        RingBuffer {
            storage: vec![None; capacity],
            read_idx: 1,
            write_idx: 0,
        }
    }

    fn push(&mut self, item: T) -> Result<(), Full> {
        if self.is_full() {
            return Err(Full);
        }
        self.storage[self.write_idx] = Some(item);
        self.write_idx = (self.write_idx + 1) % self.storage.len();
        self.write_idx = self.advance_idx(self.write_idx);
        Ok(())
    }

    fn is_full(&self) -> bool {
        self.write_idx - self.read_idx + 1 == self.storage.len()
    }

    fn advance_idx(&self, idx: usize) -> usize {
        (idx + 1) % self.storage.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        //let rb = RingBuffer::<i32>::new(10);
        //assert_eq!(rb.storage.len(), 0);
        assert_eq!(0, 5 % 5);

        let v = vec![1,2,3,4,5];
        let v = v.iter().map(|x| x + 1).filter(|x| x < 2).collect::<Vec<_>>();
        println!("{:?}", v)
    }
}
