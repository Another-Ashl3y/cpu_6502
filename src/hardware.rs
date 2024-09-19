use crate::opcodes::*;


#[derive(Clone, Copy)]
pub struct Mem {
    pub data: [u8; 1024*64]
}
impl Mem {
    pub fn new() -> Self {
        Self { data: [0; 1024*64] }
    }
    pub fn initialize(&mut self) {
        self.data = [0; 1024*64]
    }
    pub fn write_word(&mut self, data: u16, address: u32, cycles: &mut u32) {
        self.data[address as usize] = (data & 0xFF) as u8;
        self.data[address as usize + 1] = (data >> 8) as u8;
        *cycles -= 2;
    }
}

#[derive(Clone, Copy)]
pub struct CPU {
    pub PC: u16,    // Program Counter
    pub SP: u16,    // Stack Pointer (is a u8 but starts on page 1)

                    // Registers
    pub A: u8, pub X: u8, pub Y: u8,

                    // Status Flags
    pub C: bool, pub Z: bool, pub I: bool, pub D: bool,
    pub B: bool, pub V: bool, pub N: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            PC: 0,
            SP: 0,
            A: 0, X: 0, Y: 0,
            C: false, Z: false, I: false, D: false,
            B: false, V: false, N: false,
        }
    }
    pub fn reset(&mut self, mem: &mut Mem) {
        self.PC     = 0xFFFC;
        self.SP     = 0x01FF;

        self.A      = 0;
        self.X      = 0;
        self.Y      = 0;

        self.C      = false;
        self.Z      = false;
        self.I      = false;
        self.D      = false;
        self.B      = false;
        self.V      = false;
        self.N      = false;
        
        mem.initialize();
    }
    
    pub fn fetch_byte(
        &mut self, 
        cycles: &mut u32, 
        mem: &mut Mem
    ) -> u8 {
        let data = mem.data[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;
        data
    }
    
    pub fn fetch_word(
        &mut self,
        cycles: &mut u32,
        mem: &mut Mem
    ) -> u16 {
        let low = self.fetch_byte(cycles, mem);
        let high = self.fetch_byte(cycles, mem);
        low as u16 | ((high as u16) << 8)
    }

    pub fn read_byte(
        &mut self, address: u16, 
        cycles: &mut u32, 
        mem: &mut Mem
    ) -> u8 {
        let data = mem.data[address as usize];
        *cycles -= 1;
        data
    }
    pub fn read_word(
        &mut self,
        address: u16,
        cycles: &mut u32,
        mem: &mut Mem
    ) -> u16 {
        let low = self.read_byte(address, cycles, mem);
        let high = self.read_byte(address + 1, cycles, mem);
        low as u16 | ((high as u16) << 8)
    }
    pub fn LDA_set_status(&mut self) {
        self.set_zero(self.A as u16);
        self.set_negative(self.A as u16);
    }    
    pub fn set_negative(&mut self, other: u16) {
        self.N = other & 0b10000000 > 0;
    }
    pub fn set_zero(&mut self, other: u16) {
        self.Z = other == 0x00;
    }

    pub fn push_stack(&mut self, other: u8, cycles: &mut u32, mem: &mut Mem) {
        self.SP = ((self.SP - 1) & 0xFF) | 0x100;
        *cycles -= 1;
        mem.data[self.SP as usize] = other;
        *cycles -= 1;
    }
    pub fn pop_stack(&mut self, cycles: &mut u32, mem: &mut Mem) -> u8 {
        let data = mem.data[self.SP as usize];
        *cycles -= 1;
        self.SP = ((self.SP + 1) & 0xFF) | 0x100;
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, cycles: &mut u32, mem: &mut Mem) {

        while *cycles > 0 {

            let ins: u8 = self.fetch_byte(cycles, mem);
            println!("{}", ins);

            match ins {
                
                // LDA
                    LDA_IM => {
                        let value = self.fetch_byte(cycles, mem);
                        self.A = value;
                        self.LDA_set_status();
                    }
                    LDA_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        self.A = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDA_ZPX => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.X as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        self.A = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDA_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        self.A = mem.data[absolute_address as usize];
                        self.LDA_set_status();
                    }
                    LDA_AX => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.X as u16 + origin;
                        self.A = self.read_byte(address, cycles, mem);
                        if origin & 0xFF00 != address & 0xFF00 { *cycles -= 1; }
                        self.LDA_set_status();
                    }
                    LDA_AY => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.Y as u16 + origin;
                        *cycles -= 1;
                        self.A = self.read_byte(address, cycles, mem);
                        if origin & 0xFF00 != address & 0xFF00 { *cycles -= 1; }
                        self.LDA_set_status();
                    }
                    LDA_IX => {
                        let origin = self.fetch_word(cycles, mem);
                        let shifted = origin + self.X as u16;
                        *cycles -= 1;
                        let lookup: u16 = self.read_word(shifted, cycles, mem);
                        self.A = self.read_byte(lookup, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDA_IY => {
                        let origin = self.fetch_word(cycles, mem);
                        let lookup: u16 = self.read_word(origin, cycles, mem);
                        let shifted = lookup + self.Y as u16;
                        *cycles -= 1;
                        self.A = self.read_byte(shifted, cycles, mem);
                        self.LDA_set_status();
                    }
                // LDX
                    LDX_IM => {
                        let value = self.fetch_byte(cycles, mem);
                        self.X = value;
                        self.LDA_set_status();
                    }
                    LDX_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        self.X = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDX_ZPY => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.Y as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        self.X = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDX_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        self.X = mem.data[absolute_address as usize];
                        self.LDA_set_status();
                    }
                    LDX_AY => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.Y as u16 + origin;
                        *cycles -= 1;
                        self.X = self.read_byte(address, cycles, mem);
                        if origin & 0xFF00 != address & 0xFF00 { *cycles -= 1; }
                        self.LDA_set_status();
                    }
                // LDY
                    LDY_IM => {
                        let value = self.fetch_byte(cycles, mem);
                        self.Y = value;
                        self.LDA_set_status();
                    }
                    LDY_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        self.Y = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDY_ZPX => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.Y as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        self.Y = self.read_byte(zero_page_address as u16, cycles, mem);
                        self.LDA_set_status();
                    }
                    LDY_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        self.Y = mem.data[absolute_address as usize];
                        self.LDA_set_status();
                    }
                    LDY_AX => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.X as u16 + origin;
                        *cycles -= 1;
                        self.Y = self.read_byte(address, cycles, mem);
                        if origin & 0xFF00 != address & 0xFF00 { *cycles -= 1; }
                        self.LDA_set_status();
                    }
                // STA
                    STA_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        mem.data[zero_page_address as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_ZPX => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.X as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        mem.data[zero_page_address as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        
                        mem.data[absolute_address as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_AX => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.X as u16 + origin;

                        
                        mem.data[address as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_AY => {
                        let origin = self.fetch_word(cycles, mem);
                        let address = self.Y as u16 + origin;
                        *cycles -= 1;
                        
                        mem.data[address as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_IX => {
                        let origin = self.fetch_word(cycles, mem);
                        let shifted = origin + self.X as u16;
                        *cycles -= 1;
                        let lookup: u16 = self.read_word(shifted, cycles, mem);
                    
                        mem.data[lookup as usize] = self.A;
                        *cycles -= 1;
                    }
                    STA_IY => {
                        let origin = self.fetch_word(cycles, mem);
                        let lookup: u16 = self.read_word(origin, cycles, mem);
                        let shifted = lookup + self.Y as u16;
                        *cycles -= 1;
                        mem.data[shifted as usize] = self.A;
                        *cycles -= 1;
                    }
                // STX
                    STX_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        mem.data[zero_page_address as usize] = self.X;
                        *cycles -= 1;
                    }
                    STX_ZPY => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.Y as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        mem.data[zero_page_address as usize] = self.X;
                        *cycles -= 1;
                    }
                    STX_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        
                        mem.data[absolute_address as usize] = self.X;
                        *cycles -= 1;
                    }
                // STY
                    STY_ZP => {
                        let zero_page_address = self.fetch_byte(cycles, mem);
                        mem.data[zero_page_address as usize] = self.Y;
                        *cycles -= 1;
                    }
                    STY_ZPX => {
                        let mut zero_page_address = self.fetch_byte(cycles, mem);
                        zero_page_address = ((zero_page_address as usize + self.X as usize) % 0xFFFF) as u8;
                        *cycles -= 1;
                        mem.data[zero_page_address as usize] = self.Y;
                        *cycles -= 1;
                    }
                    STY_A => {
                        let absolute_address = self.fetch_word(cycles, mem);
                        
                        mem.data[absolute_address as usize] = self.Y;
                        *cycles -= 1;
                    }
                
                TAX => {
                    self.X = self.A;
                    *cycles -= 1;
                }
                TAY => {
                    self.Y = self.A;
                    *cycles -= 1;
                }
                TSX => {
                    self.X = (self.SP & 0xFF) as u8;
                    self.set_negative(self.X as u16);
                    self.set_zero(self.X as u16);
                }
                TXA => {
                    self.A = self.X;
                    *cycles -= 1;
                }
                TXS => {
                    self.SP = self.X as u16 + 0x100;
                    *cycles -=1;
                }
                TYA => {
                    self.A = self.Y;
                    *cycles -=1;
                }

                PHA => {
                    self.push_stack(self.A, cycles, mem);
                }
                PHP => {
                    self.push_stack(
                        self.C as u8
                        + ((self.Z as u8) << 1)
                        + ((self.I as u8) << 2)
                        + ((self.D as u8) << 3)
                        + ((self.B as u8) << 4)
                        + ((self.V as u8) << 6)
                        + ((self.N as u8) << 7),
                        cycles, mem
                    );
                }
                PLA => {
                    self.A = self.pop_stack(cycles, mem);
                }
                PLP => {
                    let data = self.pop_stack(cycles, mem);
                    self.C = (data & 0b00000001) > 0;
                    self.N = (data & 0b00000010) > 0;
                    self.V = (data & 0b00000100) > 0;
                    self.B = (data & 0b00001000) > 0;
                    self.D = (data & 0b00010000) > 0;
                    self.I = (data & 0b01000000) > 0;
                    self.Z = (data & 0b10000000) > 0;
                }
                
                // Dec
                    DEC_ZP => todo!(), DEC_ZPX => todo!(),
                    DEC_A  => todo!(), DEC_AX  => todo!(),
                
                DEX => todo!(),
                DEY => todo!(),
                
                // EOR
                    EOR_IM =>  todo!(), EOR_ZP => todo!(),
                    EOR_ZPX => todo!(), EOR_A =>  todo!(),
                    EOR_AX =>  todo!(), EOR_AY => todo!(),
                    EOR_IX =>  todo!(), EOR_IY => todo!(), 
                
                // INC
                    INC_ZP => todo!(), INC_ZPX => todo!(),
                    INC_A =>  todo!(), INC_AX  => todo!(),
                
                INX   => todo!(),
                INY   => todo!(),

                // ADC
                    ADC_IM => todo!(), 
                    ADC_ZP => todo!(), ADC_ZPX => todo!(), 
                    ADC_A => todo!(), ADC_AX => todo!(), ADC_AY => todo!(), 
                    ADC_IX => todo!(), ADC_IY => todo!(), 
                
                // SBC
                    SBC_IM => todo!(), 
                    SBC_ZP => todo!(), SBC_ZPX => todo!(), 
                    SBC_A => todo!(), SBC_AX => todo!(), SBC_AY => todo!(), 
                    SBC_IX => todo!(), SBC_IY => todo!(), 

                // AND
                    AND_IM => todo!(), 
                    AND_ZP => todo!(), AND_ZPX => todo!(), 
                    AND_A => todo!(), AND_AX => todo!(), AND_AY => todo!(), 
                    AND_IX => todo!(), AND_IY => todo!(), 

                JMP_A => todo!(),
                JMP_I => todo!(),

                NOP => (),

                // ORA
                    ORA_IM => todo!(), 
                    ORA_ZP => todo!(), ORA_ZPX => todo!(),
                    ORA_A =>  todo!(), ORA_AX => todo!(), ORA_AY =>  todo!(),
                    ORA_IX => todo!(), ORA_IY => todo!(), 
                
                // ROL
                    ROL_AC => todo!(),
                    ROL_ZP => todo!(), ROL_ZPX => todo!(),
                    ROL_A =>  todo!(), ROL_AX  => todo!(), 
                
                // ASL
                    ASL_AC => todo!(),
                    ASL_ZP => todo!(), ASL_ZPX => todo!(),
                    ASL_A  => todo!(), ASL_AX  => todo!(),

                // ROR
                    ROR_AC => todo!(),
                    ROR_ZP => todo!(), ROR_ZPX => todo!(),
                    ROR_A  => todo!(), ROR_AX  => todo!(),
                
                
                // LSR
                    LSR_AC => todo!(),
                    LSR_ZP => todo!(), LSR_ZPX => todo!(),
                    LSR_A  => todo!(), LSR_AX  => todo!(),
                
                // CMP
                    CMP_IM => todo!(), 
                    CMP_ZP => todo!(), CMP_ZPX => todo!(), 
                    CMP_A  => todo!(), CMP_AX => todo!(), CMP_AY => todo!(), 
                    CMP_IX => todo!(), CMP_IY => todo!(), 
                
                // CPX
                    CPX_IM => todo!(), 
                    CPX_ZP => todo!(), 
                    CPX_A  => todo!(),
                
                // CPY
                    CPY_IM => todo!(),
                    CPY_ZP => todo!(),
                    CPY_A  => todo!(),
                
                // BIT
                    BIT_ZP => todo!(),
                    BIT_A => todo!(),
                
                RTI => todo!(),
                RTS => todo!(),

                BRK => todo!(),


                SEC => todo!(),
                SED => todo!(),
                SEI => todo!(),

                CLC => todo!(), 
                CLD => todo!(), 
                CLI => todo!(), 
                CLV => todo!(), 

                BCC => todo!(), 
                BCS => todo!(), 
                BEQ => todo!(), 
                BMI => todo!(), 
                BNE => todo!(), 
                BPL => todo!(), 
                BVC => todo!(), 
                BVS => todo!(), 

                JSR => {
                    let jump_address: u16 = self.fetch_word(cycles, mem);
                    mem.write_word(self.PC - 1, self.SP as u32, cycles);
                    self.PC = jump_address;
                    *cycles -= 1;
                }
                _=>println!("{} not handled", ins),
            }
        }
    }
}
