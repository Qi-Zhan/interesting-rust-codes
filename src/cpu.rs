//! # Pattern matching on the riscv32 instruction decode
//!
//! Inspired by <https://github.com/NJU-ProjectN/nemu/blob/master/src/isa/riscv32/inst.c>
//!
//! Maybe not the best way to do this, but it works. See [`CPU::decode_run`].

#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

trait Instruction {
    fn bits(&self, start: u32, end: u32) -> u32;

    fn src1(&self) -> usize {
        self.bits(15, 19) as usize
    }

    fn src2(&self) -> usize {
        self.bits(20, 24) as usize
    }

    fn immI(&self) -> u32 {
        self.bits(20, 31)
    }

    fn immU(&self) -> u32 {
        self.bits(12, 31)
    }

    fn immS(&self) -> u32 {
        self.bits(7, 11)
    }

    fn rd(&self) -> usize {
        self.bits(7, 11) as usize
    }
}

impl Instruction for u32 {
    fn bits(&self, start: u32, end: u32) -> u32 {
        let mask = (1 << (end - start + 1)) - 1;
        (self >> start) & mask
    }
}

trait MatchInst {
    fn match_pattern(&self, pattern: &str) -> bool;
}

impl MatchInst for str {
    fn match_pattern(&self, pattern: &str) -> bool {
        // fliter out the space
        let pattern = pattern.replace(' ', "");
        if self.len() != pattern.len() {
            return false;
        }
        for (s, p) in self.chars().zip(pattern.chars()) {
            if p != '?' && s != p {
                return false;
            }
        }
        true
    }
}

impl MatchInst for u32 {
    fn match_pattern(&self, pattern: &str) -> bool {
        let inst = format!("{:032b}", self);
        inst.match_pattern(pattern)
    }
}

macro_rules! INSTPAT {
    ($inst:expr, $pattern:expr, $name:ident, $type:tt, $action:expr) => {
        if $inst.match_pattern($pattern) {
            $action;
            return Ok(());
        }
    };
}

#[derive(Debug, Default)]
pub struct CPU {
    pc: u32,
    reg: [u32; 32],
}

#[derive(Debug, Default)]
pub struct Memory {
    mem: Vec<u8>,
}

impl CPU {
    #[rustfmt::skip]
    pub fn decode_run(&mut self, inst: u32, _memory: &mut Memory) -> Result<(), String> {
        self.pc += 4;
        INSTPAT!(inst, "??????? ????? ????? ??? ????? 00101 11", auipc, U, self.reg[inst.rd()] = self.pc + inst.immU());
        INSTPAT!(inst, "0000000 ????? ????? 000 ????? 01100 11", add,   R, self.reg[inst.rd()] = self.reg[inst.src1()] + self.reg[inst.src2()]);
        INSTPAT!(inst, "??????? ????? ????? 000 ????? 00100 11", addi,  I, self.reg[inst.rd()] = self.reg[inst.src1()] + inst.immI());
        // TODO: add more instructions
        Err(format!("Unknown instruction {:08x}", inst).to_string())
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn reg(&self, idx: usize) -> u32 {
        self.reg[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        // addi x1, x0, 4
        let inst = 0x00400093;
        cpu.decode_run(inst, &mut memory).unwrap();
        assert_eq!(cpu.reg(1), 4);
        // add x4, x1, x1
        let inst = 0x00108233;
        cpu.decode_run(inst, &mut memory).unwrap();
        // dbg!(&cpu);
        assert_eq!(cpu.reg(4), 8);
        // non-exist instruction so far
        let inst = 0x0000006f;
        assert!(cpu.decode_run(inst, &mut memory).is_err());
    }
}
