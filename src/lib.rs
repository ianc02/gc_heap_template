//#![cfg_attr(not(test), no_std)]

use gc_headers::{Tracer, HeapResult, Pointer, GarbageCollectingHeap, HeapError};

#[derive(Copy, Clone, Debug)]
pub struct CopyingHeap<const HEAP_SIZE: usize, const MAX_BLOCKS: usize> {
    memory: [[u64; HEAP_SIZE]; 2],
    new: usize,
    old: usize,
    num_blocks: usize,
    next_available_block_index: usize,
    pointer_buffer: [Pointer; MAX_BLOCKS],
    block_num_buff: [usize; MAX_BLOCKS],
    block_size_buff: [usize; MAX_BLOCKS],
    block_alloc_buff: [bool; MAX_BLOCKS],

    
    // YOUR CODE HERE
}

impl<const HEAP_SIZE: usize, const MAX_BLOCKS: usize> GarbageCollectingHeap for
    CopyingHeap<HEAP_SIZE, MAX_BLOCKS>
{
    fn new() -> Self {
        let mut memory = [[0 as u64; HEAP_SIZE]; 2];
        let mut pointer_buffer = [Pointer::new(0, MAX_BLOCKS); MAX_BLOCKS];
        let mut block_num_buff = [0; MAX_BLOCKS];
        let mut block_size_buff = [0; MAX_BLOCKS];
        let mut block_alloc_buff = [false; MAX_BLOCKS];
        Self{memory, new: 1, old: 0, num_blocks: 0, next_available_block_index: 0, pointer_buffer, block_num_buff, block_size_buff, block_alloc_buff}

    }

    fn load(&self, p: Pointer) -> HeapResult<u64> {
        return HeapResult::Ok(self.memory[self.old][self.block_num_buff[p.block_num()] + p.offset()]) ;
    }

    fn store(&mut self, p: Pointer, value: u64) -> HeapResult<()> {
        if p.offset() < p.len(){
            self.memory[self.old][self.block_num_buff[p.block_num()] + p.offset()] = value;
            p.next();
            return HeapResult::Ok(())
        }
        else{
            return HeapResult::Err(HeapError::OffsetTooBig)
        }
    }

    fn malloc<T: Tracer>(&mut self, num_words: usize, tracer: &T) -> HeapResult<Pointer> {
        //println!("{:?}", self.memory);
        //if self.num_blocks < MAX_BLOCKS{
            if num_words + self.next_available_block_index <= HEAP_SIZE{
                if self.num_blocks==MAX_BLOCKS{
                    return HeapResult::Err(HeapError::OutOfBlocks);
                }
                let mut block_number = 0;
                for i in 0..MAX_BLOCKS{
                    if !self.block_alloc_buff[i]{
                        block_number = i;
                        self.block_alloc_buff[i] = true;
                        break;
                    }
                }
                let p = Pointer::new(block_number, num_words);
                self.block_num_buff[block_number] = self.next_available_block_index;
                self.block_size_buff[block_number] = num_words;
                self.num_blocks +=1;
                self.next_available_block_index += num_words;
                
                self.pointer_buffer[block_number] = p;
                
                return HeapResult::Ok(p);
            }
            else {
                self.num_blocks = 0;
                self.block_alloc_buff = [false;MAX_BLOCKS];
                let mut trace_buffer = [false; MAX_BLOCKS];
                //println!("{:?}",trace_buffer);
                tracer.trace(&mut trace_buffer);
                
                //println!("{:?}",trace_buffer);
                //println!("BREAK");
                self.memory[self.new] = [0;HEAP_SIZE];
                let mut temp_p_buff = [Pointer::new(MAX_BLOCKS+7, MAX_BLOCKS); MAX_BLOCKS];
                let mut new_index = 0;
                for (i,val) in trace_buffer.iter().enumerate(){
                    if *val{
                        // let temp_index = new_index;
                        // for j in self.block_num_buff[i]..self.block_num_buff[i]+self.block_size_buff[i]{
                        //     self.memory[self.new][new_index] = self.memory[self.old][j];
                        //     new_index +=1;
                        // }
                        // self.block_num_buff[i] = temp_index;
                        // copied_blocks +=1;
                        // self.num_blocks+=1;
                        
                        for j in 0..self.block_size_buff[i]{
                            self.memory[self.new][new_index+j] = self.memory[self.old][j+self.block_num_buff[i]];
                            //self.store(self.pointer_buffer[i], self.load(self.pointer_buffer[i]).unwrap());
                        }
                        self.block_num_buff[i] = new_index;
                        self.num_blocks+=1;
                        self.block_alloc_buff[i] = true;
                        new_index += self.block_size_buff[i];
                    }
                }
                
                self.next_available_block_index = new_index;
                let temp = self.old;
                self.old = self.new;
                self.new = temp;
                self.memory[self.new] = [0;HEAP_SIZE];
                println!("{:?}",self.memory);
                println!("{}",new_index);
                println!("{}",num_words);
                // for i in 
                if self.num_blocks==MAX_BLOCKS{
                    return HeapResult::Err(HeapError::OutOfBlocks);
                }
                if num_words + self.next_available_block_index <= HEAP_SIZE{
                    if self.num_blocks==MAX_BLOCKS{
                        return HeapResult::Err(HeapError::OutOfBlocks);
                    }
                    let mut block_number = 0;
                    for i in 0..MAX_BLOCKS{
                        if !self.block_alloc_buff[i]{
                            block_number = i;
                            self.block_alloc_buff[i] = true;
                            break;
                        }
                    }
                    let p = Pointer::new(block_number, num_words);
                    self.block_num_buff[block_number] = self.next_available_block_index;
                    self.block_size_buff[block_number] = num_words;
                    self.num_blocks +=1;
                    self.next_available_block_index += num_words;
                    
                    self.pointer_buffer[block_number] = p;
                    
                    return HeapResult::Ok(p);
                }
                else {
                    return  HeapResult::Err(HeapError::OutOfMemory);
                }
            }
        // }   
        // else{
        //     return HeapResult::Err(HeapError::OutOfMemory);
        // }
    }
}

