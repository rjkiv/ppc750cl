#![allow(unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::disasm::*;
/// The entry table allows us to quickly find the range of possible opcodes for a
/// given 6-bit prefix. 2*64 bytes should fit in a cache line (or two).
const OPCODE_ENTRIES: [(u8, u8); 64] = [
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 1),
    (1, 35),
    (0, 0),
    (0, 0),
    (35, 36),
    (36, 37),
    (0, 0),
    (37, 38),
    (38, 39),
    (39, 40),
    (40, 41),
    (41, 42),
    (42, 43),
    (43, 44),
    (44, 45),
    (45, 46),
    (46, 59),
    (59, 60),
    (60, 61),
    (0, 0),
    (61, 62),
    (62, 63),
    (63, 64),
    (64, 65),
    (65, 66),
    (66, 67),
    (67, 68),
    (0, 0),
    (68, 160),
    (160, 161),
    (161, 162),
    (162, 163),
    (163, 164),
    (164, 165),
    (165, 166),
    (166, 167),
    (167, 168),
    (168, 169),
    (169, 170),
    (170, 171),
    (171, 172),
    (172, 173),
    (173, 174),
    (174, 175),
    (175, 176),
    (176, 177),
    (177, 178),
    (178, 179),
    (179, 180),
    (180, 181),
    (181, 182),
    (182, 183),
    (183, 184),
    (184, 185),
    (185, 186),
    (0, 0),
    (186, 195),
    (195, 196),
    (196, 197),
    (0, 0),
    (197, 222),
];
/// The bitmask and pattern for each opcode.
const OPCODE_PATTERNS: [(u32, u32); 256] = [
    (0xfc000000, 0xc000000),
    (0xffe007ff, 0x100007ec),
    (0xfc00007f, 0x1000004c),
    (0xfc00007f, 0x1000000c),
    (0xfc00007f, 0x1000004e),
    (0xfc00007f, 0x1000000e),
    (0xfc1f07fe, 0x10000210),
    (0xfc0007fe, 0x1000002a),
    (0xfc6007ff, 0x10000040),
    (0xfc6007ff, 0x100000c0),
    (0xfc6007ff, 0x10000000),
    (0xfc6007ff, 0x10000080),
    (0xfc0007fe, 0x10000024),
    (0xfc00003e, 0x1000003a),
    (0xfc00003e, 0x1000001c),
    (0xfc00003e, 0x1000001e),
    (0xfc0007fe, 0x10000420),
    (0xfc0007fe, 0x10000460),
    (0xfc0007fe, 0x100004a0),
    (0xfc0007fe, 0x100004e0),
    (0xfc1f07fe, 0x10000090),
    (0xfc00003e, 0x10000038),
    (0xfc00f83e, 0x10000032),
    (0xfc00f83e, 0x10000018),
    (0xfc00f83e, 0x1000001a),
    (0xfc1f07fe, 0x10000110),
    (0xfc1f07fe, 0x10000050),
    (0xfc00003e, 0x1000003e),
    (0xfc00003e, 0x1000003c),
    (0xfc1f07fe, 0x10000030),
    (0xfc1f07fe, 0x10000034),
    (0xfc00003e, 0x1000002e),
    (0xfc0007fe, 0x10000028),
    (0xfc00003e, 0x10000014),
    (0xfc00003e, 0x10000016),
    (0xfc000000, 0x1c000000),
    (0xfc000000, 0x20000000),
    (0xfc400000, 0x28000000),
    (0xfc400000, 0x2c000000),
    (0xfc000000, 0x30000000),
    (0xfc000000, 0x34000000),
    (0xfc000000, 0x38000000),
    (0xfc000000, 0x3c000000),
    (0xfc000000, 0x40000000),
    (0xffffffff, 0x44000002),
    (0xfc000000, 0x48000000),
    (0xfc00fffe, 0x4c000420),
    (0xfc00fffe, 0x4c000020),
    (0xfc0007ff, 0x4c000202),
    (0xfc0007ff, 0x4c000102),
    (0xfc0007ff, 0x4c000242),
    (0xfc0007ff, 0x4c0001c2),
    (0xfc0007ff, 0x4c000042),
    (0xfc0007ff, 0x4c000382),
    (0xfc0007ff, 0x4c000342),
    (0xfc0007ff, 0x4c000182),
    (0xffffffff, 0x4c00012c),
    (0xfc63ffff, 0x4c000000),
    (0xffffffff, 0x4c000064),
    (0xfc000000, 0x50000000),
    (0xfc000000, 0x54000000),
    (0xfc000000, 0x5c000000),
    (0xfc000000, 0x60000000),
    (0xfc000000, 0x64000000),
    (0xfc000000, 0x68000000),
    (0xfc000000, 0x6c000000),
    (0xfc000000, 0x70000000),
    (0xfc000000, 0x74000000),
    (0xfc0003fe, 0x7c000214),
    (0xfc0003fe, 0x7c000014),
    (0xfc0003fe, 0x7c000114),
    (0xfc00fbfe, 0x7c0001d4),
    (0xfc00fbfe, 0x7c000194),
    (0xfc0007fe, 0x7c000038),
    (0xfc0007fe, 0x7c000078),
    (0xfc4007ff, 0x7c000000),
    (0xfc4007ff, 0x7c000040),
    (0xfc00fffe, 0x7c000034),
    (0xffe007ff, 0x7c0000ac),
    (0xffe007ff, 0x7c0003ac),
    (0xffe007ff, 0x7c00006c),
    (0xffe007ff, 0x7c00022c),
    (0xffe007ff, 0x7c0001ec),
    (0xffe007ff, 0x7c0007ec),
    (0xfc0003fe, 0x7c0003d6),
    (0xfc0003fe, 0x7c000396),
    (0xfc0007ff, 0x7c00026c),
    (0xfc0007ff, 0x7c00036c),
    (0xffffffff, 0x7c0006ac),
    (0xfc0007fe, 0x7c000238),
    (0xfc00fffe, 0x7c000774),
    (0xfc00fffe, 0x7c000734),
    (0xffe007fe, 0x7c0007ac),
    (0xfc0007ff, 0x7c0000ee),
    (0xfc0007ff, 0x7c0000ae),
    (0xfc0007ff, 0x7c0004ee),
    (0xfc0007ff, 0x7c0004ae),
    (0xfc0007ff, 0x7c00046e),
    (0xfc0007ff, 0x7c00042e),
    (0xfc0007ff, 0x7c0002ee),
    (0xfc0007ff, 0x7c0002ae),
    (0xfc0007ff, 0x7c00062c),
    (0xfc0007ff, 0x7c00026e),
    (0xfc0007ff, 0x7c00022e),
    (0xfc0007ff, 0x7c0004aa),
    (0xfc0007ff, 0x7c00042a),
    (0xfc0007ff, 0x7c000028),
    (0xfc0007ff, 0x7c00042c),
    (0xfc0007ff, 0x7c00006e),
    (0xfc0007ff, 0x7c00002e),
    (0xfc7fffff, 0x7c000400),
    (0xfc1fffff, 0x7c000026),
    (0xfc1fffff, 0x7c0000a6),
    (0xfc0007ff, 0x7c0002a6),
    (0xfc10ffff, 0x7c0004a6),
    (0xfc1f07ff, 0x7c000526),
    (0xfc0007ff, 0x7c0002e6),
    (0xfc100fff, 0x7c000120),
    (0xfc1fffff, 0x7c000124),
    (0xfc0007ff, 0x7c0003a6),
    (0xfc10ffff, 0x7c0001a4),
    (0xfc1f07ff, 0x7c0001e4),
    (0xfc0007fe, 0x7c000096),
    (0xfc0007fe, 0x7c000016),
    (0xfc0003fe, 0x7c0001d6),
    (0xfc0007fe, 0x7c0003b8),
    (0xfc00fbfe, 0x7c0000d0),
    (0xfc0007fe, 0x7c0000f8),
    (0xfc0007fe, 0x7c000378),
    (0xfc0007fe, 0x7c000338),
    (0xfc0007fe, 0x7c000030),
    (0xfc0007fe, 0x7c000630),
    (0xfc0007fe, 0x7c000670),
    (0xfc0007fe, 0x7c000430),
    (0xfc0007ff, 0x7c0001ee),
    (0xfc0007ff, 0x7c0001ae),
    (0xfc0007ff, 0x7c0005ee),
    (0xfc0007ff, 0x7c0005ae),
    (0xfc0007ff, 0x7c0007ae),
    (0xfc0007ff, 0x7c00056e),
    (0xfc0007ff, 0x7c00052e),
    (0xfc0007ff, 0x7c00072c),
    (0xfc0007ff, 0x7c00036e),
    (0xfc0007ff, 0x7c00032e),
    (0xfc0007ff, 0x7c0005aa),
    (0xfc0007ff, 0x7c00052a),
    (0xfc0007ff, 0x7c00052c),
    (0xfc0007ff, 0x7c00012d),
    (0xfc0007ff, 0x7c00016e),
    (0xfc0007ff, 0x7c00012e),
    (0xfc0003fe, 0x7c000050),
    (0xfc0003fe, 0x7c000010),
    (0xfc0003fe, 0x7c000110),
    (0xfc00fbfe, 0x7c0001d0),
    (0xfc00fbfe, 0x7c000190),
    (0xffffffff, 0x7c0004ac),
    (0xffff07ff, 0x7c000264),
    (0xffffffff, 0x7c00046c),
    (0xfc0007ff, 0x7c000008),
    (0xfc0007fe, 0x7c000278),
    (0xfc000000, 0x80000000),
    (0xfc000000, 0x84000000),
    (0xfc000000, 0x88000000),
    (0xfc000000, 0x8c000000),
    (0xfc000000, 0x90000000),
    (0xfc000000, 0x94000000),
    (0xfc000000, 0x98000000),
    (0xfc000000, 0x9c000000),
    (0xfc000000, 0xa0000000),
    (0xfc000000, 0xa4000000),
    (0xfc000000, 0xa8000000),
    (0xfc000000, 0xac000000),
    (0xfc000000, 0xb0000000),
    (0xfc000000, 0xb4000000),
    (0xfc000000, 0xb8000000),
    (0xfc000000, 0xbc000000),
    (0xfc000000, 0xc0000000),
    (0xfc000000, 0xc4000000),
    (0xfc000000, 0xc8000000),
    (0xfc000000, 0xcc000000),
    (0xfc000000, 0xd0000000),
    (0xfc000000, 0xd4000000),
    (0xfc000000, 0xd8000000),
    (0xfc000000, 0xdc000000),
    (0xfc000000, 0xe0000000),
    (0xfc000000, 0xe4000000),
    (0xfc0007fe, 0xec00002a),
    (0xfc0007fe, 0xec000024),
    (0xfc00003e, 0xec00003a),
    (0xfc00003e, 0xec000038),
    (0xfc00f83e, 0xec000032),
    (0xfc00003e, 0xec00003e),
    (0xfc00003e, 0xec00003c),
    (0xfc1f07fe, 0xec000030),
    (0xfc0007fe, 0xec000028),
    (0xfc000000, 0xf0000000),
    (0xfc000000, 0xf4000000),
    (0xfc1f07fe, 0xfc000210),
    (0xfc0007fe, 0xfc00002a),
    (0xfc6007ff, 0xfc000040),
    (0xfc6007ff, 0xfc000000),
    (0xfc1f07fe, 0xfc00001c),
    (0xfc1f07fe, 0xfc00001e),
    (0xfc0007fe, 0xfc000024),
    (0xfc00003e, 0xfc00003a),
    (0xfc1f07fe, 0xfc000090),
    (0xfc00003e, 0xfc000038),
    (0xfc00f83e, 0xfc000032),
    (0xfc1f07fe, 0xfc000110),
    (0xfc1f07fe, 0xfc000050),
    (0xfc00003e, 0xfc00003e),
    (0xfc00003e, 0xfc00003c),
    (0xfc1f07fe, 0xfc000018),
    (0xfc1f07fe, 0xfc000034),
    (0xfc00003e, 0xfc00002e),
    (0xfc0007fe, 0xfc000028),
    (0xfc63ffff, 0xfc000080),
    (0xfc1ffffe, 0xfc00048e),
    (0xfc1ffffe, 0xfc00008c),
    (0xfc1ffffe, 0xfc00004c),
    (0xfe0107fe, 0xfc00058e),
    (0xfc7f0ffe, 0xfc00010c),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
];
/// The name of each opcode.
const OPCODE_NAMES: [&str; 256] = [
    "twi",
    "dcbz_l",
    "psq_lux",
    "psq_lx",
    "psq_stux",
    "psq_stx",
    "ps_abs",
    "ps_add",
    "ps_cmpo0",
    "ps_cmpo1",
    "ps_cmpu0",
    "ps_cmpu1",
    "ps_div",
    "ps_madd",
    "ps_madds0",
    "ps_madds1",
    "ps_merge00",
    "ps_merge01",
    "ps_merge10",
    "ps_merge11",
    "ps_mr",
    "ps_msub",
    "ps_mul",
    "ps_muls0",
    "ps_muls1",
    "ps_nabs",
    "ps_neg",
    "ps_nmadd",
    "ps_nmsub",
    "ps_res",
    "ps_rsqrte",
    "ps_sel",
    "ps_sub",
    "ps_sum0",
    "ps_sum1",
    "mulli",
    "subfic",
    "cmpli",
    "cmpi",
    "addic",
    "addic.",
    "addi",
    "addis",
    "bc",
    "sc",
    "b",
    "bcctr",
    "bclr",
    "crand",
    "crandc",
    "creqv",
    "crnand",
    "crnor",
    "cror",
    "crorc",
    "crxor",
    "isync",
    "mcrf",
    "rfi",
    "rlwimi",
    "rlwinm",
    "rlwnm",
    "ori",
    "oris",
    "xori",
    "xoris",
    "andi.",
    "andis.",
    "add",
    "addc",
    "adde",
    "addme",
    "addze",
    "and",
    "andc",
    "cmp",
    "cmpl",
    "cntlzw",
    "dcbf",
    "dcbi",
    "dcbst",
    "dcbt",
    "dcbtst",
    "dcbz",
    "divw",
    "divwu",
    "eciwx",
    "ecowx",
    "eieio",
    "eqv",
    "extsb",
    "extsh",
    "icbi",
    "lbzux",
    "lbzx",
    "lfdux",
    "lfdx",
    "lfsux",
    "lfsx",
    "lhaux",
    "lhax",
    "lhbrx",
    "lhzux",
    "lhzx",
    "lswi",
    "lswx",
    "lwarx",
    "lwbrx",
    "lwzux",
    "lwzx",
    "mcrxr",
    "mfcr",
    "mfmsr",
    "mfspr",
    "mfsr",
    "mfsrin",
    "mftb",
    "mtcrf",
    "mtmsr",
    "mtspr",
    "mtsr",
    "mtsrin",
    "mulhw",
    "mulhwu",
    "mullw",
    "nand",
    "neg",
    "nor",
    "or",
    "orc",
    "slw",
    "sraw",
    "srawi",
    "srw",
    "stbux",
    "stbx",
    "stfdux",
    "stfdx",
    "stfiwx",
    "stfsux",
    "stfsx",
    "sthbrx",
    "sthux",
    "sthx",
    "stswi",
    "stswx",
    "stwbrx",
    "stwcx.",
    "stwux",
    "stwx",
    "subf",
    "subfc",
    "subfe",
    "subfme",
    "subfze",
    "sync",
    "tlbie",
    "tlbsync",
    "tw",
    "xor",
    "lwz",
    "lwzu",
    "lbz",
    "lbzu",
    "stw",
    "stwu",
    "stb",
    "stbu",
    "lhz",
    "lhzu",
    "lha",
    "lhau",
    "sth",
    "sthu",
    "lmw",
    "stmw",
    "lfs",
    "lfsu",
    "lfd",
    "lfdu",
    "stfs",
    "stfsu",
    "stfd",
    "stfdu",
    "psq_l",
    "psq_lu",
    "fadds",
    "fdivs",
    "fmadds",
    "fmsubs",
    "fmuls",
    "fnmadds",
    "fnmsubs",
    "fres",
    "fsubs",
    "psq_st",
    "psq_stu",
    "fabs",
    "fadd",
    "fcmpo",
    "fcmpu",
    "fctiw",
    "fctiwz",
    "fdiv",
    "fmadd",
    "fmr",
    "fmsub",
    "fmul",
    "fnabs",
    "fneg",
    "fnmadd",
    "fnmsub",
    "frsp",
    "frsqrte",
    "fsel",
    "fsub",
    "mcrfs",
    "mffs",
    "mtfsb0",
    "mtfsb1",
    "mtfsf",
    "mtfsfi",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
    "<illegal>",
];
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Opcode {
    /// An illegal or unknown opcode
    #[default]
    Illegal = u8::MAX,
    /// twi: Trap Word Immediate
    Twi = 0,
    /// dcbz_l: Data Cache Block Set to Zero Locked
    DcbzL = 1,
    /// psq_lux: Paired Single Quantized Load with Update Indexed
    PsqLux = 2,
    /// psq_lx: Paired Single Quantized Load Indexed
    PsqLx = 3,
    /// psq_stux: Paired Single Quantized Store with Update Indexed
    PsqStux = 4,
    /// psq_stx: Paired Single Quantized Store Indexed
    PsqStx = 5,
    /// ps_abs: Paired Single Absolute Value
    PsAbs = 6,
    /// ps_add: Paired Single Add
    PsAdd = 7,
    /// ps_cmpo0: Paired Singles Compare Ordered High
    PsCmpo0 = 8,
    /// ps_cmpo1: Paired Singles Compare Ordered Low
    PsCmpo1 = 9,
    /// ps_cmpu0: Paired Singles Compare Unordered High
    PsCmpu0 = 10,
    /// ps_cmpu1: Paired Singles Compare Unordered Low
    PsCmpu1 = 11,
    /// ps_div: Paired Single Divide
    PsDiv = 12,
    /// ps_madd: Paired Single Multiply-Add
    PsMadd = 13,
    /// ps_madds0: Paired Single Multiply-Add Scalar high
    PsMadds0 = 14,
    /// ps_madds1: Paired Single Multiply-Add Scalar low
    PsMadds1 = 15,
    /// ps_merge00: Paired Single MERGE high
    PsMerge00 = 16,
    /// ps_merge01: Paired Single MERGE direct
    PsMerge01 = 17,
    /// ps_merge10: Paired Single MERGE swapped
    PsMerge10 = 18,
    /// ps_merge11: Paired Single MERGE low
    PsMerge11 = 19,
    /// ps_mr: Paired Single Move Register
    PsMr = 20,
    /// ps_msub: Paired Single Multiply-Subtract
    PsMsub = 21,
    /// ps_mul: Paired Single Multiply
    PsMul = 22,
    /// ps_muls0: Paired Single Multiply Scalar high
    PsMuls0 = 23,
    /// ps_muls1: Paired Single Multiply Scalar low
    PsMuls1 = 24,
    /// ps_nabs: Paired Single Negative Absolute Value
    PsNabs = 25,
    /// ps_neg: Paired Single Negate
    PsNeg = 26,
    /// ps_nmadd: Paired Single Negative Multiply-Add
    PsNmadd = 27,
    /// ps_nmsub: Paired Single Negative Multiply-Subtract
    PsNmsub = 28,
    /// ps_res: Paired Single Reciprocal Estimate
    PsRes = 29,
    /// ps_rsqrte: Paired Single Reciprocal Square Root Estimate
    PsRsqrte = 30,
    /// ps_sel: Paired Single Select
    PsSel = 31,
    /// ps_sub: Paired Single Subtract
    PsSub = 32,
    /// ps_sum0: Paired Single vector SUM high
    PsSum0 = 33,
    /// ps_sum1: Paired Single vector SUM low
    PsSum1 = 34,
    /// mulli: Multiply Low Immediate
    Mulli = 35,
    /// subfic: Subtract from Immediate Carrying
    Subfic = 36,
    /// cmpli: Compare Logical Immediate
    Cmpli = 37,
    /// cmpi: Compare Immediate
    Cmpi = 38,
    /// addic: Add Immediate Carrying
    Addic = 39,
    /// addic.: Add Immediate Carrying and Record
    Addic_ = 40,
    /// addi: Add Immediate
    Addi = 41,
    /// addis: Add Immediate Shifted
    Addis = 42,
    /// bc: Branch Conditional
    Bc = 43,
    /// sc: System Call
    Sc = 44,
    /// b: Branch
    B = 45,
    /// bcctr: Branch Conditional to Count Register
    Bcctr = 46,
    /// bclr: Branch Conditional to Link Register
    Bclr = 47,
    /// crand: Condition Register AND
    Crand = 48,
    /// crandc: Condition Register AND with Complement
    Crandc = 49,
    /// creqv: Condition Register Equivalent
    Creqv = 50,
    /// crnand: Condition Register NAND
    Crnand = 51,
    /// crnor: Condition Register NOR
    Crnor = 52,
    /// cror: Condition Register OR
    Cror = 53,
    /// crorc: Condition Register OR with Complement
    Crorc = 54,
    /// crxor: Condition Register XOR
    Crxor = 55,
    /// isync: Instruction Synchronize
    Isync = 56,
    /// mcrf: Move Condition Register Field
    Mcrf = 57,
    /// rfi: Return from Interrupt
    Rfi = 58,
    /// rlwimi: Rotate Left Word Immediate then Mask Insert
    Rlwimi = 59,
    /// rlwinm: Rotate Left Word Immediate then AND with Mask
    Rlwinm = 60,
    /// rlwnm: Rotate Left Word then AND with Mask
    Rlwnm = 61,
    /// ori: OR Immediate
    Ori = 62,
    /// oris: OR Immediate Shifted
    Oris = 63,
    /// xori: XOR Immediate
    Xori = 64,
    /// xoris: XOR Immediate Shifted
    Xoris = 65,
    /// andi.: AND Immediate
    Andi_ = 66,
    /// andis.: AND Immediate Shifted
    Andis_ = 67,
    /// add: Add
    Add = 68,
    /// addc: Add Carrying
    Addc = 69,
    /// adde: Add Extended
    Adde = 70,
    /// addme: Add to Minus One Extended
    Addme = 71,
    /// addze: Add to Zero Extended
    Addze = 72,
    /// and: AND
    And = 73,
    /// andc: AND with Complement
    Andc = 74,
    /// cmp: Compare
    Cmp = 75,
    /// cmpl: Compare Logical
    Cmpl = 76,
    /// cntlzw: Count Leading Zeros Word
    Cntlzw = 77,
    /// dcbf: Data Cache Block Flush
    Dcbf = 78,
    /// dcbi: Data Cache Block Invalidate
    Dcbi = 79,
    /// dcbst: Data Cache Block Store
    Dcbst = 80,
    /// dcbt: Data Cache Block Touch
    Dcbt = 81,
    /// dcbtst: Data Cache Block Touch for Store
    Dcbtst = 82,
    /// dcbz: Data Cache Block Clear to Zero
    Dcbz = 83,
    /// divw: Divide Word
    Divw = 84,
    /// divwu: Divide Word Unsigned
    Divwu = 85,
    /// eciwx: External Control In Word Indexed
    Eciwx = 86,
    /// ecowx: External Control Out Word Indexed
    Ecowx = 87,
    /// eieio: Enforce In-Order Execution of I/O
    Eieio = 88,
    /// eqv: Equivalent
    Eqv = 89,
    /// extsb: Extend Sign Byte
    Extsb = 90,
    /// extsh: Extend Sign Half Word
    Extsh = 91,
    /// icbi: Instruction Cache Block Invalidate
    Icbi = 92,
    /// lbzux: Load Byte and Zero with Update Indexed
    Lbzux = 93,
    /// lbzx: Load Byte and Zero Indexed
    Lbzx = 94,
    /// lfdux: Load Floating-Point Double with Update Indexed
    Lfdux = 95,
    /// lfdx: Load Floating-Point Double Indexed
    Lfdx = 96,
    /// lfsux: Load Floating-Point Single with Update Indexed
    Lfsux = 97,
    /// lfsx: Load Floating-Point Single Indexed
    Lfsx = 98,
    /// lhaux: Load Half Word Algebraic with Update Indexed
    Lhaux = 99,
    /// lhax: Load Half Word Algebraic Indexed
    Lhax = 100,
    /// lhbrx: Load Half Word Byte-Reverse Indexed
    Lhbrx = 101,
    /// lhzux: Load Half Word and Zero with Update Indexed
    Lhzux = 102,
    /// lhzx: Load Half Word and Zero Indexed
    Lhzx = 103,
    /// lswi: Load String Word Immediate
    Lswi = 104,
    /// lswx: Load String Word Indexed
    Lswx = 105,
    /// lwarx: Load String Word and Reverse Indexed
    Lwarx = 106,
    /// lwbrx: Load String Word and Byte-Reverse Indexed
    Lwbrx = 107,
    /// lwzux: Load Word and Zero with Update Indexed
    Lwzux = 108,
    /// lwzx: Load Word and Zero Indexed
    Lwzx = 109,
    /// mcrxr: Move to Condition Register from XER
    Mcrxr = 110,
    /// mfcr: Move from Condition Register
    Mfcr = 111,
    /// mfmsr: Move from Machine State Register
    Mfmsr = 112,
    /// mfspr: Move from Special-Purpose Register
    Mfspr = 113,
    /// mfsr: Move from Segment Register
    Mfsr = 114,
    /// mfsrin: Move from Segment Register Indirect
    Mfsrin = 115,
    /// mftb: Move from Time Base
    Mftb = 116,
    /// mtcrf: Move to Condition Register Fields
    Mtcrf = 117,
    /// mtmsr: Move to Machine State Register
    Mtmsr = 118,
    /// mtspr: Move to Special-Purpose Register
    Mtspr = 119,
    /// mtsr: Move to Segment Register
    Mtsr = 120,
    /// mtsrin: Move to Segment Register Indirect
    Mtsrin = 121,
    /// mulhw: Multiply High Word
    Mulhw = 122,
    /// mulhwu: Multiply High Word Unsigned
    Mulhwu = 123,
    /// mullw: Multiply Low Word
    Mullw = 124,
    /// nand: NAND
    Nand = 125,
    /// neg: Negate
    Neg = 126,
    /// nor: NOR
    Nor = 127,
    /// or: OR
    Or = 128,
    /// orc: OR with Complement
    Orc = 129,
    /// slw: Shift Left Word
    Slw = 130,
    /// sraw: Shift Right Algebraic Word
    Sraw = 131,
    /// srawi: Shift Right Algebraic Word Immediate
    Srawi = 132,
    /// srw: Shift Right Word
    Srw = 133,
    /// stbux: Store Byte with Update Indexed
    Stbux = 134,
    /// stbx: Store Byte Indexed
    Stbx = 135,
    /// stfdux: Store Floating-Point Double with Update Indexed
    Stfdux = 136,
    /// stfdx: Store Floating-Point Double Indexed
    Stfdx = 137,
    /// stfiwx: Store Floating-Point as Integer Word Indexed
    Stfiwx = 138,
    /// stfsux: Store Floating-Point Single with Update Indexed
    Stfsux = 139,
    /// stfsx: Store Floating-Point Single Indexed
    Stfsx = 140,
    /// sthbrx: Store Half Word Byte-Reverse Indexed
    Sthbrx = 141,
    /// sthux: Store Half Word with Update Indexed
    Sthux = 142,
    /// sthx: Store Half Word Indexed
    Sthx = 143,
    /// stswi: Store String Word Immediate
    Stswi = 144,
    /// stswx: Store String Word Indexed
    Stswx = 145,
    /// stwbrx: Store Word Byte-Reverse Indexed
    Stwbrx = 146,
    /// stwcx.: Store Word Conditional Indexed
    Stwcx_ = 147,
    /// stwux: Store Word Indexed
    Stwux = 148,
    /// stwx: Store Word Indexed
    Stwx = 149,
    /// subf: Subtract From Carrying
    Subf = 150,
    /// subfc: Subtract from Carrying
    Subfc = 151,
    /// subfe: Subtract from Extended
    Subfe = 152,
    /// subfme: Subtract from Minus One Extended
    Subfme = 153,
    /// subfze: Subtract from Zero Extended
    Subfze = 154,
    /// sync: Synchronize
    Sync = 155,
    /// tlbie: Translation Lookaside Buffer Invalidate Entry
    Tlbie = 156,
    /// tlbsync: TLB Synchronize
    Tlbsync = 157,
    /// tw: Trap Word
    Tw = 158,
    /// xor: XOR
    Xor = 159,
    /// lwz: Load Word and Zero
    Lwz = 160,
    /// lwzu: Load Word and Zero with Update
    Lwzu = 161,
    /// lbz: Load Byte and Zero
    Lbz = 162,
    /// lbzu: Load Byte and Zero with Update
    Lbzu = 163,
    /// stw: Store Word
    Stw = 164,
    /// stwu: Store Word with Update
    Stwu = 165,
    /// stb: Store Byte
    Stb = 166,
    /// stbu: Store Byte with Update
    Stbu = 167,
    /// lhz: Load Half Word and Zero
    Lhz = 168,
    /// lhzu: Load Half Word and Zero with Update
    Lhzu = 169,
    /// lha: Load Half Word Algebraic
    Lha = 170,
    /// lhau: Load Half Word Algebraic with Update
    Lhau = 171,
    /// sth: Store Half Word
    Sth = 172,
    /// sthu: Store Half Word with Update
    Sthu = 173,
    /// lmw: Load Multiple Word
    Lmw = 174,
    /// stmw: Store Multiple Word
    Stmw = 175,
    /// lfs: Load Floating-Point Single
    Lfs = 176,
    /// lfsu: Load Floating-Point Single with Update
    Lfsu = 177,
    /// lfd: Load Floating-Point Double
    Lfd = 178,
    /// lfdu: Load Floating-Point Double with Update
    Lfdu = 179,
    /// stfs: Store Floating-Point Single
    Stfs = 180,
    /// stfsu: Store Floating-Point Single with Update
    Stfsu = 181,
    /// stfd: Store Floating-Point Double
    Stfd = 182,
    /// stfdu: Store Floating-Point Double with Update
    Stfdu = 183,
    /// psq_l: Paired Single Quantized Load
    PsqL = 184,
    /// psq_lu: Paired Single Quantized Load with Update
    PsqLu = 185,
    /// fadds: Floating Add (Single-Precision)
    Fadds = 186,
    /// fdivs: Floating Divide (Single-Precision)
    Fdivs = 187,
    /// fmadds: Floating Multiply-Add (Single-Precision)
    Fmadds = 188,
    /// fmsubs: Floating Multiply-Subtract (Single-Precision)
    Fmsubs = 189,
    /// fmuls: Floating Multiply (Single-Precision)
    Fmuls = 190,
    /// fnmadds: Floating Negative Multiply-Add (Single-Precision)
    Fnmadds = 191,
    /// fnmsubs: Floating Negative Multiply-Subtract (Single-Precision)
    Fnmsubs = 192,
    /// fres: Floating Reciprocal Estimate Single
    Fres = 193,
    /// fsubs: Floating Subtract (Single-Precision)
    Fsubs = 194,
    /// psq_st: Paired Single Quantized Store
    PsqSt = 195,
    /// psq_stu: Paired Single Quantized Store with Update
    PsqStu = 196,
    /// fabs: Floating Absolute Value
    Fabs = 197,
    /// fadd: Floating Add (Double-Precision)
    Fadd = 198,
    /// fcmpo: Floating Compare Ordered
    Fcmpo = 199,
    /// fcmpu: Floating Compare Unordered
    Fcmpu = 200,
    /// fctiw: Floating Convert to Integer Word
    Fctiw = 201,
    /// fctiwz: Floating Convert to Integer Word with Round toward Zero
    Fctiwz = 202,
    /// fdiv: Floating Divide (Double-Precision)
    Fdiv = 203,
    /// fmadd: Floating Multiply-Add (Double-Precision)
    Fmadd = 204,
    /// fmr: Floating Move Register (Double-Precision)
    Fmr = 205,
    /// fmsub: Floating Multiply-Subtract (Double-Precision)
    Fmsub = 206,
    /// fmul: Floating Multiply (Double-Precision)
    Fmul = 207,
    /// fnabs: Floating Negative Absolute Value
    Fnabs = 208,
    /// fneg: Floating Negate
    Fneg = 209,
    /// fnmadd: Floating Negative Multiply-Add (Double-Precision)
    Fnmadd = 210,
    /// fnmsub: Floating Negative Multiply-Subtract (Double-Precision)
    Fnmsub = 211,
    /// frsp: Floating Round to Single
    Frsp = 212,
    /// frsqrte: Floating Reciprocal Square Root Estimate
    Frsqrte = 213,
    /// fsel: Floating Select
    Fsel = 214,
    /// fsub: Floating Subtract (Double-Precision)
    Fsub = 215,
    /// mcrfs: Move to Condition Register from FPSCR
    Mcrfs = 216,
    /// mffs: Move from FPSCR
    Mffs = 217,
    /// mtfsb0: Move to FPSCR Bit 0
    Mtfsb0 = 218,
    /// mtfsb1: Move to FPSCR Bit 1
    Mtfsb1 = 219,
    /// mtfsf: Move to FPSCR Fields
    Mtfsf = 220,
    /// mtfsfi: Move to FPSCR Field Immediate
    Mtfsfi = 221,
}
impl Opcode {
    #[inline]
    pub const fn _mnemonic(self) -> &'static str {
        OPCODE_NAMES[self as usize]
    }
    #[inline]
    pub const fn _detect(code: u32) -> Self {
        let entry = OPCODE_ENTRIES[(code >> 26) as usize];
        let mut i = entry.0;
        while i < entry.1 {
            let pattern = OPCODE_PATTERNS[i as usize];
            if (code & pattern.0) == pattern.1 {
                // Safety: The enum is repr(u8) and marked non_exhaustive
                return unsafe { core::mem::transmute(i) };
            }
            i += 1;
        }
        Self::Illegal
    }
}
impl Ins {
    /// simm: Signed Immediate
    #[inline(always)]
    pub const fn field_simm(&self) -> i16 {
        ((self.code & 0xffff) ^ 0x8000).wrapping_sub(0x8000) as i16
    }
    /// uimm: Unsigned Immediate
    #[inline(always)]
    pub const fn field_uimm(&self) -> u16 {
        (self.code & 0xffff) as u16
    }
    /// offset: Branch Offset
    #[inline(always)]
    pub const fn field_offset(&self) -> i16 {
        ((self.code & 0xffff) ^ 0x8000).wrapping_sub(0x8000) as i16
    }
    /// ps_offset: Paired Single Offset
    #[inline(always)]
    pub const fn field_ps_offset(&self) -> i16 {
        ((self.code & 0xfff) ^ 0x800).wrapping_sub(0x800) as i16
    }
    /// BO: Branch Options
    #[inline(always)]
    pub const fn field_bo(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// BI: Branch Condition Register Bit
    #[inline(always)]
    pub const fn field_bi(&self) -> u8 {
        ((self.code >> 16) & 0x1f) as u8
    }
    /// BD: Branch Destination (14-bit)
    #[inline(always)]
    pub const fn field_bd(&self) -> i16 {
        ((self.code & 0xfffc) ^ 0x8000).wrapping_sub(0x8000) as i16
    }
    /// LI: Branch Destination (24-bit)
    #[inline(always)]
    pub const fn field_li(&self) -> i32 {
        ((self.code & 0x3fffffc) ^ 0x2000000).wrapping_sub(0x2000000) as i32
    }
    /// SH: Shift Amount
    #[inline(always)]
    pub const fn field_sh(&self) -> u8 {
        ((self.code >> 11) & 0x1f) as u8
    }
    /// MB: Mask Begin
    #[inline(always)]
    pub const fn field_mb(&self) -> u8 {
        ((self.code >> 6) & 0x1f) as u8
    }
    /// ME: Mask End
    #[inline(always)]
    pub const fn field_me(&self) -> u8 {
        ((self.code >> 1) & 0x1f) as u8
    }
    /// rS: Source Register
    #[inline(always)]
    pub const fn field_rs(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// rD: Destination Register
    #[inline(always)]
    pub const fn field_rd(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// rA: Register A
    #[inline(always)]
    pub const fn field_ra(&self) -> u8 {
        ((self.code >> 16) & 0x1f) as u8
    }
    /// rB: Register B
    #[inline(always)]
    pub const fn field_rb(&self) -> u8 {
        ((self.code >> 11) & 0x1f) as u8
    }
    /// sr: Segment Register
    #[inline(always)]
    pub const fn field_sr(&self) -> u8 {
        ((self.code >> 16) & 0xf) as u8
    }
    /// spr: Special Purpose Register
    #[inline(always)]
    pub const fn field_spr(&self) -> u16 {
        let value = ((self.code >> 11) & 0x3ff) as u16;
        ((value & 0b11111_00000) >> 5) | ((value & 0b00000_11111) << 5)
    }
    /// frS: Source Floating-Point Register
    #[inline(always)]
    pub const fn field_frs(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// frD: Destination Floating-Point Register
    #[inline(always)]
    pub const fn field_frd(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// frA: Floating-Point Register A
    #[inline(always)]
    pub const fn field_fra(&self) -> u8 {
        ((self.code >> 16) & 0x1f) as u8
    }
    /// frB: Floating-Point Register B
    #[inline(always)]
    pub const fn field_frb(&self) -> u8 {
        ((self.code >> 11) & 0x1f) as u8
    }
    /// frC: Floating-Point Register C
    #[inline(always)]
    pub const fn field_frc(&self) -> u8 {
        ((self.code >> 6) & 0x1f) as u8
    }
    /// crbD: Condition Register Bit Destination
    #[inline(always)]
    pub const fn field_crbd(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// crbA: Condition Register Bit A
    #[inline(always)]
    pub const fn field_crba(&self) -> u8 {
        ((self.code >> 16) & 0x1f) as u8
    }
    /// crbB: Condition Register Bit B
    #[inline(always)]
    pub const fn field_crbb(&self) -> u8 {
        ((self.code >> 11) & 0x1f) as u8
    }
    /// crfD: Condition Register Field Destination
    #[inline(always)]
    pub const fn field_crfd(&self) -> u8 {
        ((self.code >> 23) & 0x7) as u8
    }
    /// crfS: Condition Register Field Source
    #[inline(always)]
    pub const fn field_crfs(&self) -> u8 {
        ((self.code >> 18) & 0x7) as u8
    }
    /// crm: Condition Register Mask
    #[inline(always)]
    pub const fn field_crm(&self) -> u8 {
        ((self.code >> 12) & 0xff) as u8
    }
    /// ps_I
    #[inline(always)]
    pub const fn field_ps_i(&self) -> u8 {
        ((self.code >> 12) & 0x7) as u8
    }
    /// ps_IX
    #[inline(always)]
    pub const fn field_ps_ix(&self) -> u8 {
        ((self.code >> 7) & 0x7) as u8
    }
    /// ps_W
    #[inline(always)]
    pub const fn field_ps_w(&self) -> u8 {
        ((self.code >> 15) & 0x1) as u8
    }
    /// ps_WX
    #[inline(always)]
    pub const fn field_ps_wx(&self) -> u8 {
        ((self.code >> 10) & 0x1) as u8
    }
    /// NB
    #[inline(always)]
    pub const fn field_nb(&self) -> u8 {
        ((self.code >> 11) & 0x1f) as u8
    }
    /// tbr: Time Base
    #[inline(always)]
    pub const fn field_tbr(&self) -> u16 {
        let value = ((self.code >> 11) & 0x3ff) as u16;
        ((value & 0b11111_00000) >> 5) | ((value & 0b00000_11111) << 5)
    }
    /// mtfsf_FM: Field Mask for mtfsf
    #[inline(always)]
    pub const fn field_mtfsf_fm(&self) -> u8 {
        ((self.code >> 17) & 0xff) as u8
    }
    /// mtfsf_IMM: Immediate for mtfsfi
    #[inline(always)]
    pub const fn field_mtfsf_imm(&self) -> u8 {
        ((self.code >> 12) & 0xf) as u8
    }
    /// spr_SPRG: SPRG index for m[tf]sprg
    #[inline(always)]
    pub const fn field_spr_sprg(&self) -> u8 {
        ((self.code >> 16) & 0x3) as u8
    }
    /// spr_BAT: IBAT/DBAT index for m[tf][id]bat[ul]
    #[inline(always)]
    pub const fn field_spr_bat(&self) -> u8 {
        ((self.code >> 17) & 0x3) as u8
    }
    /// TO: Bitset for tw and twi
    #[inline(always)]
    pub const fn field_to(&self) -> u8 {
        ((self.code >> 21) & 0x1f) as u8
    }
    /// L: Bitset for cmp, cmpi, cmpl, cmpli
    #[inline(always)]
    pub const fn field_l(&self) -> u8 {
        ((self.code >> 21) & 0x1) as u8
    }
    /// OE: Field used by XO-form instructions to enable setting OV and SO in the XER.
    #[inline(always)]
    pub const fn field_oe(&self) -> bool {
        (self.code & 0x400) == 0x400
    }
    /// Rc: Record Bit
    #[inline(always)]
    pub const fn field_rc(&self) -> bool {
        (self.code & 0x1) == 0x1
    }
    /// LK: Link Bit
    #[inline(always)]
    pub const fn field_lk(&self) -> bool {
        (self.code & 0x1) == 0x1
    }
    /// AA: Absolute Address Bit
    #[inline(always)]
    pub const fn field_aa(&self) -> bool {
        (self.code & 0x2) == 0x2
    }
    /// BP: Predict branch to be taken
    #[inline(always)]
    pub const fn field_bp(&self) -> bool {
        (self.code & 0x200000) == 0x200000 && self.field_bd() >= 0x0
    }
    /// BNP: Predict branch not to be taken (fall through)
    #[inline(always)]
    pub const fn field_bnp(&self) -> bool {
        (self.code & 0x200000) == 0x200000 && self.field_bd() < 0x0
    }
    /// BP_ND: Predict branch to be taken (implicit dest for LR/CTR)
    #[inline(always)]
    pub const fn field_bp_nd(&self) -> bool {
        (self.code & 0x200000) == 0x200000
    }
}
pub type Arguments = [Argument; 5];
pub type MnemonicFunction = fn(&Ins) -> (&'static str, Arguments);
#[inline(always)]
const fn base_twi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "twi",
        [
            Argument::OpaqueU(OpaqueU(ins.field_to() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_twi(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_to() == 0x8 {
        return (
            "twgti",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_to() == 0x6 {
        return (
            "twllei",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_to() == 0x1f {
        return (
            "twui",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_twi(ins)
}
const fn base_dcbz_l(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbz_l",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_psq_lux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_lux",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_wx() as _)),
            Argument::GQR(GQR(ins.field_ps_ix() as _)),
        ],
    )
}
const fn base_psq_lx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_lx",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_wx() as _)),
            Argument::GQR(GQR(ins.field_ps_ix() as _)),
        ],
    )
}
const fn base_psq_stux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_stux",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_wx() as _)),
            Argument::GQR(GQR(ins.field_ps_ix() as _)),
        ],
    )
}
const fn base_psq_stx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_stx",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_wx() as _)),
            Argument::GQR(GQR(ins.field_ps_ix() as _)),
        ],
    )
}
const fn base_ps_abs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_abs", "ps_abs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_add(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_add", "ps_add."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_cmpo0(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ps_cmpo0",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_cmpo1(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ps_cmpo1",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_cmpu0(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ps_cmpu0",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_cmpu1(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ps_cmpu1",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_div(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_div", "ps_div."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_madd(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_madd", "ps_madd."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_madds0(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_madds0", "ps_madds0."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_madds1(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_madds1", "ps_madds1."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_merge00(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_merge00", "ps_merge00."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_merge01(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_merge01", "ps_merge01."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_merge10(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_merge10", "ps_merge10."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_merge11(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_merge11", "ps_merge11."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_mr(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_mr", "ps_mr."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_msub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_msub", "ps_msub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_mul(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_mul", "ps_mul."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_muls0(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_muls0", "ps_muls0."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_muls1(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_muls1", "ps_muls1."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_nabs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_nabs", "ps_nabs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_neg(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_neg", "ps_neg."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_nmadd(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_nmadd", "ps_nmadd."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_nmsub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_nmsub", "ps_nmsub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_res(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_res", "ps_res."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_rsqrte(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_rsqrte", "ps_rsqrte."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_sel(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_sel", "ps_sel."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_sub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_sub", "ps_sub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ps_sum0(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_sum0", "ps_sum0."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_ps_sum1(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["ps_sum1", "ps_sum1."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_mulli(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mulli",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subfic(ins: &Ins) -> (&'static str, Arguments) {
    (
        "subfic",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_cmpli(ins: &Ins) -> (&'static str, Arguments) {
    (
        "cmpli",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_l() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
        ],
    )
}
const fn simplified_cmpli(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x0 {
        return (
            "cmplwi",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Uimm(Uimm(ins.field_uimm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x0 {
        return (
            "cmplwi",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Uimm(Uimm(ins.field_uimm() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x1 {
        return (
            "cmpldi",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Uimm(Uimm(ins.field_uimm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x1 {
        return (
            "cmpldi",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Uimm(Uimm(ins.field_uimm() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_cmpli(ins)
}
#[inline(always)]
const fn base_cmpi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "cmpi",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_l() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
        ],
    )
}
const fn simplified_cmpi(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x0 {
        return (
            "cmpwi",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x0 {
        return (
            "cmpwi",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x1 {
        return (
            "cmpdi",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x1 {
        return (
            "cmpdi",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_cmpi(ins)
}
#[inline(always)]
const fn base_addic(ins: &Ins) -> (&'static str, Arguments) {
    (
        "addic",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_addic(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_simm() < 0x0 && ins.field_simm() != -0x8000 {
        return (
            "subic",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm((-ins.field_simm()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_addic(ins)
}
#[inline(always)]
const fn base_addic_(ins: &Ins) -> (&'static str, Arguments) {
    (
        "addic.",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_addic_(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_simm() < 0x0 && ins.field_simm() != -0x8000 {
        return (
            "subic.",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm((-ins.field_simm()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_addic_(ins)
}
#[inline(always)]
const fn base_addi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "addi",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Simm(Simm(ins.field_simm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_addi(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_ra() == 0x0 {
        return (
            "li",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::Simm(Simm(ins.field_simm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_simm() < 0x0 && ins.field_simm() != -0x8000 {
        return (
            "subi",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm((-ins.field_simm()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_addi(ins)
}
#[inline(always)]
const fn base_addis(ins: &Ins) -> (&'static str, Arguments) {
    (
        "addis",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_addis(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_ra() == 0x0 {
        return (
            "lis",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::Uimm(Uimm(ins.field_uimm() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_simm() < 0x0 && ins.field_simm() != -0x8000 {
        return (
            "subis",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::Simm(Simm((-ins.field_simm()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_addis(ins)
}
#[inline(always)]
const fn base_bc(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "bc",
            "bcl",
            "bca",
            "bcla",
            "bc+",
            "bcl+",
            "bca+",
            "bcla+",
            "bc-",
            "bcl-",
            "bca-",
            "bcla-",
            "<illegal>",
            "<illegal>",
            "<illegal>",
            "<illegal>",
        ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
            | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
        [
            Argument::OpaqueU(OpaqueU(ins.field_bo() as _)),
            Argument::CRBit(CRBit(ins.field_bi() as _)),
            Argument::BranchDest(BranchDest(ins.field_bd() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_bc(ins: &Ins) -> (&'static str, Arguments) {
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x0 {
        return (
            [
                "blt",
                "bltl",
                "blta",
                "bltla",
                "blt+",
                "bltl+",
                "blta+",
                "bltla+",
                "blt-",
                "bltl-",
                "blta-",
                "bltla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "blt",
                "bltl",
                "blta",
                "bltla",
                "blt+",
                "bltl+",
                "blta+",
                "bltla+",
                "blt-",
                "bltl-",
                "blta-",
                "bltla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x1 {
        return (
            [
                "ble",
                "blel",
                "blea",
                "blela",
                "ble+",
                "blel+",
                "blea+",
                "blela+",
                "ble-",
                "blel-",
                "blea-",
                "blela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "ble",
                "blel",
                "blea",
                "blela",
                "ble+",
                "blel+",
                "blea+",
                "blela+",
                "ble-",
                "blel-",
                "blea-",
                "blela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x2 {
        return (
            [
                "beq",
                "beql",
                "beqa",
                "beqla",
                "beq+",
                "beql+",
                "beqa+",
                "beqla+",
                "beq-",
                "beql-",
                "beqa-",
                "beqla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "beq",
                "beql",
                "beqa",
                "beqla",
                "beq+",
                "beql+",
                "beqa+",
                "beqla+",
                "beq-",
                "beql-",
                "beqa-",
                "beqla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x0 {
        return (
            [
                "bge",
                "bgel",
                "bgea",
                "bgela",
                "bge+",
                "bgel+",
                "bgea+",
                "bgela+",
                "bge-",
                "bgel-",
                "bgea-",
                "bgela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "bge",
                "bgel",
                "bgea",
                "bgela",
                "bge+",
                "bgel+",
                "bgea+",
                "bgela+",
                "bge-",
                "bgel-",
                "bgea-",
                "bgela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x1 {
        return (
            [
                "bgt",
                "bgtl",
                "bgta",
                "bgtla",
                "bgt+",
                "bgtl+",
                "bgta+",
                "bgtla+",
                "bgt-",
                "bgtl-",
                "bgta-",
                "bgtla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "bgt",
                "bgtl",
                "bgta",
                "bgtla",
                "bgt+",
                "bgtl+",
                "bgta+",
                "bgtla+",
                "bgt-",
                "bgtl-",
                "bgta-",
                "bgtla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x2 {
        return (
            [
                "bne",
                "bnel",
                "bnea",
                "bnela",
                "bne+",
                "bnel+",
                "bnea+",
                "bnela+",
                "bne-",
                "bnel-",
                "bnea-",
                "bnela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "bne",
                "bnel",
                "bnea",
                "bnela",
                "bne+",
                "bnel+",
                "bnea+",
                "bnela+",
                "bne-",
                "bnel-",
                "bnea-",
                "bnela-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x3 {
        return (
            [
                "bso",
                "bsol",
                "bsoa",
                "bsola",
                "bso+",
                "bsol+",
                "bsoa+",
                "bsola+",
                "bso-",
                "bsol-",
                "bsoa-",
                "bsola-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bso",
                "bsol",
                "bsoa",
                "bsola",
                "bso+",
                "bsol+",
                "bsoa+",
                "bsola+",
                "bso-",
                "bsol-",
                "bsoa-",
                "bsola-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x3 {
        return (
            [
                "bns",
                "bnsl",
                "bnsa",
                "bnsla",
                "bns+",
                "bnsl+",
                "bnsa+",
                "bnsla+",
                "bns-",
                "bnsl-",
                "bnsa-",
                "bnsla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bns",
                "bnsl",
                "bnsa",
                "bnsla",
                "bns+",
                "bnsl+",
                "bnsa+",
                "bnsla+",
                "bns-",
                "bnsl-",
                "bnsa-",
                "bnsla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x10 && ins.field_bi() == 0x0 {
        return (
            [
                "bdnz",
                "bdnzl",
                "bdnza",
                "bdnzla",
                "bdnz+",
                "bdnzl+",
                "bdnza+",
                "bdnzla+",
                "bdnz-",
                "bdnzl-",
                "bdnza-",
                "bdnzla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x8 {
        return (
            [
                "bdnzt",
                "bdnztl",
                "bdnzta",
                "bdnztla",
                "bdnzt+",
                "bdnztl+",
                "bdnzta+",
                "bdnztla+",
                "bdnzt-",
                "bdnztl-",
                "bdnzta-",
                "bdnztla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x0 {
        return (
            [
                "bdnzf",
                "bdnzfl",
                "bdnzfa",
                "bdnzfla",
                "bdnzf+",
                "bdnzfl+",
                "bdnzfa+",
                "bdnzfla+",
                "bdnzf-",
                "bdnzfl-",
                "bdnzfa-",
                "bdnzfla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x12 && ins.field_bi() == 0x0 {
        return (
            [
                "bdz",
                "bdzl",
                "bdza",
                "bdzla",
                "bdz+",
                "bdzl+",
                "bdza+",
                "bdzla+",
                "bdz-",
                "bdzl-",
                "bdza-",
                "bdzla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xa {
        return (
            [
                "bdzt",
                "bdztl",
                "bdzta",
                "bdztla",
                "bdzt+",
                "bdztl+",
                "bdzta+",
                "bdztla+",
                "bdzt-",
                "bdztl-",
                "bdzta-",
                "bdztla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x2 {
        return (
            [
                "bdzf",
                "bdzfl",
                "bdzfa",
                "bdzfla",
                "bdzf+",
                "bdzfl+",
                "bdzfa+",
                "bdzfla+",
                "bdzf-",
                "bdzfl-",
                "bdzfa-",
                "bdzfla-",
                "<illegal>",
                "<illegal>",
                "<illegal>",
                "<illegal>",
            ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1
                | (ins.field_bp() as usize) << 2 | (ins.field_bnp() as usize) << 3],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::BranchDest(BranchDest(ins.field_bd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_bc(ins)
}
const fn base_sc(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sc",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
const fn base_b(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "b",
            "bl",
            "ba",
            "bla",
        ][ins.field_lk() as usize | (ins.field_aa() as usize) << 1],
        [
            Argument::BranchDest(BranchDest(ins.field_li() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_bcctr(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "bcctr",
            "bcctrl",
            "bcctr+",
            "bcctrl+",
        ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
        [
            Argument::OpaqueU(OpaqueU(ins.field_bo() as _)),
            Argument::CRBit(CRBit(ins.field_bi() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_bcctr(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_bo() == 0x14 && ins.field_bi() == 0x0 {
        return (
            ["bctr", "bctrl"][ins.field_lk() as usize],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x0 {
        return (
            [
                "bltctr",
                "bltctrl",
                "bltctr+",
                "bltctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "bltctr",
                "bltctrl",
                "bltctr+",
                "bltctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x1 {
        return (
            [
                "blectr",
                "blectrl",
                "blectr+",
                "blectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "blectr",
                "blectrl",
                "blectr+",
                "blectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x2 {
        return (
            [
                "beqctr",
                "beqctrl",
                "beqctr+",
                "beqctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "beqctr",
                "beqctrl",
                "beqctr+",
                "beqctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x0 {
        return (
            [
                "bgectr",
                "bgectrl",
                "bgectr+",
                "bgectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "bgectr",
                "bgectrl",
                "bgectr+",
                "bgectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x1 {
        return (
            [
                "bgtctr",
                "bgtctrl",
                "bgtctr+",
                "bgtctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "bgtctr",
                "bgtctrl",
                "bgtctr+",
                "bgtctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x2 {
        return (
            [
                "bnectr",
                "bnectrl",
                "bnectr+",
                "bnectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "bnectr",
                "bnectrl",
                "bnectr+",
                "bnectrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x3 {
        return (
            [
                "bsoctr",
                "bsoctrl",
                "bsoctr+",
                "bsoctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bsoctr",
                "bsoctrl",
                "bsoctr+",
                "bsoctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x3 {
        return (
            [
                "bnsctr",
                "bnsctrl",
                "bnsctr+",
                "bnsctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bnsctr",
                "bnsctrl",
                "bnsctr+",
                "bnsctrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_bcctr(ins)
}
#[inline(always)]
const fn base_bclr(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "bclr",
            "bclrl",
            "bclr+",
            "bclrl+",
        ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
        [
            Argument::OpaqueU(OpaqueU(ins.field_bo() as _)),
            Argument::CRBit(CRBit(ins.field_bi() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_bclr(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_bo() == 0x14 && ins.field_bi() == 0x0 {
        return (
            ["blr", "blrl"][ins.field_lk() as usize],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x0 {
        return (
            [
                "bltlr",
                "bltlrl",
                "bltlr+",
                "bltlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "bltlr",
                "bltlrl",
                "bltlr+",
                "bltlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x1 {
        return (
            [
                "blelr",
                "blelrl",
                "blelr+",
                "blelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "blelr",
                "blelrl",
                "blelr+",
                "blelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x2 {
        return (
            [
                "beqlr",
                "beqlrl",
                "beqlr+",
                "beqlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "beqlr",
                "beqlrl",
                "beqlr+",
                "beqlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x0 {
        return (
            [
                "bgelr",
                "bgelrl",
                "bgelr+",
                "bgelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x0 {
        return (
            [
                "bgelr",
                "bgelrl",
                "bgelr+",
                "bgelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x1 {
        return (
            [
                "bgtlr",
                "bgtlrl",
                "bgtlr+",
                "bgtlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x1 {
        return (
            [
                "bgtlr",
                "bgtlrl",
                "bgtlr+",
                "bgtlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x2 {
        return (
            [
                "bnelr",
                "bnelrl",
                "bnelr+",
                "bnelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x2 {
        return (
            [
                "bnelr",
                "bnelrl",
                "bnelr+",
                "bnelrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && ins.field_bi() == 0x3 {
        return (
            [
                "bsolr",
                "bsolrl",
                "bsolr+",
                "bsolrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xc && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bsolr",
                "bsolrl",
                "bsolr+",
                "bsolrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && ins.field_bi() == 0x3 {
        return (
            [
                "bnslr",
                "bnslrl",
                "bnslr+",
                "bnslrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x4 && (ins.field_bi() & 0x3) == 0x3 {
        return (
            [
                "bnslr",
                "bnslrl",
                "bnslr+",
                "bnslrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRField(CRField(ins.field_crfs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x10 && ins.field_bi() == 0x0 {
        return (
            [
                "bdnzlr",
                "bdnzlrl",
                "bdnzlr+",
                "bdnzlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x8 {
        return (
            [
                "bdnztlr",
                "bdnztlrl",
                "bdnztlr+",
                "bdnztlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x0 {
        return (
            [
                "bdnzflr",
                "bdnzflrl",
                "bdnzflr+",
                "bdnzflrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x12 && ins.field_bi() == 0x0 {
        return (
            [
                "bdzlr",
                "bdzlrl",
                "bdzlr+",
                "bdzlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0xa {
        return (
            [
                "bdztlr",
                "bdztlrl",
                "bdztlr+",
                "bdztlrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_bo() & 0x1e) == 0x0 {
        return (
            [
                "bdzflr",
                "bdzflrl",
                "bdzflr+",
                "bdzflrl+",
            ][ins.field_lk() as usize | (ins.field_bp_nd() as usize) << 1],
            [
                Argument::CRBit(CRBit(ins.field_bi() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_bclr(ins)
}
const fn base_crand(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crand",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_crandc(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crandc",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_creqv(ins: &Ins) -> (&'static str, Arguments) {
    (
        "creqv",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_creqv(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crba() == ins.field_crbd() && ins.field_crbb() == ins.field_crbd() {
        return (
            "crset",
            [
                Argument::CRBit(CRBit(ins.field_crbd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_creqv(ins)
}
const fn base_crnand(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crnand",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_crnor(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crnor",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_crnor(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crbb() == ins.field_crba() {
        return (
            "crnot",
            [
                Argument::CRBit(CRBit(ins.field_crbd() as _)),
                Argument::CRBit(CRBit(ins.field_crba() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_crnor(ins)
}
#[inline(always)]
const fn base_cror(ins: &Ins) -> (&'static str, Arguments) {
    (
        "cror",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_cror(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crbb() == ins.field_crba() {
        return (
            "crmove",
            [
                Argument::CRBit(CRBit(ins.field_crbd() as _)),
                Argument::CRBit(CRBit(ins.field_crba() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_cror(ins)
}
const fn base_crorc(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crorc",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_crxor(ins: &Ins) -> (&'static str, Arguments) {
    (
        "crxor",
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::CRBit(CRBit(ins.field_crba() as _)),
            Argument::CRBit(CRBit(ins.field_crbb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_crxor(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crba() == ins.field_crbd() && ins.field_crbb() == ins.field_crbd() {
        return (
            "crclr",
            [
                Argument::CRBit(CRBit(ins.field_crbd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_crxor(ins)
}
const fn base_isync(ins: &Ins) -> (&'static str, Arguments) {
    (
        "isync",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
const fn base_mcrf(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mcrf",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::CRField(CRField(ins.field_crfs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_rfi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "rfi",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
const fn base_rlwimi(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["rlwimi", "rlwimi."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_mb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_me() as _)),
        ],
    )
}
#[inline(always)]
const fn base_rlwinm(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["rlwinm", "rlwinm."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_mb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_me() as _)),
        ],
    )
}
const fn simplified_rlwinm(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_sh() == 0x0 && ins.field_mb() == 0x0 {
        return (
            ["clrrwi", "clrrwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU((31 - ins.field_me()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_sh() == 0x0 && ins.field_me() == 0x1f {
        return (
            ["clrlwi", "clrlwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_mb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_mb() == 0x0 && ins.field_me() == 0x1f && ins.field_sh() <= 0x10 {
        return (
            ["rotlwi", "rotlwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_mb() == 0x0 && ins.field_me() == 0x1f && ins.field_sh() > 0x10 {
        return (
            ["rotrwi", "rotrwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU((32 - ins.field_sh()) as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_mb() == 0x0 && ins.field_me() == 31 - ins.field_sh() {
        return (
            ["slwi", "slwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_me() == 0x1f && ins.field_sh() == 32 - ins.field_mb() {
        return (
            ["srwi", "srwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_mb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_sh() < 0x20 && ins.field_me() == 31 - ins.field_sh() {
        return (
            ["clrlslwi", "clrlslwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU((ins.field_mb() + ins.field_sh()) as _)),
                Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
                Argument::None,
            ],
        );
    }
    if ins.field_mb() == 0x0 {
        return (
            ["extlwi", "extlwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU((ins.field_me() + 1) as _)),
                Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
                Argument::None,
            ],
        );
    }
    if ins.field_me() == 0x1f && ins.field_sh() >= 32 - ins.field_mb() {
        return (
            ["extrwi", "extrwi."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::OpaqueU(OpaqueU((32 - ins.field_mb()) as _)),
                Argument::OpaqueU(
                    OpaqueU((ins.field_sh() - (32 - ins.field_mb())) as _),
                ),
                Argument::None,
            ],
        );
    }
    base_rlwinm(ins)
}
#[inline(always)]
const fn base_rlwnm(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["rlwnm", "rlwnm."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_mb() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_me() as _)),
        ],
    )
}
const fn simplified_rlwnm(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_mb() == 0x0 && ins.field_me() == 0x1f {
        return (
            ["rotlw", "rotlw."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_rlwnm(ins)
}
#[inline(always)]
const fn base_ori(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ori",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_ori(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_ra() == 0x0 && ins.field_rs() == 0x0 && ins.field_uimm() == 0x0 {
        return (
            "nop",
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_ori(ins)
}
const fn base_oris(ins: &Ins) -> (&'static str, Arguments) {
    (
        "oris",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_xori(ins: &Ins) -> (&'static str, Arguments) {
    (
        "xori",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_xoris(ins: &Ins) -> (&'static str, Arguments) {
    (
        "xoris",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_andi_(ins: &Ins) -> (&'static str, Arguments) {
    (
        "andi.",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_andis_(ins: &Ins) -> (&'static str, Arguments) {
    (
        "andis.",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Uimm(Uimm(ins.field_uimm() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_add(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "add",
            "addo",
            "add.",
            "addo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_addc(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "addc",
            "addco",
            "addc.",
            "addco.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_adde(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "adde",
            "addeo",
            "adde.",
            "addeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_addme(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "addme",
            "addmeo",
            "addme.",
            "addmeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_addze(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "addze",
            "addzeo",
            "addze.",
            "addzeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_and(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["and", "and."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_andc(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["andc", "andc."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_cmp(ins: &Ins) -> (&'static str, Arguments) {
    (
        "cmp",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_l() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
        ],
    )
}
const fn simplified_cmp(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x0 {
        return (
            "cmpw",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x0 {
        return (
            "cmpw",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x1 {
        return (
            "cmpd",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x1 {
        return (
            "cmpd",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_cmp(ins)
}
#[inline(always)]
const fn base_cmpl(ins: &Ins) -> (&'static str, Arguments) {
    (
        "cmpl",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_l() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
        ],
    )
}
const fn simplified_cmpl(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x0 {
        return (
            "cmplw",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x0 {
        return (
            "cmplw",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_crfd() == 0x0 && ins.field_l() == 0x1 {
        return (
            "cmpld",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_l() == 0x1 {
        return (
            "cmpld",
            [
                Argument::CRField(CRField(ins.field_crfd() as _)),
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_cmpl(ins)
}
const fn base_cntlzw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["cntlzw", "cntlzw."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbf(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbf",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbi",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbst(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbst",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbt(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbt",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbtst(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbtst",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_dcbz(ins: &Ins) -> (&'static str, Arguments) {
    (
        "dcbz",
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_divw(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "divw",
            "divwo",
            "divw.",
            "divwo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_divwu(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "divwu",
            "divwuo",
            "divwu.",
            "divwuo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_eciwx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "eciwx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_ecowx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "ecowx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_eieio(ins: &Ins) -> (&'static str, Arguments) {
    (
        "eieio",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
const fn base_eqv(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["eqv", "eqv."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_extsb(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["extsb", "extsb."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_extsh(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["extsh", "extsh."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_icbi(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["icbi", "icbi."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lbzux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lbzux",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lbzx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lbzx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfdux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfdux",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfdx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfdx",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfsux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfsux",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfsx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfsx",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhaux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhaux",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhax(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhax",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhbrx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhbrx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhzux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhzux",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhzx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhzx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lswi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lswi",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_nb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lswx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lswx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwarx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwarx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwbrx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwbrx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwzux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwzux",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwzx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwzx",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mcrxr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mcrxr",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mfcr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mfcr",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mfmsr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mfmsr",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_mfspr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mfspr",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::SPR(SPR(ins.field_spr() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_mfspr(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_spr() == 0x1 {
        return (
            "mfxer",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x8 {
        return (
            "mflr",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x9 {
        return (
            "mfctr",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x12 {
        return (
            "mfdsisr",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x13 {
        return (
            "mfdar",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x16 {
        return (
            "mfdec",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x19 {
        return (
            "mfsdr1",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x1a {
        return (
            "mfsrr0",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x1b {
        return (
            "mfsrr1",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3fc) == 0x110 {
        return (
            "mfsprg",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_spr_sprg() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x11a {
        return (
            "mfear",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x210 {
        return (
            "mfibatu",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x211 {
        return (
            "mfibatl",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x218 {
        return (
            "mfdbatu",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x219 {
        return (
            "mfdbatl",
            [
                Argument::GPR(GPR(ins.field_rd() as _)),
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_mfspr(ins)
}
const fn base_mfsr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mfsr",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::SR(SR(ins.field_sr() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mfsrin(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mfsrin",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mftb(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mftb",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_tbr() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtcrf(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mtcrf",
        [
            Argument::OpaqueU(OpaqueU(ins.field_crm() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtmsr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mtmsr",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_mtspr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mtspr",
        [
            Argument::SPR(SPR(ins.field_spr() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_mtspr(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_spr() == 0x1 {
        return (
            "mtxer",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x8 {
        return (
            "mtlr",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x9 {
        return (
            "mtctr",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x12 {
        return (
            "mtdsisr",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x13 {
        return (
            "mtdar",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x16 {
        return (
            "mtdec",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x19 {
        return (
            "mtsdr1",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x1a {
        return (
            "mtsrr0",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x1b {
        return (
            "mtsrr1",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3fc) == 0x110 {
        return (
            "mtsprg",
            [
                Argument::OpaqueU(OpaqueU(ins.field_spr_sprg() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x11a {
        return (
            "mtear",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x11c {
        return (
            "mttbl",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_spr() == 0x11d {
        return (
            "mttbu",
            [
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x210 {
        return (
            "mtibatu",
            [
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x211 {
        return (
            "mtibatl",
            [
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x218 {
        return (
            "mtdbatu",
            [
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if (ins.field_spr() & 0x3f9) == 0x219 {
        return (
            "mtdbatl",
            [
                Argument::OpaqueU(OpaqueU(ins.field_spr_bat() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_mtspr(ins)
}
const fn base_mtsr(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mtsr",
        [
            Argument::SR(SR(ins.field_sr() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtsrin(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mtsrin",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mulhw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mulhw", "mulhw."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mulhwu(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mulhwu", "mulhwu."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mullw(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "mullw",
            "mullwo",
            "mullw.",
            "mullwo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_nand(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["nand", "nand."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_neg(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "neg",
            "nego",
            "neg.",
            "nego.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_nor(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["nor", "nor."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
#[inline(always)]
const fn base_or(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["or", "or."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_or(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_rb() == ins.field_rs() {
        return (
            ["mr", "mr."][ins.field_rc() as usize],
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rs() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_or(ins)
}
const fn base_orc(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["orc", "orc."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_slw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["slw", "slw."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sraw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["sraw", "sraw."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_srawi(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["srawi", "srawi."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_srw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["srw", "srw."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stbux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stbux",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stbx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stbx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfdux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfdux",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfdx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfdx",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfiwx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfiwx",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfsux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfsux",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfsx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfsx",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sthbrx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sthbrx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sthux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sthux",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sthx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sthx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stswi(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stswi",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_nb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stswx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stswx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stwbrx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stwbrx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stwcx_(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stwcx.",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stwux(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stwux",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stwx(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stwx",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subf(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "subf",
            "subfo",
            "subf.",
            "subfo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subfc(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "subfc",
            "subfco",
            "subfc.",
            "subfco.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subfe(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "subfe",
            "subfeo",
            "subfe.",
            "subfeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subfme(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "subfme",
            "subfmeo",
            "subfme.",
            "subfmeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_subfze(ins: &Ins) -> (&'static str, Arguments) {
    (
        [
            "subfze",
            "subfzeo",
            "subfze.",
            "subfzeo.",
        ][ins.field_oe() as usize | (ins.field_rc() as usize) << 1],
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sync(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sync",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
const fn base_tlbie(ins: &Ins) -> (&'static str, Arguments) {
    (
        "tlbie",
        [
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_tlbsync(ins: &Ins) -> (&'static str, Arguments) {
    (
        "tlbsync",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
#[inline(always)]
const fn base_tw(ins: &Ins) -> (&'static str, Arguments) {
    (
        "tw",
        [
            Argument::OpaqueU(OpaqueU(ins.field_to() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn simplified_tw(ins: &Ins) -> (&'static str, Arguments) {
    if ins.field_to() == 0x4 {
        return (
            "tweq",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_to() == 0x5 {
        return (
            "twlge",
            [
                Argument::GPR(GPR(ins.field_ra() as _)),
                Argument::GPR(GPR(ins.field_rb() as _)),
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    if ins.field_to() == 0x1f && ins.field_ra() == 0x0 && ins.field_rb() == 0x0 {
        return (
            "trap",
            [
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        );
    }
    base_tw(ins)
}
const fn base_xor(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["xor", "xor."][ins.field_rc() as usize],
        [
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::GPR(GPR(ins.field_rb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwz(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwz",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lwzu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lwzu",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lbz(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lbz",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lbzu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lbzu",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stw(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stw",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stwu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stwu",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stb(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stb",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stbu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stbu",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhz(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhz",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhzu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhzu",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lha(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lha",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lhau(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lhau",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sth(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sth",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_sthu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "sthu",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lmw(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lmw",
        [
            Argument::GPR(GPR(ins.field_rd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stmw(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stmw",
        [
            Argument::GPR(GPR(ins.field_rs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfs(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfs",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfsu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfsu",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfd(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfd",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_lfdu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "lfdu",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfs(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfs",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfsu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfsu",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfd(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfd",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_stfdu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "stfdu",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_psq_l(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_l",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_ps_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_w() as _)),
            Argument::GQR(GQR(ins.field_ps_i() as _)),
        ],
    )
}
const fn base_psq_lu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_lu",
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::Offset(Offset(ins.field_ps_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_w() as _)),
            Argument::GQR(GQR(ins.field_ps_i() as _)),
        ],
    )
}
const fn base_fadds(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fadds", "fadds."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fdivs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fdivs", "fdivs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fmadds(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmadds", "fmadds."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fmsubs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmsubs", "fmsubs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fmuls(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmuls", "fmuls."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fnmadds(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fnmadds", "fnmadds."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fnmsubs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fnmsubs", "fnmsubs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fres(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fres", "fres."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fsubs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fsubs", "fsubs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_psq_st(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_st",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_ps_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_w() as _)),
            Argument::GQR(GQR(ins.field_ps_i() as _)),
        ],
    )
}
const fn base_psq_stu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "psq_stu",
        [
            Argument::FPR(FPR(ins.field_frs() as _)),
            Argument::Offset(Offset(ins.field_ps_offset() as _)),
            Argument::GPR(GPR(ins.field_ra() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_ps_w() as _)),
            Argument::GQR(GQR(ins.field_ps_i() as _)),
        ],
    )
}
const fn base_fabs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fabs", "fabs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fadd(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fadd", "fadd."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fcmpo(ins: &Ins) -> (&'static str, Arguments) {
    (
        "fcmpo",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fcmpu(ins: &Ins) -> (&'static str, Arguments) {
    (
        "fcmpu",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fctiw(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fctiw", "fctiw."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fctiwz(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fctiwz", "fctiwz."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fdiv(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fdiv", "fdiv."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fmadd(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmadd", "fmadd."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fmr(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmr", "fmr."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fmsub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmsub", "fmsub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fmul(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fmul", "fmul."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fnabs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fnabs", "fnabs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fneg(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fneg", "fneg."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fnmadd(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fnmadd", "fnmadd."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fnmsub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fnmsub", "fnmsub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_frsp(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["frsp", "frsp."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_frsqrte(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["frsqrte", "frsqrte."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_fsel(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fsel", "fsel."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frc() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
        ],
    )
}
const fn base_fsub(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["fsub", "fsub."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::FPR(FPR(ins.field_fra() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mcrfs(ins: &Ins) -> (&'static str, Arguments) {
    (
        "mcrfs",
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::CRField(CRField(ins.field_crfs() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mffs(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mffs", "mffs."][ins.field_rc() as usize],
        [
            Argument::FPR(FPR(ins.field_frd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtfsb0(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mtfsb0", "mtfsb0."][ins.field_rc() as usize],
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtfsb1(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mtfsb1", "mtfsb1."][ins.field_rc() as usize],
        [
            Argument::CRBit(CRBit(ins.field_crbd() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtfsf(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mtfsf", "mtfsf."][ins.field_rc() as usize],
        [
            Argument::OpaqueU(OpaqueU(ins.field_mtfsf_fm() as _)),
            Argument::FPR(FPR(ins.field_frb() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn base_mtfsfi(ins: &Ins) -> (&'static str, Arguments) {
    (
        ["mtfsfi", "mtfsfi."][ins.field_rc() as usize],
        [
            Argument::CRField(CRField(ins.field_crfd() as _)),
            Argument::OpaqueU(OpaqueU(ins.field_mtfsf_imm() as _)),
            Argument::None,
            Argument::None,
            Argument::None,
        ],
    )
}
const fn mnemonic_illegal(_ins: &Ins) -> (&'static str, Arguments) {
    (
        "<illegal>",
        [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None],
    )
}
pub const BASE_MNEMONICS: [MnemonicFunction; 256] = [
    base_twi,
    base_dcbz_l,
    base_psq_lux,
    base_psq_lx,
    base_psq_stux,
    base_psq_stx,
    base_ps_abs,
    base_ps_add,
    base_ps_cmpo0,
    base_ps_cmpo1,
    base_ps_cmpu0,
    base_ps_cmpu1,
    base_ps_div,
    base_ps_madd,
    base_ps_madds0,
    base_ps_madds1,
    base_ps_merge00,
    base_ps_merge01,
    base_ps_merge10,
    base_ps_merge11,
    base_ps_mr,
    base_ps_msub,
    base_ps_mul,
    base_ps_muls0,
    base_ps_muls1,
    base_ps_nabs,
    base_ps_neg,
    base_ps_nmadd,
    base_ps_nmsub,
    base_ps_res,
    base_ps_rsqrte,
    base_ps_sel,
    base_ps_sub,
    base_ps_sum0,
    base_ps_sum1,
    base_mulli,
    base_subfic,
    base_cmpli,
    base_cmpi,
    base_addic,
    base_addic_,
    base_addi,
    base_addis,
    base_bc,
    base_sc,
    base_b,
    base_bcctr,
    base_bclr,
    base_crand,
    base_crandc,
    base_creqv,
    base_crnand,
    base_crnor,
    base_cror,
    base_crorc,
    base_crxor,
    base_isync,
    base_mcrf,
    base_rfi,
    base_rlwimi,
    base_rlwinm,
    base_rlwnm,
    base_ori,
    base_oris,
    base_xori,
    base_xoris,
    base_andi_,
    base_andis_,
    base_add,
    base_addc,
    base_adde,
    base_addme,
    base_addze,
    base_and,
    base_andc,
    base_cmp,
    base_cmpl,
    base_cntlzw,
    base_dcbf,
    base_dcbi,
    base_dcbst,
    base_dcbt,
    base_dcbtst,
    base_dcbz,
    base_divw,
    base_divwu,
    base_eciwx,
    base_ecowx,
    base_eieio,
    base_eqv,
    base_extsb,
    base_extsh,
    base_icbi,
    base_lbzux,
    base_lbzx,
    base_lfdux,
    base_lfdx,
    base_lfsux,
    base_lfsx,
    base_lhaux,
    base_lhax,
    base_lhbrx,
    base_lhzux,
    base_lhzx,
    base_lswi,
    base_lswx,
    base_lwarx,
    base_lwbrx,
    base_lwzux,
    base_lwzx,
    base_mcrxr,
    base_mfcr,
    base_mfmsr,
    base_mfspr,
    base_mfsr,
    base_mfsrin,
    base_mftb,
    base_mtcrf,
    base_mtmsr,
    base_mtspr,
    base_mtsr,
    base_mtsrin,
    base_mulhw,
    base_mulhwu,
    base_mullw,
    base_nand,
    base_neg,
    base_nor,
    base_or,
    base_orc,
    base_slw,
    base_sraw,
    base_srawi,
    base_srw,
    base_stbux,
    base_stbx,
    base_stfdux,
    base_stfdx,
    base_stfiwx,
    base_stfsux,
    base_stfsx,
    base_sthbrx,
    base_sthux,
    base_sthx,
    base_stswi,
    base_stswx,
    base_stwbrx,
    base_stwcx_,
    base_stwux,
    base_stwx,
    base_subf,
    base_subfc,
    base_subfe,
    base_subfme,
    base_subfze,
    base_sync,
    base_tlbie,
    base_tlbsync,
    base_tw,
    base_xor,
    base_lwz,
    base_lwzu,
    base_lbz,
    base_lbzu,
    base_stw,
    base_stwu,
    base_stb,
    base_stbu,
    base_lhz,
    base_lhzu,
    base_lha,
    base_lhau,
    base_sth,
    base_sthu,
    base_lmw,
    base_stmw,
    base_lfs,
    base_lfsu,
    base_lfd,
    base_lfdu,
    base_stfs,
    base_stfsu,
    base_stfd,
    base_stfdu,
    base_psq_l,
    base_psq_lu,
    base_fadds,
    base_fdivs,
    base_fmadds,
    base_fmsubs,
    base_fmuls,
    base_fnmadds,
    base_fnmsubs,
    base_fres,
    base_fsubs,
    base_psq_st,
    base_psq_stu,
    base_fabs,
    base_fadd,
    base_fcmpo,
    base_fcmpu,
    base_fctiw,
    base_fctiwz,
    base_fdiv,
    base_fmadd,
    base_fmr,
    base_fmsub,
    base_fmul,
    base_fnabs,
    base_fneg,
    base_fnmadd,
    base_fnmsub,
    base_frsp,
    base_frsqrte,
    base_fsel,
    base_fsub,
    base_mcrfs,
    base_mffs,
    base_mtfsb0,
    base_mtfsb1,
    base_mtfsf,
    base_mtfsfi,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
];
pub const SIMPLIFIED_MNEMONICS: [MnemonicFunction; 256] = [
    simplified_twi,
    base_dcbz_l,
    base_psq_lux,
    base_psq_lx,
    base_psq_stux,
    base_psq_stx,
    base_ps_abs,
    base_ps_add,
    base_ps_cmpo0,
    base_ps_cmpo1,
    base_ps_cmpu0,
    base_ps_cmpu1,
    base_ps_div,
    base_ps_madd,
    base_ps_madds0,
    base_ps_madds1,
    base_ps_merge00,
    base_ps_merge01,
    base_ps_merge10,
    base_ps_merge11,
    base_ps_mr,
    base_ps_msub,
    base_ps_mul,
    base_ps_muls0,
    base_ps_muls1,
    base_ps_nabs,
    base_ps_neg,
    base_ps_nmadd,
    base_ps_nmsub,
    base_ps_res,
    base_ps_rsqrte,
    base_ps_sel,
    base_ps_sub,
    base_ps_sum0,
    base_ps_sum1,
    base_mulli,
    base_subfic,
    simplified_cmpli,
    simplified_cmpi,
    simplified_addic,
    simplified_addic_,
    simplified_addi,
    simplified_addis,
    simplified_bc,
    base_sc,
    base_b,
    simplified_bcctr,
    simplified_bclr,
    base_crand,
    base_crandc,
    simplified_creqv,
    base_crnand,
    simplified_crnor,
    simplified_cror,
    base_crorc,
    simplified_crxor,
    base_isync,
    base_mcrf,
    base_rfi,
    base_rlwimi,
    simplified_rlwinm,
    simplified_rlwnm,
    simplified_ori,
    base_oris,
    base_xori,
    base_xoris,
    base_andi_,
    base_andis_,
    base_add,
    base_addc,
    base_adde,
    base_addme,
    base_addze,
    base_and,
    base_andc,
    simplified_cmp,
    simplified_cmpl,
    base_cntlzw,
    base_dcbf,
    base_dcbi,
    base_dcbst,
    base_dcbt,
    base_dcbtst,
    base_dcbz,
    base_divw,
    base_divwu,
    base_eciwx,
    base_ecowx,
    base_eieio,
    base_eqv,
    base_extsb,
    base_extsh,
    base_icbi,
    base_lbzux,
    base_lbzx,
    base_lfdux,
    base_lfdx,
    base_lfsux,
    base_lfsx,
    base_lhaux,
    base_lhax,
    base_lhbrx,
    base_lhzux,
    base_lhzx,
    base_lswi,
    base_lswx,
    base_lwarx,
    base_lwbrx,
    base_lwzux,
    base_lwzx,
    base_mcrxr,
    base_mfcr,
    base_mfmsr,
    simplified_mfspr,
    base_mfsr,
    base_mfsrin,
    base_mftb,
    base_mtcrf,
    base_mtmsr,
    simplified_mtspr,
    base_mtsr,
    base_mtsrin,
    base_mulhw,
    base_mulhwu,
    base_mullw,
    base_nand,
    base_neg,
    base_nor,
    simplified_or,
    base_orc,
    base_slw,
    base_sraw,
    base_srawi,
    base_srw,
    base_stbux,
    base_stbx,
    base_stfdux,
    base_stfdx,
    base_stfiwx,
    base_stfsux,
    base_stfsx,
    base_sthbrx,
    base_sthux,
    base_sthx,
    base_stswi,
    base_stswx,
    base_stwbrx,
    base_stwcx_,
    base_stwux,
    base_stwx,
    base_subf,
    base_subfc,
    base_subfe,
    base_subfme,
    base_subfze,
    base_sync,
    base_tlbie,
    base_tlbsync,
    simplified_tw,
    base_xor,
    base_lwz,
    base_lwzu,
    base_lbz,
    base_lbzu,
    base_stw,
    base_stwu,
    base_stb,
    base_stbu,
    base_lhz,
    base_lhzu,
    base_lha,
    base_lhau,
    base_sth,
    base_sthu,
    base_lmw,
    base_stmw,
    base_lfs,
    base_lfsu,
    base_lfd,
    base_lfdu,
    base_stfs,
    base_stfsu,
    base_stfd,
    base_stfdu,
    base_psq_l,
    base_psq_lu,
    base_fadds,
    base_fdivs,
    base_fmadds,
    base_fmsubs,
    base_fmuls,
    base_fnmadds,
    base_fnmsubs,
    base_fres,
    base_fsubs,
    base_psq_st,
    base_psq_stu,
    base_fabs,
    base_fadd,
    base_fcmpo,
    base_fcmpu,
    base_fctiw,
    base_fctiwz,
    base_fdiv,
    base_fmadd,
    base_fmr,
    base_fmsub,
    base_fmul,
    base_fnabs,
    base_fneg,
    base_fnmadd,
    base_fnmsub,
    base_frsp,
    base_frsqrte,
    base_fsel,
    base_fsub,
    base_mcrfs,
    base_mffs,
    base_mtfsb0,
    base_mtfsb1,
    base_mtfsf,
    base_mtfsfi,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
    mnemonic_illegal,
];
const fn defs_twi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_twi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbz_l(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbz_l(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_lux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_lux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_lx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_lx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_stux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_stux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_stx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_psq_stx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_abs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_abs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_add(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_add(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_cmpo0(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_cmpo0(ins: &Ins) -> Arguments {
    [
        if ins.field_fra() != 0 {
            Argument::FPR(FPR(ins.field_fra() as _))
        } else {
            Argument::None
        },
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_cmpo1(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_cmpo1(ins: &Ins) -> Arguments {
    [
        if ins.field_fra() != 0 {
            Argument::FPR(FPR(ins.field_fra() as _))
        } else {
            Argument::None
        },
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_cmpu0(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_cmpu0(ins: &Ins) -> Arguments {
    [
        if ins.field_fra() != 0 {
            Argument::FPR(FPR(ins.field_fra() as _))
        } else {
            Argument::None
        },
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_cmpu1(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_cmpu1(ins: &Ins) -> Arguments {
    [
        if ins.field_fra() != 0 {
            Argument::FPR(FPR(ins.field_fra() as _))
        } else {
            Argument::None
        },
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_div(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_div(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_madd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_madd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_madds0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_madds0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_madds1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_madds1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_merge00(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_merge00(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_merge01(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_merge01(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_merge10(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_merge10(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_merge11(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_merge11(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_mr(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_mr(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_msub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_msub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_mul(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_mul(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_muls0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_muls0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_muls1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_muls1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_nabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_nabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_neg(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_neg(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_nmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_nmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_nmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_nmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_res(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_res(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_rsqrte(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_rsqrte(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_sel(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_sel(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_sub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_sub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_sum0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_sum0(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ps_sum1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ps_sum1(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mulli(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mulli(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subfic(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subfic(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cmpli(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cmpli(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cmpi(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cmpi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addic(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addic(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addic_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addic_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addi(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addis(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addis(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_bc(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_bc(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_sc(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_sc(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_b(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_b(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_bcctr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_bcctr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_bclr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_bclr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_crand(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crand(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_crandc(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crandc(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_creqv(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_creqv(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_crnand(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crnand(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_crnor(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crnor(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cror(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cror(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_crorc(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crorc(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_crxor(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_crxor(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crba() as _)),
        Argument::CRBit(CRBit(ins.field_crbb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_isync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_isync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mcrf(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mcrf(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_rfi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_rfi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_rlwimi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_rlwimi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_rlwinm(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_rlwinm(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::OpaqueU(OpaqueU(ins.field_sh() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_rlwnm(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_rlwnm(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ori(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_ori(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_oris(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_oris(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_xori(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_xori(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_xoris(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_xoris(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_andi_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_andi_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_andis_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_andis_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_add(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_add(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_adde(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_adde(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addme(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addme(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_addze(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_addze(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_and(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_and(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_andc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_andc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cmp(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cmp(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cmpl(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cmpl(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_cntlzw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_cntlzw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbf(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbf(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbi(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbst(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbst(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbt(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbt(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbtst(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbtst(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_dcbz(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_dcbz(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_divw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_divw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_divwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_divwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_eciwx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_eciwx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_ecowx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_ecowx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_eieio(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_eieio(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_eqv(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_eqv(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_extsb(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_extsb(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_extsh(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_extsh(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_icbi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_icbi(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lbzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lbzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lbzx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lbzx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfdux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfdux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfdx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfdx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfsux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfsux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfsx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfsx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhaux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhaux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhax(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhax(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhbrx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhbrx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhzx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhzx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lswi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lswi(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lswx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lswx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwarx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwarx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwbrx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwbrx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwzux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwzx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwzx(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mcrxr(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mcrxr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mfcr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mfcr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mfmsr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mfmsr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mfspr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mfspr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mfsr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mfsr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mfsrin(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mfsrin(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mftb(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mftb(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mtcrf(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtcrf(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mtmsr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtmsr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mtspr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtspr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mtsr(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtsr(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mtsrin(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtsrin(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mulhw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mulhw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mulhwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mulhwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mullw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mullw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_nand(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_nand(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_neg(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_neg(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_nor(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_nor(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_or(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_or(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_orc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_orc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_slw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_slw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sraw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_sraw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_srawi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_srawi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_srw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_srw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stbux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stbux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stbx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stbx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfdux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stfdux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfdx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stfdx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfiwx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stfiwx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfsux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stfsux(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfsx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stfsx(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sthbrx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_sthbrx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sthux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_sthux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sthx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_sthx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stswi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stswi(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stswx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stswx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stwbrx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stwbrx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stwcx_(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stwcx_(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stwux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stwux(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stwx(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stwx(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subf(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subf(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subfc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subfc(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subfe(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subfe(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subfme(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subfme(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_subfze(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_subfze(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_sync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_tlbie(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_tlbie(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_tlbsync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_tlbsync(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_tw(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_tw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_xor(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_xor(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_rb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwz(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwz(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lwzu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lwzu(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lbz(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lbz(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lbzu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lbzu(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stw(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stwu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stb(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stb(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stbu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stbu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhz(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhz(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhzu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhzu(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lha(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lha(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lhau(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lhau(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sth(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_sth(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_sthu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_sthu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lmw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lmw(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stmw(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stmw(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_rs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfs(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfsu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfsu(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfd(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_lfdu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_lfdu(ins: &Ins) -> Arguments {
    [
        Argument::Offset(Offset(ins.field_offset() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfs(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stfs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfsu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stfsu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfd(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_stfd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_stfdu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_stfdu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_l(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_l(ins: &Ins) -> Arguments {
    [
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_lu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_lu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fdivs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fdivs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmuls(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmuls(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fnmadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fnmadds(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fnmsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fnmsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fres(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fres(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fsubs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_st(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_psq_st(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        if ins.field_ra() != 0 {
            Argument::GPR(GPR(ins.field_ra() as _))
        } else {
            Argument::None
        },
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_psq_stu(ins: &Ins) -> Arguments {
    [
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_psq_stu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frs() as _)),
        Argument::GPR(GPR(ins.field_ra() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fcmpo(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fcmpo(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fcmpu(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fcmpu(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fctiw(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fctiw(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fctiwz(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fctiwz(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fdiv(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fdiv(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmr(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmr(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fmul(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fmul(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fnabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fnabs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fneg(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fneg(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fnmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fnmadd(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fnmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fnmsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_frsp(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_frsp(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_frsqrte(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_frsqrte(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fsel(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fsel(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frc() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
    ]
}
const fn defs_fsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_fsub(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_fra() as _)),
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mcrfs(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mcrfs(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfs() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mffs(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mffs(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mtfsb0(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mtfsb0(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mtfsb1(ins: &Ins) -> Arguments {
    [
        Argument::CRBit(CRBit(ins.field_crbd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mtfsb1(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_mtfsf(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn uses_mtfsf(ins: &Ins) -> Arguments {
    [
        Argument::FPR(FPR(ins.field_frb() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn defs_mtfsfi(ins: &Ins) -> Arguments {
    [
        Argument::CRField(CRField(ins.field_crfd() as _)),
        Argument::None,
        Argument::None,
        Argument::None,
        Argument::None,
    ]
}
const fn uses_mtfsfi(ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
const fn defs_uses_illegal(_ins: &Ins) -> Arguments {
    [Argument::None, Argument::None, Argument::None, Argument::None, Argument::None]
}
pub type DefsUsesFunction = fn(&Ins) -> Arguments;
pub const DEFS_FUNCTIONS: [DefsUsesFunction; 256] = [
    defs_twi,
    defs_dcbz_l,
    defs_psq_lux,
    defs_psq_lx,
    defs_psq_stux,
    defs_psq_stx,
    defs_ps_abs,
    defs_ps_add,
    defs_ps_cmpo0,
    defs_ps_cmpo1,
    defs_ps_cmpu0,
    defs_ps_cmpu1,
    defs_ps_div,
    defs_ps_madd,
    defs_ps_madds0,
    defs_ps_madds1,
    defs_ps_merge00,
    defs_ps_merge01,
    defs_ps_merge10,
    defs_ps_merge11,
    defs_ps_mr,
    defs_ps_msub,
    defs_ps_mul,
    defs_ps_muls0,
    defs_ps_muls1,
    defs_ps_nabs,
    defs_ps_neg,
    defs_ps_nmadd,
    defs_ps_nmsub,
    defs_ps_res,
    defs_ps_rsqrte,
    defs_ps_sel,
    defs_ps_sub,
    defs_ps_sum0,
    defs_ps_sum1,
    defs_mulli,
    defs_subfic,
    defs_cmpli,
    defs_cmpi,
    defs_addic,
    defs_addic_,
    defs_addi,
    defs_addis,
    defs_bc,
    defs_sc,
    defs_b,
    defs_bcctr,
    defs_bclr,
    defs_crand,
    defs_crandc,
    defs_creqv,
    defs_crnand,
    defs_crnor,
    defs_cror,
    defs_crorc,
    defs_crxor,
    defs_isync,
    defs_mcrf,
    defs_rfi,
    defs_rlwimi,
    defs_rlwinm,
    defs_rlwnm,
    defs_ori,
    defs_oris,
    defs_xori,
    defs_xoris,
    defs_andi_,
    defs_andis_,
    defs_add,
    defs_addc,
    defs_adde,
    defs_addme,
    defs_addze,
    defs_and,
    defs_andc,
    defs_cmp,
    defs_cmpl,
    defs_cntlzw,
    defs_dcbf,
    defs_dcbi,
    defs_dcbst,
    defs_dcbt,
    defs_dcbtst,
    defs_dcbz,
    defs_divw,
    defs_divwu,
    defs_eciwx,
    defs_ecowx,
    defs_eieio,
    defs_eqv,
    defs_extsb,
    defs_extsh,
    defs_icbi,
    defs_lbzux,
    defs_lbzx,
    defs_lfdux,
    defs_lfdx,
    defs_lfsux,
    defs_lfsx,
    defs_lhaux,
    defs_lhax,
    defs_lhbrx,
    defs_lhzux,
    defs_lhzx,
    defs_lswi,
    defs_lswx,
    defs_lwarx,
    defs_lwbrx,
    defs_lwzux,
    defs_lwzx,
    defs_mcrxr,
    defs_mfcr,
    defs_mfmsr,
    defs_mfspr,
    defs_mfsr,
    defs_mfsrin,
    defs_mftb,
    defs_mtcrf,
    defs_mtmsr,
    defs_mtspr,
    defs_mtsr,
    defs_mtsrin,
    defs_mulhw,
    defs_mulhwu,
    defs_mullw,
    defs_nand,
    defs_neg,
    defs_nor,
    defs_or,
    defs_orc,
    defs_slw,
    defs_sraw,
    defs_srawi,
    defs_srw,
    defs_stbux,
    defs_stbx,
    defs_stfdux,
    defs_stfdx,
    defs_stfiwx,
    defs_stfsux,
    defs_stfsx,
    defs_sthbrx,
    defs_sthux,
    defs_sthx,
    defs_stswi,
    defs_stswx,
    defs_stwbrx,
    defs_stwcx_,
    defs_stwux,
    defs_stwx,
    defs_subf,
    defs_subfc,
    defs_subfe,
    defs_subfme,
    defs_subfze,
    defs_sync,
    defs_tlbie,
    defs_tlbsync,
    defs_tw,
    defs_xor,
    defs_lwz,
    defs_lwzu,
    defs_lbz,
    defs_lbzu,
    defs_stw,
    defs_stwu,
    defs_stb,
    defs_stbu,
    defs_lhz,
    defs_lhzu,
    defs_lha,
    defs_lhau,
    defs_sth,
    defs_sthu,
    defs_lmw,
    defs_stmw,
    defs_lfs,
    defs_lfsu,
    defs_lfd,
    defs_lfdu,
    defs_stfs,
    defs_stfsu,
    defs_stfd,
    defs_stfdu,
    defs_psq_l,
    defs_psq_lu,
    defs_fadds,
    defs_fdivs,
    defs_fmadds,
    defs_fmsubs,
    defs_fmuls,
    defs_fnmadds,
    defs_fnmsubs,
    defs_fres,
    defs_fsubs,
    defs_psq_st,
    defs_psq_stu,
    defs_fabs,
    defs_fadd,
    defs_fcmpo,
    defs_fcmpu,
    defs_fctiw,
    defs_fctiwz,
    defs_fdiv,
    defs_fmadd,
    defs_fmr,
    defs_fmsub,
    defs_fmul,
    defs_fnabs,
    defs_fneg,
    defs_fnmadd,
    defs_fnmsub,
    defs_frsp,
    defs_frsqrte,
    defs_fsel,
    defs_fsub,
    defs_mcrfs,
    defs_mffs,
    defs_mtfsb0,
    defs_mtfsb1,
    defs_mtfsf,
    defs_mtfsfi,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
];
pub const USES_FUNCTIONS: [DefsUsesFunction; 256] = [
    uses_twi,
    uses_dcbz_l,
    uses_psq_lux,
    uses_psq_lx,
    uses_psq_stux,
    uses_psq_stx,
    uses_ps_abs,
    uses_ps_add,
    uses_ps_cmpo0,
    uses_ps_cmpo1,
    uses_ps_cmpu0,
    uses_ps_cmpu1,
    uses_ps_div,
    uses_ps_madd,
    uses_ps_madds0,
    uses_ps_madds1,
    uses_ps_merge00,
    uses_ps_merge01,
    uses_ps_merge10,
    uses_ps_merge11,
    uses_ps_mr,
    uses_ps_msub,
    uses_ps_mul,
    uses_ps_muls0,
    uses_ps_muls1,
    uses_ps_nabs,
    uses_ps_neg,
    uses_ps_nmadd,
    uses_ps_nmsub,
    uses_ps_res,
    uses_ps_rsqrte,
    uses_ps_sel,
    uses_ps_sub,
    uses_ps_sum0,
    uses_ps_sum1,
    uses_mulli,
    uses_subfic,
    uses_cmpli,
    uses_cmpi,
    uses_addic,
    uses_addic_,
    uses_addi,
    uses_addis,
    uses_bc,
    uses_sc,
    uses_b,
    uses_bcctr,
    uses_bclr,
    uses_crand,
    uses_crandc,
    uses_creqv,
    uses_crnand,
    uses_crnor,
    uses_cror,
    uses_crorc,
    uses_crxor,
    uses_isync,
    uses_mcrf,
    uses_rfi,
    uses_rlwimi,
    uses_rlwinm,
    uses_rlwnm,
    uses_ori,
    uses_oris,
    uses_xori,
    uses_xoris,
    uses_andi_,
    uses_andis_,
    uses_add,
    uses_addc,
    uses_adde,
    uses_addme,
    uses_addze,
    uses_and,
    uses_andc,
    uses_cmp,
    uses_cmpl,
    uses_cntlzw,
    uses_dcbf,
    uses_dcbi,
    uses_dcbst,
    uses_dcbt,
    uses_dcbtst,
    uses_dcbz,
    uses_divw,
    uses_divwu,
    uses_eciwx,
    uses_ecowx,
    uses_eieio,
    uses_eqv,
    uses_extsb,
    uses_extsh,
    uses_icbi,
    uses_lbzux,
    uses_lbzx,
    uses_lfdux,
    uses_lfdx,
    uses_lfsux,
    uses_lfsx,
    uses_lhaux,
    uses_lhax,
    uses_lhbrx,
    uses_lhzux,
    uses_lhzx,
    uses_lswi,
    uses_lswx,
    uses_lwarx,
    uses_lwbrx,
    uses_lwzux,
    uses_lwzx,
    uses_mcrxr,
    uses_mfcr,
    uses_mfmsr,
    uses_mfspr,
    uses_mfsr,
    uses_mfsrin,
    uses_mftb,
    uses_mtcrf,
    uses_mtmsr,
    uses_mtspr,
    uses_mtsr,
    uses_mtsrin,
    uses_mulhw,
    uses_mulhwu,
    uses_mullw,
    uses_nand,
    uses_neg,
    uses_nor,
    uses_or,
    uses_orc,
    uses_slw,
    uses_sraw,
    uses_srawi,
    uses_srw,
    uses_stbux,
    uses_stbx,
    uses_stfdux,
    uses_stfdx,
    uses_stfiwx,
    uses_stfsux,
    uses_stfsx,
    uses_sthbrx,
    uses_sthux,
    uses_sthx,
    uses_stswi,
    uses_stswx,
    uses_stwbrx,
    uses_stwcx_,
    uses_stwux,
    uses_stwx,
    uses_subf,
    uses_subfc,
    uses_subfe,
    uses_subfme,
    uses_subfze,
    uses_sync,
    uses_tlbie,
    uses_tlbsync,
    uses_tw,
    uses_xor,
    uses_lwz,
    uses_lwzu,
    uses_lbz,
    uses_lbzu,
    uses_stw,
    uses_stwu,
    uses_stb,
    uses_stbu,
    uses_lhz,
    uses_lhzu,
    uses_lha,
    uses_lhau,
    uses_sth,
    uses_sthu,
    uses_lmw,
    uses_stmw,
    uses_lfs,
    uses_lfsu,
    uses_lfd,
    uses_lfdu,
    uses_stfs,
    uses_stfsu,
    uses_stfd,
    uses_stfdu,
    uses_psq_l,
    uses_psq_lu,
    uses_fadds,
    uses_fdivs,
    uses_fmadds,
    uses_fmsubs,
    uses_fmuls,
    uses_fnmadds,
    uses_fnmsubs,
    uses_fres,
    uses_fsubs,
    uses_psq_st,
    uses_psq_stu,
    uses_fabs,
    uses_fadd,
    uses_fcmpo,
    uses_fcmpu,
    uses_fctiw,
    uses_fctiwz,
    uses_fdiv,
    uses_fmadd,
    uses_fmr,
    uses_fmsub,
    uses_fmul,
    uses_fnabs,
    uses_fneg,
    uses_fnmadd,
    uses_fnmsub,
    uses_frsp,
    uses_frsqrte,
    uses_fsel,
    uses_fsub,
    uses_mcrfs,
    uses_mffs,
    uses_mtfsb0,
    uses_mtfsb1,
    uses_mtfsf,
    uses_mtfsfi,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
    defs_uses_illegal,
];
