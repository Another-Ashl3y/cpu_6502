// LDA
pub const LDA_IM:   u8 = 0xA9;
pub const LDA_ZP:   u8 = 0xA5;
pub const LDA_ZPX:  u8 = 0xB5;
pub const LDA_A:    u8 = 0xAD;
pub const LDA_AX:   u8 = 0xBD;
pub const LDA_AY:   u8 = 0xB9;
pub const LDA_IX:   u8 = 0xA1;
pub const LDA_IY:   u8 = 0xB1;
// LDX
pub const LDX_IM:   u8 = 0xA2;
pub const LDX_ZP:   u8 = 0xA6;
pub const LDX_ZPY:  u8 = 0xB6;
pub const LDX_A:    u8 = 0xAE;
pub const LDX_AY:   u8 = 0xBE;
// LDY
pub const LDY_IM:   u8 = 0xA0;
pub const LDY_ZP:   u8 = 0xA4;
pub const LDY_ZPX:  u8 = 0xB4;
pub const LDY_A:    u8 = 0xAC;
pub const LDY_AX:   u8 = 0xBC;

// STA
pub const STA_ZP:   u8 = 0x85;
pub const STA_ZPX:  u8 = 0x95;
pub const STA_A:    u8 = 0x8D;
pub const STA_AX:   u8 = 0x9D;
pub const STA_AY:   u8 = 0x99;
pub const STA_IX:   u8 = 0x81;
pub const STA_IY:   u8 = 0x91;
// STX
pub const STX_ZP:   u8 = 0x86;
pub const STX_ZPY:  u8 = 0x96;
pub const STX_A:    u8 = 0x8E;
// STY
pub const STY_ZP:   u8 = 0x84;
pub const STY_ZPX:  u8 = 0x94;
pub const STY_A:    u8 = 0x8C;
// TAX
pub const TAX:      u8 = 0xAA;
// TAY
pub const TAY:      u8 = 0xA8;
// TSX
pub const TSX:      u8 = 0xBA;
// TXA
pub const TXA:      u8 = 0x8A;
// TXS
pub const TXS:      u8 = 0x9A;
// TYA
pub const TYA:      u8 = 0x98;
// PHA
pub const PHA:      u8 = 0x48;
// PHP
pub const PHP:      u8 = 0x08;
// PLA
pub const PLA:      u8 = 0x68;
// PLP
pub const PLP:      u8 = 0x28;
// DEC
pub const DEC_ZP:   u8 = 0xC6;
pub const DEC_ZPX:  u8 = 0xD6;
pub const DEC_A:    u8 = 0xCE;
pub const DEC_AX:   u8 = 0xDE;
// DEX
pub const DEX:      u8 = 0xCA;
// DEY
pub const DEY:      u8 = 0x88;
// INC
pub const INC_ZP:   u8 = 0xE6;
pub const INC_ZPX:  u8 = 0xF6;
pub const INC_A:    u8 = 0xEE;
pub const INC_AX:   u8 = 0xFE;
// INX
pub const INX:      u8 = 0xE8;
// INY
pub const INY:      u8 = 0xC8;
// ADC
pub const ADC_IM:   u8 = 0x69;
pub const ADC_ZP:   u8 = 0x65;
pub const ADC_ZPX:  u8 = 0x75;
pub const ADC_A:    u8 = 0x6D;
pub const ADC_AX:   u8 = 0x7D;
pub const ADC_AY:   u8 = 0x79;
pub const ADC_IX:   u8 = 0x61;
pub const ADC_IY:   u8 = 0x71;
// SBC
pub const SBC_IM:   u8 = 0xE9;
pub const SBC_ZP:   u8 = 0xE5;
pub const SBC_ZPX:  u8 = 0xF5;
pub const SBC_A:    u8 = 0xED;
pub const SBC_AX:   u8 = 0xFD;
pub const SBC_AY:   u8 = 0xF9;
pub const SBC_IX:   u8 = 0xE1;
pub const SBC_IY:   u8 = 0xF1;
// AND
pub const AND_IM:   u8 = 0x29;
pub const AND_ZP:   u8 = 0x25;
pub const AND_ZPX:  u8 = 0x35;
pub const AND_A:    u8 = 0x2D;
pub const AND_AX:   u8 = 0x3D;
pub const AND_AY:   u8 = 0x39;
pub const AND_IX:   u8 = 0x21;
pub const AND_IY:   u8 = 0x31;
// EOR
pub const EOR_IM:   u8 = 0x49;
pub const EOR_ZP:   u8 = 0x45;
pub const EOR_ZPX:  u8 = 0x55;
pub const EOR_A:    u8 = 0x4D;
pub const EOR_AX:   u8 = 0x5D;
pub const EOR_AY:   u8 = 0x59;
pub const EOR_IX:   u8 = 0x41;
pub const EOR_IY:   u8 = 0x51;
// ORA
pub const ORA_IM:   u8 = 0x09;
pub const ORA_ZP:   u8 = 0x05;
pub const ORA_ZPX:  u8 = 0x15;
pub const ORA_A:    u8 = 0x0D;
pub const ORA_AX:   u8 = 0x1D;
pub const ORA_AY:   u8 = 0x19;
pub const ORA_IX:   u8 = 0x01;
pub const ORA_IY:   u8 = 0x11;
// ASL
pub const ASL_AC:   u8 = 0x0A;
pub const ASL_ZP:   u8 = 0x06;
pub const ASL_ZPX:  u8 = 0x16;
pub const ASL_A:    u8 = 0x0E;
pub const ASL_AX:   u8 = 0x1E;
// LSR
pub const LSR_AC:   u8 = 0x4A;
pub const LSR_ZP:   u8 = 0x46;
pub const LSR_ZPX:  u8 = 0x56;
pub const LSR_A:    u8 = 0x4E;
pub const LSR_AX:   u8 = 0x5E;
// ROL
pub const ROL_AC:   u8 = 0x2A;
pub const ROL_ZP:   u8 = 0x26;
pub const ROL_ZPX:  u8 = 0x36;
pub const ROL_A:    u8 = 0x2E;
pub const ROL_AX:   u8 = 0x3E;
// ROR
pub const ROR_AC:   u8 = 0x6A;
pub const ROR_ZP:   u8 = 0x66;
pub const ROR_ZPX:  u8 = 0x76;
pub const ROR_A:    u8 = 0x6E;
pub const ROR_AX:   u8 = 0x7E;

