pub mod runtime {
    use core::panic;

    use super::runtime_types::*;

    impl Context {
        pub fn new() -> Self {
            Self {
                stack: vec![],
                call_stack: vec![CallStack {
                    end: 0,
                    code_ptr: 0,
                }],
                registers: [Types::Null; 4],
                code: vec![],
                code_ptr: 0,
                heap: vec![],
                heap_registry: vec![HeapRegistry {
                    prev: None,
                    next: None,
                    len: 0,
                    dels: 0,
                }],
            }
        }
        pub fn run(&mut self) {
            while self.read_line() {}
            //println!("{:?}", self.heap);
        }
        pub fn read_line(&mut self) -> bool {
            use Instructions::*;
            match self.code[self.code_ptr] {
                Wr(stack_offset) => {
                    let end = self.stack_end();
                    self.stack[end - stack_offset] = self.registers[0];
                    self.next_line();
                }
                Rd(stack_offset, reg) => {
                    let end = self.stack_end();
                    self.registers[reg] = self.stack[end - stack_offset];
                    self.next_line();
                }
                Wrp(reg1, reg2) => {
                    if let Types::Pointer(u_size, kind) = self.registers[reg1] {
                        match kind {
                            PointerTypes::Stack => {
                                self.stack[u_size] = self.registers[reg2];
                            }
                            PointerTypes::Heap => {
                                self.heap[u_size] = self.registers[reg2];
                            }
                            PointerTypes::HeapReg => {
                                if let Some((_, heap_pos)) = self.heap_reg_idx(u_size) {
                                    self.heap[heap_pos] = self.registers[reg2];
                                    // should never be used, but I will include it just in case
                                } else {
                                    panic!("Somehow you just managed to use broken pointer on feature, that shouldnt even exist. wow")
                                }
                            }
                        }
                    } else {
                        panic!("Pointer must be of type 'Pointer'")
                    }
                    self.next_line();
                }
                Rdp(reg1, reg2) => {
                    if let Types::Pointer(u_size, kind) = self.registers[reg2] {
                        match kind {
                            PointerTypes::Stack => {
                                self.registers[reg1] = self.stack[u_size];
                            }
                            PointerTypes::Heap => {
                                self.registers[reg1] = self.heap[u_size];
                            }
                            PointerTypes::HeapReg => {
                                if let Some((_, heap_pos)) = self.heap_reg_idx(u_size) {
                                    self.registers[reg2] = self.heap[heap_pos];
                                    // should never be used, but I will include it just in case
                                } else {
                                    panic!("Somehow you just managed to use broken pointer on feature, that shouldnt even exist. wow")
                                }
                                todo!()
                            }
                        }
                    } else {
                        panic!("Pointer must be of type 'Pointer'")
                    }
                    self.next_line();
                }
                Rdc(stack_pos, reg) => {
                    self.registers[reg] = self.stack[stack_pos];
                    self.next_line();
                }
                Ptr(stack_offset) => {
                    self.registers[0] =
                        Types::Pointer(self.stack_end() - stack_offset, PointerTypes::Stack);
                    self.next_line();
                }
                Repp(reg) => {
                    if let Types::Pointer(u_size, kind) = self.registers[reg] {
                        if let PointerTypes::HeapReg = kind {
                            if let Some((_, loc)) = self.heap_reg_idx(u_size) {
                                //println!("{:?}", self.heap_registry);
                                //println!("{loc}");
                                self.registers[reg] = Types::Pointer(loc, PointerTypes::Heap);
                            }
                        }
                    }
                    self.next_line();
                }
                Idx(reg1, reg2) => {
                    if let Types::Pointer(u_size, kind) = self.registers[reg1] {
                        if let PointerTypes::HeapReg = kind {
                            if let Some((_, loc)) = self.heap_reg_idx(u_size) {
                                if let Types::Pointer(u_size2, kind2) = self.registers[reg2] {
                                    self.registers[reg1] = Types::Pointer(loc + u_size2, kind2);
                                }
                            }
                        } else {
                            if let Types::Pointer(u_size2, kind2) = self.registers[reg2] {
                                self.registers[reg1] = Types::Pointer(u_size + u_size2, kind2);
                            }
                        }
                    }
                    self.next_line();
                }
                Alc(reg, size_reg) => {
                    self.registers[reg] = Types::Pointer(
                        self.heap_push(self.registers[size_reg]),
                        PointerTypes::HeapReg,
                    );
                    self.next_line();
                }
                Dalc(reg) => {
                    if let Types::Pointer(u_size, _) = self.registers[reg] {
                        self.heap_reg_del(u_size);
                    }
                    self.next_line();
                }
                RAlc(reg, size) => {
                    if let Types::Pointer(u_size, _) = self.registers[reg] {
                        if let Types::Usize(new_size) = self.registers[size] {
                            if let Some((idx, loc)) = self.heap_reg_idx(u_size) {
                                while new_size > self.heap_registry[idx].len {
                                    self.heap
                                        .insert(loc + self.heap_registry[idx].len, Types::Null);
                                    self.heap_registry[idx].len += 1;
                                }
                                if new_size < self.heap_registry[idx].len {
                                    self.heap.drain(
                                        (new_size + loc)..(self.heap_registry[idx].len + loc),
                                    );
                                }
                            }
                        }
                    }
                    self.next_line();
                }
                Goto(pos) => {
                    self.code_ptr = pos;
                }
                Brnc(pos1, pos2) => {
                    if let Types::Bool(bool) = self.registers[0] {
                        self.code_ptr = if bool { pos1 } else { pos2 };
                    }
                }
                Ret => {
                    self.code_ptr = self.call_stack[self.call_stack.len() - 1].code_ptr;
                    self.call_stack.pop();
                    self.next_line();
                }
                Res(size) => {
                    let end = self.stack_end() + size - 1;
                    self.call_stack.push(CallStack {
                        end,
                        code_ptr: self.code_ptr,
                    });
                    if end > self.stack.len() {
                        self.stack.resize(end + 1, Types::Null);
                    }
                    self.next_line();
                }
                Mov(reg1, reg2) => {
                    let temp = self.registers[reg1];
                    self.registers[reg1] = self.registers[reg2];
                    self.registers[reg2] = temp;
                    self.next_line();
                }
                Add => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Int(num1 + num2);
                            } else {
                                panic!(
                                    "Operation '{:?}' failed: Cross-type operation {:?}, {:?}",
                                    self.code[self.code_ptr], self.registers[0], self.registers[1]
                                );
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 + num2)
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 + num2)
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 + num2)
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-number values.")
                        }
                    }
                    self.next_line();
                }
                Sub => {}
                Mul => {}
                Div => {}
                Mod => {}
                Equ => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2);
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            }
                        }
                        Types::Pointer(num1, _) => {
                            if let Types::Pointer(num2, _) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            }
                        }
                        Types::Bool(var1) => {
                            if let Types::Bool(var2) = self.registers[1] {
                                self.registers[0] = Types::Bool(var1 == var2)
                            }
                        }
                        Types::Char(char1) => {
                            if let Types::Char(char2) = self.registers[1] {
                                self.registers[0] = Types::Bool(char1 == char2)
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                Grt => {}
                Less => {}
                And => {}
                Or => {}
                Not => {}
                Cal(procedure, args) => {}
                End => {
                    return false;
                }
                Debug(reg) => {
                    println!("{:?}", self.registers[reg]);
                    self.next_line();
                }
            }
            return true;
        }
        pub fn stack_end(&self) -> usize {
            self.call_stack[self.call_stack.len() - 1].end
        }
        pub fn next_line(&mut self) {
            self.code_ptr += 1;
        }
        pub fn heap_push(&mut self, size: Types) -> usize {
            return if let Types::Usize(s) = size {
                self.heap_reg_push(s);
                self.heap.resize(self.heap.len() + s, Types::Null);
                self.heap_reg_len() - 1
            } else {
                panic!("")
            };
        }
        pub fn heap_reg_del(&mut self, idx: usize) {
            if let Some((index, loc)) = self.heap_reg_idx(idx) {
                let heap_range = loc..self.heap_registry[index].len;
                self.heap.drain(heap_range);
                if let Some(prev) = self.heap_registry[index].prev {
                    self.heap_registry[prev].next = self.heap_registry[index].next;
                    self.heap_registry[prev].dels += self.heap_registry[index].dels + 1;
                }
                if let Some(next) = self.heap_registry[index].next {
                    self.heap_registry[next].prev = self.heap_registry[index].prev;
                }
                self.heap_registry.remove(index);
            }
        }
        pub fn heap_reg_idx(&self, idx: usize) -> Option<(usize, usize)> {
            if self.heap_registry.len() == 0 {
                return None;
            }
            let mut hr_path = (0, 0);
            // find the first elem of linked list
            loop {
                if let None = self.heap_registry[hr_path.0].prev {
                    break;
                }
                hr_path.0 += 1;
            }

            let mut i = 0;
            while i < idx {
                if let Some(next) = self.heap_registry[hr_path.0].next {
                    //println!("{} {}", hr_path.1, self.heap_registry[hr_path.0].len);
                    hr_path.0 = next;
                    hr_path.1 += self.heap_registry[hr_path.0].len;
                } else {
                    return None;
                }
                i += self.heap_registry[hr_path.0].dels + 1;
            }
            hr_path.1 -= 1;
            //println!("{:?}", hr_path);
            Some(hr_path)
        }
        pub fn heap_reg_push(&mut self, len: usize) {
            let reg_len = self.heap_registry.len();
            for (idx, node) in self.heap_registry.iter_mut().enumerate() {
                if let None = node.next {
                    node.next = Some(reg_len);
                    self.heap_registry.push(HeapRegistry {
                        prev: Some(idx),
                        next: None,
                        len,
                        dels: 0,
                    });
                    return;
                }
            }
            self.heap_registry.push(HeapRegistry {
                prev: None,
                next: None,
                len,
                dels: 0,
            });
        }
        pub fn heap_reg_len(&self) -> usize {
            let mut len = 0;
            for reg in self.heap_registry.iter() {
                len += reg.dels + 1;
            }
            len
        }
    }
}

