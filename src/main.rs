use std::num::Wrapping;


pub struct Sap {
    // Instruction Register
    pub instruction_reg: u8,

    // Program counter
    pub prog_count: u8,

    // Memory
    pub memory: [u8; 16],

    // Registers
    pub a_reg: u8,
    pub b_reg: u8,

    // ALU
    pub alu: u8,

    // Output Register
    pub out: u8,

    // Carry Register
    pub carry: bool,

    // Zero Register
    pub zero: bool,
}

fn read_mem(memory: [u8; 16], addr: u8) -> u8 {
    memory[addr as usize] as u8
}

impl Sap {
    pub fn process_clock_cycle(&mut self) {
        let instruction: u8 = read_mem(self.memory, self.prog_count);
        self.prog_count += 1;
        if self.prog_count > 15 {
            self.prog_count = 0
        }
        self.execute_instruction(instruction);
        println!("OUT: {}", self.out);
    }

    pub fn execute_instruction(&mut self, instruction: u8) {
        let most_sig: u8 = instruction >> 4;
        let least_sig: u8 = instruction & 15;
        match most_sig {
            // 0000 NOP
            0 => return,
            // 0001 LDA
            1 => { 
                let val = read_mem(self.memory, least_sig);
                self.a_reg = val;
            },
            // 0010 ADD
            2 => {
                self.b_reg = read_mem(self.memory, least_sig);
                let sum = (Wrapping(self.a_reg) + Wrapping(self.b_reg)).0;
                if sum == 0 {
                    self.zero = true;
                }
                match self.a_reg.checked_add(self.b_reg) {
                    Some(_) => {}
                    None => { self.carry = true; }
                }
                self.a_reg = sum;
            },
            // 0011 SUB
            3 => {
                let dif = self.a_reg + self.b_reg;
                if dif == 0 {
                    self.zero = true;
                }
                self.a_reg = dif
            },
            // 0100 STA
            4 => self.memory[(least_sig as usize)] = self.a_reg,
            // 0101 LDI
            5 => self.a_reg = least_sig,
            // 0110 JMP
            6 => self.prog_count = least_sig,
            // 0111 JC
            7 => {
                if self.carry {
                    self.prog_count = least_sig;
                }
            },
            // 1000 JZ
            8 => {
                if self.zero {
                    self.prog_count = least_sig;
                }
            },
            // 1110 OUT
            14 => self.out = self.a_reg,
            // 1111 HLT
            15 => loop{}
            _ => (),

        }
    }
}

fn main() {
   let mut SAP: Sap = Sap {
        instruction_reg: 0,
        prog_count: 0,
        memory: [80, 47, 224, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        a_reg: 0,
        b_reg: 0,
        alu: 0,
        out: 0,
        carry: false,
        zero: false,
    }; 

  loop{
       SAP.process_clock_cycle();
  }

}

