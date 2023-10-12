use std::cmp::max;
use std::{env, io};
use std::io::Read;
use std::ops::{AddAssign, MulAssign, SubAssign};
use std::process::exit;
use std::string::ToString;

const INDENT: &str = "      ";
const LETTER_SINGLE_SIZE: usize = 1000000;
const TRAILING_ZEROS_NUM: usize = 6;

type VarT = char;
type AddrT = usize;

fn var_to_index(var: VarT) -> usize { var as usize - 'a' as usize }

struct InstrContainer {
    instructions: Vec<Instr>,
}

impl InstrContainer {
    fn new() -> InstrContainer {
        InstrContainer {
            instructions: vec![]
        }
    }


    pub fn parse_data(&mut self) {
        let mut input: Vec<u8> = vec![];
        io::stdin().read_to_end(&mut input).expect("Could not read data");
        let input = String::from_utf8(input).expect("Could not parse data to string");
        let mut input_chars = input.chars();

        let mut ind = 0;
        while ind < input.len() {
            let c = input_chars.next().unwrap();
            match c {
                'a'..='z' => {
                    self.add(Instr::INC(c));
                    ind += 1;
                }
                '-' => {
                    let var1 = input_chars.next().unwrap();
                    self.add(Instr::DEC(var1));
                    ind += 2;
                }
                '=' => {
                    let var1 = input_chars.next().unwrap();
                    self.add(Instr::PRT(var1));
                    ind += 2;
                }
                '(' => {
                    let len = InstrContainer::find_loop_length(&input[ind..]);
                    self.parse_loop(&input[ind..ind + len]);
                    for _ in 1..len {
                        input_chars.next();
                    }
                    ind += len;
                }
                _ => { ind += 1 }
            }
        }

        self.add(Instr::HLT);
    }

    fn find_loop_length(loop_str: &str) -> usize {
        let mut len: usize = 0;
        let mut open = 0;
        let mut close = 0;

        for c in loop_str.chars() {
            len += 1;
            match c {
                '(' => open += 1,
                ')' => close += 1,
                _ => {}
            }
            if open == close {
                return len;
            }
        }
        len
    }

    fn parse_loop(&mut self, input: &str) -> usize {
        if input.len() < 3 {
            return 0;
        }
        if InstrContainer::can_simplify_loop(input) {
            return InstrContainer::parse_simplified_loop(self, input);
        }

        let mut instr_counter: usize = 0;

        let djz_address = self.add(Instr::DJZ(input.chars().nth(1).unwrap(), 0));
        instr_counter += 1;

        let mut main_index: usize = 2;
        while main_index < input.len() {
            let letter: VarT = input.chars().nth(main_index).unwrap();
            match letter {
                'a'..='z' => {
                    self.add(Instr::INC(letter));
                    instr_counter += 1;
                    main_index += 1;
                }
                '(' => {
                    let inner_loop_length = InstrContainer::find_loop_length(&input[main_index..]);
                    instr_counter += self.parse_loop(&input[main_index..main_index + inner_loop_length]);
                    main_index += inner_loop_length;
                }
                _ => {
                    main_index += 1;
                }
            }
        }

        self.add(Instr::JMP(djz_address));
        instr_counter += 1;

        self.set_djz_address(djz_address, djz_address + instr_counter);

        instr_counter
    }

    fn parse_simplified_loop(&mut self, loop_str: &str) -> usize {
        let mut instr_counter: usize = 0;
        let main_letter: VarT = loop_str.chars().nth(1).unwrap();
        for c in loop_str[2..].chars() {
            match c {
                'a'..='z' => {
                    self.add(Instr::ADD(c, main_letter));
                    instr_counter += 1;
                }
                _ => {}
            }
        }
        self.add(Instr::CLR(main_letter));
        instr_counter += 1;
        instr_counter
    }

    fn can_simplify_loop(loop_str: &str) -> bool {
        let djz_main_letter: VarT = loop_str.chars().nth(1).unwrap();
        for c in loop_str[2..].chars() {
            match c {
                '(' => { return false; }
                letter if letter == djz_main_letter => { return false; }
                _ => {}
            }
        }
        true
    }

