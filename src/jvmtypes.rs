#![allow(dead_code)]

use enum_primitive::*;

enum_from_primitive! {
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum JvmOps {
    Nop             = 0x00,
    AconstNull      = 0x01,
    IconstM1        = 0x02,
    Iconst0         = 0x03,
    Iconst1         = 0x04,
    Iconst2         = 0x05,
    Iconst3         = 0x06,
    Iconst4         = 0x07,
    Iconst5         = 0x08,
    Lconst0         = 0x09,
    Lconst1         = 0x0a,
    Fconst0         = 0x0b,
    Fconst1         = 0x0c,
    Fconst2         = 0x0d,
    Dconst0         = 0x0e,
    Dconst1         = 0x0f,
    Bipush          = 0x10,
    Sipush          = 0x11,
    Ldc             = 0x12,
    LdcW            = 0x13,
    Ldc2W           = 0x14,
    Iload           = 0x15,
    Lload           = 0x16,
    Fload           = 0x17,
    Dload           = 0x18,
    Aload           = 0x19,
    Iload0          = 0x1a,
    Iload1          = 0x1b,
    Iload2          = 0x1c,
    Iload3          = 0x1d,
    Lload0          = 0x1e,
    Lload1          = 0x1f,
    Lload2          = 0x20,
    Lload3          = 0x21,
    Fload0          = 0x22,
    Fload1          = 0x23,
    Fload2          = 0x24,
    Fload3          = 0x25,
    Dload0          = 0x26,
    Dload1          = 0x27,
    Dload2          = 0x28,
    Dload3          = 0x29,
    Aload0          = 0x2a,
    Aload1          = 0x2b,
    Aload2          = 0x2c,
    Aload3          = 0x2d,
    Iaload          = 0x2e,
    Laload          = 0x2f,
    Faload          = 0x30,
    Daload          = 0x31,
    Aaload          = 0x32,
    Baload          = 0x33,
    Caload          = 0x34,
    Saload          = 0x35,
    Istore          = 0x36,
    Lstore          = 0x37,
    Fstore          = 0x38,
    Dstore          = 0x39,
    Astore          = 0x3a,
    Istore0         = 0x3b,
    Istore1         = 0x3c,
    Istore2         = 0x3d,
    Istore3         = 0x3e,
    Lstore0         = 0x3f,
    Lstore1         = 0x40,
    Lstore2         = 0x41,
    Lstore3         = 0x42,
    Fstore0         = 0x43,
    Fstore1         = 0x44,
    Fstore2         = 0x45,
    Fstore3         = 0x46,
    Dstore0         = 0x47,
    Dstore1         = 0x48,
    Dstore2         = 0x49,
    Dstore3         = 0x4a,
    Astore0         = 0x4b,
    Astore1         = 0x4c,
    Astore2         = 0x4d,
    Astore3         = 0x4e,
    Iastore         = 0x4f,
    Lastore         = 0x50,
    Fastore         = 0x51,
    Dastore         = 0x52,
    Aastore         = 0x53,
    Bastore         = 0x54,
    Castore         = 0x55,
    Sastore         = 0x56,
    Pop             = 0x57,
    Pop2            = 0x58,
    Dup             = 0x59,
    DupX1           = 0x5a,
    DupX2           = 0x5b,
    Dup2            = 0x5c,
    Dup2X1          = 0x5d,
    Dup2X2          = 0x5e,
    Swap            = 0x5f,
    Iadd            = 0x60,
    Ladd            = 0x61,
    Fadd            = 0x62,
    Dadd            = 0x63,
    Isub            = 0x64,
    Lsub            = 0x65,
    Fsub            = 0x66,
    Dsub            = 0x67,
    Imul            = 0x68,
    Lmul            = 0x69,
    Fmul            = 0x6a,
    Dmul            = 0x6b,
    Idiv            = 0x6c,
    Ldiv            = 0x6d,
    Fdiv            = 0x6e,
    Ddiv            = 0x6f,
    Irem            = 0x70,
    Lrem            = 0x71,
    Frem            = 0x72,
    Drem            = 0x73,
    Ineg            = 0x74,
    Lneg            = 0x75,
    Fneg            = 0x76,
    Dneg            = 0x77,
    Ishl            = 0x78,
    Lshl            = 0x79,
    Ishr            = 0x7a,
    Lshr            = 0x7b,
    Iushr           = 0x7c,
    Lushr           = 0x7d,
    Iand            = 0x7e,
    Land            = 0x7f,
    Ior             = 0x80,
    Lor             = 0x81,
    Ixor            = 0x82,
    Lxor            = 0x83,
    Iinc            = 0x84,
    I2l             = 0x85,
    I2f             = 0x86,
    I2d             = 0x87,
    L2i             = 0x88,
    L2f             = 0x89,
    L2d             = 0x8a,
    F2i             = 0x8b,
    F2l             = 0x8c,
    F2d             = 0x8d,
    D2i             = 0x8e,
    D2l             = 0x8f,
    D2f             = 0x90,
    I2b             = 0x91,
    I2c             = 0x92,
    I2s             = 0x93,
    Lcmp            = 0x94,
    Fcmpl           = 0x95,
    Fcmpg           = 0x96,
    Dcmpl           = 0x97,
    Dcmpg           = 0x98,
    Ifeq            = 0x99,
    Ifne            = 0x9a,
    Iflt            = 0x9b,
    Ifge            = 0x9c,
    Ifgt            = 0x9d,
    Ifle            = 0x9e,
    IfIcmpeq        = 0x9f,
    IfIcmpne        = 0xa0,
    IfIcmplt        = 0xa1,
    IfIcmpge        = 0xa2,
    IfIcmpgt        = 0xa3,
    IfIcmple        = 0xa4,
    IfAcmpeq        = 0xa5,
    IfAcmpne        = 0xa6,
    Goto            = 0xa7,
    Jsr             = 0xa8,
    Ret             = 0xa9,
    Tableswitch     = 0xaa,
    Lookupswitch    = 0xab,
    Ireturn         = 0xac,
    Lreturn         = 0xad,
    Freturn         = 0xae,
    Dreturn         = 0xaf,
    Areturn         = 0xb0,
    Return          = 0xb1,
    Getstatic       = 0xb2,
    Putstatic       = 0xb3,
    Getfield        = 0xb4,
    Putfield        = 0xb5,
    Invokevirtual   = 0xb6,
    Invokespecial   = 0xb7,
    Invokestatic    = 0xb8,
    Invokeinterface = 0xb9,
    Invokedynamic   = 0xba,
    New             = 0xbb,
    Newarray        = 0xbc,
    Anewarray       = 0xbd,
    Arraylength     = 0xbe,
    Athrow          = 0xbf,
    Checkcast       = 0xc0,
    Instanceof      = 0xc1,
    Monitorenter    = 0xc2,
    Monitorexit     = 0xc3,
    Wide            = 0xc4,
    Multianewarray  = 0xc5,
    Ifnull          = 0xc6,
    Ifnonnull       = 0xc7,
    GotoW           = 0xc8,
    JsrW            = 0xc9,
    Breakpoint      = 0xca,
    /* cb - fd reserved */
    Impdep1         = 0xfe,
    Impdep2         = 0xff,
}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JType {
    Int,
    Long,
    Float,
    Double,
    Object,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Comparison {
    Eq,
    Ne,
    Lt,
    Ge,
    Gt,
    Le,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
    Branch   { kind : JType, way : Comparison, target : u16 },
    Constant { kind : JType, value : i8 },
    Jump     { target : u16 },
    Leave,   /* i.e. void return */
    Length,  /* i.e. arraylength */
    Load     { kind : JType, index : u8 },
    Noop,
    Store    { kind : JType, index : u8 },
    Subtract { kind : JType },
    Yield    { kind : JType }, /* i.e. return */
    Unhandled(u8),
}

#[derive(Debug)]
pub struct AddressedOperation {
    pub address : u16,
    pub op : Operation,
}

// returns any Operation parsed and the number of bytes consumed
fn decode_op(stream : &[u8]) -> (Option<Operation>, usize) {
    use JType::*;
    use JvmOps::*;
    use Operation::*;

    let byte = stream[0];
    return match JvmOps::from_u8(byte) {
        None => (None, 0),
        Some(code) => {
            let opt = match code {
                Nop
                    => Some(Noop),
                AconstNull
                    => Some(Constant { kind : Object, value : 0 }),
                IconstM1 | Iconst0 | Iconst1 | Iconst2 | Iconst3 | Iconst4 | Iconst5
                    => Some(Constant { kind : Int, value : (byte as i8 - Iconst0 as i8) }),
                Lconst0 | Lconst1
                    => Some(Constant { kind : Long, value : (byte - Lconst0 as u8) as i8 }),
                Fconst0 | Fconst1 | Fconst2
                    => Some(Constant { kind : Float, value : (byte - Fconst0 as u8) as i8 }),
                Dconst0 | Dconst1
                    => Some(Constant { kind : Double, value : (byte - Dconst0 as u8) as i8 }),
                _
                    => Some(Unhandled(byte)), // TODO eventually unreachable!()
            };
            let consumed = match code {
                Nop
                    | AconstNull
                    | IconstM1 | Iconst0 | Iconst1 | Iconst2 | Iconst3 | Iconst4 | Iconst5
                    | Lconst0 | Lconst1
                    | Fconst0 | Fconst1 | Fconst2
                    | Dconst0 | Dconst1
                    => 1,
                _ => 0,
            };

            (opt, consumed)
        }
    };
}

#[test]
fn test_get_op() {
    use JType::*;
    use JvmOps::*;

    assert_eq!((Some(Operation::Constant { kind : Int, value : 3 }), 1),
                decode_op(&vec![ Iconst3 as u8 ]));

    for b in 0..=255 {
        if let Some(_) = JvmOps::from_u8(b){
            let arr = vec![ b, 0u8, 0u8, 0u8, 0u8, 0u8 ];
            let v = decode_op(&arr);
            match v {
                (Some(Operation::Unhandled(_)), 0) => {},
                (Some(_), x) => assert!(x != 0),
                _ => panic!("unhandled"),
            };
        }
    }
}

