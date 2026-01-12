use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use num_traits::{PrimInt, Unsigned, WrappingAdd, WrappingSub};

trait Cell:
    PrimInt
    + Unsigned
    + Copy
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + Hash
    + WrappingAdd
    + WrappingSub
{
}

impl<
    T: PrimInt
        + Unsigned
        + Copy
        + Display
        + Debug
        + PartialEq
        + PartialOrd
        + Ord
        + Eq
        + Hash
        + WrappingAdd
        + WrappingSub,
> Cell for T
{
}

#[derive(Copy, Clone, Debug)]
enum Program {
    Zero,
    Push,
    Pop,
    Inc,
    Dec,
    OutNum,
    OutChar,
    Label,
    Jz,
    Jnz,
    Cmp,
    Swap,
    Debug,
}

struct Stack<T: Cell> {
    stack: Vec<T>,
}

impl<T: Cell> Stack<T> {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> T {
        self.stack.pop().unwrap_or(T::zero())
    }

    #[inline]
    pub fn swap(&mut self) {
        let a = self.stack.len() - 2;
        let b = a - 1;
        self.stack.swap(a, b);
    }
}

struct Machine<T: Cell> {
    program: Vec<Program>,
    pc: usize,
    jump_table: HashMap<T, usize>,
    stack: Stack<T>,
    accumulator: T,
    compared: bool,
}

impl<T: Cell> Machine<T> {
    pub fn new(program: Vec<Program>) -> Self {
        Self {
            program,
            pc: 0,
            jump_table: HashMap::new(),
            stack: Stack::new(),
            accumulator: T::zero(),
            compared: false,
        }
    }

    #[inline]
    pub fn accumulator(&self) -> &T {
        &self.accumulator
    }

    #[inline]
    pub fn push(&mut self) {
        self.stack.push(self.accumulator);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.accumulator = self.stack.pop()
    }

    #[inline]
    pub fn increment(&mut self) {
        self.accumulator = self.accumulator.wrapping_add(&T::one());
    }

    #[inline]
    pub fn decrement(&mut self) {
        self.accumulator = self.accumulator.wrapping_sub(&T::one());
    }

    #[inline]
    pub fn init_accumulator(&mut self) {
        self.accumulator = T::zero();
    }

    #[inline]
    pub fn label(&mut self) {
        self.jump_table.insert(self.accumulator, self.pc);
    }

    #[inline]
    pub fn swap(&mut self) {
        self.push();
        self.stack.swap();
        self.pop();
    }

    pub fn jump_if_zero(&mut self) {
        if !self.compared {
            return;
        }

        if !self.jump_table.contains_key(self.accumulator()) {
            return;
        }

        self.pc = self.jump_table.get(&self.accumulator).unwrap().clone();
    }

    pub fn jump_if_not_zero(&mut self) {
        if self.compared {
            return;
        }

        if !self.jump_table.contains_key(self.accumulator()) {
            return;
        }

        self.pc = self.jump_table.get(&self.accumulator).unwrap().clone();
    }

    pub fn cmp(&mut self) {
        self.compared = &self.accumulator == self.stack.stack().last().unwrap_or(&T::zero())
    }

    #[inline]
    pub fn print_ascii(&self) {
        print!("{}", self.accumulator.to_u8().unwrap() as char);
    }

    #[inline]
    pub fn print_number(&self) {
        print!("{}", self.accumulator.to_u8().unwrap());
    }

    #[inline]
    fn halt(&self) -> ! {
        std::process::exit(0);
    }

    fn fetch(&self) -> &Program {
        if let Some(p) = self.program.get(self.pc) {
            p
        } else {
            self.halt();
        }
    }

    fn debug(&self) {
        println!("program:");
        for (i, p) in self.program.iter().enumerate() {
            if i == self.pc {
                print!("->");
            } else {
                print!("  ");
            }
            println!("{}: {:?} ", i, p);
        }
        println!("pc: {}/{}", self.pc, self.program.len());
        println!("A: {}", self.accumulator());
        println!("stack: {:?}", self.stack.stack());
        println!("jump_table: {:?}", self.jump_table);
        println!("compared: {}", self.compared)
    }

    fn execute(&mut self) -> ! {
        loop {
            let p = self.fetch();
            match p {
                Program::Zero => self.init_accumulator(),
                Program::Push => self.push(),
                Program::Pop => self.pop(),
                Program::Inc => self.increment(),
                Program::Dec => self.decrement(),
                Program::OutNum => self.print_number(),
                Program::OutChar => self.print_ascii(),
                Program::Label => self.label(),
                Program::Jz => self.jump_if_zero(),
                Program::Jnz => self.jump_if_not_zero(),
                Program::Cmp => self.cmp(),
                Program::Swap => self.swap(),
                Program::Debug => self.debug(),
            }

            self.pc += 1;
            if self.pc >= self.program.len() {
                println!("");
                self.halt();
            }
        }
    }
}

struct Parser {}

impl Parser {
    pub fn parse(source: impl Into<String>) -> Vec<Program> {
        let mnemonic_table = HashMap::from([
            ("アル中", Program::Zero),
            ("！", Program::Inc),
            ("!", Program::Inc),
            ("？", Program::Dec),
            ("?", Program::Dec),
            ("かも", Program::Push),
            ("おいしー", Program::Pop),
            ("あつい", Program::Label),
            ("あっつい", Program::Label),
            ("あづい", Program::Label),
            ("あっづい", Program::Label),
            ("まじぇまじぇ", Program::Swap),
            ("ふう", Program::Jz),
            ("またね", Program::Jnz),
            ("ぷはー", Program::Cmp),
            ("ぷっはー", Program::Cmp),
            ("できた", Program::OutChar),
            ("でぎだ", Program::OutChar),
            ("b", Program::OutNum),
            ("ｂ", Program::OutNum),
            ("デバッグ", Program::Debug),
        ]);
        let flatten_keys = mnemonic_table
            .keys()
            .map(|k| k.chars().collect::<Vec<_>>())
            .flatten()
            .collect::<String>();
        let flatten_keys_first_letter = mnemonic_table
            .keys()
            .map(|k| k.chars().next().unwrap())
            .collect::<String>();

        let mut token = String::new();
        let mut result = vec![];

        for b in source.into().chars() {
            if b.is_whitespace() || b.is_ascii_whitespace() {
                continue;
            }

            if token.len() == 0 && !flatten_keys_first_letter.contains(b) {
                continue;
            }

            token.push(b);
            // println!("{}", token);
            if !flatten_keys.contains(&token) {
                token = String::new();
                continue;
            }

            if mnemonic_table.contains_key(token.as_str()) {
                result.push(*mnemonic_table.get(token.as_str()).unwrap());
                token = String::new();
            }
        }

        result
    }
}

#[derive(clap::Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    let source = std::fs::read_to_string(args.file).unwrap();

    let program = Parser::parse(source);
    let mut vm = Machine::<u8>::new(program);
    vm.execute();
}