    fn add(&mut self, instr: Instr) -> usize {
        self.instructions.push(instr);
        self.instructions.len() - 1
    }

    fn set_djz_address(&mut self, index: usize, new_address: usize) {
        let instr = &mut self.instructions[index];
        if let Instr::DJZ(letter, _) = instr {
            *instr = Instr::DJZ(letter.clone(), new_address);
        }
    }

    pub fn print_instructions(&self) {
        let number_indent: usize = self.instructions.len().to_string().chars().count();

        println!("INSTRUCTIONS:");
        for (ind, instr) in self.instructions.iter().enumerate() {
            let ind_indent: usize = ind.to_string().chars().count();
            let number_indent: String = (0..number_indent - ind_indent).map(|_| " ").collect::<String>();

            print!("{INDENT}{number_indent}{ind}: ");
            instr.print();
        }
    }
}

struct SuperAlphabet {
    letters: Vec<SuperLetter>,
}

impl SuperAlphabet {
    fn new() -> SuperAlphabet {
        SuperAlphabet {
            letters: vec![SuperLetter::default(); 26]
        }
    }

    fn execute_instructions(&mut self, container: &InstrContainer) {
        let mut ind: usize = 0;
        let mut should_inc_ind: bool = true;
        let mut exit: bool = false;
        while ind < container.instructions.len() {
            let instr = &container.instructions[ind];
            match instr {
                Instr::INC(var1) => {
                    self.letters[var_to_index(*var1)].increment();
                }
                Instr::DEC(var1) => {
                    self.letters[var_to_index(*var1)].decrement();
                }
                Instr::ADD(var1, var2) => {
                    let other_letter = &self.letters[var_to_index(*var2)].clone();
                    self.letters[var_to_index(*var1)]
                        .add(other_letter)
                }
                Instr::CLR(var1) => {
                    self.letters[var_to_index(*var1)].clear();
                }
                Instr::JMP(addr) => {
                    ind = *addr;
                    should_inc_ind = false;
                }
                Instr::DJZ(var1, addr) => {
                    let should_jump: bool = !self.letters[var_to_index(*var1)].decrement();
                    if should_jump {
                        ind = *addr;
                        should_inc_ind = false;
                    }
                }
                Instr::PRT(var1) => {
                    self.letters[var_to_index(*var1)].print();
                }
                Instr::HLT => {
                    exit = true;
                }
            }

            if exit {
                return;
            }

            if should_inc_ind {
                ind += 1;
            } else {
                should_inc_ind = true;
            }
        }
    }
}

#[derive(Clone)]
struct SuperLetter {
    numbers: Vec<i32>,
}

impl Default for SuperLetter {
    fn default() -> Self {
        SuperLetter::new()
    }
}

impl SuperLetter {
    fn new() -> SuperLetter {
        SuperLetter {
            numbers: vec![0; 1]
        }
    }

    fn check_resizing_up(&mut self) {
        let mut following: i32 = 0;
        for number in self.numbers.iter_mut() {
            number.add_assign(following);

            if number >= &mut (LETTER_SINGLE_SIZE as i32) {
                following = 1;
                number.sub_assign(LETTER_SINGLE_SIZE as i32);
            } else {
                following = 0;
                break;
            }
        }

        if following == 1 {
            self.numbers.push(1);
        }
    }

    fn check_resizing_down(&mut self) {
        let mut following: i32 = 0;
        for number in self.numbers.iter_mut() {
            number.sub_assign(following);

            if number < &mut 0 {
                following = 1;
                number.add_assign(LETTER_SINGLE_SIZE as i32);
            } else {
                break;
            }
        }

        if self.numbers.len() != 1 && self.numbers.last_mut().unwrap() == &mut 0 {
            self.numbers.pop();
        }
    }

    fn resize(&mut self, desired_size: usize) {
        self.numbers.resize(desired_size, 0);
    }

    fn is_equal_zero(&self) -> bool {
        if self.numbers.len() == 1 && self.numbers.last().unwrap() == &0 {
            return true;
        }
        false
    }

    pub fn increment(&mut self) {
        self.numbers.first_mut().unwrap().add_assign(1);
        self.check_resizing_up();
    }

