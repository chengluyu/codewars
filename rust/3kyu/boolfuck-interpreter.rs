const MASK_BEGIN: u8 = 0b_0000_0001_u8;

const MASK_END: u8 = 0b_1000_0000_u8;

struct Pointer {
    index: usize,
    mask: u8,
}

impl Pointer {
    fn new() -> Pointer {
        Pointer {
            index: 0,
            mask: MASK_BEGIN,
        }
    }

    #[inline]
    fn get(&self, data: &Vec<u8>) -> bool {
        data[self.index] & self.mask > 0
    }

    #[inline]
    fn set(&self, value: bool, data: &mut Vec<u8>) {
        if value {
            data[self.index] |= self.mask;
        } else {
            data[self.index] &= !self.mask;
        }
    }

    #[inline]
    fn flip(&self, data: &mut Vec<u8>) {
        data[self.index] ^= self.mask;
    }

    #[inline]
    fn move_near(&mut self) -> bool {
        if self.mask == MASK_BEGIN {
            if self.index == 0 {
                true
            } else {
                self.index -= 1;
                self.mask = MASK_END;
                false
            }
        } else {
            self.mask >>= 1;
            false
        }
    }

    #[inline]
    fn move_far_no_check(&mut self) {
        if self.mask == MASK_END {
            self.mask = MASK_BEGIN;
            self.index += 1;
        } else {
            self.mask <<= 1;
        }
    }

    #[inline]
    fn move_far(&mut self, data: &mut Vec<u8>) {
        if self.mask == MASK_END {
            self.mask = MASK_BEGIN;
            self.index += 1;
            if self.index == data.len() {
                data.push(0u8);
            }
        } else {
            self.mask <<= 1;
        }
    }

    #[inline]
    fn shrink_to_fit(&mut self, data: &mut Vec<u8>) {
        if self.mask == MASK_BEGIN {
            self.move_near();
            data.pop();
        }
    }
}

struct Memory {
    data: Vec<u8>,
    pointer: Pointer,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            data: vec![0u8],
            pointer: Pointer::new(),
        }
    }

    #[inline]
    fn get(&self) -> bool {
        self.pointer.get(&self.data)
    }

    #[inline]
    fn set(&mut self, value: bool) {
        self.pointer.set(value, &mut self.data)
    }

    #[inline]
    fn flip(&mut self) {
        self.pointer.flip(&mut self.data)
    }

    #[inline]
    fn move_near(&mut self) -> bool {
        self.pointer.move_near()
    }

    #[inline]
    fn move_far(&mut self) {
        self.pointer.move_far(&mut self.data);
    }


    fn debug(&self) {
        print!("{},{:b}::", self.pointer.index, self.pointer.mask);
        self.data.iter().for_each(|&x| print!("{:b}", x));
    }
}

struct Machine {
    left: Memory,
    right: Memory,
    on_right: bool,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            left: Memory::new(),
            right: Memory::new(),
            on_right: true,
        }
    }

    #[inline]
    fn flip(&mut self) {
        (if self.on_right { &mut self.right } else { &mut self.left }).flip();
    }

    #[inline]
    fn get(&self) -> bool {
        (if self.on_right { &self.right } else { &self.left }).get()
    }

    #[inline]
    fn set(&mut self, value: bool) {
        (if self.on_right { &mut self.right } else { &mut self.left }).set(value);
    }

    #[inline]
    fn move_left(&mut self) {
        if self.on_right {
            if self.right.move_near() {
                self.on_right = false;
            }
        } else {
            self.left.move_far();
        }
    }

    #[inline]
    fn move_right(&mut self) {
        if self.on_right {
            self.right.move_far();
        } else if self.left.move_near() {
            self.on_right = true
        }
    }
}

fn create_jump_table(code: &Vec<char>) -> Vec<Option<usize>> {
    let mut jump_table: Vec<Option<usize>> = Vec::new();
    let mut left_stack: Vec<usize> = Vec::new();
    for (i, &ch) in code.iter().enumerate() {
        match ch {
            '[' => {
                left_stack.push(i);
                jump_table.push(None);
            },
            ']' => {
                let h = left_stack.pop().unwrap();
                jump_table[h] = Some(i);
                jump_table.push(Some(h));
            },
            _ => jump_table.push(None),
        }
    }
    jump_table
}

fn boolfuck(code_str: &str, input: Vec<u8>) -> Vec<u8> {
    let code: Vec<char> = code_str.chars().filter(|&x| match x {
        '+' | ',' | ';' | '<' | '>' | '[' | ']' => true,
        _ => false,
    }).collect();
    let jump_table = create_jump_table(&code);
    let mut machine = Machine::new();
    let mut read_pointer = Pointer::new();
    let mut write_pointer = Pointer::new();
    let mut output: Vec<u8> = vec![0u8];
    let mut pc = 0usize;
    while pc < code.len() {
        let mut should_inc_pc = true;
        match code[pc] {
            '+' => {
                machine.flip();
            },
            ',' => {
                machine.set(read_pointer.get(&input));
                read_pointer.move_far_no_check();
            },
            ';' => {
                write_pointer.set(machine.get(), &mut output);
                write_pointer.move_far(&mut output);
            },
            '<' => { machine.move_left() },
            '>' => { machine.move_right() },
            '[' => if !machine.get() {
                pc = jump_table[pc].unwrap();
            },
            ']' => if machine.get() {
                pc = jump_table[pc].unwrap();
                should_inc_pc = false;
            },
            _ => continue,
        }
        if should_inc_pc {
            pc += 1;
        }
    }
    write_pointer.shrink_to_fit(&mut output);
    output
}
