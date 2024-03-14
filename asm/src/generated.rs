#![allow(unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::types::*;
pub type Arguments = [Argument; 5];
fn gen_add(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000214 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_addc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000014 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_adde(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000114 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_addi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x38000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_addic(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x30000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_addic_(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x34000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_addis(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x3c000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_addme(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0001d4 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_addze(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000194 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_and(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000038 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_andc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000078 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_andi_(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x70000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_andis_(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x74000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_b(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x48000000 | modifiers;
    // LI
    code |= parse_signed(args, 0, -0x2000000, 0x2000000)? as u32 & 0x3fffffc;
    Ok(code)
}
fn gen_bc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x40000000 | modifiers;
    // BO
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // BI
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // BD
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_bcctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x4c000420 | modifiers;
    // BO
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // BI
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_bclr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x4c000020 | modifiers;
    // BO
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // BI
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_cmp(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x7c000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // L
    code |= (parse_unsigned(args, 1, 0x0, 0x1)? & 0x1) << 21;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_cmpi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x2c000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // L
    code |= (parse_unsigned(args, 1, 0x0, 0x1)? & 0x1) << 21;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 3, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_cmpl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x7c000040 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // L
    code |= (parse_unsigned(args, 1, 0x0, 0x1)? & 0x1) << 21;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_cmpli(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x28000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // L
    code |= (parse_unsigned(args, 1, 0x0, 0x1)? & 0x1) << 21;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // uimm
    code |= parse_unsigned(args, 3, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_cntlzw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000034 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_crand(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000202 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crandc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000102 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_creqv(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000242 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crnand(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c0001c2 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crnor(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000042 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_cror(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000382 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crorc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000342 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crxor(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x4c000182 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0000ac | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0003ac | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbst(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c00006c | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbt(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c00022c | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbtst(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0001ec | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0007ec | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_dcbz_l(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x100007ec | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_divw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0003d6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_divwu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000396 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_eciwx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00026c | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ecowx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00036c | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_eieio(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x7c0006ac | modifiers;
    Ok(code)
}
fn gen_eqv(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000238 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_extsb(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000774 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_extsh(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000734 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_fabs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000210 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fadd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc00002a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fadds(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xec00002a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fcmpo(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc000040 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fcmpu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fctiw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc00001c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fctiwz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc00001e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fdiv(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc000024 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fdivs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xec000024 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmadd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xfc00003a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmadds(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xec00003a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000090 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmsub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xfc000038 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmsubs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xec000038 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fmul(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc000032 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_fmuls(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xec000032 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_fnabs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000110 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fneg(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000050 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fnmadd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xfc00003e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fnmadds(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xec00003e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fnmsub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xfc00003c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fnmsubs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xec00003c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fres(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xec000030 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_frsp(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000018 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_frsqrte(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000034 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fsel(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0xfc00002e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fsub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xfc000028 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_fsubs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xec000028 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_icbi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0007ac | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_isync(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4c00012c | modifiers;
    Ok(code)
}
fn gen_lbz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x88000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lbzu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x8c000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lbzux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0000ee | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lbzx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0000ae | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lfd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xc8000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lfdu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xcc000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lfdux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0004ee | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lfdx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0004ae | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lfs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xc0000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lfsu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xc4000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lfsux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00046e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lfsx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00042e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lha(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xa8000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lhau(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xac000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lhaux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0002ee | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lhax(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0002ae | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lhbrx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00062c | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lhz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xa0000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lhzu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xa4000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lhzux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00026e | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lhzx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00022e | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lmw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xb8000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lswi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0004aa | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // NB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lswx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00042a | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lwarx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000028 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lwbrx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00042c | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lwz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x80000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lwzu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x84000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_lwzux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00006e | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lwzx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00002e | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mcrf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x4c000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // crfS
    code |= (parse_unsigned(args, 1, 0x0, 0x7)? & 0x7) << 18;
    Ok(code)
}
fn gen_mcrfs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc000080 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // crfS
    code |= (parse_unsigned(args, 1, 0x0, 0x7)? & 0x7) << 18;
    Ok(code)
}
fn gen_mcrxr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c000400 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    Ok(code)
}
fn gen_mfcr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c000026 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mffs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0xfc00048e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfmsr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0000a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfspr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0002a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr
    code
        |= {
            let mut value = parse_unsigned(args, 1, 0x0, 0x3ff)?;
            value = ((value & 0b11111_00000) >> 5) | ((value & 0b00000_11111) << 5);
            (value & 0x3ff) << 11
        };
    Ok(code)
}
fn gen_mfsr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0004a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // sr
    code |= (parse_unsigned(args, 1, 0x0, 0xf)? & 0xf) << 16;
    Ok(code)
}
fn gen_mfsrin(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000526 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mftb(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0002e6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // tbr
    code
        |= {
            let mut value = parse_unsigned(args, 1, 0x0, 0x3ff)?;
            value = ((value & 0b11111_00000) >> 5) | ((value & 0b00000_11111) << 5);
            (value & 0x3ff) << 11
        };
    Ok(code)
}
fn gen_mtcrf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000120 | modifiers;
    // crm
    code |= (parse_unsigned(args, 0, 0x0, 0xff)? & 0xff) << 12;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtfsb0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0xfc00008c | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtfsb1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0xfc00004c | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtfsf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc00058e | modifiers;
    // mtfsf_FM
    code |= (parse_unsigned(args, 0, 0x0, 0xff)? & 0xff) << 17;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mtfsfi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfc00010c | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // mtfsf_IMM
    code |= (parse_unsigned(args, 1, 0x0, 0xf)? & 0xf) << 12;
    Ok(code)
}
fn gen_mtmsr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c000124 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtspr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0003a6 | modifiers;
    // spr
    code
        |= {
            let mut value = parse_unsigned(args, 0, 0x0, 0x3ff)?;
            value = ((value & 0b11111_00000) >> 5) | ((value & 0b00000_11111) << 5);
            (value & 0x3ff) << 11
        };
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtsr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0001a4 | modifiers;
    // sr
    code |= (parse_unsigned(args, 0, 0x0, 0xf)? & 0xf) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtsrin(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0001e4 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mulhw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000096 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mulhwu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000016 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mulli(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x1c000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_mullw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0001d6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_nand(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0003b8 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_neg(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0000d0 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_nor(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0000f8 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_or(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000378 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_orc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000338 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ori(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x60000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_oris(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x64000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_psq_l(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0xe0000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // ps_offset
    code |= parse_signed(args, 1, -0x800, 0x800)? as u32 & 0xfff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // ps_W
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 15;
    // ps_I
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 12;
    Ok(code)
}
fn gen_psq_lu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0xe4000000 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // ps_offset
    code |= parse_signed(args, 1, -0x800, 0x800)? as u32 & 0xfff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // ps_W
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 15;
    // ps_I
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 12;
    Ok(code)
}
fn gen_psq_lux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x1000004c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // ps_WX
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 10;
    // ps_IX
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 7;
    Ok(code)
}
fn gen_psq_lx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x1000000c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // ps_WX
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 10;
    // ps_IX
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 7;
    Ok(code)
}
fn gen_psq_st(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0xf0000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // ps_offset
    code |= parse_signed(args, 1, -0x800, 0x800)? as u32 & 0xfff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // ps_W
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 15;
    // ps_I
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 12;
    Ok(code)
}
fn gen_psq_stu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0xf4000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // ps_offset
    code |= parse_signed(args, 1, -0x800, 0x800)? as u32 & 0xfff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    // ps_W
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 15;
    // ps_I
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 12;
    Ok(code)
}
fn gen_psq_stux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x1000004e | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // ps_WX
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 10;
    // ps_IX
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 7;
    Ok(code)
}
fn gen_psq_stx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x1000000e | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // ps_WX
    code |= (parse_unsigned(args, 3, 0x0, 0x1)? & 0x1) << 10;
    // ps_IX
    code |= (parse_unsigned(args, 4, 0x0, 0x7)? & 0x7) << 7;
    Ok(code)
}
fn gen_ps_abs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000210 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_add(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x1000002a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_cmpo0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000040 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_cmpo1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x100000c0 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_cmpu0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000000 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_cmpu1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000080 | modifiers;
    // crfD
    code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_div(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000024 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_madd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000003a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_madds0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000001c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_madds1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000001e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_merge00(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000420 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_merge01(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000460 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_merge10(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x100004a0 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_merge11(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x100004e0 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_mr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000090 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_msub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x10000038 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_mul(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000032 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_ps_muls0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000018 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_ps_muls1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x1000001a | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_ps_nabs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000110 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_neg(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000050 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_nmadd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000003e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_nmsub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000003c | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_res(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000030 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_rsqrte(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x10000034 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_sel(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x1000002e | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_sub(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x10000028 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_sum0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x10000014 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_ps_sum1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x10000016 | modifiers;
    // frD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // frA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // frC
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // frB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_rfi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4c000064 | modifiers;
    Ok(code)
}
fn gen_rlwimi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x50000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // MB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 6;
    // ME
    code |= (parse_unsigned(args, 4, 0x0, 0x1f)? & 0x1f) << 1;
    Ok(code)
}
fn gen_rlwinm(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x54000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // MB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 6;
    // ME
    code |= (parse_unsigned(args, 4, 0x0, 0x1f)? & 0x1f) << 1;
    Ok(code)
}
fn gen_rlwnm(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 5)?;
    let mut code = 0x5c000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // MB
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 6;
    // ME
    code |= (parse_unsigned(args, 4, 0x0, 0x1f)? & 0x1f) << 1;
    Ok(code)
}
fn gen_sc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x44000002 | modifiers;
    Ok(code)
}
fn gen_slw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000030 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_sraw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000630 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_srawi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000670 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_srw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000430 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stb(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x98000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stbu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x9c000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stbux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0001ee | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stbx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0001ae | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stfd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xd8000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stfdu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xdc000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stfdux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0005ee | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stfdx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0005ae | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stfiwx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0007ae | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stfs(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xd0000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stfsu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xd4000000 | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stfsux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00056e | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stfsx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00052e | modifiers;
    // frS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_sth(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xb0000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_sthbrx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00072c | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_sthu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xb4000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_sthux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00036e | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_sthx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00032e | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stmw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xbc000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stswi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c0005aa | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // NB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stswx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00052a | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x90000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stwbrx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00052c | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stwcx_(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00012d | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stwu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x94000000 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // offset
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    // rA
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_stwux(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00016e | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_stwx(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c00012e | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_subf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000050 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_subfc(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000010 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_subfe(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000110 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_subfic(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x20000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_subfme(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c0001d0 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_subfze(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000190 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_sync(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x7c0004ac | modifiers;
    Ok(code)
}
fn gen_tlbie(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c000264 | modifiers;
    // rB
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_tlbsync(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x7c00046c | modifiers;
    Ok(code)
}
fn gen_tw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000008 | modifiers;
    // TO
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_twi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0xc000000 | modifiers;
    // TO
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_xor(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x7c000278 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_xori(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x68000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_xoris(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x6c000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_bdnzlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4e000020 | modifiers;
    Ok(code)
}
fn gen_mfxer(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0102a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfsdr1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1902a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfdbatu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1882a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr_BAT
    code |= (parse_unsigned(args, 1, 0x0, 0x3)? & 0x3) << 17;
    Ok(code)
}
fn gen_mfsrr0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1a02a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_bltlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d800020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d800020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_mtear(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1a43a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_extrwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x5400003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // MB
    code |= ((32 - parse_unsigned(args, 2, 0x0, 0x1f)?) & 0x1f) << 6;
    // SH
    code
        |= ((parse_unsigned(args, 3, 0x0, 0x1f)? + parse_unsigned(args, 2, 0x0, 0x1f)?)
            & 0x1f) << 11;
    Ok(code)
}
fn gen_mtdar(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1303a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_rotlw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x5c00003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mtsrr1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1b03a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfsrr1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1b02a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_blectr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c810420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c810420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_bnslr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c830020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c830020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_crnot(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x4c000042 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mfdsisr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1202a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_slwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x54000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    // ME
    code |= ((31 - parse_unsigned(args, 2, 0x0, 0x1f)?) & 0x1f) << 1;
    Ok(code)
}
fn gen_twlge(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7ca00008 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mttbu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1d43a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfibatl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1182a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr_BAT
    code |= (parse_unsigned(args, 1, 0x0, 0x3)? & 0x3) << 17;
    Ok(code)
}
fn gen_beqctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d820420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d820420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_bgtlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d810020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d810020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_cmplw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x7c000040 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        3 => {
            let mut code = 0x7c000040 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_mtlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0803a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtsdr1(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1903a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c000378 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_twgti(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xd000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_bge(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x40800000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x40800000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_bltctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d800420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d800420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_mtibatu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1083a6 | modifiers;
    // spr_BAT
    code |= (parse_unsigned(args, 0, 0x0, 0x3)? & 0x3) << 17;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_subi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x38000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= (-parse_signed(args, 2, -0x8000, 0x8000)?) as u32 & 0xffff;
    Ok(code)
}
fn gen_nop(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x60000000 | modifiers;
    Ok(code)
}
fn gen_mttbl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1c43a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_blelr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c810020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c810020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_crmove(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x4c000382 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mfdbatl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1982a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr_BAT
    code |= (parse_unsigned(args, 1, 0x0, 0x3)? & 0x3) << 17;
    Ok(code)
}
fn gen_mfibatu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1082a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr_BAT
    code |= (parse_unsigned(args, 1, 0x0, 0x3)? & 0x3) << 17;
    Ok(code)
}
fn gen_cmpd(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x7c200000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        3 => {
            let mut code = 0x7c200000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_clrlwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x5400003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // MB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    Ok(code)
}
fn gen_cmpw(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x7c000000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        3 => {
            let mut code = 0x7c000000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_bdzf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x40400000 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // BD
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_subic(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x30000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= (-parse_signed(args, 2, -0x8000, 0x8000)?) as u32 & 0xffff;
    Ok(code)
}
fn gen_mtdsisr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1203a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_beq(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x41820000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x41820000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_mflr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0802a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_clrrwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x54000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // ME
    code |= ((31 - parse_unsigned(args, 2, 0x0, 0x1f)?) & 0x1f) << 1;
    Ok(code)
}
fn gen_bns(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x40830000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x40830000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_mfear(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1a42a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_bdz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x42400000 | modifiers;
    // BD
    code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_subis(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x3c000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= (-parse_signed(args, 2, -0x8000, 0x8000)?) as u32 & 0xffff;
    Ok(code)
}
fn gen_bctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4e800420 | modifiers;
    Ok(code)
}
fn gen_bnsctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c830420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c830420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_mtdbatl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1983a6 | modifiers;
    // spr_BAT
    code |= (parse_unsigned(args, 0, 0x0, 0x3)? & 0x3) << 17;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mtsprg(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1043a6 | modifiers;
    // spr_SPRG
    code |= (parse_unsigned(args, 0, 0x0, 0x3)? & 0x3) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0902a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_clrlslwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x54000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // MB
    code
        |= ((parse_unsigned(args, 2, 0x0, 0x1f)? - parse_unsigned(args, 3, 0x0, 0x1f)?)
            & 0x1f) << 6;
    // SH
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    // ME
    code |= ((31 - parse_unsigned(args, 3, 0x0, 0x1f)?) & 0x1f) << 1;
    Ok(code)
}
fn gen_bgt(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x41810000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x41810000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_cmplwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x28000000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // uimm
            code |= parse_unsigned(args, 1, 0x0, 0xffff)? & 0xffff;
            Ok(code)
        }
        3 => {
            let mut code = 0x28000000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // uimm
            code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_bne(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x40820000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x40820000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_cmpldi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x28200000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // uimm
            code |= parse_unsigned(args, 1, 0x0, 0xffff)? & 0xffff;
            Ok(code)
        }
        3 => {
            let mut code = 0x28200000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // uimm
            code |= parse_unsigned(args, 2, 0x0, 0xffff)? & 0xffff;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_mtibatl(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1183a6 | modifiers;
    // spr_BAT
    code |= (parse_unsigned(args, 0, 0x0, 0x3)? & 0x3) << 17;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_blt(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x41800000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x41800000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_bdzt(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x41400000 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // BD
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_blr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4e800020 | modifiers;
    Ok(code)
}
fn gen_beqlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d820020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d820020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_twui(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xfe00000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_bdnzt(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x41000000 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // BD
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_mtdbatu(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1883a6 | modifiers;
    // spr_BAT
    code |= (parse_unsigned(args, 0, 0x0, 0x3)? & 0x3) << 17;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_cmpld(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x7c200040 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        3 => {
            let mut code = 0x7c200040 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // rB
            code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_srwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x5400003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // MB
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 6;
    // SH
    code |= ((32 - parse_unsigned(args, 2, 0x0, 0x1f)?) & 0x1f) << 11;
    Ok(code)
}
fn gen_cmpdi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x2c200000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // simm
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
            Ok(code)
        }
        3 => {
            let mut code = 0x2c200000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // simm
            code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_rotrwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x5400003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= ((32 - parse_unsigned(args, 2, 0x0, 0x1f)?) & 0x1f) << 11;
    Ok(code)
}
fn gen_bgectr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c800420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c800420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_bdnz(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x42000000 | modifiers;
    // BD
    code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_bsoctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d830420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d830420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_subic_(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x34000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // rA
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= (-parse_signed(args, 2, -0x8000, 0x8000)?) as u32 & 0xffff;
    Ok(code)
}
fn gen_bnelr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c820020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c820020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_bdzlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x4e400020 | modifiers;
    Ok(code)
}
fn gen_bdzflr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4c000020 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_rotlwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 3)?;
    let mut code = 0x5400003e | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // SH
    code |= (parse_unsigned(args, 2, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_lis(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x3c000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // uimm
    code |= parse_unsigned(args, 1, 0x0, 0xffff)? & 0xffff;
    Ok(code)
}
fn gen_ble(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x40810000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x40810000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_bso(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        1 => {
            let mut code = 0x41830000 | modifiers;
            // BD
            code |= parse_signed(args, 0, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        2 => {
            let mut code = 0x41830000 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            // BD
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 2,
            })
        }
    }
}
fn gen_bdnzf(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x40000000 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // BD
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xfffc;
    Ok(code)
}
fn gen_cmpwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        2 => {
            let mut code = 0x2c000000 | modifiers;
            // rA
            code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
            // simm
            code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
            Ok(code)
        }
        3 => {
            let mut code = 0x2c000000 | modifiers;
            // crfD
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 23;
            // rA
            code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 16;
            // simm
            code |= parse_signed(args, 2, -0x8000, 0x8000)? as u32 & 0xffff;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 3,
            })
        }
    }
}
fn gen_bdztlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4d400020 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_mtxer(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0103a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfsprg(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c1042a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // spr_SPRG
    code |= (parse_unsigned(args, 1, 0x0, 0x3)? & 0x3) << 16;
    Ok(code)
}
fn gen_bgtctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d810420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d810420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_bsolr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4d830020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4d830020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_tweq(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x7c800008 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rB
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_crclr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4c000182 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mtdec(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1603a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_crset(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4c000242 | modifiers;
    // crbD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // crbA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // crbB
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_mfdar(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1302a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_mfdec(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1602a6 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_bnectr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c820420 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c820420 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_mtctr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c0903a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_extlwi(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 4)?;
    let mut code = 0x54000000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // rS
    code |= (parse_unsigned(args, 1, 0x0, 0x1f)? & 0x1f) << 21;
    // ME
    code |= ((parse_unsigned(args, 2, 0x0, 0x1f)? - 1) & 0x1f) << 1;
    // SH
    code |= (parse_unsigned(args, 3, 0x0, 0x1f)? & 0x1f) << 11;
    Ok(code)
}
fn gen_li(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0x38000000 | modifiers;
    // rD
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    // simm
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_mtsrr0(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x7c1a03a6 | modifiers;
    // rS
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 21;
    Ok(code)
}
fn gen_bdnztlr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4d000020 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
fn gen_trap(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 0)?;
    let mut code = 0x7fe00008 | modifiers;
    Ok(code)
}
fn gen_bgelr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    match arg_count(args) {
        0 => {
            let mut code = 0x4c800020 | modifiers;
            Ok(code)
        }
        1 => {
            let mut code = 0x4c800020 | modifiers;
            // crfS
            code |= (parse_unsigned(args, 0, 0x0, 0x7)? & 0x7) << 18;
            Ok(code)
        }
        value => {
            Err(ArgumentError::ArgCount {
                value,
                expected: 1,
            })
        }
    }
}
fn gen_twllei(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 2)?;
    let mut code = 0xcc00000 | modifiers;
    // rA
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    // simm
    code |= parse_signed(args, 1, -0x8000, 0x8000)? as u32 & 0xffff;
    Ok(code)
}
fn gen_bdnzflr(args: &Arguments, modifiers: u32) -> Result<u32, ArgumentError> {
    check_arg_count(args, 1)?;
    let mut code = 0x4c000020 | modifiers;
    // BI
    code |= (parse_unsigned(args, 0, 0x0, 0x1f)? & 0x1f) << 16;
    Ok(code)
}
type MnemonicFn = fn(&Arguments, u32) -> Result<u32, ArgumentError>;
const MNEMONIC_MAP: phf::Map<&'static str, (MnemonicFn, u32)> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 8),
        (0, 241),
        (0, 25),
        (0, 34),
        (0, 0),
        (0, 9),
        (0, 36),
        (0, 0),
        (1, 466),
        (0, 36),
        (0, 137),
        (0, 27),
        (0, 39),
        (0, 1),
        (0, 115),
        (0, 44),
        (0, 54),
        (0, 242),
        (0, 172),
        (1, 477),
        (2, 186),
        (0, 217),
        (0, 81),
        (0, 293),
        (0, 548),
        (0, 0),
        (0, 4),
        (19, 476),
        (0, 4),
        (0, 0),
        (0, 0),
        (0, 610),
        (0, 20),
        (1, 100),
        (0, 66),
        (0, 10),
        (0, 0),
        (0, 0),
        (0, 22),
        (0, 35),
        (1, 329),
        (0, 28),
        (2, 141),
        (0, 0),
        (0, 0),
        (0, 23),
        (1, 34),
        (0, 23),
        (0, 1),
        (0, 35),
        (1, 169),
        (0, 23),
        (0, 3),
        (0, 6),
        (0, 34),
        (0, 0),
        (2, 465),
        (0, 53),
        (0, 502),
        (1, 22),
        (5, 67),
        (0, 335),
        (7, 423),
        (0, 2),
        (0, 6),
        (0, 0),
        (0, 24),
        (0, 43),
        (0, 42),
        (3, 566),
        (0, 298),
        (1, 36),
        (1, 262),
        (0, 130),
        (0, 80),
        (0, 29),
        (0, 0),
        (0, 45),
        (0, 14),
        (0, 3),
        (0, 5),
        (0, 31),
        (3, 165),
        (8, 649),
        (0, 95),
        (0, 426),
        (0, 121),
        (0, 59),
        (0, 27),
        (0, 0),
        (0, 16),
        (2, 105),
        (0, 441),
        (2, 155),
        (1, 362),
        (0, 352),
        (3, 181),
        (33, 126),
        (5, 693),
        (0, 16),
        (2, 356),
        (4, 98),
        (5, 227),
        (0, 12),
        (48, 629),
        (0, 62),
        (1, 455),
        (0, 4),
        (0, 0),
        (6, 587),
        (0, 121),
        (1, 8),
        (3, 590),
        (0, 376),
        (0, 0),
        (0, 233),
        (30, 339),
        (0, 95),
        (0, 2),
        (0, 3),
        (5, 286),
        (0, 172),
        (0, 23),
        (3, 114),
        (2, 644),
        (0, 262),
        (0, 30),
        (0, 290),
        (19, 621),
        (0, 517),
        (17, 110),
        (0, 18),
        (1, 390),
        (24, 77),
        (0, 1),
        (0, 0),
        (0, 1),
        (0, 3),
        (0, 155),
        (0, 40),
    ],
    entries: &[
        ("mfdbatu", (gen_mfdbatu, 0x0)),
        ("bdnztlr", (gen_bdnztlr, 0x0)),
        ("ps_mr", (gen_ps_mr, 0x0)),
        ("rotlwi", (gen_rotlwi, 0x0)),
        ("addc.", (gen_addc, 0x1)),
        ("fmr", (gen_fmr, 0x0)),
        ("mtdec", (gen_mtdec, 0x0)),
        ("fmadds.", (gen_fmadds, 0x1)),
        ("beq+", (gen_beq, 0x200000)),
        ("bgt+", (gen_bgt, 0x200000)),
        ("li", (gen_li, 0x0)),
        ("tlbie", (gen_tlbie, 0x0)),
        ("bsol-", (gen_bso, 0x200001)),
        ("lfsux", (gen_lfsux, 0x0)),
        ("crset", (gen_crset, 0x0)),
        ("icbi", (gen_icbi, 0x0)),
        ("bgtctrl+", (gen_bgtctr, 0x200001)),
        ("ps_msub.", (gen_ps_msub, 0x1)),
        ("bgtlr", (gen_bgtlr, 0x0)),
        ("fmsub.", (gen_fmsub, 0x1)),
        ("crxor", (gen_crxor, 0x0)),
        ("blt-", (gen_blt, 0x200000)),
        ("frsqrte.", (gen_frsqrte, 0x1)),
        ("fctiwz", (gen_fctiwz, 0x0)),
        ("fctiwz.", (gen_fctiwz, 0x1)),
        ("bsol", (gen_bso, 0x1)),
        ("bdzl-", (gen_bdz, 0x200001)),
        ("ps_sub.", (gen_ps_sub, 0x1)),
        ("fnmadds", (gen_fnmadds, 0x0)),
        ("neg", (gen_neg, 0x0)),
        ("bnela+", (gen_bne, 0x200003)),
        ("sth", (gen_sth, 0x0)),
        ("creqv", (gen_creqv, 0x0)),
        ("mtfsf", (gen_mtfsf, 0x0)),
        ("mullw.", (gen_mullw, 0x1)),
        ("mfdbatl", (gen_mfdbatl, 0x0)),
        ("stwu", (gen_stwu, 0x0)),
        ("dcbz_l", (gen_dcbz_l, 0x0)),
        ("beqla-", (gen_beq, 0x200003)),
        ("ps_cmpu0", (gen_ps_cmpu0, 0x0)),
        ("bnel+", (gen_bne, 0x200001)),
        ("and", (gen_and, 0x0)),
        ("slw.", (gen_slw, 0x1)),
        ("bdztla-", (gen_bdzt, 0x200003)),
        ("bgtl", (gen_bgt, 0x1)),
        ("cror", (gen_cror, 0x0)),
        ("beqa+", (gen_beq, 0x200002)),
        ("mfcr", (gen_mfcr, 0x0)),
        ("mcrxr", (gen_mcrxr, 0x0)),
        ("eqv.", (gen_eqv, 0x1)),
        ("bc", (gen_bc, 0x0)),
        ("bdnztlrl", (gen_bdnztlr, 0x1)),
        ("ps_nabs.", (gen_ps_nabs, 0x1)),
        ("ps_merge01.", (gen_ps_merge01, 0x1)),
        ("tw", (gen_tw, 0x0)),
        ("subfme", (gen_subfme, 0x0)),
        ("fneg.", (gen_fneg, 0x1)),
        ("fneg", (gen_fneg, 0x0)),
        ("bsolr+", (gen_bsolr, 0x200000)),
        ("subf.", (gen_subf, 0x1)),
        ("bdnzfla-", (gen_bdnzf, 0x200003)),
        ("mtctr", (gen_mtctr, 0x0)),
        ("ble", (gen_ble, 0x0)),
        ("extlwi", (gen_extlwi, 0x0)),
        ("bgtctr+", (gen_bgtctr, 0x200000)),
        ("cmpdi", (gen_cmpdi, 0x0)),
        ("andis.", (gen_andis_, 0x0)),
        ("bdnzfla+", (gen_bdnzf, 0x200003)),
        ("bdnza-", (gen_bdnz, 0x200002)),
        ("ps_sum0.", (gen_ps_sum0, 0x1)),
        ("fcmpu", (gen_fcmpu, 0x0)),
        ("subfo.", (gen_subf, 0x401)),
        ("andc.", (gen_andc, 0x1)),
        ("subic.", (gen_subic_, 0x0)),
        ("frsqrte", (gen_frsqrte, 0x0)),
        ("trap", (gen_trap, 0x0)),
        ("bnsa-", (gen_bns, 0x200002)),
        ("crclr", (gen_crclr, 0x0)),
        ("bdzlr", (gen_bdzlr, 0x0)),
        ("bcla-", (gen_bc, 0x200003)),
        ("mtibatu", (gen_mtibatu, 0x0)),
        ("extsh.", (gen_extsh, 0x1)),
        ("blrl", (gen_blr, 0x1)),
        ("mulhwu", (gen_mulhwu, 0x0)),
        ("bclr+", (gen_bclr, 0x200000)),
        ("cmpl", (gen_cmpl, 0x0)),
        ("bgectrl+", (gen_bgectr, 0x200001)),
        ("blela", (gen_ble, 0x3)),
        ("lfdu", (gen_lfdu, 0x0)),
        ("divwo.", (gen_divw, 0x401)),
        ("bdnzflrl+", (gen_bdnzflr, 0x200001)),
        ("addeo.", (gen_adde, 0x401)),
        ("blelr", (gen_blelr, 0x0)),
        ("ps_madd", (gen_ps_madd, 0x0)),
        ("lwzu", (gen_lwzu, 0x0)),
        ("bltl+", (gen_blt, 0x200001)),
        ("clrlwi", (gen_clrlwi, 0x0)),
        ("mtlr", (gen_mtlr, 0x0)),
        ("lfd", (gen_lfd, 0x0)),
        ("mtfsb0", (gen_mtfsb0, 0x0)),
        ("bdz", (gen_bdz, 0x0)),
        ("bcctr", (gen_bcctr, 0x0)),
        ("sthu", (gen_sthu, 0x0)),
        ("mtsprg", (gen_mtsprg, 0x0)),
        ("subfze", (gen_subfze, 0x0)),
        ("bdzflr+", (gen_bdzflr, 0x200000)),
        ("psq_stx", (gen_psq_stx, 0x0)),
        ("lbz", (gen_lbz, 0x0)),
        ("fctiw.", (gen_fctiw, 0x1)),
        ("bdz+", (gen_bdz, 0x200000)),
        ("bdz-", (gen_bdz, 0x200000)),
        ("mfdec", (gen_mfdec, 0x0)),
        ("bso-", (gen_bso, 0x200000)),
        ("addis", (gen_addis, 0x0)),
        ("bdnza+", (gen_bdnz, 0x200002)),
        ("ps_cmpo0", (gen_ps_cmpo0, 0x0)),
        ("bdzfl", (gen_bdzf, 0x1)),
        ("bnsl-", (gen_bns, 0x200001)),
        ("fnmsub.", (gen_fnmsub, 0x1)),
        ("blelr+", (gen_blelr, 0x200000)),
        ("bdzta-", (gen_bdzt, 0x200002)),
        ("bgta", (gen_bgt, 0x2)),
        ("lswi", (gen_lswi, 0x0)),
        ("bltlrl", (gen_bltlr, 0x1)),
        ("mfxer", (gen_mfxer, 0x0)),
        ("bdnzfa-", (gen_bdnzf, 0x200002)),
        ("srw", (gen_srw, 0x0)),
        ("bdnzt", (gen_bdnzt, 0x0)),
        ("bge-", (gen_bge, 0x200000)),
        ("mtear", (gen_mtear, 0x0)),
        ("slwi", (gen_slwi, 0x0)),
        ("fres.", (gen_fres, 0x1)),
        ("crnand", (gen_crnand, 0x0)),
        ("fsub", (gen_fsub, 0x0)),
        ("bdzfl+", (gen_bdzf, 0x200001)),
        ("beqlr", (gen_beqlr, 0x0)),
        ("bsoctrl", (gen_bsoctr, 0x1)),
        ("bltctrl+", (gen_bltctr, 0x200001)),
        ("mtfsf.", (gen_mtfsf, 0x1)),
        ("blela-", (gen_ble, 0x200003)),
        ("twui", (gen_twui, 0x0)),
        ("bsolrl", (gen_bsolr, 0x1)),
        ("stb", (gen_stb, 0x0)),
        ("bso+", (gen_bso, 0x200000)),
        ("bgtl-", (gen_bgt, 0x200001)),
        ("sraw", (gen_sraw, 0x0)),
        ("beqlrl", (gen_beqlr, 0x1)),
        ("fabs.", (gen_fabs, 0x1)),
        ("bsola-", (gen_bso, 0x200003)),
        ("stfdu", (gen_stfdu, 0x0)),
        ("bdnzfa+", (gen_bdnzf, 0x200002)),
        ("cntlzw", (gen_cntlzw, 0x0)),
        ("stwux", (gen_stwux, 0x0)),
        ("xoris", (gen_xoris, 0x0)),
        ("fnmadd.", (gen_fnmadd, 0x1)),
        ("bsoa+", (gen_bso, 0x200002)),
        ("addi", (gen_addi, 0x0)),
        ("fnmadd", (gen_fnmadd, 0x0)),
        ("subfmeo.", (gen_subfme, 0x401)),
        ("bsolr", (gen_bsolr, 0x0)),
        ("bdnzflr+", (gen_bdnzflr, 0x200000)),
        ("fadd.", (gen_fadd, 0x1)),
        ("bsoctr", (gen_bsoctr, 0x0)),
        ("blta+", (gen_blt, 0x200002)),
        ("ps_sub", (gen_ps_sub, 0x0)),
        ("subfco.", (gen_subfc, 0x401)),
        ("sc", (gen_sc, 0x0)),
        ("sthbrx", (gen_sthbrx, 0x0)),
        ("ps_rsqrte", (gen_ps_rsqrte, 0x0)),
        ("bdztlrl+", (gen_bdztlr, 0x200001)),
        ("beqa-", (gen_beq, 0x200002)),
        ("ps_mul", (gen_ps_mul, 0x0)),
        ("bdzlrl+", (gen_bdzlr, 0x200001)),
        ("cmpw", (gen_cmpw, 0x0)),
        ("mtfsb1.", (gen_mtfsb1, 0x1)),
        ("bgtlr+", (gen_bgtlr, 0x200000)),
        ("blela+", (gen_ble, 0x200003)),
        ("addmeo", (gen_addme, 0x400)),
        ("fmr.", (gen_fmr, 0x1)),
        ("blectr", (gen_blectr, 0x0)),
        ("cmp", (gen_cmp, 0x0)),
        ("bdzta", (gen_bdzt, 0x2)),
        ("bdnztlr+", (gen_bdnztlr, 0x200000)),
        ("divwu.", (gen_divwu, 0x1)),
        ("blea-", (gen_ble, 0x200002)),
        ("stswx", (gen_stswx, 0x0)),
        ("addc", (gen_addc, 0x0)),
        ("mulhw.", (gen_mulhw, 0x1)),
        ("subic", (gen_subic, 0x0)),
        ("clrlslwi", (gen_clrlslwi, 0x0)),
        ("bgelrl", (gen_bgelr, 0x1)),
        ("add.", (gen_add, 0x1)),
        ("subfzeo", (gen_subfze, 0x400)),
        ("bltlr+", (gen_bltlr, 0x200000)),
        ("extsh", (gen_extsh, 0x0)),
        ("mfdar", (gen_mfdar, 0x0)),
        ("mtdbatu", (gen_mtdbatu, 0x0)),
        ("mtibatl", (gen_mtibatl, 0x0)),
        ("fnabs", (gen_fnabs, 0x0)),
        ("extlwi.", (gen_extlwi, 0x1)),
        ("stswi", (gen_stswi, 0x0)),
        ("bltlrl+", (gen_bltlr, 0x200001)),
        ("lhax", (gen_lhax, 0x0)),
        ("ps_merge00.", (gen_ps_merge00, 0x1)),
        ("ps_madds0", (gen_ps_madds0, 0x0)),
        ("extrwi", (gen_extrwi, 0x0)),
        ("bsoctrl+", (gen_bsoctr, 0x200001)),
        ("eieio", (gen_eieio, 0x0)),
        ("mfibatl", (gen_mfibatl, 0x0)),
        ("bltl", (gen_blt, 0x1)),
        ("blt", (gen_blt, 0x0)),
        ("mulhw", (gen_mulhw, 0x0)),
        ("blta-", (gen_blt, 0x200002)),
        ("rfi", (gen_rfi, 0x0)),
        ("bdnzfa", (gen_bdnzf, 0x2)),
        ("ps_neg", (gen_ps_neg, 0x0)),
        ("lfsu", (gen_lfsu, 0x0)),
        ("bgectr", (gen_bgectr, 0x0)),
        ("ps_rsqrte.", (gen_ps_rsqrte, 0x1)),
        ("mfsr", (gen_mfsr, 0x0)),
        ("ori", (gen_ori, 0x0)),
        ("bdnzf+", (gen_bdnzf, 0x200000)),
        ("bca+", (gen_bc, 0x200002)),
        ("psq_stux", (gen_psq_stux, 0x0)),
        ("ps_res", (gen_ps_res, 0x0)),
        ("bgtla+", (gen_bgt, 0x200003)),
        ("subfmeo", (gen_subfme, 0x400)),
        ("bnsa+", (gen_bns, 0x200002)),
        ("bgelrl+", (gen_bgelr, 0x200001)),
        ("bcctrl+", (gen_bcctr, 0x200001)),
        ("fnmsubs", (gen_fnmsubs, 0x0)),
        ("cmplw", (gen_cmplw, 0x0)),
        ("bgt", (gen_bgt, 0x0)),
        ("srw.", (gen_srw, 0x1)),
        ("blectrl", (gen_blectr, 0x1)),
        ("mfmsr", (gen_mfmsr, 0x0)),
        ("bsol+", (gen_bso, 0x200001)),
        ("beq-", (gen_beq, 0x200000)),
        ("beqa", (gen_beq, 0x2)),
        ("fmadds", (gen_fmadds, 0x0)),
        ("bdzfla+", (gen_bdzf, 0x200003)),
        ("beqctr+", (gen_beqctr, 0x200000)),
        ("lfsx", (gen_lfsx, 0x0)),
        ("bsoa", (gen_bso, 0x2)),
        ("bdza-", (gen_bdz, 0x200002)),
        ("ps_neg.", (gen_ps_neg, 0x1)),
        ("addme", (gen_addme, 0x0)),
        ("bdnz-", (gen_bdnz, 0x200000)),
        ("bnsla-", (gen_bns, 0x200003)),
        ("ps_muls0.", (gen_ps_muls0, 0x1)),
        ("clrlwi.", (gen_clrlwi, 0x1)),
        ("fmul", (gen_fmul, 0x0)),
        ("andc", (gen_andc, 0x0)),
        ("clrrwi.", (gen_clrrwi, 0x1)),
        ("addco", (gen_addc, 0x400)),
        ("addo", (gen_add, 0x400)),
        ("beqlrl+", (gen_beqlr, 0x200001)),
        ("bgela+", (gen_bge, 0x200003)),
        ("srawi.", (gen_srawi, 0x1)),
        ("srawi", (gen_srawi, 0x0)),
        ("bl", (gen_b, 0x1)),
        ("fmadd.", (gen_fmadd, 0x1)),
        ("mr.", (gen_mr, 0x1)),
        ("bdzt+", (gen_bdzt, 0x200000)),
        ("rlwnm.", (gen_rlwnm, 0x1)),
        ("blectr+", (gen_blectr, 0x200000)),
        ("bdztl-", (gen_bdzt, 0x200001)),
        ("addzeo", (gen_addze, 0x400)),
        ("isync", (gen_isync, 0x0)),
        ("fsel", (gen_fsel, 0x0)),
        ("bnectrl+", (gen_bnectr, 0x200001)),
        ("ps_res.", (gen_ps_res, 0x1)),
        ("bne-", (gen_bne, 0x200000)),
        ("bla", (gen_b, 0x3)),
        ("bnea-", (gen_bne, 0x200002)),
        ("ps_merge10", (gen_ps_merge10, 0x0)),
        ("mfear", (gen_mfear, 0x0)),
        ("srwi", (gen_srwi, 0x0)),
        ("lfdux", (gen_lfdux, 0x0)),
        ("nop", (gen_nop, 0x0)),
        ("mtmsr", (gen_mtmsr, 0x0)),
        ("bcl-", (gen_bc, 0x200001)),
        ("dcbt", (gen_dcbt, 0x0)),
        ("cmpi", (gen_cmpi, 0x0)),
        ("ecowx", (gen_ecowx, 0x0)),
        ("fnabs.", (gen_fnabs, 0x1)),
        ("ps_merge11.", (gen_ps_merge11, 0x1)),
        ("bnectr+", (gen_bnectr, 0x200000)),
        ("bcla+", (gen_bc, 0x200003)),
        ("blr", (gen_blr, 0x0)),
        ("clrrwi", (gen_clrrwi, 0x0)),
        ("bns", (gen_bns, 0x0)),
        ("ps_muls1.", (gen_ps_muls1, 0x1)),
        ("fadds.", (gen_fadds, 0x1)),
        ("bdnzt+", (gen_bdnzt, 0x200000)),
        ("lfdx", (gen_lfdx, 0x0)),
        ("psq_stu", (gen_psq_stu, 0x0)),
        ("bnectr", (gen_bnectr, 0x0)),
        ("bsoa-", (gen_bso, 0x200002)),
        ("mtspr", (gen_mtspr, 0x0)),
        ("bdnztla+", (gen_bdnzt, 0x200003)),
        ("mflr", (gen_mflr, 0x0)),
        ("nand.", (gen_nand, 0x1)),
        ("bgtctrl", (gen_bgtctr, 0x1)),
        ("addme.", (gen_addme, 0x1)),
        ("xor", (gen_xor, 0x0)),
        ("mullwo", (gen_mullw, 0x400)),
        ("crandc", (gen_crandc, 0x0)),
        ("bltla-", (gen_blt, 0x200003)),
        ("mfctr", (gen_mfctr, 0x0)),
        ("ps_div", (gen_ps_div, 0x0)),
        ("bgel+", (gen_bge, 0x200001)),
        ("blelrl", (gen_blelr, 0x1)),
        ("stwx", (gen_stwx, 0x0)),
        ("lwarx", (gen_lwarx, 0x0)),
        ("bdnztla-", (gen_bdnzt, 0x200003)),
        ("lwz", (gen_lwz, 0x0)),
        ("bdnzl", (gen_bdnz, 0x1)),
        ("ps_nabs", (gen_ps_nabs, 0x0)),
        ("bge", (gen_bge, 0x0)),
        ("dcbi", (gen_dcbi, 0x0)),
        ("divwo", (gen_divw, 0x400)),
        ("ps_merge01", (gen_ps_merge01, 0x0)),
        ("bdzt-", (gen_bdzt, 0x200000)),
        ("cmplwi", (gen_cmplwi, 0x0)),
        ("subfic", (gen_subfic, 0x0)),
        ("bdnzfl-", (gen_bdnzf, 0x200001)),
        ("sthux", (gen_sthux, 0x0)),
        ("bdzta+", (gen_bdzt, 0x200002)),
        ("eciwx", (gen_eciwx, 0x0)),
        ("ps_mul.", (gen_ps_mul, 0x1)),
        ("mttbu", (gen_mttbu, 0x0)),
        ("mr", (gen_mr, 0x0)),
        ("bnea+", (gen_bne, 0x200002)),
        ("bgea-", (gen_bge, 0x200002)),
        ("beqlr+", (gen_beqlr, 0x200000)),
        ("bdzfla", (gen_bdzf, 0x3)),
        ("bdnzfl+", (gen_bdnzf, 0x200001)),
        ("subfo", (gen_subf, 0x400)),
        ("mtfsb0.", (gen_mtfsb0, 0x1)),
        ("lwzx", (gen_lwzx, 0x0)),
        ("bns-", (gen_bns, 0x200000)),
        ("lhzu", (gen_lhzu, 0x0)),
        ("and.", (gen_and, 0x1)),
        ("fctiw", (gen_fctiw, 0x0)),
        ("ps_nmadd.", (gen_ps_nmadd, 0x1)),
        ("subfe", (gen_subfe, 0x0)),
        ("addzeo.", (gen_addze, 0x401)),
        ("bdnzlrl", (gen_bdnzlr, 0x1)),
        ("bdzfla-", (gen_bdzf, 0x200003)),
        ("mtdbatl", (gen_mtdbatl, 0x0)),
        ("bc-", (gen_bc, 0x200000)),
        ("bdnztlrl+", (gen_bdnztlr, 0x200001)),
        ("divw.", (gen_divw, 0x1)),
        ("subfe.", (gen_subfe, 0x1)),
        ("bnel", (gen_bne, 0x1)),
        ("fres", (gen_fres, 0x0)),
        ("bnsla+", (gen_bns, 0x200003)),
        ("bdzlrl", (gen_bdzlr, 0x1)),
        ("mtsr", (gen_mtsr, 0x0)),
        ("sthx", (gen_sthx, 0x0)),
        ("stbux", (gen_stbux, 0x0)),
        ("fdivs.", (gen_fdivs, 0x1)),
        ("lhaux", (gen_lhaux, 0x0)),
        ("ps_add", (gen_ps_add, 0x0)),
        ("xor.", (gen_xor, 0x1)),
        ("cmpli", (gen_cmpli, 0x0)),
        ("orc", (gen_orc, 0x0)),
        ("stmw", (gen_stmw, 0x0)),
        ("dcbtst", (gen_dcbtst, 0x0)),
        ("bdnzt-", (gen_bdnzt, 0x200000)),
        ("bdnzlr+", (gen_bdnzlr, 0x200000)),
        ("addmeo.", (gen_addme, 0x401)),
        ("lwbrx", (gen_lwbrx, 0x0)),
        ("subfc", (gen_subfc, 0x0)),
        ("subfc.", (gen_subfc, 0x1)),
        ("stfsu", (gen_stfsu, 0x0)),
        ("crnot", (gen_crnot, 0x0)),
        ("bgea+", (gen_bge, 0x200002)),
        ("bdnzla+", (gen_bdnz, 0x200003)),
        ("bdzl+", (gen_bdz, 0x200001)),
        ("bdzfa+", (gen_bdzf, 0x200002)),
        ("stfdx", (gen_stfdx, 0x0)),
        ("bnslrl+", (gen_bnslr, 0x200001)),
        ("twgti", (gen_twgti, 0x0)),
        ("slw", (gen_slw, 0x0)),
        ("bdnzfla", (gen_bdnzf, 0x3)),
        ("bdzla+", (gen_bdz, 0x200003)),
        ("bgel-", (gen_bge, 0x200001)),
        ("beqctrl+", (gen_beqctr, 0x200001)),
        ("lmw", (gen_lmw, 0x0)),
        ("mfdsisr", (gen_mfdsisr, 0x0)),
        ("bge+", (gen_bge, 0x200000)),
        ("cntlzw.", (gen_cntlzw, 0x1)),
        ("blel+", (gen_ble, 0x200001)),
        ("dcbz", (gen_dcbz, 0x0)),
        ("ps_nmadd", (gen_ps_nmadd, 0x0)),
        ("lha", (gen_lha, 0x0)),
        ("bdnzl+", (gen_bdnz, 0x200001)),
        ("fsubs.", (gen_fsubs, 0x1)),
        ("dcbf", (gen_dcbf, 0x0)),
        ("blt+", (gen_blt, 0x200000)),
        ("lwzux", (gen_lwzux, 0x0)),
        ("ps_add.", (gen_ps_add, 0x1)),
        ("bdztla+", (gen_bdzt, 0x200003)),
        ("stfiwx", (gen_stfiwx, 0x0)),
        ("subi", (gen_subi, 0x0)),
        ("ble+", (gen_ble, 0x200000)),
        ("subfeo.", (gen_subfe, 0x401)),
        ("ps_madds1.", (gen_ps_madds1, 0x1)),
        ("bclr", (gen_bclr, 0x0)),
        ("or", (gen_or, 0x0)),
        ("rotlw.", (gen_rotlw, 0x1)),
        ("ps_sel", (gen_ps_sel, 0x0)),
        ("bnelrl+", (gen_bnelr, 0x200001)),
        ("bnectrl", (gen_bnectr, 0x1)),
        ("blelrl+", (gen_blelr, 0x200001)),
        ("bdzfa", (gen_bdzf, 0x2)),
        ("bdnzla-", (gen_bdnz, 0x200003)),
        ("mtfsfi", (gen_mtfsfi, 0x0)),
        ("dcbst", (gen_dcbst, 0x0)),
        ("beql+", (gen_beq, 0x200001)),
        ("bdnzlr", (gen_bdnzlr, 0x0)),
        ("bltctr+", (gen_bltctr, 0x200000)),
        ("bdnztl", (gen_bdnzt, 0x1)),
        ("psq_l", (gen_psq_l, 0x0)),
        ("lhau", (gen_lhau, 0x0)),
        ("bnelr", (gen_bnelr, 0x0)),
        ("bnslr+", (gen_bnslr, 0x200000)),
        ("bclrl", (gen_bclr, 0x1)),
        ("bdzla-", (gen_bdz, 0x200003)),
        ("cmpldi", (gen_cmpldi, 0x0)),
        ("psq_st", (gen_psq_st, 0x0)),
        ("bnsl", (gen_bns, 0x1)),
        ("ps_sel.", (gen_ps_sel, 0x1)),
        ("rlwimi", (gen_rlwimi, 0x0)),
        ("adde", (gen_adde, 0x0)),
        ("ps_sum1.", (gen_ps_sum1, 0x1)),
        ("bdztlr+", (gen_bdztlr, 0x200000)),
        ("divwuo", (gen_divwu, 0x400)),
        ("stfs", (gen_stfs, 0x0)),
        ("bdzflr", (gen_bdzflr, 0x0)),
        ("bgtl+", (gen_bgt, 0x200001)),
        ("mfsdr1", (gen_mfsdr1, 0x0)),
        ("stfdux", (gen_stfdux, 0x0)),
        ("or.", (gen_or, 0x1)),
        ("stbu", (gen_stbu, 0x0)),
        ("mtfsb1", (gen_mtfsb1, 0x0)),
        ("nor.", (gen_nor, 0x1)),
        ("bnslr", (gen_bnslr, 0x0)),
        ("rlwimi.", (gen_rlwimi, 0x1)),
        ("beqctrl", (gen_beqctr, 0x1)),
        ("bdztl", (gen_bdzt, 0x1)),
        ("bctr", (gen_bctr, 0x0)),
        ("crorc", (gen_crorc, 0x0)),
        ("bgta-", (gen_bgt, 0x200002)),
        ("ps_nmsub.", (gen_ps_nmsub, 0x1)),
        ("divwuo.", (gen_divwu, 0x401)),
        ("icbi.", (gen_icbi, 0x1)),
        ("ps_sum1", (gen_ps_sum1, 0x0)),
        ("bnsctr+", (gen_bnsctr, 0x200000)),
        ("blea+", (gen_ble, 0x200002)),
        ("bgtctr", (gen_bgtctr, 0x0)),
        ("mfspr", (gen_mfspr, 0x0)),
        ("blectrl+", (gen_blectr, 0x200001)),
        ("bns+", (gen_bns, 0x200000)),
        ("nego", (gen_neg, 0x400)),
        ("mtsrin", (gen_mtsrin, 0x0)),
        ("subfeo", (gen_subfe, 0x400)),
        ("ps_abs.", (gen_ps_abs, 0x1)),
        ("fdivs", (gen_fdivs, 0x0)),
        ("bsolrl+", (gen_bsolr, 0x200001)),
        ("fmul.", (gen_fmul, 0x1)),
        ("rotlw", (gen_rotlw, 0x0)),
        ("bcla", (gen_bc, 0x3)),
        ("fmuls", (gen_fmuls, 0x0)),
        ("bdzt", (gen_bdzt, 0x0)),
        ("bdza+", (gen_bdz, 0x200002)),
        ("fnmadds.", (gen_fnmadds, 0x1)),
        ("twlge", (gen_twlge, 0x0)),
        ("ps_madd.", (gen_ps_madd, 0x1)),
        ("bne+", (gen_bne, 0x200000)),
        ("beql", (gen_beq, 0x1)),
        ("cmpd", (gen_cmpd, 0x0)),
        ("mtfsfi.", (gen_mtfsfi, 0x1)),
        ("bnela-", (gen_bne, 0x200003)),
        ("stfsux", (gen_stfsux, 0x0)),
        ("rotlwi.", (gen_rotlwi, 0x1)),
        ("extrwi.", (gen_extrwi, 0x1)),
        ("bgelr+", (gen_bgelr, 0x200000)),
        ("lfs", (gen_lfs, 0x0)),
        ("bdzf-", (gen_bdzf, 0x200000)),
        ("bltla", (gen_blt, 0x3)),
        ("divwu", (gen_divwu, 0x0)),
        ("fnmsubs.", (gen_fnmsubs, 0x1)),
        ("bnel-", (gen_bne, 0x200001)),
        ("subfme.", (gen_subfme, 0x1)),
        ("mcrf", (gen_mcrf, 0x0)),
        ("frsp.", (gen_frsp, 0x1)),
        ("bcctr+", (gen_bcctr, 0x200000)),
        ("extsb", (gen_extsb, 0x0)),
        ("clrlslwi.", (gen_clrlslwi, 0x1)),
        ("bso", (gen_bso, 0x0)),
        ("bdza", (gen_bdz, 0x2)),
        ("bdnzlrl+", (gen_bdnzlr, 0x200001)),
        ("bdnz", (gen_bdnz, 0x0)),
        ("mullw", (gen_mullw, 0x0)),
        ("sraw.", (gen_sraw, 0x1)),
        ("bgea", (gen_bge, 0x2)),
        ("bgtla-", (gen_bgt, 0x200003)),
        ("frsp", (gen_frsp, 0x0)),
        ("blea", (gen_ble, 0x2)),
        ("bdztla", (gen_bdzt, 0x3)),
        ("ps_merge11", (gen_ps_merge11, 0x0)),
        ("twllei", (gen_twllei, 0x0)),
        ("ps_mr.", (gen_ps_mr, 0x1)),
        ("mttbl", (gen_mttbl, 0x0)),
        ("bltla+", (gen_blt, 0x200003)),
        ("mcrfs", (gen_mcrfs, 0x0)),
        ("bdzla", (gen_bdz, 0x3)),
        ("rlwinm", (gen_rlwinm, 0x0)),
        ("bne", (gen_bne, 0x0)),
        ("sync", (gen_sync, 0x0)),
        ("bca-", (gen_bc, 0x200002)),
        ("orc.", (gen_orc, 0x1)),
        ("eqv", (gen_eqv, 0x0)),
        ("andi.", (gen_andi_, 0x0)),
        ("mfsrr1", (gen_mfsrr1, 0x0)),
        ("fsel.", (gen_fsel, 0x1)),
        ("stfd", (gen_stfd, 0x0)),
        ("bnslrl", (gen_bnslr, 0x1)),
        ("subfco", (gen_subfc, 0x400)),
        ("mfsrr0", (gen_mfsrr0, 0x0)),
        ("subfze.", (gen_subfze, 0x1)),
        ("mffs", (gen_mffs, 0x0)),
        ("fdiv.", (gen_fdiv, 0x1)),
        ("bdnzta-", (gen_bdnzt, 0x200002)),
        ("blta", (gen_blt, 0x2)),
        ("ps_muls1", (gen_ps_muls1, 0x0)),
        ("bsola", (gen_bso, 0x3)),
        ("b", (gen_b, 0x0)),
        ("bltl-", (gen_blt, 0x200001)),
        ("bcl+", (gen_bc, 0x200001)),
        ("nego.", (gen_neg, 0x401)),
        ("bgel", (gen_bge, 0x1)),
        ("ps_merge00", (gen_ps_merge00, 0x0)),
        ("stwcx.", (gen_stwcx_, 0x0)),
        ("add", (gen_add, 0x0)),
        ("fsubs", (gen_fsubs, 0x0)),
        ("bcl", (gen_bc, 0x1)),
        ("ba", (gen_b, 0x2)),
        ("bdnzflrl", (gen_bdnzflr, 0x1)),
        ("bltctr", (gen_bltctr, 0x0)),
        ("bnsctrl+", (gen_bnsctr, 0x200001)),
        ("lhz", (gen_lhz, 0x0)),
        ("lhzux", (gen_lhzux, 0x0)),
        ("bgtlrl", (gen_bgtlr, 0x1)),
        ("bdnztl+", (gen_bdnzt, 0x200001)),
        ("fadd", (gen_fadd, 0x0)),
        ("fadds", (gen_fadds, 0x0)),
        ("ps_madds1", (gen_ps_madds1, 0x0)),
        ("mfibatu", (gen_mfibatu, 0x0)),
        ("addze.", (gen_addze, 0x1)),
        ("mftb", (gen_mftb, 0x0)),
        ("bcctrl", (gen_bcctr, 0x1)),
        ("bgta+", (gen_bgt, 0x200002)),
        ("bdzl", (gen_bdz, 0x1)),
        ("mtxer", (gen_mtxer, 0x0)),
        ("beql-", (gen_beq, 0x200001)),
        ("beq", (gen_beq, 0x0)),
        ("mffs.", (gen_mffs, 0x1)),
        ("bltctrl", (gen_bltctr, 0x1)),
        ("mtsrr1", (gen_mtsrr1, 0x0)),
        ("stw", (gen_stw, 0x0)),
        ("subf", (gen_subf, 0x0)),
        ("lis", (gen_lis, 0x0)),
        ("bca", (gen_bc, 0x2)),
        ("bdnzla", (gen_bdnz, 0x3)),
        ("bnsla", (gen_bns, 0x3)),
        ("lswx", (gen_lswx, 0x0)),
        ("bdzfa-", (gen_bdzf, 0x200002)),
        ("bdnzta", (gen_bdnzt, 0x2)),
        ("bdnztl-", (gen_bdnzt, 0x200001)),
        ("bdzf+", (gen_bdzf, 0x200000)),
        ("bdnzta+", (gen_bdnzt, 0x200002)),
        ("rlwinm.", (gen_rlwinm, 0x1)),
        ("mtdar", (gen_mtdar, 0x0)),
        ("bdztlr", (gen_bdztlr, 0x0)),
        ("nor", (gen_nor, 0x0)),
        ("srwi.", (gen_srwi, 0x1)),
        ("bsoctr+", (gen_bsoctr, 0x200000)),
        ("xori", (gen_xori, 0x0)),
        ("cmpld", (gen_cmpld, 0x0)),
        ("crand", (gen_crand, 0x0)),
        ("bnsctrl", (gen_bnsctr, 0x1)),
        ("bdztl+", (gen_bdzt, 0x200001)),
        ("bgtla", (gen_bgt, 0x3)),
        ("bdnzf-", (gen_bdnzf, 0x200000)),
        ("extsb.", (gen_extsb, 0x1)),
        ("ps_div.", (gen_ps_div, 0x1)),
        ("fsub.", (gen_fsub, 0x1)),
        ("bclrl+", (gen_bclr, 0x200001)),
        ("lbzux", (gen_lbzux, 0x0)),
        ("beqla+", (gen_beq, 0x200003)),
        ("rlwnm", (gen_rlwnm, 0x0)),
        ("lhbrx", (gen_lhbrx, 0x0)),
        ("crmove", (gen_crmove, 0x0)),
        ("bdnztla", (gen_bdnzt, 0x3)),
        ("fnmsub", (gen_fnmsub, 0x0)),
        ("subis", (gen_subis, 0x0)),
        ("ps_cmpo1", (gen_ps_cmpo1, 0x0)),
        ("ps_msub", (gen_ps_msub, 0x0)),
        ("bdnz+", (gen_bdnz, 0x200000)),
        ("bdnzl-", (gen_bdnz, 0x200001)),
        ("bnela", (gen_bne, 0x3)),
        ("fmuls.", (gen_fmuls, 0x1)),
        ("ps_merge10.", (gen_ps_merge10, 0x1)),
        ("lhzx", (gen_lhzx, 0x0)),
        ("psq_lux", (gen_psq_lux, 0x0)),
        ("bdnzfl", (gen_bdnzf, 0x1)),
        ("tlbsync", (gen_tlbsync, 0x0)),
        ("ps_sum0", (gen_ps_sum0, 0x0)),
        ("bc+", (gen_bc, 0x200000)),
        ("mfsprg", (gen_mfsprg, 0x0)),
        ("fabs", (gen_fabs, 0x0)),
        ("mullwo.", (gen_mullw, 0x401)),
        ("mulhwu.", (gen_mulhwu, 0x1)),
        ("mtsdr1", (gen_mtsdr1, 0x0)),
        ("bnsl+", (gen_bns, 0x200001)),
        ("divw", (gen_divw, 0x0)),
        ("addic.", (gen_addic_, 0x0)),
        ("bctrl", (gen_bctr, 0x1)),
        ("mtsrr0", (gen_mtsrr0, 0x0)),
        ("bgela", (gen_bge, 0x3)),
        ("ble-", (gen_ble, 0x200000)),
        ("bdztlrl", (gen_bdztlr, 0x1)),
        ("mfsrin", (gen_mfsrin, 0x0)),
        ("bgectrl", (gen_bgectr, 0x1)),
        ("fmsubs.", (gen_fmsubs, 0x1)),
        ("lbzx", (gen_lbzx, 0x0)),
        ("bgt-", (gen_bgt, 0x200000)),
        ("bdnza", (gen_bdnz, 0x2)),
        ("addo.", (gen_add, 0x401)),
        ("bdzf", (gen_bdzf, 0x0)),
        ("crnor", (gen_crnor, 0x0)),
        ("ps_muls0", (gen_ps_muls0, 0x0)),
        ("neg.", (gen_neg, 0x1)),
        ("ps_nmsub", (gen_ps_nmsub, 0x0)),
        ("ps_cmpu1", (gen_ps_cmpu1, 0x0)),
        ("bdzflrl+", (gen_bdzflr, 0x200001)),
        ("blel-", (gen_ble, 0x200001)),
        ("bnea", (gen_bne, 0x2)),
        ("twi", (gen_twi, 0x0)),
        ("rotrwi.", (gen_rotrwi, 0x1)),
        ("bnelr+", (gen_bnelr, 0x200000)),
        ("addze", (gen_addze, 0x0)),
        ("blel", (gen_ble, 0x1)),
        ("fmsub", (gen_fmsub, 0x0)),
        ("bsola+", (gen_bso, 0x200003)),
        ("addeo", (gen_adde, 0x400)),
        ("beqctr", (gen_beqctr, 0x0)),
        ("adde.", (gen_adde, 0x1)),
        ("fmsubs", (gen_fmsubs, 0x0)),
        ("bdzflrl", (gen_bdzflr, 0x1)),
        ("bdnzf", (gen_bdnzf, 0x0)),
        ("fdiv", (gen_fdiv, 0x0)),
        ("bltlr", (gen_bltlr, 0x0)),
        ("bdnzflr", (gen_bdnzflr, 0x0)),
        ("mtdsisr", (gen_mtdsisr, 0x0)),
        ("fcmpo", (gen_fcmpo, 0x0)),
        ("mulli", (gen_mulli, 0x0)),
        ("cmpwi", (gen_cmpwi, 0x0)),
        ("addco.", (gen_addc, 0x401)),
        ("ps_abs", (gen_ps_abs, 0x0)),
        ("psq_lu", (gen_psq_lu, 0x0)),
        ("oris", (gen_oris, 0x0)),
        ("bnsa", (gen_bns, 0x2)),
        ("ps_madds0.", (gen_ps_madds0, 0x1)),
        ("fmadd", (gen_fmadd, 0x0)),
        ("psq_lx", (gen_psq_lx, 0x0)),
        ("bdzfl-", (gen_bdzf, 0x200001)),
        ("bgela-", (gen_bge, 0x200003)),
        ("bnelrl", (gen_bnelr, 0x1)),
        ("subfzeo.", (gen_subfze, 0x401)),
        ("nand", (gen_nand, 0x0)),
        ("mtcrf", (gen_mtcrf, 0x0)),
        ("lbzu", (gen_lbzu, 0x0)),
        ("tweq", (gen_tweq, 0x0)),
        ("stwbrx", (gen_stwbrx, 0x0)),
        ("slwi.", (gen_slwi, 0x1)),
        ("bnsctr", (gen_bnsctr, 0x0)),
        ("bdzlr+", (gen_bdzlr, 0x200000)),
        ("stfsx", (gen_stfsx, 0x0)),
        ("bgtlrl+", (gen_bgtlr, 0x200001)),
        ("stbx", (gen_stbx, 0x0)),
        ("addic", (gen_addic, 0x0)),
        ("rotrwi", (gen_rotrwi, 0x0)),
        ("bgectr+", (gen_bgectr, 0x200000)),
        ("beqla", (gen_beq, 0x3)),
        ("bgelr", (gen_bgelr, 0x0)),
    ],
};
pub fn assemble(mnemonic: &str, args: &Arguments) -> Result<u32, ArgumentError> {
    if let Some(&(fn_ptr, modifiers)) = MNEMONIC_MAP.get(mnemonic) {
        fn_ptr(args, modifiers)
    } else {
        Err(ArgumentError::UnknownMnemonic)
    }
}
