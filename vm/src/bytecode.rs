

pub enum Target {
    Byte(u8),
    Short(u8),
    Integer(u8),
    Long(u8),
    Float(u8),
    Double(u8),
    Zero,
    Seed,
    IntExn,
    FloatExn,
    Trap,
    TrapCause,
    TrapVal,
    Inter,
}

impl Target {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Target::Byte(b) => vec![0x00, *b],
            Target::Short(b) => vec![0x01, *b],
            Target::Integer(b) => vec![0x02, *b],
            Target::Long(b) => vec![0x03, *b],
            Target::Float(b) => vec![0x04, *b],
            Target::Double(b) => vec![0x05, *b],
            Target::Zero => vec![0x06],
            Target::Seed => vec![0x07],
            Target::IntExn => vec![0x08],
            Target::FloatExn => vec![0x09],
            Target::Trap => vec![0x0a],
            Target::TrapCause => vec![0x0b],
            Target::TrapVal => vec![0x0c],
            Target::Inter => vec![0x0d],
        }
    }
}

pub enum Bytecode {
    LoadImmediate8(Target, u8),
    LoadImmediate16(Target, u16),
    LoadImmediate32(Target, u32),
    LoadImmediate64(Target, u64),
    LoadRegister(Target, Target),
    LoadAddress(Target, Target),
    StoreRegister(Target, Target),
    AddSigned(Target, Target, Target),
    AddUnsigned(Target, Target, Target),
    SubSigned(Target, Target, Target),
    SubUnsigned(Target, Target, Target),
    MulSigned(Target, Target, Target),
    MulUnsigned(Target, Target, Target),
    DivSigned(Target, Target, Target, Target),
    DivUnsigned(Target, Target, Target, Target),
    AddFloat(Target, Target, Target),
    SubFloat(Target, Target, Target),
    MulFloat(Target, Target, Target),
    DivFloat(Target, Target, Target),
    Sqrt(Target, Target, Target),
    ConvertFloatInt(Target, Target),
    ConvertIntFloat(Target, Target),
    Or(Target, Target, Target),
    And(Target, Target, Target),
    Xor(Target, Target, Target),
    Complement(Target, Target),
    ShiftRightArithmetic(Target, Target),
    ShiftRightLogical(Target, Target),
    ShiftLeftArithmetic(Target, Target),
    ShiftLeftLogical(Target, Target),
    RotateRight(Target, Target),
    RotateLeft(Target, Target),
    Equal(Target, Target, Target),
    LessThanSigned(Target, Target, Target),
    LessThanUnsigned(Target, Target, Target),
    GreaterThanSigned(Target, Target, Target),
    GreaterThanUnsigned(Target, Target, Target),
    LessThanEqualSigned(Target, Target, Target),
    LessThanEqualUnsigned(Target, Target, Target),
    GreaterThanEqualSigned(Target, Target, Target),
    GreaterThanEqualUnsigned(Target, Target, Target),
    EqualFloat(Target, Target, Target),
    LessThanFloat(Target, Target, Target),
    GreaterThanFloat(Target, Target, Target),
    LessThanEqualFloat(Target, Target, Target),
    GreaterThanEqualFloat(Target, Target, Target),
    JumpOffset16(Target, Target),
    JumpTo(Target, Target),
    BranchEq(Target, Target, Target),
    BranchNe(Target, Target, Target),
    BranchLts(Target, Target, Target),
    BranchLtu(Target, Target, Target),
    BranchGts(Target, Target, Target),
    BranchGtu(Target, Target, Target),
    BranchLes(Target, Target, Target),
    BranchLeu(Target, Target, Target),
    BranchGes(Target, Target, Target),
    BranchGeu(Target, Target, Target),
    CardCount(Target),
    CardName(Target, Target),
    CardCall,
}

impl Bytecode {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Bytecode::LoadImmediate8(target, b) => {
                let mut bytes = vec![0x00, 0x00];
                bytes.extend(target.to_bytes());
                bytes.push(*b);
                bytes
            }
            Bytecode::LoadImmediate16(target, short) => {
                let mut bytes = vec![0x01, 0x00];
                bytes.extend(target.to_bytes());
                bytes.extend(&short.to_le_bytes());
                bytes
            }
            Bytecode::LoadImmediate32(target, int) => {
                let mut bytes = vec![0x02, 0x00];
                bytes.extend(target.to_bytes());
                bytes.extend(&int.to_le_bytes());
                bytes
            }
            Bytecode::LoadImmediate64(target, long) => {
                let mut bytes = vec![0x03, 0x00];
                bytes.extend(target.to_bytes());
                bytes.extend(&long.to_le_bytes());
                bytes
            }
            Bytecode::LoadRegister(target, src) => {
                let mut bytes = vec![0x04, 0x00];
                bytes.extend(target.to_bytes());
                bytes.extend(src.to_bytes());
                bytes
            }
            Bytecode::LoadAddress(target, src) => {
                let mut bytes = vec![0x05, 0x00];
                bytes.extend(target.to_bytes());
                bytes.extend(src.to_bytes());
                bytes
            }
            Bytecode::StoreRegister(dest, src) => {
                let mut bytes = vec![0x06, 0x00];
                bytes.extend(dest.to_bytes());
                bytes.extend(src.to_bytes());
                bytes
            }
        }
    }
}