    pub fn decrement(&mut self) -> bool {
        if self.is_equal_zero() {
            return false;
        }
        self.numbers.first_mut().unwrap().sub_assign(1);
        self.check_resizing_down();
        true
    }

    pub fn add(&mut self, other: &SuperLetter) {
        let bigger_len = max(self.numbers.len(), other.numbers.len());
        self.resize(bigger_len);

        let mut following: i32 = 0;
        let mut main_index: usize = 0;

        for (index, value) in other.numbers.iter().enumerate() {
            self.numbers[index] += value + following;
            if self.numbers[index] >= LETTER_SINGLE_SIZE as i32 {
                self.numbers[index] %= LETTER_SINGLE_SIZE as i32;
                following = 1;
            } else {
                following = 0;
            }
            main_index += 1;
        }

        if following == 1 {
            if main_index == bigger_len {
                self.numbers.resize(bigger_len + 1, 1);
            } else {
                self.numbers[main_index] += 1;
            }
        }
    }

    fn check_trailing_zeros(mut number: i32) -> usize {
        if number == 0 {
            return TRAILING_ZEROS_NUM - 1;
        }

        let mut result: usize = 0;
        while number > 0 {
            number /= 10;
            result += 1;
        }

        TRAILING_ZEROS_NUM - result
    }

    pub fn print(&self) {
        for (index, number) in self.numbers.iter().rev().enumerate() {
            if index != 0 {
                let trailing_zeros = "0".repeat(SuperLetter::check_trailing_zeros(*number));
                print!("{trailing_zeros}");
            }
            print!("{number}");
        }
        println!();
    }

    pub fn clear(&mut self) {
        self.numbers.resize(1, 0);
        self.numbers.last_mut().unwrap().mul_assign(0);
    }
}

enum Instr {
    INC(VarT),
    DEC(VarT),
    ADD(VarT, VarT),
    CLR(VarT),
    JMP(AddrT),
    DJZ(VarT, AddrT),
    PRT(VarT),
    HLT,
}

impl Instr {
    pub fn print(&self) {
        match self {
            Instr::INC(var1) => println!("INC {var1}"),
            Instr::DEC(var1) => println!("DEC {var1}"),
            Instr::ADD(var1, var2) => println!("ADD {var1} {var2}"),
            Instr::CLR(var1) => println!("CLR {var1}"),
            Instr::JMP(addr) => println!("JMP {addr}"),
            Instr::DJZ(var1, addr) => println!("DJZ {var1} {addr}"),
            Instr::PRT(var1) => println!("PRT {var1}"),
            Instr::HLT => println!("HLT"),
        }
    }
}

struct ProgramArgs {
    pub print_help: bool,
    pub print_instr: bool,
    pub print_instr_only: bool,
}

impl ProgramArgs {
    fn print_usage() {
        print!("Usage: ./petlik [OPTIONS]\n\
                    \tOptions:\n\
                    \t-h, --help : printing program usage\n\
                    \t-g         : printing generated instructions\n\
                    \t-gf        : printing generated instructions ONLY, without executing them\n");
    }
}

fn parse_program_args(args: Vec<String>) -> Result<ProgramArgs, &'static str> {
    let mut print_help = false;
    let mut print_instr = false;
    let mut print_instr_only = false;

    for arg in &args[1..] {
        if arg.eq("-h") || arg.eq("--help") {
            print_help = true;
        } else if arg.eq("-g") {
            print_instr = true;
        } else if arg.eq("-gf") {
            print_instr_only = true;
        } else {
            return Err("Invalid program arguments");
        }
    }

    Ok(ProgramArgs {
        print_help,
        print_instr,
        print_instr_only,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = parse_program_args(args);

    if let Err(error) = args {
        println!("{error}");
        ProgramArgs::print_usage();
        exit(1);
    }

    let args = args.unwrap();

    if args.print_help {
        ProgramArgs::print_usage();
        exit(0);
    }

    let mut container = InstrContainer::new();
    container.parse_data();


    if args.print_instr || args.print_instr_only {
        container.print_instructions();
    }

    if !args.print_instr_only {
        let mut alphabet = SuperAlphabet::new();
        alphabet.execute_instructions(&container);
    }
}
