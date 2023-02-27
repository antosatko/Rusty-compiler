pub mod test {
    use crate::runtime::runtime_types::{Context, Instructions::*, Types::*};

    const ID: usize = 5;
    pub fn test_init(id: Option<usize>, context: &mut Context) {
        let test_id = if let Some(num) = id { num } else { ID };
        match test_id {
            0 => {
                context.stack = vec![Int(0)];
                context.code = vec![End];
            }
            1 => {
                // equivalent to for loop
                context.stack = vec![
                    // initialization of values on stack
                    Int(100), // initial value
                    Usize(1), // post-process data report key
                    Bool(true), // dunno why thats here
                    Int(5000000), // max value
                    Int(1), // step
                ];
                context.code = vec![
                    // reserves memory on stack
                    Res(5),
                    // reads values from stack
                    Rd(4, 0), // current idx
                    Rd(0, 1), // add value
                    // writes result of their addition
                    Add,
                    Wr(4), // write idx to mem
                    // repeats if number is less than stack(1)
                    Rd(1, 1), // max number
                    Less,
                    Brnc(1, 8),
                    // prints end result
                    Rd(4, 0), // idx
                    Debug(0),
                    Rd(3, 0),
                    End,
                ];
            }
            2 => {
                // writing and reading with heap
                context.stack = vec![
                    // size we want to allocate
                    Usize(1),
                    // value we want to store
                    Int(50),
                    // placeholder for pointer
                    Null,
                    // placeholder to write value from heap to
                    Bool(false),
                ];
                context.code = vec![
                    // boilerplate stack reservation
                    Res(4),
                    // reading size, allocationg and storing pointer on stack
                    Rd(3, 0),
                    Alc(0, 0),
                    Wr(1),
                    // reading value to write on heap and writing it
                    Rd(2, 1),
                    Wrp(0, 1),
                    // reading from heap and storing its value on stack
                    Rdp(1, 0),
                    Mov(0, 1),
                    Wr(0),
                    // reading stored value and printing it
                    Rd(0, 0),
                    Debug(0),
                    // trigger post-process data report
                    Rd(3, 0),
                    End,
                ];
            }
            3 => {
                // heap commands test
                context.stack = vec![
                    // size we want to allocate
                    Usize(1),
                    // value we want to store
                    Int(50),
                    // placeholder for pointer
                    Null,
                    // placeholder to write value from heap to
                    Bool(false),
                ];
                context.code = vec![
                    //------------------------------------------------
                    // exact copy of test 2 to setup heap
                    //------------------------------------------------
                    // boilerplate stack reservation
                    Res(4),
                    // reading size, allocationg and storing pointer on stack
                    Rd(3, 0),
                    Alc(0, 0),
                    Wr(1),
                    // reading value to write on heap and writing it
                    Rd(2, 1),
                    Wrp(0, 1),
                    // reading from heap and storing its value on stack
                    Rdp(1, 0),
                    Mov(0, 1),
                    Wr(0),
                    // reading stored value and printing it
                    Rd(0, 0),
                    Debug(0),
                    //---------------------------------------------
                    // here starts the real test
                    //---------------------------------------------
                    // adding Usize(1) + Usize(1)
                    Rd(3, 0),
                    Rd(3, 1),
                    Add,
                    // reading pointer and reallocating it to fit new size Usize(2)
                    Rd(1, 1),
                    RAlc(1, 0),
                    // load idx and create new pointer pointing at the correct place
                    Rd(1,0),
                    Idx(1, 0),
                    // write to the pointer
                    Rd(3, 0),
                    Wrp(1, 0),
                    // read from it using the already constructed index and printing value
                    Rdp(0, 1),
                    Debug(0),
                    // freeing allocated memory
                    Rd(1, 0),
                    Debug(0),
                    Dalc(0),
                    // trigger post-process data report
                    Rd(3, 0),
                    End,
                ];
            }
            4 => {
                context.stack = vec![Int(0), Usize(0)];
                context.code = vec![
                    Rdc(0,0),
                    Rdc(1,1),
                    Add,
                    Debug(0),
                    End
                ];
            }
            5 => {
                context.stack = vec![Int(0)];
                context.code = vec![
                    Ptr(0),
                    Dalc(0),
                    Debug(0),
                    End
                ];
            }
            _ => {
                context.stack = vec![Int(0)];
                context.code = vec![End];
                println!("Test id: {test_id} not found.");
            }
        }
    }
}