pub mod runtime_types {
    pub struct Context {
        pub stack: Vec<Types>,
        pub call_stack: Vec<CallStack>,
        pub registers: [Types; 4],
        pub code: Vec<Instructions>,
        pub code_ptr: usize,
        pub heap: Vec<Types>,
        pub heap_registry: Vec<HeapRegistry>,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct HeapRegistry {
        pub prev: Option<usize>,
        pub next: Option<usize>,
        pub len: usize,
        pub dels: usize,
    }
    #[allow(unused)]
    #[derive(Clone, Copy, Debug)]
    pub enum Types {
        Int(i32),
        Float(f64),
        Usize(usize),
        Char(char),
        Byte(u8),
        Bool(bool),
        Pointer(usize, PointerTypes),
        Null,
    }
    #[derive(Clone, Copy, Debug)]
    pub enum PointerTypes {
        Stack,
        HeapReg,
        Heap,
    }
    #[allow(unused)]
    #[derive(Clone, Copy, Debug)]
    pub enum Instructions {
        Debug(usize),
        Wr(usize),
        Rd(usize, usize),
        Wrp(usize, usize),
        Rdp(usize, usize),
        Rdc(usize, usize),
        Ptr(usize),
        Idx(usize, usize),
        Repp(usize),
        Alc(usize, usize),
        Dalc(usize),
        RAlc(usize, usize),
        Goto(usize),
        Brnc(usize, usize),
        Ret,
        Res(usize),
        Mov(usize, usize),
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        Equ,
        Grt,
        Less,
        And,
        Or,
        Not,
        Cal(usize, usize),
        End,
    }
    pub struct CallStack {
        pub end: usize,
        pub code_ptr: usize,
    }
}