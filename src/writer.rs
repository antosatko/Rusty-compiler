pub mod writer {
    use crate::runtime::runtime_types::{Instructions, Types};
    pub fn write(code: &Vec<Instructions>, consts: &Vec<Types>) {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create("target.dasm").expect("nevim");
        file.write_all(to_string(code, consts).as_bytes())
            .expect("furt nevim");
    }
    pub fn to_string(code: &Vec<Instructions>, consts: &Vec<Types>) -> String {
        let mut str = String::new();
        let mut i = 0;
        while i < consts.len() {
            if let Types::Char(_) = consts[i] {
                if let Some(string) = get_str(i, &consts) {
                    str.push_str(&string);
                    i += string.len() - 1;
                } else {
                    str.push_str(&val_to_string(consts[i]));
                    i += 1;
                }
            } else {
                str.push_str(&val_to_string(consts[i]));
                i += 1;
            }
        }
        str.push_str("?");
        for instr in code.iter() {
            str.push_str(&instr_to_str(*instr))
        }
        str.push_str("?");
        str
    }
    pub fn get_str(mut index: usize, consts: &Vec<Types>) -> Option<String> {
        let mut str = String::from("\"");
        while let Types::Char(char) = consts[index] {
            if index >= consts.len() {
                break;
            }
            if char == '\0' {
                str.push('"');
                return Some(str);
            }
            index += 1;
            str.push(char);
        }
        None
    }
    pub fn val_to_string(val: Types) -> String {
        use Types::*;
        match val {
            Int(int) => {
                let bytes = unsafe { std::mem::transmute::<i32, u32>(int) };
                format!("{}{:8x}", 65 as char, bytes).replace(" ", "0")
            }
            Float(float) => {
                let bytes = unsafe { std::mem::transmute::<f64, u64>(float) };
                format!("{}{:16x}", 66 as char, bytes).replace(" ", "0")
            }
            Byte(byte) => format!("{}{byte:2x}", 67 as char).replace(" ", "0"),
            Char(char) => format!("{}{:2x}", 68 as char, char as u8).replace(" ", "0"),
            Usize(usize) => format!("{}{usize:32x}", 69 as char).replace(" ", "0"),
            Bool(bool) => {
                let num = if bool { 1 } else { 0 };
                format!("{}{:1x}", 70 as char, num).replace(" ", "0")
            }
            Pointer(_, _) => String::new(),
            Null => format!("{}", 71 as char),
            Enum(offset) => format!("{}{offset:2x}", 72 as char).replace(" ", "0"),
            CodePointer(u_size) => format!("{}{u_size:32x}", 73 as char).replace(" ", "0"),
        }
    }
    pub fn instr_to_str(instr: Instructions) -> String {
        use Instructions::*;
        match instr {
            Wr(n) => format!("{}{}", 65 as char, num_to_hbytes2(n)),
            Rd(n, n1) => format!("{}{}{}", 66 as char, num_to_hbytes2(n), num_to_hbytes1(n1)),
            Wrp(n, n1) => format!("{}{}{}", 67 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Rdp(n, n1) => format!("{}{}{}", 68 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Rdc(n, n1) => format!("{}{}{}", 69 as char, num_to_hbytes2(n), num_to_hbytes1(n1)),
            Ptr(n) => format!("{}{}", 70 as char, num_to_hbytes2(n)),
            Alc(n, n1) => format!("{}{}{}", 71 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Goto(n) => format!("{}{}", 72 as char, num_to_hbytes4(n)),
            Brnc(n, n1) => format!("{}{}{}", 73 as char, num_to_hbytes4(n), num_to_hbytes4(n1)),
            Ret => format!("{}", 74 as char),
            Res(n) => format!("{}{}", 75 as char, num_to_hbytes2(n)),
            Mov(n, n1) => format!("{}{}{}", 76 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Add => format!("{}", 77 as char),
            Sub => format!("{}", 78 as char),
            Mul => format!("{}", 79 as char),
            Div => format!("{}", 80 as char),
            Mod => format!("{}", 81 as char),
            Equ => format!("{}", 82 as char),
            Grt => format!("{}", 83 as char),
            And => format!("{}", 84 as char),
            Or => format!("{}", 85 as char),
            Not => format!("{}", 86 as char),
            Cal(n, n1) => format!("{}{}{}", 87 as char, num_to_hbytes3(n), num_to_hbytes2(n1)),
            End => format!("{}", 88 as char),
            Dalc(n) => format!("{}{}", 89 as char, num_to_hbytes1(n)),
            RAlc(n, n1) => format!("{}{}{}", 90 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Idx(n, n1) => format!("{}{}{}", 91 as char, num_to_hbytes1(n), num_to_hbytes1(n1)),
            Repp(n) => format!("{}{}", 92 as char, num_to_hbytes1(n)),
            Less => format!("{}", 93 as char),
            Debug(n) => format!("{}{}", 94 as char, num_to_hbytes1(n)),
            Gotop(n) => format!("{}{}", 95 as char, num_to_hbytes4(n)),
        }
    }
    pub fn num_to_hbytes1(num: usize) -> String {
        format!("{:1x}", num).replace(" ", "0")
    }
    pub fn num_to_hbytes2(num: usize) -> String {
        format!("{:2x}", num).replace(" ", "0")
    }
    pub fn num_to_hbytes3(num: usize) -> String {
        format!("{:3x}", num).replace(" ", "0")
    }
    pub fn num_to_hbytes4(num: usize) -> String {
        format!("{:4x}", num).replace(" ", "0")
    }
}