impl<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>
    CopyingHeap<HEAP_SIZE, MAX_BLOCKS>
{
    pub fn is_allocated(&self, block: usize) -> bool {
        return self.block_alloc_buff[block];
    }   
            
    pub fn num_allocated_blocks(&self) -> usize {
        return self.num_blocks;
    }       
            
    pub fn size_of(&self, block: usize) -> usize {
        return self.block_size_buff[block];
    }       

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_many_blocks() {
        let mut allocator = CopyingHeap::<96, 12>::new();
        let mut tracer = TestTracer::default();
        for request in [2, 10, 4, 8, 6, 12, 6, 24, 4, 8, 2, 8] {
            tracer.allocate_next(request, &mut allocator).unwrap();
        }
        assert_eq!(tracer.len(), allocator.num_allocated_blocks());
        println!("{:?}",allocator);
        assert!(tracer.matches(&allocator));

        assert_eq!(tracer.total_allocated(), 94);

        match tracer.allocate_next(1, &mut allocator) {
            HeapResult::Ok(_) => panic!("Should be an error!"),
            HeapResult::Err(e) => assert_eq!(e, HeapError::OutOfBlocks),
        }
        
        tracer.test_in_bounds(&mut allocator);
        for _ in 0..(tracer.len() / 2) {
            tracer.deallocate_next_even();
        }
        
        assert!(tracer.matches(&allocator));
        assert_eq!(tracer.total_allocated(), 24);

        tracer.test_in_bounds(&mut allocator);
        println!("{:?}",allocator);
        tracer.allocate_next(4, &mut allocator).unwrap();
        assert_eq!(tracer.len(), allocator.num_allocated_blocks());

        tracer.test_in_bounds(&mut allocator);

        tracer.allocate_next(68, &mut allocator).unwrap();
        assert!(tracer.matches(&allocator));
        assert_eq!(tracer.total_allocated(), 96);

        match tracer.allocate_next(1, &mut allocator) {
            HeapResult::Ok(_) => panic!("Should be an error!"),
            HeapResult::Err(e) => assert_eq!(e, HeapError::OutOfMemory),
        }

        tracer.test_in_bounds(&mut allocator);
    }

    #[test]
    fn test_countdown_allocations() {
        const NUM_WORDS: usize = 1024;
        let mut allocator = CopyingHeap::<NUM_WORDS, NUM_WORDS>::new();
        let mut tracer = CountdownTracer::new(362, &mut allocator);
        while tracer.counts > 0 {
            tracer.iterate(&mut allocator);
        }
    }

    struct CountdownTracer {
        counts: u64,
        count_ptr: Option<Pointer>,
    }

    impl Tracer for CountdownTracer {
        fn trace(&self, blocks_used: &mut [bool]) {
            self.count_ptr.map(|p| {
                blocks_used[p.block_num()] = true;
            });
        }
    }

    impl CountdownTracer {
        fn new<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(start: u64, allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>) -> Self {
            let mut result = Self {counts: start, count_ptr: None};
            let literal_ptr = allocator.malloc(1, &mut result).unwrap();
            allocator.store(literal_ptr, start).unwrap();
            let stored = allocator.load(literal_ptr).unwrap();
            let count_ptr = allocator.malloc(1, &mut result).unwrap();
            allocator.store(count_ptr, stored).unwrap();
            result.count_ptr = Some(count_ptr);
            result
        }

        fn iterate<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(&mut self, allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>) {
            let p = allocator.malloc(1, self).unwrap();
            allocator.store(p, 0).unwrap();
            let count = allocator.load(self.count_ptr.unwrap()).unwrap();
            assert_eq!(count, self.counts);
            let zero = allocator.load(p).unwrap();
            assert_eq!(0, zero);
            let p = allocator.malloc(1, self).unwrap();
            allocator.store(p, 18446744073709551615).unwrap();
            let p = allocator.malloc(1, self).unwrap();
            allocator.store(p, 1).unwrap();
            
            println!("{:?}",MAX_BLOCKS);
            println!("looking up {:?}", self.count_ptr.unwrap());
            let count = allocator.load(self.count_ptr.unwrap()).unwrap();
            assert_eq!(count, self.counts);
            let drop = allocator.load(p).unwrap();
            self.counts -= drop;
            let p = allocator.malloc(1, self).unwrap();
            allocator.store(p, self.counts).unwrap();
            self.count_ptr = Some(p);
        }
    }

    #[derive(Default, Debug)]
    struct TestTracer {
        allocations: VecDeque<Pointer>,
    }

    impl Tracer for TestTracer {
        fn trace(&self, blocks_used: &mut [bool]) {
            for p in self.allocations.iter() {
                blocks_used[p.block_num()] = true;
            }
        }
    }

    impl TestTracer {
        fn matches<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &self,
            allocator: &CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) -> bool {
            for p in self.allocations.iter() {
                if !allocator.is_allocated(p.block_num()) || allocator.size_of(p.block_num()) != p.len() {
                    return false;
                }
            }
            true
        }

        fn allocate_next<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &mut self,
            request: usize,
            allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) -> HeapResult<()> {
            match allocator.malloc(request, self) {
                HeapResult::Ok(p) => {
                    self.allocations.push_back(p);
                    HeapResult::Ok(())
                }
                HeapResult::Err(e) => HeapResult::Err(e),
            }
        }

        fn deallocate_next_even(&mut self) {
            if self.allocations.len() >= 2 {
                let popped = self.allocations.pop_front().unwrap();
                self.allocations.pop_front().unwrap();
                self.allocations.push_back(popped);
            }
        }

        fn len(&self) -> usize {
            self.allocations.len()
        }

        fn total_allocated(&self) -> usize {
            self.allocations.iter().map(|p| p.len()).sum()
        }

        fn test_in_bounds<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &self,
            allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) {
            let mut value = 0;
            for p in self.allocations.iter() {
                let len = p.len();
                let mut p = Some(*p);
                for _ in 0..len {
                    let pt = p.unwrap();
                    allocator.store(pt, value).unwrap();
                    assert_eq!(value, allocator.load(pt).unwrap());
                    value += 1;
                    p = pt.next();
                }
            }

            value = 0;
            for p in self.allocations.iter() {
                let len = p.len();
                let mut p = Some(*p);
                for _ in 0..len {
                    let pt = p.unwrap();
                    assert_eq!(value, allocator.load(pt).unwrap());
                    value += 1;
                    p = pt.next();
                }
            }
        }
    }
}