// CLC
pub const CLC:      u8 = 0x18;
// CLD
pub const CLD:      u8 = 0xD8;
// CLI
pub const CLI:      u8 = 0x58;
// CLV
pub const CLV:      u8 = 0xB8;

// SEC
pub const SEC:      u8 = 0x38;
// SED
pub const SED:      u8 = 0xF8;
// SEI
pub const SEI:      u8 = 0x78;

// CMP
pub const CMP_IM:   u8 = 0xC9;
pub const CMP_ZP:   u8 = 0xC5;
pub const CMP_ZPX:  u8 = 0xD5;
pub const CMP_A:    u8 = 0xCD;
pub const CMP_AX:   u8 = 0xDD;
pub const CMP_AY:   u8 = 0xD9;
pub const CMP_IX:   u8 = 0xC1;
pub const CMP_IY:   u8 = 0xD1;
// CPX
pub const CPX_IM:   u8 = 0xE0;
pub const CPX_ZP:   u8 = 0xE4;
pub const CPX_A:    u8 = 0xEC;
// CPY
pub const CPY_IM:   u8 = 0xC0;
pub const CPY_ZP:   u8 = 0xC4;
pub const CPY_A:    u8 = 0xCC;

// BCC
pub const BCC:      u8 = 0x90;
// BCS
pub const BCS:      u8 = 0xB0;
// BEQ
pub const BEQ:      u8 = 0xF0;
// BMI
pub const BMI:      u8 = 0x30;
// BNE
pub const BNE:      u8 = 0xD0;
// BPL
pub const BPL:      u8 = 0x10;
// BVC
pub const BVC:      u8 = 0x50;
// BVS
pub const BVS:      u8 = 0x70;


// JMP
pub const JMP_A:    u8 = 0x4C;
pub const JMP_I:    u8 = 0x6C;
// JSR
pub const JSR:      u8 = 0x20;
// RTS
pub const RTS:      u8 = 0x60;
// BRK
pub const BRK:      u8 = 0x00;
// RTI
pub const RTI:      u8 = 0x40;

// BIT
pub const BIT_ZP:   u8 = 0x24;
pub const BIT_A:    u8 = 0x2C;

// NOP
pub const NOP:      u8 = 0xEA;