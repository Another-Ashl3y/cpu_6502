#![allow(non_snake_case)]

mod opcodes;
use opcodes::*;

mod hardware;
use hardware::*;

fn main() {
    let mut cycles: u32 = 9;
    let mut mem: Mem = Mem::new();
    let mut cpu: CPU = CPU::new();
    cpu.reset(&mut mem);
    // start    - inline little program
    mem.data[0xFFFC] = JSR;
    mem.data[0xFFFD] = 0x42;
    mem.data[0xFFFE] = 0x42;
    mem.data[0x4242] = LDA_IM;
    mem.data[0x4243] = 0x84;
    // end      - inline little program
    cpu.execute(&mut cycles, &mut mem);
    println!("ACC: {}", cpu.A);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn LDA_ZPX_test() {
        
        let mut cycles: u32 = 4;
        let mut mem: Mem = Mem::new();
        let mut cpu: CPU = CPU::new();
        cpu.reset(&mut mem);
        // start    - inline little program
        cpu.X = 5;
        mem.data[0xFFFC] = LDA_ZPX;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0047] = 0x37;
        // end      - inline little program

        let pre_cpu = cpu.clone();
        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.A, 0x37);
        assert_eq!(cpu.Z, false);
        assert_eq!(cpu.N, false);
        assert_eq!(cpu.C, pre_cpu.C);
        assert_eq!(cpu.B, pre_cpu.B);
        assert_eq!(cpu.D, pre_cpu.D);
        assert_eq!(cpu.I, pre_cpu.I);
        assert_eq!(cpu.V, pre_cpu.V);
        assert_eq!(cycles, 0);
    }
    #[test]
   fn LDA_ZPX_wrap_test() {
        
        let mut cycles: u32 = 4;
        let mut mem: Mem = Mem::new();
        let mut cpu: CPU = CPU::new();
        cpu.reset(&mut mem);
        // start    - inline little program
        cpu.X = 0xFF;
        mem.data[0xFFFC] = LDA_ZPX;
        mem.data[0xFFFD] = 0x80;
        mem.data[0x007F] = 0x37;
        // end      - inline little program
        
        let pre_cpu = cpu.clone();
        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.A, 0x37);
        assert_eq!(cpu.Z, false);
        assert_eq!(cpu.N, false);
        assert_eq!(cpu.C, pre_cpu.C);
        assert_eq!(cpu.B, pre_cpu.B);
        assert_eq!(cpu.D, pre_cpu.D);
        assert_eq!(cpu.I, pre_cpu.I);
        assert_eq!(cpu.V, pre_cpu.V);
        assert_eq!(cycles, 0);
    }
    #[test]
    fn LDA_IM_test() {
        
        let mut cycles: u32 = 2;
        let mut mem: Mem = Mem::new();
        let mut cpu: CPU = CPU::new();
        cpu.reset(&mut mem);
        // start    - inline little program
        cpu.X = 5;
        mem.data[0xFFFC] = LDA_IM;
        mem.data[0xFFFD] = 0x84;
        // end      - inline little program

        let pre_cpu = cpu.clone();
        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.A, 0x84);
        assert_eq!(cpu.Z, false);
        assert_eq!(cpu.N, true);
        assert_eq!(cpu.C, pre_cpu.C);
        assert_eq!(cpu.B, pre_cpu.B);
        assert_eq!(cpu.D, pre_cpu.D);
        assert_eq!(cpu.I, pre_cpu.I);
        assert_eq!(cpu.V, pre_cpu.V);
        assert_eq!(cycles, 0);
    }
    #[test]
    fn underflow_test() {
        let mut sp: u16 = 0x1FF;
        for i in 0..0x100 {
            sp = ((sp & 0xFF) | 0x100) - 1;
            assert_eq!(sp - 0xFF, 0xFF - i);
        }
    }
}


