pub mod runtime {
    use core::panic;

    use super::runtime_error::*;
    use super::runtime_types::*;

    impl Context {
        pub fn new() -> Self {
            Self {
                stack: vec![],
                call_stack: [CallStack {
                    end: 0,
                    code_ptr: 0,
                    reg_freeze: [Types::Null; 4],
                }; 100],
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
                stack_ptr: 0,
            }
        }
        pub fn run(&mut self) -> bool {
            while self.read_line() {}
            return true;
        }
        pub fn run_for(&mut self, lenght: usize) -> bool {
            for _ in 0..lenght {
                if !self.read_line() {
                    return true;
                }
            }
            return false;
        }
        fn read_line(&mut self) -> bool {
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
                Wrp(pointer_reg, value_reg) => {
                    if let Types::Pointer(u_size, kind) = self.registers[pointer_reg] {
                        match kind {
                            PointerTypes::Stack => {
                                self.stack[u_size] = self.registers[value_reg];
                            }
                            PointerTypes::Heap => {
                                self.heap[u_size] = self.registers[value_reg];
                            }
                            PointerTypes::HeapReg => {
                                if let Some((_, heap_pos)) = self.heap_reg_idx(u_size) {
                                    self.heap[heap_pos] = self.registers[value_reg];
                                } else {
                                    panic!("Pointer trying to write to non-existent adress")
                                }
                            }
                        }
                    } else {
                        // runtime err: Pointer must be of type 'Pointer'
                    }
                    self.next_line();
                }
                Rdp(cash_reg, pointer_reg) => {
                    if let Types::Pointer(u_size, kind) = self.registers[pointer_reg] {
                        match kind {
                            PointerTypes::Stack => {
                                self.registers[cash_reg] = self.stack[u_size];
                            }
                            PointerTypes::Heap => {
                                self.registers[cash_reg] = self.heap[u_size];
                            }
                            PointerTypes::HeapReg => {
                                if let Some((_, heap_pos)) = self.heap_reg_idx(u_size) {
                                    self.registers[pointer_reg] = self.heap[heap_pos];
                                } else {
                                    panic!("Pointer trying to read from non-existent adress")
                                }
                            }
                        }
                    } else {
                        return panic_rt(ErrTypes::InvalidType(
                            self.registers[pointer_reg],
                            String::from("Pointer"),
                        ));
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
                                self.registers[reg] = Types::Pointer(loc, PointerTypes::Heap);
                            }
                        }
                    }
                    self.next_line();
                }
                Idx(pointer_reg, increment_by_reg) => {
                    if let Types::Pointer(u_size, kind) = self.registers[pointer_reg] {
                        if let PointerTypes::HeapReg = kind {
                            if let Some((_, loc)) = self.heap_reg_idx(u_size) {
                                if let Types::Usize(size) = self.registers[increment_by_reg] {
                                    self.registers[pointer_reg] =
                                        Types::Pointer(loc + size, PointerTypes::Heap);
                                }
                            }
                        } else {
                            if let Types::Pointer(u_size2, kind2) = self.registers[increment_by_reg]
                            {
                                self.registers[pointer_reg] =
                                    Types::Pointer(u_size + u_size2, kind2);
                            }
                        }
                    }
                    self.next_line();
                }
                Alc(reg, size_reg) => {
                    if let Some(size) = self.heap_alloc(self.registers[size_reg]) {
                        self.registers[reg] = Types::Pointer(size, PointerTypes::HeapReg);
                    } else {
                        self.registers[reg] = Types::Null;
                        //
                    }
                    self.next_line();
                }
                Dalc(reg) => {
                    if let Types::Pointer(u_size, pointer_type) = self.registers[reg] {
                        match pointer_type {
                            PointerTypes::Heap => {
                                // runtime err: only raw pointers can be deleted
                            }
                            PointerTypes::HeapReg => {
                                self.heap_reg_del(u_size);
                            }
                            PointerTypes::Stack => {
                                // runtime err: can not delete stack pointer
                            }
                        }
                    }
                    self.next_line();
                }
                RAlc(pointer_reg, size_reg) => {
                    if let Types::Pointer(u_size, _) = self.registers[pointer_reg] {
                        if let Types::Usize(new_size) = self.registers[size_reg] {
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
                Gotop(pos) => {
                    if let Types::CodePointer(u_size) = self.registers[pos] {
                        self.code_ptr = u_size
                    } else {
                        return panic_rt(ErrTypes::InvalidType(
                            self.registers[pos],
                            String::from("Function"),
                        ));
                    }
                }
                Brnc(pos1, pos2) => {
                    if let Types::Bool(bool) = self.registers[0] {
                        self.code_ptr = if bool { pos1 } else { pos2 };
                    }
                }
                Ret => {
                    self.code_ptr = self.call_stack[self.stack_ptr].code_ptr;
                    self.stack_ptr -= 1;
                    self.next_line();
                }
                RRet => {
                    self.registers
                        .copy_from_slice(&self.call_stack[self.stack_ptr + 1].reg_freeze);
                    self.next_line();
                }
                Res(size) => {
                    let end = self.stack_end() + size - 1;
                    self.stack_ptr += 1;
                    self.call_stack[self.stack_ptr].end = end;
                    self.call_stack[self.stack_ptr].code_ptr = self.code_ptr;
                    self.call_stack[self.stack_ptr]
                        .reg_freeze
                        .copy_from_slice(&self.registers);
                    if end > self.stack_ptr {
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
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 + num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 + num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 + num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-numeric types.")
                        }
                    }
                    self.next_line();
                }
                Sub => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Int(num1 - num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 - num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 - num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 - num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-numeric types.")
                        }
                    }
                    self.next_line();
                }
                Mul => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Int(num1 * num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 * num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 * num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 * num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-numeric types.")
                        }
                    }
                    self.next_line();
                }
                Div => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Int(num1 / num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 / num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 / num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 / num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-numeric types.")
                        }
                    }
                    self.next_line();
                }
                Mod => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Int(num1 % num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Float(num1 % num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Byte(num1 % num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Usize(num1 % num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {
                            panic!("Can not perform math operations on non-numeric types.")
                        }
                    }
                    self.next_line();
                }
                Equ => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Pointer(num1, _) => {
                            if let Types::Pointer(num2, _) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 == num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Bool(var1) => {
                            if let Types::Bool(var2) = self.registers[1] {
                                self.registers[0] = Types::Bool(var1 == var2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Char(char1) => {
                            if let Types::Char(char2) = self.registers[1] {
                                self.registers[0] = Types::Bool(char1 == char2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                Grt => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 > num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 > num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 > num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 > num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Char(char1) => {
                            if let Types::Char(char2) = self.registers[1] {
                                self.registers[0] = Types::Bool(char1 > char2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                Less => {
                    match self.registers[0] {
                        Types::Int(num1) => {
                            if let Types::Int(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 < num2);
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Float(num1) => {
                            if let Types::Float(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 < num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Byte(num1) => {
                            if let Types::Byte(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 < num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Usize(num1) => {
                            if let Types::Usize(num2) = self.registers[1] {
                                self.registers[0] = Types::Bool(num1 < num2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        Types::Char(char1) => {
                            if let Types::Char(char2) = self.registers[1] {
                                self.registers[0] = Types::Bool(char1 < char2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                And => {
                    match self.registers[0] {
                        Types::Bool(var1) => {
                            if let Types::Bool(var2) = self.registers[1] {
                                self.registers[0] = Types::Bool(var1 && var2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                Or => {
                    match self.registers[0] {
                        Types::Bool(var1) => {
                            if let Types::Bool(var2) = self.registers[1] {
                                self.registers[0] = Types::Bool(var1 || var2)
                            } else {
                                return panic_rt(ErrTypes::CrossTypeOperation(
                                    self.registers[0],
                                    self.registers[1],
                                    self.code[self.code_ptr],
                                ));
                            }
                        }
                        _ => {}
                    }
                    self.next_line();
                }
                Not => {
                    match self.registers[0] {
                        Types::Bool(var) => self.registers[0] = Types::Bool(!var),
                        _ => {
                            return panic_rt(ErrTypes::WrongTypeOperation(
                                self.registers[0],
                                self.code[self.code_ptr],
                            ));
                        }
                    }
                    self.next_line();
                }
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
        fn stack_end(&self) -> usize {
            self.call_stack[self.stack_ptr].end
        }
        fn next_line(&mut self) {
            self.code_ptr += 1;
        }
        /// creates object on heap of specified size
        fn heap_alloc(&mut self, size: Types) -> Option<usize> {
            return if let Types::Usize(s) = size {
                self.heap_reg_push(s);
                self.heap.resize(self.heap.len() + s, Types::Null);
                Some(self.heap_reg_len() - 1)
            } else {
                // runtime err: size of heap-allocated objects must be of type Usize
                None
            };
        }
        /// ramoves object from heap
        fn heap_reg_del(&mut self, idx: usize) {
            if let Some((index, heap_loc)) = self.heap_reg_idx(idx) {
                let heap_range = heap_loc..self.heap_registry[index].len + heap_loc;
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
        /// returns index in heap registry and index in heap
        fn heap_reg_idx(&self, idx: usize) -> Option<(usize, usize)> {
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
                    hr_path.1 += self.heap_registry[hr_path.0].len;
                    hr_path.0 = next;
                } else {
                    return None;
                }
                i += self.heap_registry[hr_path.0].dels + 1;
            }
            Some(hr_path)
        }
        /// creates representative for heap-allocated-objects on heap_reg
        fn heap_reg_push(&mut self, len: usize) {
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

        fn heap_reg_len(&self) -> usize {
            let mut len = 0;
            for reg in self.heap_registry.iter() {
                len += reg.dels + 1;
            }
            len
        }
    }
}

pub mod runtime_error {
    use super::runtime_types::*;
    pub enum ErrTypes {
        CrossTypeOperation(Types, Types, Instructions),
        WrongTypeOperation(Types, Instructions),
        InvalidType(Types, String),
    }
    pub fn panic_rt(kind: ErrTypes) -> bool {
        match kind {
            ErrTypes::CrossTypeOperation(var1, var2, instr) => {
                println!(
                    "Operation '{:?}' failed: Cross-type operation {:?}, {:?}",
                    instr, var1, var2
                )
            }
            ErrTypes::WrongTypeOperation(var1, instr) => {
                println!(
                    "Operation '{:?}' failed: Wrong-type operation {:?}",
                    instr, var1
                )
            }
            ErrTypes::InvalidType(typ, operation) => {
                println!("Invalid Type: {:?} must be of type '{:?}'", typ, operation)
            }
        }
        false
    }
}

pub mod runtime_types {
    /// context for a single thread of execution (may include multiple threads in future updates)
    pub struct Context {
        pub stack: Vec<Types>,
        pub call_stack: [CallStack; 100],
        pub stack_ptr: usize,
        pub registers: [Types; 4],
        pub code: Vec<Instructions>,
        pub code_ptr: usize,
        pub heap: Vec<Types>,
        pub heap_registry: Vec<HeapRegistry>,
    }
    /// a structure used to register data on heap
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
        CodePointer(usize),
        Null,
    }
    /// runtime
    #[derive(Clone, Copy, Debug)]
    pub enum PointerTypes {
        /// location on stack
        ///
        /// never expires
        Stack,
        /// heap pointer in "broken state"
        /// needs to be transformed into heap pointer
        ///
        /// never expires
        HeapReg,
        /// location on heap
        ///
        /// may expire at any time
        Heap,
    }
    /// complete list of runtime instructions
    #[allow(unused)]
    #[derive(Clone, Copy, Debug)]
    pub enum Instructions {
        /// debug reg        | prints value of reg(<reg>)
        Debug(usize),
        /// write stack_offset           | moves value from reg(0) to stack(stack_end - <stack_offset>)
        Wr(usize),
        /// read stack_offset reg | reads value from stack(stack_end - <stack_offset>) to its reg(<reg>)
        Rd(usize, usize),
        /// write pointer    | moves value from reg(<reg2>) to stack(<reg1>)
        Wrp(usize, usize),
        /// read_pointer pointer_reg reg | reads value from stack(reg1) to its reg(<reg2>)
        Rdp(usize, usize),
        /// read constant    | reads value from stack(<stack_pos>) to its reg(<reg>)
        Rdc(usize, usize),
        /// pointer stack_pos | stores pointer to stack(stack_end - <stack_offset>) in reg(0)
        Ptr(usize),
        /// Index pointer idx<usize> | gets pointer from reg(<reg1>) repairs it and adds reg(<reg2>)
        Idx(usize, usize),
        /// Repair pointer   | Repairs pointer in reg(<reg>)
        Repp(usize),
        /// allocate pointer size_reg | reserves <size> on heap and stores location in registers(<reg>)
        Alc(usize, usize),
        /// deallocate pointer | frees heap(<reg>)
        Dalc(usize),
        /// reallocate pointer size_reg | resizes heap(<reg>) for <size>; additional space is filled with null
        RAlc(usize, usize),
        /// goto pos         | moves code_pointer to <pos>
        Goto(usize),
        /// goto pos_reg     | moves code pointer to reg(<reg>)
        Gotop(usize),
        /// branch pos1 pos2 | if reg(0), goto <pos1> else goto <pos2>
        Brnc(usize, usize),
        /// return           | moves code_pointer to the last position in stack retrieved from stack
        Ret,
        /// register return  | returns registers to their freezed previous state
        RRet,
        /// reserve size     | reserves <size> on stack and saves current reg(0)
        Res(usize),
        /// move reg1 reg2   | moves value of <reg1> to <reg2>
        Mov(usize, usize),
        /// add              | reg(0) is set to the result of operation: reg(0) + reg(1)
        Add,
        /// subtract         | reg(0) is set to the result of operation: reg(0) - reg(1)
        Sub,
        /// multiply         | reg(0) is set to the result of operation: reg(0) * reg(1)
        Mul,
        /// divide           | reg(0) is set to the result of operation: reg(0) / reg(1)
        Div,
        /// modulus          | reg(0) is set to the result of operation: reg(0) % reg(1)
        Mod,
        /// equals           | reg(0) is set to the result of operation: reg(0) = reg(1)
        Equ,
        /// greater than     | reg(0) is set to the result of operation: reg(0) > reg(1)
        Grt,
        /// less than        | reg(0) is set to the result of operation: reg(0) < reg(1)
        Less,
        /// and              | reg(0) is set to the result of operation: reg(0) & reg(1)
        And,
        /// or               | reg(0) is set to the result of operation: reg(0) | reg(1)
        Or,
        /// not              | reg(0) is set to the result of operation: !reg(0)
        Not,
        /// call             | calls external <procedure>(program state, <args>) written in rust (for syscalls etc..)
        Cal(usize, usize),
        /// end              | terminates program
        End,
    }
    /// holds information of where to jump after function call ends
    #[derive(Clone, Copy)]
    pub struct CallStack {
        pub reg_freeze: [Types; 4],
        pub end: usize,
        pub code_ptr: usize,
    }
}
