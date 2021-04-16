//! A peephole optimization (baseline) pass.

use crate::cursor::{Cursor, FuncCursor};
// use crate::isa::TargetIsa;
use crate::timing;
use crate::ir::Inst;
use crate::ir::instructions::Opcode;
//use crate::ir::condcodes::IntCC;
//use cranelift_codegen_shared::condcodes::IntCC;
use crate::ir::dfg::ValueDef;
use crate::ir::{Function, InstBuilder, InstructionData};
use crate::ir::types::I32;

fn superopt_1(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -1 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_2(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 2 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 32);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_3(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_4 = pos.ins().iconst(I32, 1_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_4, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_4(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 8 {
if rhs_1 == 16 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 24);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_5(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 1 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_6(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_1 == 0 {
if arg_0[0] != arg_1 {
pos.func.dfg.replace(inst).isub(arg_1, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_7(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 5 {
if rhs_1 == 5 {
pos.func.dfg.replace(inst).band_imm(arg_1, 134217727);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_8(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967294);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_9(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 3 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_1, 536870911);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_10(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 3 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967288);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_11(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bxor => {
if rhs_1 == -1 {
if arg_1[0] != arg_1[1] {
pos.func.dfg.replace(inst).bxor_not(arg_1[0], arg_1[1]);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_12(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_1 != arg_0[1] {
pos.func.dfg.replace(inst).band_not(arg_0[1], arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_13(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == -2139062144 {
let rhs_inst_4 = pos.ins().iconst(I32, 2155905152_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_4, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_14(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 25 {
if rhs_1 == 8 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_5 == 25 {
if arg_3 == arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 257);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_15(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -16843009 {
if arg_2 == arg_1 {
pos.func.dfg.replace(inst).band_not(arg_0[1], arg_2);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_16(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == -16843009 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_3 == -1 {
if arg_2 == arg_1 {
pos.func.dfg.replace(inst).band_not(arg_0[0], arg_2);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_17(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == -2139062144 {
if rhs_1 == -2139062144 {
let rhs_inst_4 = pos.ins().iconst(I32, 2155905152_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_4, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_18(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 2 {
if rhs_1 == 28 {
if arg_0[0] != arg_2 {
pos.func.dfg.replace(inst).ushr(arg_2, arg_1);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_19(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_20(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 3 {
if arg_2 == arg_0[0] {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 3);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967292);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_21(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 8 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 9);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_22(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == -1 {
if arg_2 == arg_1[0] {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 4294967295);
pos.func.dfg.replace(inst).band_not(rhs_inst_5, arg_2);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_23(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 4 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 5);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_24(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 2 {
if rhs_1 == 4 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 6);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_25(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 3 {
if rhs_1 == 4 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 7);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_26(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 2 {
if rhs_1 == 8 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 10);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_27(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == -1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 9 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_28(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 3 {
if rhs_1 == 8 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 11);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_29(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == -2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 9 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 7);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_30(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 3 {
if rhs_1 == 12 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 15);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_31(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 16 {
if rhs_1 == 3 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 19);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_32(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 16 {
if rhs_1 == 1 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_33(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 2 {
if rhs_1 == 12 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 14);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_34(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == 16 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_35(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 16 {
if rhs_1 == 2 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 18);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_36(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == 16 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 18);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_37(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == 16 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 3 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 19);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_38(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 1057700 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 1057704);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_5);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_39(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if rhs_2 == 1 {
if rhs_1 == 1057700 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 1057708);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_5);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_40(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 12 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 13);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_41(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 1057700 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 1057702);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_5);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_42(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_7);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_43(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_0[1] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).iadd(arg_0[1], rhs_inst_7);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_44(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 1124168 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1124172);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_45(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == 1124168 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1124176);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_46(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if rhs_2 == 2 {
if rhs_1 == 1124168 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1124184);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_47(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 8 {
if rhs_2 == 3 {
if rhs_1 == 1056208 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1056272);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_48(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 3 {
if rhs_1 == 1056208 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1056216);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_49(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 8 {
if rhs_2 == 2 {
if rhs_1 == 1124168 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1124200);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_50(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if rhs_2 == 3 {
if rhs_1 == 1056208 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1056240);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_51(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 3 {
if rhs_1 == 1056208 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1056224);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_52(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1077980);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_53(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1077968);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_54(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078012);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_55(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1077984);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_56(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 8 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078080);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_57(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 8 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078076);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_58(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078016);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_59(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 31 {
if rhs_2 == 18 {
if rhs_1 == 1835008 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 7);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 262144);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_60(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1077964);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_61(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 15 {
if rhs_2 == 3 {
if rhs_1 == 1056208 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1056328);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_62(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 66 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1079008);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_63(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 33 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078480);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_64(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 66 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1079004);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_65(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 33 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078476);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_66(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 17 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078220);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_67(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 17 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1078224);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_68(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 133 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1080080);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_69(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 531 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1086444);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_70(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 266 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1082204);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_71(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 133 {
if rhs_2 == 4 {
if rhs_1 == 1077948 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1080076);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_72(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 531 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1086448);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_73(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 266 {
if rhs_2 == 4 {
if rhs_1 == 1077952 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1082208);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_74(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -42946 {
if rhs_2 == 2 {
if rhs_1 == 1112864 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 941080);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_75(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -42922 {
if rhs_2 == 2 {
if rhs_1 == 1112824 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 941136);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_76(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -39 {
if rhs_1 == -2 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967255);
pos.func.dfg.replace(inst).iadd(arg_1[1], rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_77(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if rhs_2 == -8 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 1073741823);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 536870911);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_78(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -39 {
if rhs_1 == -4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967253);
pos.func.dfg.replace(inst).iadd(arg_1[1], rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_79(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_1 == -1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -128 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967167);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_80(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 19 {
if rhs_2 == -16 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 4294967292);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_81(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -128 {
if rhs_1 == 127 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967295);
pos.func.dfg.replace(inst).iadd(arg_2, rhs_inst_6);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_82(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 7 {
if rhs_2 == -1 {
if rhs_1 == 16843009 {
let rhs_inst_5 = pos.ins().iconst(I32, 2155905153_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).ushr_imm(rhs_inst_6, 7);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_83(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == 1125204 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967288);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1125204);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_84(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -8 {
if rhs_1 == 20 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[0], 12);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_7);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_85(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -1 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -128 {
if rhs_1 == 128 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 0);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_86(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 0 {
if rhs_2 == 3 {
if rhs_1 == 4 {
if arg_3 == arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_3, 4294966271);
let rhs_inst_7 = pos.ins().bor_imm(rhs_inst_6, 4294967291);
pos.func.dfg.replace(inst).bnot(rhs_inst_7);

}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_87(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_5 == 1 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_8 = pos.ins().band_not(rhs_inst_7, arg_3);
let rhs_inst_9 = pos.ins().iadd_imm(rhs_inst_8, 1);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_9);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_88(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 25 {
if rhs_2 == 8 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_6 == 25 {
if rhs_1 == 16 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_6 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_6[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_9 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_7) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_10 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_10 == 25 {
if rhs_9 == 8 {
match pos.func.dfg.value_def(arg_6[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_13 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_13 == 25 {
if arg_4 == arg_8 && arg_4 == arg_9 && arg_4 == arg_5 && arg_8 == arg_9 && arg_8 == arg_5 && arg_9 == arg_5 {
let rhs_inst_7 = pos.ins().imul_imm(arg_7, 16843009);
let rhs_inst_8 = pos.ins().bnot(rhs_inst_7);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_89(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == 148 {
if arg_1[0] != arg_3 {
let rhs_inst_7 = pos.ins().imul_imm(arg_3, 4);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 156);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_8);

}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_90(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_1 == 0 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_3 != arg_1[0] && arg_3 != arg_2 && arg_1[0] != arg_2 {
let rhs_inst_8 = pos.ins().irsub_imm(arg_2, 1);
let rhs_inst_9 = pos.ins().iadd(arg_3, rhs_inst_8);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_9);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_91(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2[0] != arg_3 && arg_2[0] != arg_1[0] && arg_3 != arg_1[0] {
let rhs_inst_8 = pos.ins().iadd_imm(arg_3, 2);
let rhs_inst_9 = pos.ins().iadd(arg_2[0], rhs_inst_8);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_9);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_92(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -8 {
if rhs_1 == 16 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 4294967288);
let rhs_inst_8 = pos.ins().isub(arg_3, rhs_inst_7);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_8);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_93(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 1 {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 1124984 {
let rhs_inst_6 = pos.ins().imul_imm(arg_3, 4294967288);
let rhs_inst_7 = pos.ins().iadd_imm(rhs_inst_6, 4293842307);
pos.func.dfg.replace(inst).bnot(rhs_inst_7);

}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_94(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 3 {
if rhs_3 == -8 {
if rhs_2 == 3 {
if rhs_1 == 1 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_3, 536870912);
let rhs_inst_7 = pos.ins().band_imm(rhs_inst_6, 3758096384);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_7);

}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_95(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_0[1] != arg_1[1] && arg_0[1] != arg_3 && arg_1[1] != arg_3 {
let rhs_inst_8 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_9 = pos.ins().band_not(rhs_inst_8, arg_3);
let rhs_inst_10 = pos.ins().iadd(arg_1[1], rhs_inst_9);
pos.func.dfg.replace(inst).iadd(arg_0[1], rhs_inst_10);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_96(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_3 != arg_1[0] && arg_3 != arg_1[1] && arg_1[0] != arg_1[1] {
let rhs_inst_8 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_9 = pos.ins().band_not(rhs_inst_8, arg_3);
let rhs_inst_10 = pos.ins().iadd(arg_1[1], rhs_inst_9);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_10);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_97(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Iadd => {
if arg_2 != arg_3[1] && arg_2 != arg_3[0] && arg_3[1] != arg_3[0] {
let rhs_inst_8 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_9 = pos.ins().band_not(rhs_inst_8, arg_2);
let rhs_inst_10 = pos.ins().iadd(arg_3[1], rhs_inst_9);
pos.func.dfg.replace(inst).iadd(arg_3[0], rhs_inst_10);

}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_98(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 7 {
if rhs_1 == -4 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 4 {
if arg_2 == arg_3 {
let rhs_inst_6 = pos.ins().imul_imm(arg_3, 4294967279);
let rhs_inst_7 = pos.ins().iconst(I32, 4294967292_u64 as i64);
let rhs_inst_8 = pos.ins().band_not(rhs_inst_7, rhs_inst_6);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_8, 8);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_99(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_4 == 0 {
if rhs_3 == 3 {
if rhs_2 == 4 {
if rhs_1 == 24 {
if arg_4 == arg_2[0] {
let rhs_inst_7 = pos.ins().iconst(I32, 4_u64 as i64);
let rhs_inst_8 = pos.ins().band_not(rhs_inst_7, arg_3);
let rhs_inst_9 = pos.ins().iadd_imm(rhs_inst_8, 4294967267);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_100(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_6 == 1 {
if rhs_1 == -1 {
if arg_4 != arg_3 {
let rhs_inst_8 = pos.ins().bor_imm(arg_4, 4294967294);
let rhs_inst_9 = pos.ins().isub(rhs_inst_8, arg_4);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_101(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 7 {
if rhs_2 == -1 {
if rhs_1 == 16843009 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_7 == 2139062143 {
if arg_3 == arg_4 {
let rhs_inst_7 = pos.ins().iconst(I32, 16843009_u64 as i64);
let rhs_inst_8 = pos.ins().band_not(rhs_inst_7, arg_2);
let rhs_inst_9 = pos.ins().imul_imm(rhs_inst_8, 127);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_102(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 0 {
if rhs_2 == 3 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_6 == -4 {
if rhs_1 == -4 {
if arg_2[0] == arg_4 && arg_2[0] != arg_5 && arg_4 != arg_5 {
let rhs_inst_9 = pos.ins().isub(arg_3, arg_1[1]);
let rhs_inst_10 = pos.ins().bor_imm(rhs_inst_9, 3);
pos.func.dfg.replace(inst).bnot(rhs_inst_10);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_103(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 3 {
if rhs_1 == 1 {
if arg_4[1] != arg_4[0] && arg_4[1] != arg_2[1] && arg_4[1] != arg_1[0] && arg_4[0] != arg_2[1] && arg_4[0] != arg_1[0] && arg_2[1] != arg_1[0] {
let rhs_inst_10 = pos.ins().iadd_imm(arg_3, 4);
let rhs_inst_11 = pos.ins().iadd(arg_2[1], rhs_inst_10);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_11);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_104(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 3 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_4) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_5) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_6 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_6[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_7) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_8 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_8 == 0 {
if rhs_7 == 3 {
if rhs_6 == 4 {
if rhs_5 == 24 {
if arg_6[0] == arg_3 && arg_6[0] == arg_8 && arg_6[0] == arg_1[0] && arg_3 == arg_8 && arg_3 == arg_1[0] && arg_8 == arg_1[0] {
let rhs_inst_8 = pos.ins().band_imm(arg_7, 4294967288);
let rhs_inst_9 = pos.ins().iadd_imm(rhs_inst_8, 4294967271);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_105(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_5) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_6 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 25 {
if rhs_3 == 8 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_7 == 25 {
if rhs_2 == 16 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_8 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_8[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_10 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_9) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_10 = arg;
let rhs_11 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_11 == 25 {
if rhs_10 == 8 {
match pos.func.dfg.value_def(arg_8[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_11 = arg;
let rhs_14 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_14 == 25 {
if rhs_1 == -1 {
if arg_1[0] != arg_10 && arg_1[0] != arg_6 && arg_1[0] != arg_7 && arg_1[0] != arg_11 && arg_10 == arg_6 && arg_10 == arg_7 && arg_10 == arg_11 && arg_6 == arg_7 && arg_6 == arg_11 && arg_7 == arg_11 {
let rhs_inst_10 = pos.ins().imul_imm(arg_9, 16843009);
let rhs_inst_11 = pos.ins().bxor(arg_11, rhs_inst_10);
pos.func.dfg.replace(inst).bnot(rhs_inst_11);

}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_106(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_3[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_5 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_5[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_6 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_6) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_7 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_7[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_4 == 0 {
if rhs_3 == -1 {
match pos.func.dfg.value_def(arg_5[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_9) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_10 = arg;
let rhs_8 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_10) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_11 = arg;
let rhs_9 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_11) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_12 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_12[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_13 = arg;
let rhs_10 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_10 == 0 {
if rhs_9 == -1 {
if rhs_8 == 12 {
if rhs_7 == 16 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_14 = arg;
let rhs_15 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_14) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_15 = arg;
let rhs_16 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_15) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_16 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_16[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_17 = arg;
let rhs_17 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_17) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_18 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_18[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_19 = arg;
let rhs_18 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_18 == 0 {
if rhs_17 == -1 {
match pos.func.dfg.value_def(arg_16[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_20 = arg;
let rhs_21 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_20) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_21 = arg;
let rhs_22 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_21) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_22 = arg;
let rhs_23 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_22) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_23 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_23[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_24 = arg;
let rhs_24 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_24 == 0 {
if rhs_23 == -1 {
if rhs_22 == 12 {
if rhs_21 == 16 {
if rhs_16 == 5 {
if rhs_15 == 8 {
match pos.func.dfg.value_def(arg_3[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_25 = arg;
let rhs_31 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_25) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_26 = arg;
let rhs_32 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_26) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_27 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_27[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_28 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_28[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_29 = arg;
let rhs_33 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_29) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_30 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_30[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_31 = arg;
let rhs_34 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_34 == 0 {
if rhs_33 == -1 {
match pos.func.dfg.value_def(arg_28[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_32 = arg;
let rhs_37 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_32) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_33 = arg;
let rhs_38 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_33) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_34 = arg;
let rhs_39 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_34) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_35 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_35[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_36 = arg;
let rhs_40 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_40 == 0 {
if rhs_39 == -1 {
if rhs_38 == 12 {
if rhs_37 == 16 {
match pos.func.dfg.value_def(arg_27[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_37 = arg;
let rhs_45 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_37) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_38 = arg;
let rhs_46 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_38) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_39 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_39[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_40 = arg;
let rhs_47 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_40) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_41 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_41[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_42 = arg;
let rhs_48 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_48 == 0 {
if rhs_47 == -1 {
match pos.func.dfg.value_def(arg_39[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_43 = arg;
let rhs_51 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_43) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_44 = arg;
let rhs_52 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_44) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_45 = arg;
let rhs_53 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_45) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_46 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_46[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_47 = arg;
let rhs_54 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_54 == 0 {
if rhs_53 == -1 {
if rhs_52 == 12 {
if rhs_51 == 16 {
if rhs_46 == 5 {
if rhs_45 == 8 {
if rhs_32 == 2 {
if rhs_31 == 4 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_48 = arg;
let rhs_63 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_48) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_49 = arg;
let rhs_64 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_49) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_50 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_50[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_51 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_51[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_52 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_52[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_53 = arg;
let rhs_65 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_53) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_54 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_54[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_55 = arg;
let rhs_66 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_66 == 0 {
if rhs_65 == -1 {
match pos.func.dfg.value_def(arg_52[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_56 = arg;
let rhs_69 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_56) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_57 = arg;
let rhs_70 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_57) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_58 = arg;
let rhs_71 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_58) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_59 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_59[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_60 = arg;
let rhs_72 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_72 == 0 {
if rhs_71 == -1 {
if rhs_70 == 12 {
if rhs_69 == 16 {
match pos.func.dfg.value_def(arg_51[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_61 = arg;
let rhs_77 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_61) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_62 = arg;
let rhs_78 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_62) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_63 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_63[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_64 = arg;
let rhs_79 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_64) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_65 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_65[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_66 = arg;
let rhs_80 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_80 == 0 {
if rhs_79 == -1 {
match pos.func.dfg.value_def(arg_63[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_67 = arg;
let rhs_83 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_67) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_68 = arg;
let rhs_84 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_68) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_69 = arg;
let rhs_85 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_69) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_70 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_70[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_71 = arg;
let rhs_86 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_86 == 0 {
if rhs_85 == -1 {
if rhs_84 == 12 {
if rhs_83 == 16 {
if rhs_78 == 5 {
if rhs_77 == 8 {
match pos.func.dfg.value_def(arg_50[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_72 = arg;
let rhs_93 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_72) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_73 = arg;
let rhs_94 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_73) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_74 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_74[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_75 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_75[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_76 = arg;
let rhs_95 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_76) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_77 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_77[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_78 = arg;
let rhs_96 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_96 == 0 {
if rhs_95 == -1 {
match pos.func.dfg.value_def(arg_75[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_79 = arg;
let rhs_99 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_79) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_80 = arg;
let rhs_100 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_80) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_81 = arg;
let rhs_101 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_81) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_82 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_82[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_83 = arg;
let rhs_102 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_102 == 0 {
if rhs_101 == -1 {
if rhs_100 == 12 {
if rhs_99 == 16 {
match pos.func.dfg.value_def(arg_74[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_84 = arg;
let rhs_107 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_84) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_85 = arg;
let rhs_108 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_85) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_86 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_86[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_87 = arg;
let rhs_109 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_87) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_88 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_88[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_89 = arg;
let rhs_110 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_110 == 0 {
if rhs_109 == -1 {
match pos.func.dfg.value_def(arg_86[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_90 = arg;
let rhs_113 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_90) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_91 = arg;
let rhs_114 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_91) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_92 = arg;
let rhs_115 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_92) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_93 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_93[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_94 = arg;
let rhs_116 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_116 == 0 {
if rhs_115 == -1 {
if rhs_114 == 12 {
if rhs_113 == 16 {
if rhs_108 == 5 {
if rhs_107 == 8 {
if rhs_94 == 2 {
if rhs_93 == 4 {
if rhs_64 == 1 {
if rhs_63 == 2 {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_36 == arg_19 && arg_36 == arg_41[0] && arg_36 == arg_70[0] && arg_36 == arg_83 && arg_36 == arg_18[0] && arg_36 == arg_7[0] && arg_36 == arg_65[0] && arg_36 == arg_77[0] && arg_36 == arg_55 && arg_36 == arg_24 && arg_36 == arg_54[0] && arg_36 == arg_35[0] && arg_36 == arg_66 && arg_36 == arg_78 && arg_36 == arg_47 && arg_36 == arg_31 && arg_36 == arg_71 && arg_36 == arg_82[0] && arg_36 == arg_60 && arg_36 == arg_12[0] && arg_36 == arg_88[0] && arg_36 == arg_93[0] && arg_36 == arg_13 && arg_36 == arg_30[0] && arg_36 == arg_59[0] && arg_36 == arg_8 && arg_36 == arg_23[0] && arg_36 == arg_46[0] && arg_36 == arg_42 && arg_36 == arg_89 && arg_36 == arg_94 && arg_19 == arg_41[0] && arg_19 == arg_70[0] && arg_19 == arg_83 && arg_19 == arg_18[0] && arg_19 == arg_7[0] && arg_19 == arg_65[0] && arg_19 == arg_77[0] && arg_19 == arg_55 && arg_19 == arg_24 && arg_19 == arg_54[0] && arg_19 == arg_35[0] && arg_19 == arg_66 && arg_19 == arg_78 && arg_19 == arg_47 && arg_19 == arg_31 && arg_19 == arg_71 && arg_19 == arg_82[0] && arg_19 == arg_60 && arg_19 == arg_12[0] && arg_19 == arg_88[0] && arg_19 == arg_93[0] && arg_19 == arg_13 && arg_19 == arg_30[0] && arg_19 == arg_59[0] && arg_19 == arg_8 && arg_19 == arg_23[0] && arg_19 == arg_46[0] && arg_19 == arg_42 && arg_19 == arg_89 && arg_19 == arg_94 && arg_41[0] == arg_70[0] && arg_41[0] == arg_83 && arg_41[0] == arg_18[0] && arg_41[0] == arg_7[0] && arg_41[0] == arg_65[0] && arg_41[0] == arg_77[0] && arg_41[0] == arg_55 && arg_41[0] == arg_24 && arg_41[0] == arg_54[0] && arg_41[0] == arg_35[0] && arg_41[0] == arg_66 && arg_41[0] == arg_78 && arg_41[0] == arg_47 && arg_41[0] == arg_31 && arg_41[0] == arg_71 && arg_41[0] == arg_82[0] && arg_41[0] == arg_60 && arg_41[0] == arg_12[0] && arg_41[0] == arg_88[0] && arg_41[0] == arg_93[0] && arg_41[0] == arg_13 && arg_41[0] == arg_30[0] && arg_41[0] == arg_59[0] && arg_41[0] == arg_8 && arg_41[0] == arg_23[0] && arg_41[0] == arg_46[0] && arg_41[0] == arg_42 && arg_41[0] == arg_89 && arg_41[0] == arg_94 && arg_70[0] == arg_83 && arg_70[0] == arg_18[0] && arg_70[0] == arg_7[0] && arg_70[0] == arg_65[0] && arg_70[0] == arg_77[0] && arg_70[0] == arg_55 && arg_70[0] == arg_24 && arg_70[0] == arg_54[0] && arg_70[0] == arg_35[0] && arg_70[0] == arg_66 && arg_70[0] == arg_78 && arg_70[0] == arg_47 && arg_70[0] == arg_31 && arg_70[0] == arg_71 && arg_70[0] == arg_82[0] && arg_70[0] == arg_60 && arg_70[0] == arg_12[0] && arg_70[0] == arg_88[0] && arg_70[0] == arg_93[0] && arg_70[0] == arg_13 && arg_70[0] == arg_30[0] && arg_70[0] == arg_59[0] && arg_70[0] == arg_8 && arg_70[0] == arg_23[0] && arg_70[0] == arg_46[0] && arg_70[0] == arg_42 && arg_70[0] == arg_89 && arg_70[0] == arg_94 && arg_83 == arg_18[0] && arg_83 == arg_7[0] && arg_83 == arg_65[0] && arg_83 == arg_77[0] && arg_83 == arg_55 && arg_83 == arg_24 && arg_83 == arg_54[0] && arg_83 == arg_35[0] && arg_83 == arg_66 && arg_83 == arg_78 && arg_83 == arg_47 && arg_83 == arg_31 && arg_83 == arg_71 && arg_83 == arg_82[0] && arg_83 == arg_60 && arg_83 == arg_12[0] && arg_83 == arg_88[0] && arg_83 == arg_93[0] && arg_83 == arg_13 && arg_83 == arg_30[0] && arg_83 == arg_59[0] && arg_83 == arg_8 && arg_83 == arg_23[0] && arg_83 == arg_46[0] && arg_83 == arg_42 && arg_83 == arg_89 && arg_83 == arg_94 && arg_18[0] == arg_7[0] && arg_18[0] == arg_65[0] && arg_18[0] == arg_77[0] && arg_18[0] == arg_55 && arg_18[0] == arg_24 && arg_18[0] == arg_54[0] && arg_18[0] == arg_35[0] && arg_18[0] == arg_66 && arg_18[0] == arg_78 && arg_18[0] == arg_47 && arg_18[0] == arg_31 && arg_18[0] == arg_71 && arg_18[0] == arg_82[0] && arg_18[0] == arg_60 && arg_18[0] == arg_12[0] && arg_18[0] == arg_88[0] && arg_18[0] == arg_93[0] && arg_18[0] == arg_13 && arg_18[0] == arg_30[0] && arg_18[0] == arg_59[0] && arg_18[0] == arg_8 && arg_18[0] == arg_23[0] && arg_18[0] == arg_46[0] && arg_18[0] == arg_42 && arg_18[0] == arg_89 && arg_18[0] == arg_94 && arg_7[0] == arg_65[0] && arg_7[0] == arg_77[0] && arg_7[0] == arg_55 && arg_7[0] == arg_24 && arg_7[0] == arg_54[0] && arg_7[0] == arg_35[0] && arg_7[0] == arg_66 && arg_7[0] == arg_78 && arg_7[0] == arg_47 && arg_7[0] == arg_31 && arg_7[0] == arg_71 && arg_7[0] == arg_82[0] && arg_7[0] == arg_60 && arg_7[0] == arg_12[0] && arg_7[0] == arg_88[0] && arg_7[0] == arg_93[0] && arg_7[0] == arg_13 && arg_7[0] == arg_30[0] && arg_7[0] == arg_59[0] && arg_7[0] == arg_8 && arg_7[0] == arg_23[0] && arg_7[0] == arg_46[0] && arg_7[0] == arg_42 && arg_7[0] == arg_89 && arg_7[0] == arg_94 && arg_65[0] == arg_77[0] && arg_65[0] == arg_55 && arg_65[0] == arg_24 && arg_65[0] == arg_54[0] && arg_65[0] == arg_35[0] && arg_65[0] == arg_66 && arg_65[0] == arg_78 && arg_65[0] == arg_47 && arg_65[0] == arg_31 && arg_65[0] == arg_71 && arg_65[0] == arg_82[0] && arg_65[0] == arg_60 && arg_65[0] == arg_12[0] && arg_65[0] == arg_88[0] && arg_65[0] == arg_93[0] && arg_65[0] == arg_13 && arg_65[0] == arg_30[0] && arg_65[0] == arg_59[0] && arg_65[0] == arg_8 && arg_65[0] == arg_23[0] && arg_65[0] == arg_46[0] && arg_65[0] == arg_42 && arg_65[0] == arg_89 && arg_65[0] == arg_94 && arg_77[0] == arg_55 && arg_77[0] == arg_24 && arg_77[0] == arg_54[0] && arg_77[0] == arg_35[0] && arg_77[0] == arg_66 && arg_77[0] == arg_78 && arg_77[0] == arg_47 && arg_77[0] == arg_31 && arg_77[0] == arg_71 && arg_77[0] == arg_82[0] && arg_77[0] == arg_60 && arg_77[0] == arg_12[0] && arg_77[0] == arg_88[0] && arg_77[0] == arg_93[0] && arg_77[0] == arg_13 && arg_77[0] == arg_30[0] && arg_77[0] == arg_59[0] && arg_77[0] == arg_8 && arg_77[0] == arg_23[0] && arg_77[0] == arg_46[0] && arg_77[0] == arg_42 && arg_77[0] == arg_89 && arg_77[0] == arg_94 && arg_55 == arg_24 && arg_55 == arg_54[0] && arg_55 == arg_35[0] && arg_55 == arg_66 && arg_55 == arg_78 && arg_55 == arg_47 && arg_55 == arg_31 && arg_55 == arg_71 && arg_55 == arg_82[0] && arg_55 == arg_60 && arg_55 == arg_12[0] && arg_55 == arg_88[0] && arg_55 == arg_93[0] && arg_55 == arg_13 && arg_55 == arg_30[0] && arg_55 == arg_59[0] && arg_55 == arg_8 && arg_55 == arg_23[0] && arg_55 == arg_46[0] && arg_55 == arg_42 && arg_55 == arg_89 && arg_55 == arg_94 && arg_24 == arg_54[0] && arg_24 == arg_35[0] && arg_24 == arg_66 && arg_24 == arg_78 && arg_24 == arg_47 && arg_24 == arg_31 && arg_24 == arg_71 && arg_24 == arg_82[0] && arg_24 == arg_60 && arg_24 == arg_12[0] && arg_24 == arg_88[0] && arg_24 == arg_93[0] && arg_24 == arg_13 && arg_24 == arg_30[0] && arg_24 == arg_59[0] && arg_24 == arg_8 && arg_24 == arg_23[0] && arg_24 == arg_46[0] && arg_24 == arg_42 && arg_24 == arg_89 && arg_24 == arg_94 && arg_54[0] == arg_35[0] && arg_54[0] == arg_66 && arg_54[0] == arg_78 && arg_54[0] == arg_47 && arg_54[0] == arg_31 && arg_54[0] == arg_71 && arg_54[0] == arg_82[0] && arg_54[0] == arg_60 && arg_54[0] == arg_12[0] && arg_54[0] == arg_88[0] && arg_54[0] == arg_93[0] && arg_54[0] == arg_13 && arg_54[0] == arg_30[0] && arg_54[0] == arg_59[0] && arg_54[0] == arg_8 && arg_54[0] == arg_23[0] && arg_54[0] == arg_46[0] && arg_54[0] == arg_42 && arg_54[0] == arg_89 && arg_54[0] == arg_94 && arg_35[0] == arg_66 && arg_35[0] == arg_78 && arg_35[0] == arg_47 && arg_35[0] == arg_31 && arg_35[0] == arg_71 && arg_35[0] == arg_82[0] && arg_35[0] == arg_60 && arg_35[0] == arg_12[0] && arg_35[0] == arg_88[0] && arg_35[0] == arg_93[0] && arg_35[0] == arg_13 && arg_35[0] == arg_30[0] && arg_35[0] == arg_59[0] && arg_35[0] == arg_8 && arg_35[0] == arg_23[0] && arg_35[0] == arg_46[0] && arg_35[0] == arg_42 && arg_35[0] == arg_89 && arg_35[0] == arg_94 && arg_66 == arg_78 && arg_66 == arg_47 && arg_66 == arg_31 && arg_66 == arg_71 && arg_66 == arg_82[0] && arg_66 == arg_60 && arg_66 == arg_12[0] && arg_66 == arg_88[0] && arg_66 == arg_93[0] && arg_66 == arg_13 && arg_66 == arg_30[0] && arg_66 == arg_59[0] && arg_66 == arg_8 && arg_66 == arg_23[0] && arg_66 == arg_46[0] && arg_66 == arg_42 && arg_66 == arg_89 && arg_66 == arg_94 && arg_78 == arg_47 && arg_78 == arg_31 && arg_78 == arg_71 && arg_78 == arg_82[0] && arg_78 == arg_60 && arg_78 == arg_12[0] && arg_78 == arg_88[0] && arg_78 == arg_93[0] && arg_78 == arg_13 && arg_78 == arg_30[0] && arg_78 == arg_59[0] && arg_78 == arg_8 && arg_78 == arg_23[0] && arg_78 == arg_46[0] && arg_78 == arg_42 && arg_78 == arg_89 && arg_78 == arg_94 && arg_47 == arg_31 && arg_47 == arg_71 && arg_47 == arg_82[0] && arg_47 == arg_60 && arg_47 == arg_12[0] && arg_47 == arg_88[0] && arg_47 == arg_93[0] && arg_47 == arg_13 && arg_47 == arg_30[0] && arg_47 == arg_59[0] && arg_47 == arg_8 && arg_47 == arg_23[0] && arg_47 == arg_46[0] && arg_47 == arg_42 && arg_47 == arg_89 && arg_47 == arg_94 && arg_31 == arg_71 && arg_31 == arg_82[0] && arg_31 == arg_60 && arg_31 == arg_12[0] && arg_31 == arg_88[0] && arg_31 == arg_93[0] && arg_31 == arg_13 && arg_31 == arg_30[0] && arg_31 == arg_59[0] && arg_31 == arg_8 && arg_31 == arg_23[0] && arg_31 == arg_46[0] && arg_31 == arg_42 && arg_31 == arg_89 && arg_31 == arg_94 && arg_71 == arg_82[0] && arg_71 == arg_60 && arg_71 == arg_12[0] && arg_71 == arg_88[0] && arg_71 == arg_93[0] && arg_71 == arg_13 && arg_71 == arg_30[0] && arg_71 == arg_59[0] && arg_71 == arg_8 && arg_71 == arg_23[0] && arg_71 == arg_46[0] && arg_71 == arg_42 && arg_71 == arg_89 && arg_71 == arg_94 && arg_82[0] == arg_60 && arg_82[0] == arg_12[0] && arg_82[0] == arg_88[0] && arg_82[0] == arg_93[0] && arg_82[0] == arg_13 && arg_82[0] == arg_30[0] && arg_82[0] == arg_59[0] && arg_82[0] == arg_8 && arg_82[0] == arg_23[0] && arg_82[0] == arg_46[0] && arg_82[0] == arg_42 && arg_82[0] == arg_89 && arg_82[0] == arg_94 && arg_60 == arg_12[0] && arg_60 == arg_88[0] && arg_60 == arg_93[0] && arg_60 == arg_13 && arg_60 == arg_30[0] && arg_60 == arg_59[0] && arg_60 == arg_8 && arg_60 == arg_23[0] && arg_60 == arg_46[0] && arg_60 == arg_42 && arg_60 == arg_89 && arg_60 == arg_94 && arg_12[0] == arg_88[0] && arg_12[0] == arg_93[0] && arg_12[0] == arg_13 && arg_12[0] == arg_30[0] && arg_12[0] == arg_59[0] && arg_12[0] == arg_8 && arg_12[0] == arg_23[0] && arg_12[0] == arg_46[0] && arg_12[0] == arg_42 && arg_12[0] == arg_89 && arg_12[0] == arg_94 && arg_88[0] == arg_93[0] && arg_88[0] == arg_13 && arg_88[0] == arg_30[0] && arg_88[0] == arg_59[0] && arg_88[0] == arg_8 && arg_88[0] == arg_23[0] && arg_88[0] == arg_46[0] && arg_88[0] == arg_42 && arg_88[0] == arg_89 && arg_88[0] == arg_94 && arg_93[0] == arg_13 && arg_93[0] == arg_30[0] && arg_93[0] == arg_59[0] && arg_93[0] == arg_8 && arg_93[0] == arg_23[0] && arg_93[0] == arg_46[0] && arg_93[0] == arg_42 && arg_93[0] == arg_89 && arg_93[0] == arg_94 && arg_13 == arg_30[0] && arg_13 == arg_59[0] && arg_13 == arg_8 && arg_13 == arg_23[0] && arg_13 == arg_46[0] && arg_13 == arg_42 && arg_13 == arg_89 && arg_13 == arg_94 && arg_30[0] == arg_59[0] && arg_30[0] == arg_8 && arg_30[0] == arg_23[0] && arg_30[0] == arg_46[0] && arg_30[0] == arg_42 && arg_30[0] == arg_89 && arg_30[0] == arg_94 && arg_59[0] == arg_8 && arg_59[0] == arg_23[0] && arg_59[0] == arg_46[0] && arg_59[0] == arg_42 && arg_59[0] == arg_89 && arg_59[0] == arg_94 && arg_8 == arg_23[0] && arg_8 == arg_46[0] && arg_8 == arg_42 && arg_8 == arg_89 && arg_8 == arg_94 && arg_23[0] == arg_46[0] && arg_23[0] == arg_42 && arg_23[0] == arg_89 && arg_23[0] == arg_94 && arg_46[0] == arg_42 && arg_46[0] == arg_89 && arg_46[0] == arg_94 && arg_42 == arg_89 && arg_42 == arg_94 && arg_89 == arg_94 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
}
}
}
}
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_107(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_3[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_5) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_6 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_6[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_4 == 0 {
if rhs_3 == -1 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_8) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_8 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_9) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_10 = arg;
let rhs_9 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_10) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_11 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_11[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_12 = arg;
let rhs_10 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_10 == 0 {
if rhs_9 == -1 {
if rhs_8 == 12 {
if rhs_7 == 16 {
match pos.func.dfg.value_def(arg_3[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_13 = arg;
let rhs_15 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_13) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_14 = arg;
let rhs_16 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_14) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_15 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_15[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_16 = arg;
let rhs_17 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_16) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_17 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_17[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_18 = arg;
let rhs_18 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_18 == 0 {
if rhs_17 == -1 {
match pos.func.dfg.value_def(arg_15[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_19 = arg;
let rhs_21 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_19) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_20 = arg;
let rhs_22 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_20) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_21 = arg;
let rhs_23 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_21) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_22 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_22[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_23 = arg;
let rhs_24 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_24 == 0 {
if rhs_23 == -1 {
if rhs_22 == 12 {
if rhs_21 == 16 {
if rhs_16 == 5 {
if rhs_15 == 8 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_24 = arg;
let rhs_31 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_24) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_25 = arg;
let rhs_32 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_25) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_26 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_26[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_27 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_27[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_28 = arg;
let rhs_33 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_28) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_29 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_29[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_30 = arg;
let rhs_34 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_34 == 0 {
if rhs_33 == -1 {
match pos.func.dfg.value_def(arg_27[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_31 = arg;
let rhs_37 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_31) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_32 = arg;
let rhs_38 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_32) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_33 = arg;
let rhs_39 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_33) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_34 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_34[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_35 = arg;
let rhs_40 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_40 == 0 {
if rhs_39 == -1 {
if rhs_38 == 12 {
if rhs_37 == 16 {
match pos.func.dfg.value_def(arg_26[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_36 = arg;
let rhs_45 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_36) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_37 = arg;
let rhs_46 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_37) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_38 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_38[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_39 = arg;
let rhs_47 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_39) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_40 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_40[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_41 = arg;
let rhs_48 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_48 == 0 {
if rhs_47 == -1 {
match pos.func.dfg.value_def(arg_38[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_42 = arg;
let rhs_51 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_42) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_43 = arg;
let rhs_52 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_43) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_44 = arg;
let rhs_53 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_44) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_45 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_45[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_46 = arg;
let rhs_54 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_54 == 0 {
if rhs_53 == -1 {
if rhs_52 == 12 {
if rhs_51 == 16 {
if rhs_46 == 5 {
if rhs_45 == 8 {
if rhs_32 == 2 {
if rhs_31 == 4 {
if rhs_2 == 1 {
if rhs_1 == 2 {
if arg_22[0] == arg_11[0] && arg_22[0] == arg_34[0] && arg_22[0] == arg_35 && arg_22[0] == arg_29[0] && arg_22[0] == arg_7 && arg_22[0] == arg_18 && arg_22[0] == arg_12 && arg_22[0] == arg_41 && arg_22[0] == arg_17[0] && arg_22[0] == arg_40[0] && arg_22[0] == arg_45[0] && arg_22[0] == arg_23 && arg_22[0] == arg_46 && arg_22[0] == arg_30 && arg_22[0] == arg_6[0] && arg_11[0] == arg_34[0] && arg_11[0] == arg_35 && arg_11[0] == arg_29[0] && arg_11[0] == arg_7 && arg_11[0] == arg_18 && arg_11[0] == arg_12 && arg_11[0] == arg_41 && arg_11[0] == arg_17[0] && arg_11[0] == arg_40[0] && arg_11[0] == arg_45[0] && arg_11[0] == arg_23 && arg_11[0] == arg_46 && arg_11[0] == arg_30 && arg_11[0] == arg_6[0] && arg_34[0] == arg_35 && arg_34[0] == arg_29[0] && arg_34[0] == arg_7 && arg_34[0] == arg_18 && arg_34[0] == arg_12 && arg_34[0] == arg_41 && arg_34[0] == arg_17[0] && arg_34[0] == arg_40[0] && arg_34[0] == arg_45[0] && arg_34[0] == arg_23 && arg_34[0] == arg_46 && arg_34[0] == arg_30 && arg_34[0] == arg_6[0] && arg_35 == arg_29[0] && arg_35 == arg_7 && arg_35 == arg_18 && arg_35 == arg_12 && arg_35 == arg_41 && arg_35 == arg_17[0] && arg_35 == arg_40[0] && arg_35 == arg_45[0] && arg_35 == arg_23 && arg_35 == arg_46 && arg_35 == arg_30 && arg_35 == arg_6[0] && arg_29[0] == arg_7 && arg_29[0] == arg_18 && arg_29[0] == arg_12 && arg_29[0] == arg_41 && arg_29[0] == arg_17[0] && arg_29[0] == arg_40[0] && arg_29[0] == arg_45[0] && arg_29[0] == arg_23 && arg_29[0] == arg_46 && arg_29[0] == arg_30 && arg_29[0] == arg_6[0] && arg_7 == arg_18 && arg_7 == arg_12 && arg_7 == arg_41 && arg_7 == arg_17[0] && arg_7 == arg_40[0] && arg_7 == arg_45[0] && arg_7 == arg_23 && arg_7 == arg_46 && arg_7 == arg_30 && arg_7 == arg_6[0] && arg_18 == arg_12 && arg_18 == arg_41 && arg_18 == arg_17[0] && arg_18 == arg_40[0] && arg_18 == arg_45[0] && arg_18 == arg_23 && arg_18 == arg_46 && arg_18 == arg_30 && arg_18 == arg_6[0] && arg_12 == arg_41 && arg_12 == arg_17[0] && arg_12 == arg_40[0] && arg_12 == arg_45[0] && arg_12 == arg_23 && arg_12 == arg_46 && arg_12 == arg_30 && arg_12 == arg_6[0] && arg_41 == arg_17[0] && arg_41 == arg_40[0] && arg_41 == arg_45[0] && arg_41 == arg_23 && arg_41 == arg_46 && arg_41 == arg_30 && arg_41 == arg_6[0] && arg_17[0] == arg_40[0] && arg_17[0] == arg_45[0] && arg_17[0] == arg_23 && arg_17[0] == arg_46 && arg_17[0] == arg_30 && arg_17[0] == arg_6[0] && arg_40[0] == arg_45[0] && arg_40[0] == arg_23 && arg_40[0] == arg_46 && arg_40[0] == arg_30 && arg_40[0] == arg_6[0] && arg_45[0] == arg_23 && arg_45[0] == arg_46 && arg_45[0] == arg_30 && arg_45[0] == arg_6[0] && arg_23 == arg_46 && arg_23 == arg_30 && arg_23 == arg_6[0] && arg_46 == arg_30 && arg_46 == arg_6[0] && arg_30 == arg_6[0] {
let rhs_inst_16 = pos.ins().iconst(I32, 1073807618_u64 as i64);
let rhs_inst_17 = pos.ins().sshr(rhs_inst_16, arg_24);
let rhs_inst_18 = pos.ins().bor_imm(rhs_inst_17, 4294967293);
pos.func.dfg.replace(inst).bnot(rhs_inst_18);

}
}
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_108(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_3[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_5) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_6 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_6[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 0 {
if rhs_2 == -1 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_8) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_9) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_10 = arg;
let rhs_8 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_10) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_11 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_11[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_12 = arg;
let rhs_9 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_9 == 0 {
if rhs_8 == -1 {
if rhs_7 == 12 {
if rhs_6 == 16 {
match pos.func.dfg.value_def(arg_3[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_13 = arg;
let rhs_14 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_13) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_14 = arg;
let rhs_15 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_14) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_15 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_15[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_16 = arg;
let rhs_16 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_16) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_17 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_17[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_18 = arg;
let rhs_17 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_17 == 0 {
if rhs_16 == -1 {
match pos.func.dfg.value_def(arg_15[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_19 = arg;
let rhs_20 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_19) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_20 = arg;
let rhs_21 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_20) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_21 = arg;
let rhs_22 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_21) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_22 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_22[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_23 = arg;
let rhs_23 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_23 == 0 {
if rhs_22 == -1 {
if rhs_21 == 12 {
if rhs_20 == 16 {
if rhs_15 == 5 {
if rhs_14 == 8 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_24 = arg;
let rhs_30 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_24) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_25 = arg;
let rhs_31 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_25) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_26 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_26[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_27 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_27[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_28 = arg;
let rhs_32 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_28) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_29 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_29[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_30 = arg;
let rhs_33 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_33 == 0 {
if rhs_32 == -1 {
match pos.func.dfg.value_def(arg_27[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_31 = arg;
let rhs_36 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_31) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_32 = arg;
let rhs_37 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_32) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_33 = arg;
let rhs_38 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_33) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_34 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_34[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_35 = arg;
let rhs_39 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_39 == 0 {
if rhs_38 == -1 {
if rhs_37 == 12 {
if rhs_36 == 16 {
match pos.func.dfg.value_def(arg_26[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_36 = arg;
let rhs_44 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_36) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_37 = arg;
let rhs_45 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_37) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_38 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_38[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_39 = arg;
let rhs_46 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_39) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_40 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_40[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_41 = arg;
let rhs_47 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_47 == 0 {
if rhs_46 == -1 {
match pos.func.dfg.value_def(arg_38[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_42 = arg;
let rhs_50 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_42) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_43 = arg;
let rhs_51 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_43) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_44 = arg;
let rhs_52 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_44) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_45 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_45[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_46 = arg;
let rhs_53 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_53 == 0 {
if rhs_52 == -1 {
if rhs_51 == 12 {
if rhs_50 == 16 {
if rhs_45 == 5 {
if rhs_44 == 8 {
if rhs_31 == 2 {
if rhs_30 == 4 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_47 = arg;
let rhs_62 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_47) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_48 = arg;
let rhs_63 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_48) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_49 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_49[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_50 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_50[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_51 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_51[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_52 = arg;
let rhs_64 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_52) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_53 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_53[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_54 = arg;
let rhs_65 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_65 == 0 {
if rhs_64 == -1 {
match pos.func.dfg.value_def(arg_51[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_55 = arg;
let rhs_68 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_55) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_56 = arg;
let rhs_69 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_56) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_57 = arg;
let rhs_70 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_57) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_58 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_58[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_59 = arg;
let rhs_71 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_71 == 0 {
if rhs_70 == -1 {
if rhs_69 == 12 {
if rhs_68 == 16 {
match pos.func.dfg.value_def(arg_50[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_60 = arg;
let rhs_76 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_60) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_61 = arg;
let rhs_77 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_61) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_62 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_62[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_63 = arg;
let rhs_78 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_63) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_64 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_64[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_65 = arg;
let rhs_79 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_79 == 0 {
if rhs_78 == -1 {
match pos.func.dfg.value_def(arg_62[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_66 = arg;
let rhs_82 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_66) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_67 = arg;
let rhs_83 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_67) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_68 = arg;
let rhs_84 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_68) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_69 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_69[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_70 = arg;
let rhs_85 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_85 == 0 {
if rhs_84 == -1 {
if rhs_83 == 12 {
if rhs_82 == 16 {
if rhs_77 == 5 {
if rhs_76 == 8 {
match pos.func.dfg.value_def(arg_49[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_71 = arg;
let rhs_92 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_71) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_72 = arg;
let rhs_93 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_72) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_73 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_73[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_74 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_74[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_75 = arg;
let rhs_94 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_75) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_76 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_76[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_77 = arg;
let rhs_95 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_95 == 0 {
if rhs_94 == -1 {
match pos.func.dfg.value_def(arg_74[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_78 = arg;
let rhs_98 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_78) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_79 = arg;
let rhs_99 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_79) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_80 = arg;
let rhs_100 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_80) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_81 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_81[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_82 = arg;
let rhs_101 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_101 == 0 {
if rhs_100 == -1 {
if rhs_99 == 12 {
if rhs_98 == 16 {
match pos.func.dfg.value_def(arg_73[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_83 = arg;
let rhs_106 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_83) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_84 = arg;
let rhs_107 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_84) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_85 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_85[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_86 = arg;
let rhs_108 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_86) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_87 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_87[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_88 = arg;
let rhs_109 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_109 == 0 {
if rhs_108 == -1 {
match pos.func.dfg.value_def(arg_85[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_89 = arg;
let rhs_112 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_89) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_90 = arg;
let rhs_113 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_90) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_91 = arg;
let rhs_114 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_91) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_92 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_92[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_93 = arg;
let rhs_115 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_115 == 0 {
if rhs_114 == -1 {
if rhs_113 == 12 {
if rhs_112 == 16 {
if rhs_107 == 5 {
if rhs_106 == 8 {
if rhs_93 == 2 {
if rhs_92 == 4 {
if rhs_63 == 1 {
if rhs_62 == 2 {
if rhs_1 == 1 {
if arg_41 == arg_92[0] && arg_41 == arg_65 && arg_41 == arg_54 && arg_41 == arg_34[0] && arg_41 == arg_45[0] && arg_41 == arg_17[0] && arg_41 == arg_88 && arg_41 == arg_22[0] && arg_41 == arg_82 && arg_41 == arg_35 && arg_41 == arg_29[0] && arg_41 == arg_30 && arg_41 == arg_77 && arg_41 == arg_11[0] && arg_41 == arg_7 && arg_41 == arg_53[0] && arg_41 == arg_87[0] && arg_41 == arg_46 && arg_41 == arg_81[0] && arg_41 == arg_40[0] && arg_41 == arg_59 && arg_41 == arg_6[0] && arg_41 == arg_70 && arg_41 == arg_93 && arg_41 == arg_76[0] && arg_41 == arg_64[0] && arg_41 == arg_69[0] && arg_41 == arg_23 && arg_41 == arg_18 && arg_41 == arg_12 && arg_41 == arg_58[0] && arg_92[0] == arg_65 && arg_92[0] == arg_54 && arg_92[0] == arg_34[0] && arg_92[0] == arg_45[0] && arg_92[0] == arg_17[0] && arg_92[0] == arg_88 && arg_92[0] == arg_22[0] && arg_92[0] == arg_82 && arg_92[0] == arg_35 && arg_92[0] == arg_29[0] && arg_92[0] == arg_30 && arg_92[0] == arg_77 && arg_92[0] == arg_11[0] && arg_92[0] == arg_7 && arg_92[0] == arg_53[0] && arg_92[0] == arg_87[0] && arg_92[0] == arg_46 && arg_92[0] == arg_81[0] && arg_92[0] == arg_40[0] && arg_92[0] == arg_59 && arg_92[0] == arg_6[0] && arg_92[0] == arg_70 && arg_92[0] == arg_93 && arg_92[0] == arg_76[0] && arg_92[0] == arg_64[0] && arg_92[0] == arg_69[0] && arg_92[0] == arg_23 && arg_92[0] == arg_18 && arg_92[0] == arg_12 && arg_92[0] == arg_58[0] && arg_65 == arg_54 && arg_65 == arg_34[0] && arg_65 == arg_45[0] && arg_65 == arg_17[0] && arg_65 == arg_88 && arg_65 == arg_22[0] && arg_65 == arg_82 && arg_65 == arg_35 && arg_65 == arg_29[0] && arg_65 == arg_30 && arg_65 == arg_77 && arg_65 == arg_11[0] && arg_65 == arg_7 && arg_65 == arg_53[0] && arg_65 == arg_87[0] && arg_65 == arg_46 && arg_65 == arg_81[0] && arg_65 == arg_40[0] && arg_65 == arg_59 && arg_65 == arg_6[0] && arg_65 == arg_70 && arg_65 == arg_93 && arg_65 == arg_76[0] && arg_65 == arg_64[0] && arg_65 == arg_69[0] && arg_65 == arg_23 && arg_65 == arg_18 && arg_65 == arg_12 && arg_65 == arg_58[0] && arg_54 == arg_34[0] && arg_54 == arg_45[0] && arg_54 == arg_17[0] && arg_54 == arg_88 && arg_54 == arg_22[0] && arg_54 == arg_82 && arg_54 == arg_35 && arg_54 == arg_29[0] && arg_54 == arg_30 && arg_54 == arg_77 && arg_54 == arg_11[0] && arg_54 == arg_7 && arg_54 == arg_53[0] && arg_54 == arg_87[0] && arg_54 == arg_46 && arg_54 == arg_81[0] && arg_54 == arg_40[0] && arg_54 == arg_59 && arg_54 == arg_6[0] && arg_54 == arg_70 && arg_54 == arg_93 && arg_54 == arg_76[0] && arg_54 == arg_64[0] && arg_54 == arg_69[0] && arg_54 == arg_23 && arg_54 == arg_18 && arg_54 == arg_12 && arg_54 == arg_58[0] && arg_34[0] == arg_45[0] && arg_34[0] == arg_17[0] && arg_34[0] == arg_88 && arg_34[0] == arg_22[0] && arg_34[0] == arg_82 && arg_34[0] == arg_35 && arg_34[0] == arg_29[0] && arg_34[0] == arg_30 && arg_34[0] == arg_77 && arg_34[0] == arg_11[0] && arg_34[0] == arg_7 && arg_34[0] == arg_53[0] && arg_34[0] == arg_87[0] && arg_34[0] == arg_46 && arg_34[0] == arg_81[0] && arg_34[0] == arg_40[0] && arg_34[0] == arg_59 && arg_34[0] == arg_6[0] && arg_34[0] == arg_70 && arg_34[0] == arg_93 && arg_34[0] == arg_76[0] && arg_34[0] == arg_64[0] && arg_34[0] == arg_69[0] && arg_34[0] == arg_23 && arg_34[0] == arg_18 && arg_34[0] == arg_12 && arg_34[0] == arg_58[0] && arg_45[0] == arg_17[0] && arg_45[0] == arg_88 && arg_45[0] == arg_22[0] && arg_45[0] == arg_82 && arg_45[0] == arg_35 && arg_45[0] == arg_29[0] && arg_45[0] == arg_30 && arg_45[0] == arg_77 && arg_45[0] == arg_11[0] && arg_45[0] == arg_7 && arg_45[0] == arg_53[0] && arg_45[0] == arg_87[0] && arg_45[0] == arg_46 && arg_45[0] == arg_81[0] && arg_45[0] == arg_40[0] && arg_45[0] == arg_59 && arg_45[0] == arg_6[0] && arg_45[0] == arg_70 && arg_45[0] == arg_93 && arg_45[0] == arg_76[0] && arg_45[0] == arg_64[0] && arg_45[0] == arg_69[0] && arg_45[0] == arg_23 && arg_45[0] == arg_18 && arg_45[0] == arg_12 && arg_45[0] == arg_58[0] && arg_17[0] == arg_88 && arg_17[0] == arg_22[0] && arg_17[0] == arg_82 && arg_17[0] == arg_35 && arg_17[0] == arg_29[0] && arg_17[0] == arg_30 && arg_17[0] == arg_77 && arg_17[0] == arg_11[0] && arg_17[0] == arg_7 && arg_17[0] == arg_53[0] && arg_17[0] == arg_87[0] && arg_17[0] == arg_46 && arg_17[0] == arg_81[0] && arg_17[0] == arg_40[0] && arg_17[0] == arg_59 && arg_17[0] == arg_6[0] && arg_17[0] == arg_70 && arg_17[0] == arg_93 && arg_17[0] == arg_76[0] && arg_17[0] == arg_64[0] && arg_17[0] == arg_69[0] && arg_17[0] == arg_23 && arg_17[0] == arg_18 && arg_17[0] == arg_12 && arg_17[0] == arg_58[0] && arg_88 == arg_22[0] && arg_88 == arg_82 && arg_88 == arg_35 && arg_88 == arg_29[0] && arg_88 == arg_30 && arg_88 == arg_77 && arg_88 == arg_11[0] && arg_88 == arg_7 && arg_88 == arg_53[0] && arg_88 == arg_87[0] && arg_88 == arg_46 && arg_88 == arg_81[0] && arg_88 == arg_40[0] && arg_88 == arg_59 && arg_88 == arg_6[0] && arg_88 == arg_70 && arg_88 == arg_93 && arg_88 == arg_76[0] && arg_88 == arg_64[0] && arg_88 == arg_69[0] && arg_88 == arg_23 && arg_88 == arg_18 && arg_88 == arg_12 && arg_88 == arg_58[0] && arg_22[0] == arg_82 && arg_22[0] == arg_35 && arg_22[0] == arg_29[0] && arg_22[0] == arg_30 && arg_22[0] == arg_77 && arg_22[0] == arg_11[0] && arg_22[0] == arg_7 && arg_22[0] == arg_53[0] && arg_22[0] == arg_87[0] && arg_22[0] == arg_46 && arg_22[0] == arg_81[0] && arg_22[0] == arg_40[0] && arg_22[0] == arg_59 && arg_22[0] == arg_6[0] && arg_22[0] == arg_70 && arg_22[0] == arg_93 && arg_22[0] == arg_76[0] && arg_22[0] == arg_64[0] && arg_22[0] == arg_69[0] && arg_22[0] == arg_23 && arg_22[0] == arg_18 && arg_22[0] == arg_12 && arg_22[0] == arg_58[0] && arg_82 == arg_35 && arg_82 == arg_29[0] && arg_82 == arg_30 && arg_82 == arg_77 && arg_82 == arg_11[0] && arg_82 == arg_7 && arg_82 == arg_53[0] && arg_82 == arg_87[0] && arg_82 == arg_46 && arg_82 == arg_81[0] && arg_82 == arg_40[0] && arg_82 == arg_59 && arg_82 == arg_6[0] && arg_82 == arg_70 && arg_82 == arg_93 && arg_82 == arg_76[0] && arg_82 == arg_64[0] && arg_82 == arg_69[0] && arg_82 == arg_23 && arg_82 == arg_18 && arg_82 == arg_12 && arg_82 == arg_58[0] && arg_35 == arg_29[0] && arg_35 == arg_30 && arg_35 == arg_77 && arg_35 == arg_11[0] && arg_35 == arg_7 && arg_35 == arg_53[0] && arg_35 == arg_87[0] && arg_35 == arg_46 && arg_35 == arg_81[0] && arg_35 == arg_40[0] && arg_35 == arg_59 && arg_35 == arg_6[0] && arg_35 == arg_70 && arg_35 == arg_93 && arg_35 == arg_76[0] && arg_35 == arg_64[0] && arg_35 == arg_69[0] && arg_35 == arg_23 && arg_35 == arg_18 && arg_35 == arg_12 && arg_35 == arg_58[0] && arg_29[0] == arg_30 && arg_29[0] == arg_77 && arg_29[0] == arg_11[0] && arg_29[0] == arg_7 && arg_29[0] == arg_53[0] && arg_29[0] == arg_87[0] && arg_29[0] == arg_46 && arg_29[0] == arg_81[0] && arg_29[0] == arg_40[0] && arg_29[0] == arg_59 && arg_29[0] == arg_6[0] && arg_29[0] == arg_70 && arg_29[0] == arg_93 && arg_29[0] == arg_76[0] && arg_29[0] == arg_64[0] && arg_29[0] == arg_69[0] && arg_29[0] == arg_23 && arg_29[0] == arg_18 && arg_29[0] == arg_12 && arg_29[0] == arg_58[0] && arg_30 == arg_77 && arg_30 == arg_11[0] && arg_30 == arg_7 && arg_30 == arg_53[0] && arg_30 == arg_87[0] && arg_30 == arg_46 && arg_30 == arg_81[0] && arg_30 == arg_40[0] && arg_30 == arg_59 && arg_30 == arg_6[0] && arg_30 == arg_70 && arg_30 == arg_93 && arg_30 == arg_76[0] && arg_30 == arg_64[0] && arg_30 == arg_69[0] && arg_30 == arg_23 && arg_30 == arg_18 && arg_30 == arg_12 && arg_30 == arg_58[0] && arg_77 == arg_11[0] && arg_77 == arg_7 && arg_77 == arg_53[0] && arg_77 == arg_87[0] && arg_77 == arg_46 && arg_77 == arg_81[0] && arg_77 == arg_40[0] && arg_77 == arg_59 && arg_77 == arg_6[0] && arg_77 == arg_70 && arg_77 == arg_93 && arg_77 == arg_76[0] && arg_77 == arg_64[0] && arg_77 == arg_69[0] && arg_77 == arg_23 && arg_77 == arg_18 && arg_77 == arg_12 && arg_77 == arg_58[0] && arg_11[0] == arg_7 && arg_11[0] == arg_53[0] && arg_11[0] == arg_87[0] && arg_11[0] == arg_46 && arg_11[0] == arg_81[0] && arg_11[0] == arg_40[0] && arg_11[0] == arg_59 && arg_11[0] == arg_6[0] && arg_11[0] == arg_70 && arg_11[0] == arg_93 && arg_11[0] == arg_76[0] && arg_11[0] == arg_64[0] && arg_11[0] == arg_69[0] && arg_11[0] == arg_23 && arg_11[0] == arg_18 && arg_11[0] == arg_12 && arg_11[0] == arg_58[0] && arg_7 == arg_53[0] && arg_7 == arg_87[0] && arg_7 == arg_46 && arg_7 == arg_81[0] && arg_7 == arg_40[0] && arg_7 == arg_59 && arg_7 == arg_6[0] && arg_7 == arg_70 && arg_7 == arg_93 && arg_7 == arg_76[0] && arg_7 == arg_64[0] && arg_7 == arg_69[0] && arg_7 == arg_23 && arg_7 == arg_18 && arg_7 == arg_12 && arg_7 == arg_58[0] && arg_53[0] == arg_87[0] && arg_53[0] == arg_46 && arg_53[0] == arg_81[0] && arg_53[0] == arg_40[0] && arg_53[0] == arg_59 && arg_53[0] == arg_6[0] && arg_53[0] == arg_70 && arg_53[0] == arg_93 && arg_53[0] == arg_76[0] && arg_53[0] == arg_64[0] && arg_53[0] == arg_69[0] && arg_53[0] == arg_23 && arg_53[0] == arg_18 && arg_53[0] == arg_12 && arg_53[0] == arg_58[0] && arg_87[0] == arg_46 && arg_87[0] == arg_81[0] && arg_87[0] == arg_40[0] && arg_87[0] == arg_59 && arg_87[0] == arg_6[0] && arg_87[0] == arg_70 && arg_87[0] == arg_93 && arg_87[0] == arg_76[0] && arg_87[0] == arg_64[0] && arg_87[0] == arg_69[0] && arg_87[0] == arg_23 && arg_87[0] == arg_18 && arg_87[0] == arg_12 && arg_87[0] == arg_58[0] && arg_46 == arg_81[0] && arg_46 == arg_40[0] && arg_46 == arg_59 && arg_46 == arg_6[0] && arg_46 == arg_70 && arg_46 == arg_93 && arg_46 == arg_76[0] && arg_46 == arg_64[0] && arg_46 == arg_69[0] && arg_46 == arg_23 && arg_46 == arg_18 && arg_46 == arg_12 && arg_46 == arg_58[0] && arg_81[0] == arg_40[0] && arg_81[0] == arg_59 && arg_81[0] == arg_6[0] && arg_81[0] == arg_70 && arg_81[0] == arg_93 && arg_81[0] == arg_76[0] && arg_81[0] == arg_64[0] && arg_81[0] == arg_69[0] && arg_81[0] == arg_23 && arg_81[0] == arg_18 && arg_81[0] == arg_12 && arg_81[0] == arg_58[0] && arg_40[0] == arg_59 && arg_40[0] == arg_6[0] && arg_40[0] == arg_70 && arg_40[0] == arg_93 && arg_40[0] == arg_76[0] && arg_40[0] == arg_64[0] && arg_40[0] == arg_69[0] && arg_40[0] == arg_23 && arg_40[0] == arg_18 && arg_40[0] == arg_12 && arg_40[0] == arg_58[0] && arg_59 == arg_6[0] && arg_59 == arg_70 && arg_59 == arg_93 && arg_59 == arg_76[0] && arg_59 == arg_64[0] && arg_59 == arg_69[0] && arg_59 == arg_23 && arg_59 == arg_18 && arg_59 == arg_12 && arg_59 == arg_58[0] && arg_6[0] == arg_70 && arg_6[0] == arg_93 && arg_6[0] == arg_76[0] && arg_6[0] == arg_64[0] && arg_6[0] == arg_69[0] && arg_6[0] == arg_23 && arg_6[0] == arg_18 && arg_6[0] == arg_12 && arg_6[0] == arg_58[0] && arg_70 == arg_93 && arg_70 == arg_76[0] && arg_70 == arg_64[0] && arg_70 == arg_69[0] && arg_70 == arg_23 && arg_70 == arg_18 && arg_70 == arg_12 && arg_70 == arg_58[0] && arg_93 == arg_76[0] && arg_93 == arg_64[0] && arg_93 == arg_69[0] && arg_93 == arg_23 && arg_93 == arg_18 && arg_93 == arg_12 && arg_93 == arg_58[0] && arg_76[0] == arg_64[0] && arg_76[0] == arg_69[0] && arg_76[0] == arg_23 && arg_76[0] == arg_18 && arg_76[0] == arg_12 && arg_76[0] == arg_58[0] && arg_64[0] == arg_69[0] && arg_64[0] == arg_23 && arg_64[0] == arg_18 && arg_64[0] == arg_12 && arg_64[0] == arg_58[0] && arg_69[0] == arg_23 && arg_69[0] == arg_18 && arg_69[0] == arg_12 && arg_69[0] == arg_58[0] && arg_23 == arg_18 && arg_23 == arg_12 && arg_23 == arg_58[0] && arg_18 == arg_12 && arg_18 == arg_58[0] && arg_12 == arg_58[0] {
let rhs_inst_18 = pos.ins().iconst(I32, 3221225607_u64 as i64);
let rhs_inst_19 = pos.ins().sshr(rhs_inst_18, arg_48);
let rhs_inst_20 = pos.ins().bor_imm(rhs_inst_19, 1073741822);
pos.func.dfg.replace(inst).bnot(rhs_inst_20);

}
}
}
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_109(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_3[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_5) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_6 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_6[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_7 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == -1 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_8 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_8) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_9 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_9) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_10 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_10) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_11 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_11[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_12 = arg;
let rhs_8 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_8 == 0 {
if rhs_7 == -1 {
if rhs_6 == 12 {
if rhs_5 == 16 {
match pos.func.dfg.value_def(arg_3[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_13 = arg;
let rhs_13 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_13) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_14 = arg;
let rhs_14 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_14) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_15 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_15[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_16 = arg;
let rhs_15 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_16) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_17 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_17[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_18 = arg;
let rhs_16 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_16 == 0 {
if rhs_15 == -1 {
match pos.func.dfg.value_def(arg_15[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_19 = arg;
let rhs_19 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_19) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_20 = arg;
let rhs_20 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_20) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_21 = arg;
let rhs_21 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_21) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_22 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_22[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_23 = arg;
let rhs_22 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_22 == 0 {
if rhs_21 == -1 {
if rhs_20 == 12 {
if rhs_19 == 16 {
if rhs_14 == 5 {
if rhs_13 == 8 {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_24 = arg;
let rhs_29 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_24) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_25 = arg;
let rhs_30 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_25) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_26 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_26[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_27 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_27[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_28 = arg;
let rhs_31 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_28) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_29 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_29[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_30 = arg;
let rhs_32 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_32 == 0 {
if rhs_31 == -1 {
match pos.func.dfg.value_def(arg_27[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_31 = arg;
let rhs_35 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_31) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_32 = arg;
let rhs_36 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_32) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_33 = arg;
let rhs_37 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_33) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_34 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_34[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_35 = arg;
let rhs_38 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_38 == 0 {
if rhs_37 == -1 {
if rhs_36 == 12 {
if rhs_35 == 16 {
match pos.func.dfg.value_def(arg_26[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_36 = arg;
let rhs_43 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_36) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_37 = arg;
let rhs_44 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_37) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_38 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_38[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_39 = arg;
let rhs_45 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_39) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_40 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_40[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_41 = arg;
let rhs_46 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_46 == 0 {
if rhs_45 == -1 {
match pos.func.dfg.value_def(arg_38[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_42 = arg;
let rhs_49 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_42) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_43 = arg;
let rhs_50 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_43) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_44 = arg;
let rhs_51 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_44) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_45 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_45[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_46 = arg;
let rhs_52 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_52 == 0 {
if rhs_51 == -1 {
if rhs_50 == 12 {
if rhs_49 == 16 {
if rhs_44 == 5 {
if rhs_43 == 8 {
if rhs_30 == 2 {
if rhs_29 == 4 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_47 = arg;
let rhs_61 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_47) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_48 = arg;
let rhs_62 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_48) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_49 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_49[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_50 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_50[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_51 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_51[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_52 = arg;
let rhs_63 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_52) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_53 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_53[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_54 = arg;
let rhs_64 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_64 == 0 {
if rhs_63 == -1 {
match pos.func.dfg.value_def(arg_51[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_55 = arg;
let rhs_67 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_55) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_56 = arg;
let rhs_68 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_56) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_57 = arg;
let rhs_69 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_57) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_58 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_58[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_59 = arg;
let rhs_70 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_70 == 0 {
if rhs_69 == -1 {
if rhs_68 == 12 {
if rhs_67 == 16 {
match pos.func.dfg.value_def(arg_50[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_60 = arg;
let rhs_75 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_60) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_61 = arg;
let rhs_76 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_61) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_62 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_62[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_63 = arg;
let rhs_77 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_63) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_64 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_64[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_65 = arg;
let rhs_78 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_78 == 0 {
if rhs_77 == -1 {
match pos.func.dfg.value_def(arg_62[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_66 = arg;
let rhs_81 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_66) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_67 = arg;
let rhs_82 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_67) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_68 = arg;
let rhs_83 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_68) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_69 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_69[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_70 = arg;
let rhs_84 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_84 == 0 {
if rhs_83 == -1 {
if rhs_82 == 12 {
if rhs_81 == 16 {
if rhs_76 == 5 {
if rhs_75 == 8 {
match pos.func.dfg.value_def(arg_49[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_71 = arg;
let rhs_91 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_71) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_72 = arg;
let rhs_92 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_72) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_73 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_73[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_74 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_74[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_75 = arg;
let rhs_93 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_75) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_76 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_76[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_77 = arg;
let rhs_94 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_94 == 0 {
if rhs_93 == -1 {
match pos.func.dfg.value_def(arg_74[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_78 = arg;
let rhs_97 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_78) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_79 = arg;
let rhs_98 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_79) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_80 = arg;
let rhs_99 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_80) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_81 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_81[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_82 = arg;
let rhs_100 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_100 == 0 {
if rhs_99 == -1 {
if rhs_98 == 12 {
if rhs_97 == 16 {
match pos.func.dfg.value_def(arg_73[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_83 = arg;
let rhs_105 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_83) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_84 = arg;
let rhs_106 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_84) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_85 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_85[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_86 = arg;
let rhs_107 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_86) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_87 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_87[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_88 = arg;
let rhs_108 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_108 == 0 {
if rhs_107 == -1 {
match pos.func.dfg.value_def(arg_85[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_89 = arg;
let rhs_111 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_89) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_90 = arg;
let rhs_112 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_90) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_91 = arg;
let rhs_113 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_91) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_92 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_92[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_93 = arg;
let rhs_114 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_114 == 0 {
if rhs_113 == -1 {
if rhs_112 == 12 {
if rhs_111 == 16 {
if rhs_106 == 5 {
if rhs_105 == 8 {
if rhs_92 == 2 {
if rhs_91 == 4 {
if rhs_62 == 1 {
if rhs_61 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_94 = arg;
let rhs_125 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_94) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_95 = arg;
let rhs_126 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_95) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_96 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_96[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_97 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_97[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_98 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_98[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_99 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_99[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_100 = arg;
let rhs_127 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_100) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_101 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_101[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_102 = arg;
let rhs_128 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_128 == 0 {
if rhs_127 == -1 {
match pos.func.dfg.value_def(arg_99[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_103 = arg;
let rhs_131 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_103) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_104 = arg;
let rhs_132 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_104) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_105 = arg;
let rhs_133 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_105) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_106 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_106[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_107 = arg;
let rhs_134 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_134 == 0 {
if rhs_133 == -1 {
if rhs_132 == 12 {
if rhs_131 == 16 {
match pos.func.dfg.value_def(arg_98[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_108 = arg;
let rhs_139 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_108) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_109 = arg;
let rhs_140 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_109) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_110 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_110[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_111 = arg;
let rhs_141 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_111) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_112 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_112[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_113 = arg;
let rhs_142 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_142 == 0 {
if rhs_141 == -1 {
match pos.func.dfg.value_def(arg_110[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_114 = arg;
let rhs_145 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_114) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_115 = arg;
let rhs_146 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_115) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_116 = arg;
let rhs_147 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_116) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_117 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_117[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_118 = arg;
let rhs_148 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_148 == 0 {
if rhs_147 == -1 {
if rhs_146 == 12 {
if rhs_145 == 16 {
if rhs_140 == 5 {
if rhs_139 == 8 {
match pos.func.dfg.value_def(arg_97[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_119 = arg;
let rhs_155 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_119) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_120 = arg;
let rhs_156 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_120) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_121 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_121[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_122 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_122[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_123 = arg;
let rhs_157 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_123) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_124 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_124[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_125 = arg;
let rhs_158 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_158 == 0 {
if rhs_157 == -1 {
match pos.func.dfg.value_def(arg_122[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_126 = arg;
let rhs_161 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_126) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_127 = arg;
let rhs_162 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_127) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_128 = arg;
let rhs_163 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_128) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_129 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_129[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_130 = arg;
let rhs_164 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_164 == 0 {
if rhs_163 == -1 {
if rhs_162 == 12 {
if rhs_161 == 16 {
match pos.func.dfg.value_def(arg_121[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_131 = arg;
let rhs_169 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_131) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_132 = arg;
let rhs_170 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_132) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_133 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_133[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_134 = arg;
let rhs_171 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_134) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_135 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_135[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_136 = arg;
let rhs_172 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_172 == 0 {
if rhs_171 == -1 {
match pos.func.dfg.value_def(arg_133[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_137 = arg;
let rhs_175 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_137) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_138 = arg;
let rhs_176 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_138) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_139 = arg;
let rhs_177 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_139) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_140 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_140[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_141 = arg;
let rhs_178 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_178 == 0 {
if rhs_177 == -1 {
if rhs_176 == 12 {
if rhs_175 == 16 {
if rhs_170 == 5 {
if rhs_169 == 8 {
if rhs_156 == 2 {
if rhs_155 == 4 {
match pos.func.dfg.value_def(arg_96[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_142 = arg;
let rhs_187 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_142) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_143 = arg;
let rhs_188 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_143) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_144 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_144[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_145 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_145[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_146 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_146[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_147 = arg;
let rhs_189 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_147) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_148 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_148[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_149 = arg;
let rhs_190 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_190 == 0 {
if rhs_189 == -1 {
match pos.func.dfg.value_def(arg_146[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_150 = arg;
let rhs_193 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_150) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_151 = arg;
let rhs_194 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_151) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_152 = arg;
let rhs_195 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_152) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_153 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_153[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_154 = arg;
let rhs_196 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_196 == 0 {
if rhs_195 == -1 {
if rhs_194 == 12 {
if rhs_193 == 16 {
match pos.func.dfg.value_def(arg_145[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_155 = arg;
let rhs_201 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_155) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_156 = arg;
let rhs_202 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_156) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_157 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_157[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_158 = arg;
let rhs_203 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_158) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_159 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_159[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_160 = arg;
let rhs_204 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_204 == 0 {
if rhs_203 == -1 {
match pos.func.dfg.value_def(arg_157[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_161 = arg;
let rhs_207 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_161) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_162 = arg;
let rhs_208 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_162) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_163 = arg;
let rhs_209 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_163) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_164 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_164[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_165 = arg;
let rhs_210 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_210 == 0 {
if rhs_209 == -1 {
if rhs_208 == 12 {
if rhs_207 == 16 {
if rhs_202 == 5 {
if rhs_201 == 8 {
match pos.func.dfg.value_def(arg_144[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_166 = arg;
let rhs_217 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_166) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_167 = arg;
let rhs_218 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_167) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_168 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_168[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_169 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_169[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_170 = arg;
let rhs_219 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_170) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_171 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_171[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_172 = arg;
let rhs_220 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_220 == 0 {
if rhs_219 == -1 {
match pos.func.dfg.value_def(arg_169[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_173 = arg;
let rhs_223 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_173) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_174 = arg;
let rhs_224 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_174) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_175 = arg;
let rhs_225 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_175) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_176 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_176[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_177 = arg;
let rhs_226 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_226 == 0 {
if rhs_225 == -1 {
if rhs_224 == 12 {
if rhs_223 == 16 {
match pos.func.dfg.value_def(arg_168[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_178 = arg;
let rhs_231 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_178) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_179 = arg;
let rhs_232 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_179) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_180 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_180[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_181 = arg;
let rhs_233 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_181) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_182 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_182[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_183 = arg;
let rhs_234 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_234 == 0 {
if rhs_233 == -1 {
match pos.func.dfg.value_def(arg_180[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_184 = arg;
let rhs_237 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_184) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_185 = arg;
let rhs_238 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_185) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_186 = arg;
let rhs_239 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_186) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_187 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_187[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_188 = arg;
let rhs_240 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_240 == 0 {
if rhs_239 == -1 {
if rhs_238 == 12 {
if rhs_237 == 16 {
if rhs_232 == 5 {
if rhs_231 == 8 {
if rhs_218 == 2 {
if rhs_217 == 4 {
if rhs_188 == 1 {
if rhs_187 == 2 {
if rhs_126 == 1 {
if rhs_125 == 1 {
if arg_153[0] == arg_160 && arg_153[0] == arg_23 && arg_153[0] == arg_124[0] && arg_153[0] == arg_188 && arg_153[0] == arg_107 && arg_153[0] == arg_7 && arg_153[0] == arg_87[0] && arg_153[0] == arg_34[0] && arg_153[0] == arg_53[0] && arg_153[0] == arg_30 && arg_153[0] == arg_141 && arg_153[0] == arg_11[0] && arg_153[0] == arg_65 && arg_153[0] == arg_172 && arg_153[0] == arg_69[0] && arg_153[0] == arg_76[0] && arg_153[0] == arg_77 && arg_153[0] == arg_41 && arg_153[0] == arg_93 && arg_153[0] == arg_183 && arg_153[0] == arg_54 && arg_153[0] == arg_113 && arg_153[0] == arg_17[0] && arg_153[0] == arg_45[0] && arg_153[0] == arg_187[0] && arg_153[0] == arg_82 && arg_153[0] == arg_136 && arg_153[0] == arg_12 && arg_153[0] == arg_129[0] && arg_153[0] == arg_165 && arg_153[0] == arg_102 && arg_153[0] == arg_176[0] && arg_153[0] == arg_81[0] && arg_153[0] == arg_140[0] && arg_153[0] == arg_149 && arg_153[0] == arg_59 && arg_153[0] == arg_70 && arg_153[0] == arg_88 && arg_153[0] == arg_35 && arg_153[0] == arg_46 && arg_153[0] == arg_148[0] && arg_153[0] == arg_159[0] && arg_153[0] == arg_29[0] && arg_153[0] == arg_118 && arg_153[0] == arg_130 && arg_153[0] == arg_58[0] && arg_153[0] == arg_135[0] && arg_153[0] == arg_6[0] && arg_153[0] == arg_112[0] && arg_153[0] == arg_164[0] && arg_153[0] == arg_125 && arg_153[0] == arg_92[0] && arg_153[0] == arg_171[0] && arg_153[0] == arg_22[0] && arg_153[0] == arg_18 && arg_153[0] == arg_154 && arg_153[0] == arg_101[0] && arg_153[0] == arg_177 && arg_153[0] == arg_64[0] && arg_153[0] == arg_106[0] && arg_153[0] == arg_40[0] && arg_153[0] == arg_182[0] && arg_153[0] == arg_117[0] && arg_160 == arg_23 && arg_160 == arg_124[0] && arg_160 == arg_188 && arg_160 == arg_107 && arg_160 == arg_7 && arg_160 == arg_87[0] && arg_160 == arg_34[0] && arg_160 == arg_53[0] && arg_160 == arg_30 && arg_160 == arg_141 && arg_160 == arg_11[0] && arg_160 == arg_65 && arg_160 == arg_172 && arg_160 == arg_69[0] && arg_160 == arg_76[0] && arg_160 == arg_77 && arg_160 == arg_41 && arg_160 == arg_93 && arg_160 == arg_183 && arg_160 == arg_54 && arg_160 == arg_113 && arg_160 == arg_17[0] && arg_160 == arg_45[0] && arg_160 == arg_187[0] && arg_160 == arg_82 && arg_160 == arg_136 && arg_160 == arg_12 && arg_160 == arg_129[0] && arg_160 == arg_165 && arg_160 == arg_102 && arg_160 == arg_176[0] && arg_160 == arg_81[0] && arg_160 == arg_140[0] && arg_160 == arg_149 && arg_160 == arg_59 && arg_160 == arg_70 && arg_160 == arg_88 && arg_160 == arg_35 && arg_160 == arg_46 && arg_160 == arg_148[0] && arg_160 == arg_159[0] && arg_160 == arg_29[0] && arg_160 == arg_118 && arg_160 == arg_130 && arg_160 == arg_58[0] && arg_160 == arg_135[0] && arg_160 == arg_6[0] && arg_160 == arg_112[0] && arg_160 == arg_164[0] && arg_160 == arg_125 && arg_160 == arg_92[0] && arg_160 == arg_171[0] && arg_160 == arg_22[0] && arg_160 == arg_18 && arg_160 == arg_154 && arg_160 == arg_101[0] && arg_160 == arg_177 && arg_160 == arg_64[0] && arg_160 == arg_106[0] && arg_160 == arg_40[0] && arg_160 == arg_182[0] && arg_160 == arg_117[0] && arg_23 == arg_124[0] && arg_23 == arg_188 && arg_23 == arg_107 && arg_23 == arg_7 && arg_23 == arg_87[0] && arg_23 == arg_34[0] && arg_23 == arg_53[0] && arg_23 == arg_30 && arg_23 == arg_141 && arg_23 == arg_11[0] && arg_23 == arg_65 && arg_23 == arg_172 && arg_23 == arg_69[0] && arg_23 == arg_76[0] && arg_23 == arg_77 && arg_23 == arg_41 && arg_23 == arg_93 && arg_23 == arg_183 && arg_23 == arg_54 && arg_23 == arg_113 && arg_23 == arg_17[0] && arg_23 == arg_45[0] && arg_23 == arg_187[0] && arg_23 == arg_82 && arg_23 == arg_136 && arg_23 == arg_12 && arg_23 == arg_129[0] && arg_23 == arg_165 && arg_23 == arg_102 && arg_23 == arg_176[0] && arg_23 == arg_81[0] && arg_23 == arg_140[0] && arg_23 == arg_149 && arg_23 == arg_59 && arg_23 == arg_70 && arg_23 == arg_88 && arg_23 == arg_35 && arg_23 == arg_46 && arg_23 == arg_148[0] && arg_23 == arg_159[0] && arg_23 == arg_29[0] && arg_23 == arg_118 && arg_23 == arg_130 && arg_23 == arg_58[0] && arg_23 == arg_135[0] && arg_23 == arg_6[0] && arg_23 == arg_112[0] && arg_23 == arg_164[0] && arg_23 == arg_125 && arg_23 == arg_92[0] && arg_23 == arg_171[0] && arg_23 == arg_22[0] && arg_23 == arg_18 && arg_23 == arg_154 && arg_23 == arg_101[0] && arg_23 == arg_177 && arg_23 == arg_64[0] && arg_23 == arg_106[0] && arg_23 == arg_40[0] && arg_23 == arg_182[0] && arg_23 == arg_117[0] && arg_124[0] == arg_188 && arg_124[0] == arg_107 && arg_124[0] == arg_7 && arg_124[0] == arg_87[0] && arg_124[0] == arg_34[0] && arg_124[0] == arg_53[0] && arg_124[0] == arg_30 && arg_124[0] == arg_141 && arg_124[0] == arg_11[0] && arg_124[0] == arg_65 && arg_124[0] == arg_172 && arg_124[0] == arg_69[0] && arg_124[0] == arg_76[0] && arg_124[0] == arg_77 && arg_124[0] == arg_41 && arg_124[0] == arg_93 && arg_124[0] == arg_183 && arg_124[0] == arg_54 && arg_124[0] == arg_113 && arg_124[0] == arg_17[0] && arg_124[0] == arg_45[0] && arg_124[0] == arg_187[0] && arg_124[0] == arg_82 && arg_124[0] == arg_136 && arg_124[0] == arg_12 && arg_124[0] == arg_129[0] && arg_124[0] == arg_165 && arg_124[0] == arg_102 && arg_124[0] == arg_176[0] && arg_124[0] == arg_81[0] && arg_124[0] == arg_140[0] && arg_124[0] == arg_149 && arg_124[0] == arg_59 && arg_124[0] == arg_70 && arg_124[0] == arg_88 && arg_124[0] == arg_35 && arg_124[0] == arg_46 && arg_124[0] == arg_148[0] && arg_124[0] == arg_159[0] && arg_124[0] == arg_29[0] && arg_124[0] == arg_118 && arg_124[0] == arg_130 && arg_124[0] == arg_58[0] && arg_124[0] == arg_135[0] && arg_124[0] == arg_6[0] && arg_124[0] == arg_112[0] && arg_124[0] == arg_164[0] && arg_124[0] == arg_125 && arg_124[0] == arg_92[0] && arg_124[0] == arg_171[0] && arg_124[0] == arg_22[0] && arg_124[0] == arg_18 && arg_124[0] == arg_154 && arg_124[0] == arg_101[0] && arg_124[0] == arg_177 && arg_124[0] == arg_64[0] && arg_124[0] == arg_106[0] && arg_124[0] == arg_40[0] && arg_124[0] == arg_182[0] && arg_124[0] == arg_117[0] && arg_188 == arg_107 && arg_188 == arg_7 && arg_188 == arg_87[0] && arg_188 == arg_34[0] && arg_188 == arg_53[0] && arg_188 == arg_30 && arg_188 == arg_141 && arg_188 == arg_11[0] && arg_188 == arg_65 && arg_188 == arg_172 && arg_188 == arg_69[0] && arg_188 == arg_76[0] && arg_188 == arg_77 && arg_188 == arg_41 && arg_188 == arg_93 && arg_188 == arg_183 && arg_188 == arg_54 && arg_188 == arg_113 && arg_188 == arg_17[0] && arg_188 == arg_45[0] && arg_188 == arg_187[0] && arg_188 == arg_82 && arg_188 == arg_136 && arg_188 == arg_12 && arg_188 == arg_129[0] && arg_188 == arg_165 && arg_188 == arg_102 && arg_188 == arg_176[0] && arg_188 == arg_81[0] && arg_188 == arg_140[0] && arg_188 == arg_149 && arg_188 == arg_59 && arg_188 == arg_70 && arg_188 == arg_88 && arg_188 == arg_35 && arg_188 == arg_46 && arg_188 == arg_148[0] && arg_188 == arg_159[0] && arg_188 == arg_29[0] && arg_188 == arg_118 && arg_188 == arg_130 && arg_188 == arg_58[0] && arg_188 == arg_135[0] && arg_188 == arg_6[0] && arg_188 == arg_112[0] && arg_188 == arg_164[0] && arg_188 == arg_125 && arg_188 == arg_92[0] && arg_188 == arg_171[0] && arg_188 == arg_22[0] && arg_188 == arg_18 && arg_188 == arg_154 && arg_188 == arg_101[0] && arg_188 == arg_177 && arg_188 == arg_64[0] && arg_188 == arg_106[0] && arg_188 == arg_40[0] && arg_188 == arg_182[0] && arg_188 == arg_117[0] && arg_107 == arg_7 && arg_107 == arg_87[0] && arg_107 == arg_34[0] && arg_107 == arg_53[0] && arg_107 == arg_30 && arg_107 == arg_141 && arg_107 == arg_11[0] && arg_107 == arg_65 && arg_107 == arg_172 && arg_107 == arg_69[0] && arg_107 == arg_76[0] && arg_107 == arg_77 && arg_107 == arg_41 && arg_107 == arg_93 && arg_107 == arg_183 && arg_107 == arg_54 && arg_107 == arg_113 && arg_107 == arg_17[0] && arg_107 == arg_45[0] && arg_107 == arg_187[0] && arg_107 == arg_82 && arg_107 == arg_136 && arg_107 == arg_12 && arg_107 == arg_129[0] && arg_107 == arg_165 && arg_107 == arg_102 && arg_107 == arg_176[0] && arg_107 == arg_81[0] && arg_107 == arg_140[0] && arg_107 == arg_149 && arg_107 == arg_59 && arg_107 == arg_70 && arg_107 == arg_88 && arg_107 == arg_35 && arg_107 == arg_46 && arg_107 == arg_148[0] && arg_107 == arg_159[0] && arg_107 == arg_29[0] && arg_107 == arg_118 && arg_107 == arg_130 && arg_107 == arg_58[0] && arg_107 == arg_135[0] && arg_107 == arg_6[0] && arg_107 == arg_112[0] && arg_107 == arg_164[0] && arg_107 == arg_125 && arg_107 == arg_92[0] && arg_107 == arg_171[0] && arg_107 == arg_22[0] && arg_107 == arg_18 && arg_107 == arg_154 && arg_107 == arg_101[0] && arg_107 == arg_177 && arg_107 == arg_64[0] && arg_107 == arg_106[0] && arg_107 == arg_40[0] && arg_107 == arg_182[0] && arg_107 == arg_117[0] && arg_7 == arg_87[0] && arg_7 == arg_34[0] && arg_7 == arg_53[0] && arg_7 == arg_30 && arg_7 == arg_141 && arg_7 == arg_11[0] && arg_7 == arg_65 && arg_7 == arg_172 && arg_7 == arg_69[0] && arg_7 == arg_76[0] && arg_7 == arg_77 && arg_7 == arg_41 && arg_7 == arg_93 && arg_7 == arg_183 && arg_7 == arg_54 && arg_7 == arg_113 && arg_7 == arg_17[0] && arg_7 == arg_45[0] && arg_7 == arg_187[0] && arg_7 == arg_82 && arg_7 == arg_136 && arg_7 == arg_12 && arg_7 == arg_129[0] && arg_7 == arg_165 && arg_7 == arg_102 && arg_7 == arg_176[0] && arg_7 == arg_81[0] && arg_7 == arg_140[0] && arg_7 == arg_149 && arg_7 == arg_59 && arg_7 == arg_70 && arg_7 == arg_88 && arg_7 == arg_35 && arg_7 == arg_46 && arg_7 == arg_148[0] && arg_7 == arg_159[0] && arg_7 == arg_29[0] && arg_7 == arg_118 && arg_7 == arg_130 && arg_7 == arg_58[0] && arg_7 == arg_135[0] && arg_7 == arg_6[0] && arg_7 == arg_112[0] && arg_7 == arg_164[0] && arg_7 == arg_125 && arg_7 == arg_92[0] && arg_7 == arg_171[0] && arg_7 == arg_22[0] && arg_7 == arg_18 && arg_7 == arg_154 && arg_7 == arg_101[0] && arg_7 == arg_177 && arg_7 == arg_64[0] && arg_7 == arg_106[0] && arg_7 == arg_40[0] && arg_7 == arg_182[0] && arg_7 == arg_117[0] && arg_87[0] == arg_34[0] && arg_87[0] == arg_53[0] && arg_87[0] == arg_30 && arg_87[0] == arg_141 && arg_87[0] == arg_11[0] && arg_87[0] == arg_65 && arg_87[0] == arg_172 && arg_87[0] == arg_69[0] && arg_87[0] == arg_76[0] && arg_87[0] == arg_77 && arg_87[0] == arg_41 && arg_87[0] == arg_93 && arg_87[0] == arg_183 && arg_87[0] == arg_54 && arg_87[0] == arg_113 && arg_87[0] == arg_17[0] && arg_87[0] == arg_45[0] && arg_87[0] == arg_187[0] && arg_87[0] == arg_82 && arg_87[0] == arg_136 && arg_87[0] == arg_12 && arg_87[0] == arg_129[0] && arg_87[0] == arg_165 && arg_87[0] == arg_102 && arg_87[0] == arg_176[0] && arg_87[0] == arg_81[0] && arg_87[0] == arg_140[0] && arg_87[0] == arg_149 && arg_87[0] == arg_59 && arg_87[0] == arg_70 && arg_87[0] == arg_88 && arg_87[0] == arg_35 && arg_87[0] == arg_46 && arg_87[0] == arg_148[0] && arg_87[0] == arg_159[0] && arg_87[0] == arg_29[0] && arg_87[0] == arg_118 && arg_87[0] == arg_130 && arg_87[0] == arg_58[0] && arg_87[0] == arg_135[0] && arg_87[0] == arg_6[0] && arg_87[0] == arg_112[0] && arg_87[0] == arg_164[0] && arg_87[0] == arg_125 && arg_87[0] == arg_92[0] && arg_87[0] == arg_171[0] && arg_87[0] == arg_22[0] && arg_87[0] == arg_18 && arg_87[0] == arg_154 && arg_87[0] == arg_101[0] && arg_87[0] == arg_177 && arg_87[0] == arg_64[0] && arg_87[0] == arg_106[0] && arg_87[0] == arg_40[0] && arg_87[0] == arg_182[0] && arg_87[0] == arg_117[0] && arg_34[0] == arg_53[0] && arg_34[0] == arg_30 && arg_34[0] == arg_141 && arg_34[0] == arg_11[0] && arg_34[0] == arg_65 && arg_34[0] == arg_172 && arg_34[0] == arg_69[0] && arg_34[0] == arg_76[0] && arg_34[0] == arg_77 && arg_34[0] == arg_41 && arg_34[0] == arg_93 && arg_34[0] == arg_183 && arg_34[0] == arg_54 && arg_34[0] == arg_113 && arg_34[0] == arg_17[0] && arg_34[0] == arg_45[0] && arg_34[0] == arg_187[0] && arg_34[0] == arg_82 && arg_34[0] == arg_136 && arg_34[0] == arg_12 && arg_34[0] == arg_129[0] && arg_34[0] == arg_165 && arg_34[0] == arg_102 && arg_34[0] == arg_176[0] && arg_34[0] == arg_81[0] && arg_34[0] == arg_140[0] && arg_34[0] == arg_149 && arg_34[0] == arg_59 && arg_34[0] == arg_70 && arg_34[0] == arg_88 && arg_34[0] == arg_35 && arg_34[0] == arg_46 && arg_34[0] == arg_148[0] && arg_34[0] == arg_159[0] && arg_34[0] == arg_29[0] && arg_34[0] == arg_118 && arg_34[0] == arg_130 && arg_34[0] == arg_58[0] && arg_34[0] == arg_135[0] && arg_34[0] == arg_6[0] && arg_34[0] == arg_112[0] && arg_34[0] == arg_164[0] && arg_34[0] == arg_125 && arg_34[0] == arg_92[0] && arg_34[0] == arg_171[0] && arg_34[0] == arg_22[0] && arg_34[0] == arg_18 && arg_34[0] == arg_154 && arg_34[0] == arg_101[0] && arg_34[0] == arg_177 && arg_34[0] == arg_64[0] && arg_34[0] == arg_106[0] && arg_34[0] == arg_40[0] && arg_34[0] == arg_182[0] && arg_34[0] == arg_117[0] && arg_53[0] == arg_30 && arg_53[0] == arg_141 && arg_53[0] == arg_11[0] && arg_53[0] == arg_65 && arg_53[0] == arg_172 && arg_53[0] == arg_69[0] && arg_53[0] == arg_76[0] && arg_53[0] == arg_77 && arg_53[0] == arg_41 && arg_53[0] == arg_93 && arg_53[0] == arg_183 && arg_53[0] == arg_54 && arg_53[0] == arg_113 && arg_53[0] == arg_17[0] && arg_53[0] == arg_45[0] && arg_53[0] == arg_187[0] && arg_53[0] == arg_82 && arg_53[0] == arg_136 && arg_53[0] == arg_12 && arg_53[0] == arg_129[0] && arg_53[0] == arg_165 && arg_53[0] == arg_102 && arg_53[0] == arg_176[0] && arg_53[0] == arg_81[0] && arg_53[0] == arg_140[0] && arg_53[0] == arg_149 && arg_53[0] == arg_59 && arg_53[0] == arg_70 && arg_53[0] == arg_88 && arg_53[0] == arg_35 && arg_53[0] == arg_46 && arg_53[0] == arg_148[0] && arg_53[0] == arg_159[0] && arg_53[0] == arg_29[0] && arg_53[0] == arg_118 && arg_53[0] == arg_130 && arg_53[0] == arg_58[0] && arg_53[0] == arg_135[0] && arg_53[0] == arg_6[0] && arg_53[0] == arg_112[0] && arg_53[0] == arg_164[0] && arg_53[0] == arg_125 && arg_53[0] == arg_92[0] && arg_53[0] == arg_171[0] && arg_53[0] == arg_22[0] && arg_53[0] == arg_18 && arg_53[0] == arg_154 && arg_53[0] == arg_101[0] && arg_53[0] == arg_177 && arg_53[0] == arg_64[0] && arg_53[0] == arg_106[0] && arg_53[0] == arg_40[0] && arg_53[0] == arg_182[0] && arg_53[0] == arg_117[0] && arg_30 == arg_141 && arg_30 == arg_11[0] && arg_30 == arg_65 && arg_30 == arg_172 && arg_30 == arg_69[0] && arg_30 == arg_76[0] && arg_30 == arg_77 && arg_30 == arg_41 && arg_30 == arg_93 && arg_30 == arg_183 && arg_30 == arg_54 && arg_30 == arg_113 && arg_30 == arg_17[0] && arg_30 == arg_45[0] && arg_30 == arg_187[0] && arg_30 == arg_82 && arg_30 == arg_136 && arg_30 == arg_12 && arg_30 == arg_129[0] && arg_30 == arg_165 && arg_30 == arg_102 && arg_30 == arg_176[0] && arg_30 == arg_81[0] && arg_30 == arg_140[0] && arg_30 == arg_149 && arg_30 == arg_59 && arg_30 == arg_70 && arg_30 == arg_88 && arg_30 == arg_35 && arg_30 == arg_46 && arg_30 == arg_148[0] && arg_30 == arg_159[0] && arg_30 == arg_29[0] && arg_30 == arg_118 && arg_30 == arg_130 && arg_30 == arg_58[0] && arg_30 == arg_135[0] && arg_30 == arg_6[0] && arg_30 == arg_112[0] && arg_30 == arg_164[0] && arg_30 == arg_125 && arg_30 == arg_92[0] && arg_30 == arg_171[0] && arg_30 == arg_22[0] && arg_30 == arg_18 && arg_30 == arg_154 && arg_30 == arg_101[0] && arg_30 == arg_177 && arg_30 == arg_64[0] && arg_30 == arg_106[0] && arg_30 == arg_40[0] && arg_30 == arg_182[0] && arg_30 == arg_117[0] && arg_141 == arg_11[0] && arg_141 == arg_65 && arg_141 == arg_172 && arg_141 == arg_69[0] && arg_141 == arg_76[0] && arg_141 == arg_77 && arg_141 == arg_41 && arg_141 == arg_93 && arg_141 == arg_183 && arg_141 == arg_54 && arg_141 == arg_113 && arg_141 == arg_17[0] && arg_141 == arg_45[0] && arg_141 == arg_187[0] && arg_141 == arg_82 && arg_141 == arg_136 && arg_141 == arg_12 && arg_141 == arg_129[0] && arg_141 == arg_165 && arg_141 == arg_102 && arg_141 == arg_176[0] && arg_141 == arg_81[0] && arg_141 == arg_140[0] && arg_141 == arg_149 && arg_141 == arg_59 && arg_141 == arg_70 && arg_141 == arg_88 && arg_141 == arg_35 && arg_141 == arg_46 && arg_141 == arg_148[0] && arg_141 == arg_159[0] && arg_141 == arg_29[0] && arg_141 == arg_118 && arg_141 == arg_130 && arg_141 == arg_58[0] && arg_141 == arg_135[0] && arg_141 == arg_6[0] && arg_141 == arg_112[0] && arg_141 == arg_164[0] && arg_141 == arg_125 && arg_141 == arg_92[0] && arg_141 == arg_171[0] && arg_141 == arg_22[0] && arg_141 == arg_18 && arg_141 == arg_154 && arg_141 == arg_101[0] && arg_141 == arg_177 && arg_141 == arg_64[0] && arg_141 == arg_106[0] && arg_141 == arg_40[0] && arg_141 == arg_182[0] && arg_141 == arg_117[0] && arg_11[0] == arg_65 && arg_11[0] == arg_172 && arg_11[0] == arg_69[0] && arg_11[0] == arg_76[0] && arg_11[0] == arg_77 && arg_11[0] == arg_41 && arg_11[0] == arg_93 && arg_11[0] == arg_183 && arg_11[0] == arg_54 && arg_11[0] == arg_113 && arg_11[0] == arg_17[0] && arg_11[0] == arg_45[0] && arg_11[0] == arg_187[0] && arg_11[0] == arg_82 && arg_11[0] == arg_136 && arg_11[0] == arg_12 && arg_11[0] == arg_129[0] && arg_11[0] == arg_165 && arg_11[0] == arg_102 && arg_11[0] == arg_176[0] && arg_11[0] == arg_81[0] && arg_11[0] == arg_140[0] && arg_11[0] == arg_149 && arg_11[0] == arg_59 && arg_11[0] == arg_70 && arg_11[0] == arg_88 && arg_11[0] == arg_35 && arg_11[0] == arg_46 && arg_11[0] == arg_148[0] && arg_11[0] == arg_159[0] && arg_11[0] == arg_29[0] && arg_11[0] == arg_118 && arg_11[0] == arg_130 && arg_11[0] == arg_58[0] && arg_11[0] == arg_135[0] && arg_11[0] == arg_6[0] && arg_11[0] == arg_112[0] && arg_11[0] == arg_164[0] && arg_11[0] == arg_125 && arg_11[0] == arg_92[0] && arg_11[0] == arg_171[0] && arg_11[0] == arg_22[0] && arg_11[0] == arg_18 && arg_11[0] == arg_154 && arg_11[0] == arg_101[0] && arg_11[0] == arg_177 && arg_11[0] == arg_64[0] && arg_11[0] == arg_106[0] && arg_11[0] == arg_40[0] && arg_11[0] == arg_182[0] && arg_11[0] == arg_117[0] && arg_65 == arg_172 && arg_65 == arg_69[0] && arg_65 == arg_76[0] && arg_65 == arg_77 && arg_65 == arg_41 && arg_65 == arg_93 && arg_65 == arg_183 && arg_65 == arg_54 && arg_65 == arg_113 && arg_65 == arg_17[0] && arg_65 == arg_45[0] && arg_65 == arg_187[0] && arg_65 == arg_82 && arg_65 == arg_136 && arg_65 == arg_12 && arg_65 == arg_129[0] && arg_65 == arg_165 && arg_65 == arg_102 && arg_65 == arg_176[0] && arg_65 == arg_81[0] && arg_65 == arg_140[0] && arg_65 == arg_149 && arg_65 == arg_59 && arg_65 == arg_70 && arg_65 == arg_88 && arg_65 == arg_35 && arg_65 == arg_46 && arg_65 == arg_148[0] && arg_65 == arg_159[0] && arg_65 == arg_29[0] && arg_65 == arg_118 && arg_65 == arg_130 && arg_65 == arg_58[0] && arg_65 == arg_135[0] && arg_65 == arg_6[0] && arg_65 == arg_112[0] && arg_65 == arg_164[0] && arg_65 == arg_125 && arg_65 == arg_92[0] && arg_65 == arg_171[0] && arg_65 == arg_22[0] && arg_65 == arg_18 && arg_65 == arg_154 && arg_65 == arg_101[0] && arg_65 == arg_177 && arg_65 == arg_64[0] && arg_65 == arg_106[0] && arg_65 == arg_40[0] && arg_65 == arg_182[0] && arg_65 == arg_117[0] && arg_172 == arg_69[0] && arg_172 == arg_76[0] && arg_172 == arg_77 && arg_172 == arg_41 && arg_172 == arg_93 && arg_172 == arg_183 && arg_172 == arg_54 && arg_172 == arg_113 && arg_172 == arg_17[0] && arg_172 == arg_45[0] && arg_172 == arg_187[0] && arg_172 == arg_82 && arg_172 == arg_136 && arg_172 == arg_12 && arg_172 == arg_129[0] && arg_172 == arg_165 && arg_172 == arg_102 && arg_172 == arg_176[0] && arg_172 == arg_81[0] && arg_172 == arg_140[0] && arg_172 == arg_149 && arg_172 == arg_59 && arg_172 == arg_70 && arg_172 == arg_88 && arg_172 == arg_35 && arg_172 == arg_46 && arg_172 == arg_148[0] && arg_172 == arg_159[0] && arg_172 == arg_29[0] && arg_172 == arg_118 && arg_172 == arg_130 && arg_172 == arg_58[0] && arg_172 == arg_135[0] && arg_172 == arg_6[0] && arg_172 == arg_112[0] && arg_172 == arg_164[0] && arg_172 == arg_125 && arg_172 == arg_92[0] && arg_172 == arg_171[0] && arg_172 == arg_22[0] && arg_172 == arg_18 && arg_172 == arg_154 && arg_172 == arg_101[0] && arg_172 == arg_177 && arg_172 == arg_64[0] && arg_172 == arg_106[0] && arg_172 == arg_40[0] && arg_172 == arg_182[0] && arg_172 == arg_117[0] && arg_69[0] == arg_76[0] && arg_69[0] == arg_77 && arg_69[0] == arg_41 && arg_69[0] == arg_93 && arg_69[0] == arg_183 && arg_69[0] == arg_54 && arg_69[0] == arg_113 && arg_69[0] == arg_17[0] && arg_69[0] == arg_45[0] && arg_69[0] == arg_187[0] && arg_69[0] == arg_82 && arg_69[0] == arg_136 && arg_69[0] == arg_12 && arg_69[0] == arg_129[0] && arg_69[0] == arg_165 && arg_69[0] == arg_102 && arg_69[0] == arg_176[0] && arg_69[0] == arg_81[0] && arg_69[0] == arg_140[0] && arg_69[0] == arg_149 && arg_69[0] == arg_59 && arg_69[0] == arg_70 && arg_69[0] == arg_88 && arg_69[0] == arg_35 && arg_69[0] == arg_46 && arg_69[0] == arg_148[0] && arg_69[0] == arg_159[0] && arg_69[0] == arg_29[0] && arg_69[0] == arg_118 && arg_69[0] == arg_130 && arg_69[0] == arg_58[0] && arg_69[0] == arg_135[0] && arg_69[0] == arg_6[0] && arg_69[0] == arg_112[0] && arg_69[0] == arg_164[0] && arg_69[0] == arg_125 && arg_69[0] == arg_92[0] && arg_69[0] == arg_171[0] && arg_69[0] == arg_22[0] && arg_69[0] == arg_18 && arg_69[0] == arg_154 && arg_69[0] == arg_101[0] && arg_69[0] == arg_177 && arg_69[0] == arg_64[0] && arg_69[0] == arg_106[0] && arg_69[0] == arg_40[0] && arg_69[0] == arg_182[0] && arg_69[0] == arg_117[0] && arg_76[0] == arg_77 && arg_76[0] == arg_41 && arg_76[0] == arg_93 && arg_76[0] == arg_183 && arg_76[0] == arg_54 && arg_76[0] == arg_113 && arg_76[0] == arg_17[0] && arg_76[0] == arg_45[0] && arg_76[0] == arg_187[0] && arg_76[0] == arg_82 && arg_76[0] == arg_136 && arg_76[0] == arg_12 && arg_76[0] == arg_129[0] && arg_76[0] == arg_165 && arg_76[0] == arg_102 && arg_76[0] == arg_176[0] && arg_76[0] == arg_81[0] && arg_76[0] == arg_140[0] && arg_76[0] == arg_149 && arg_76[0] == arg_59 && arg_76[0] == arg_70 && arg_76[0] == arg_88 && arg_76[0] == arg_35 && arg_76[0] == arg_46 && arg_76[0] == arg_148[0] && arg_76[0] == arg_159[0] && arg_76[0] == arg_29[0] && arg_76[0] == arg_118 && arg_76[0] == arg_130 && arg_76[0] == arg_58[0] && arg_76[0] == arg_135[0] && arg_76[0] == arg_6[0] && arg_76[0] == arg_112[0] && arg_76[0] == arg_164[0] && arg_76[0] == arg_125 && arg_76[0] == arg_92[0] && arg_76[0] == arg_171[0] && arg_76[0] == arg_22[0] && arg_76[0] == arg_18 && arg_76[0] == arg_154 && arg_76[0] == arg_101[0] && arg_76[0] == arg_177 && arg_76[0] == arg_64[0] && arg_76[0] == arg_106[0] && arg_76[0] == arg_40[0] && arg_76[0] == arg_182[0] && arg_76[0] == arg_117[0] && arg_77 == arg_41 && arg_77 == arg_93 && arg_77 == arg_183 && arg_77 == arg_54 && arg_77 == arg_113 && arg_77 == arg_17[0] && arg_77 == arg_45[0] && arg_77 == arg_187[0] && arg_77 == arg_82 && arg_77 == arg_136 && arg_77 == arg_12 && arg_77 == arg_129[0] && arg_77 == arg_165 && arg_77 == arg_102 && arg_77 == arg_176[0] && arg_77 == arg_81[0] && arg_77 == arg_140[0] && arg_77 == arg_149 && arg_77 == arg_59 && arg_77 == arg_70 && arg_77 == arg_88 && arg_77 == arg_35 && arg_77 == arg_46 && arg_77 == arg_148[0] && arg_77 == arg_159[0] && arg_77 == arg_29[0] && arg_77 == arg_118 && arg_77 == arg_130 && arg_77 == arg_58[0] && arg_77 == arg_135[0] && arg_77 == arg_6[0] && arg_77 == arg_112[0] && arg_77 == arg_164[0] && arg_77 == arg_125 && arg_77 == arg_92[0] && arg_77 == arg_171[0] && arg_77 == arg_22[0] && arg_77 == arg_18 && arg_77 == arg_154 && arg_77 == arg_101[0] && arg_77 == arg_177 && arg_77 == arg_64[0] && arg_77 == arg_106[0] && arg_77 == arg_40[0] && arg_77 == arg_182[0] && arg_77 == arg_117[0] && arg_41 == arg_93 && arg_41 == arg_183 && arg_41 == arg_54 && arg_41 == arg_113 && arg_41 == arg_17[0] && arg_41 == arg_45[0] && arg_41 == arg_187[0] && arg_41 == arg_82 && arg_41 == arg_136 && arg_41 == arg_12 && arg_41 == arg_129[0] && arg_41 == arg_165 && arg_41 == arg_102 && arg_41 == arg_176[0] && arg_41 == arg_81[0] && arg_41 == arg_140[0] && arg_41 == arg_149 && arg_41 == arg_59 && arg_41 == arg_70 && arg_41 == arg_88 && arg_41 == arg_35 && arg_41 == arg_46 && arg_41 == arg_148[0] && arg_41 == arg_159[0] && arg_41 == arg_29[0] && arg_41 == arg_118 && arg_41 == arg_130 && arg_41 == arg_58[0] && arg_41 == arg_135[0] && arg_41 == arg_6[0] && arg_41 == arg_112[0] && arg_41 == arg_164[0] && arg_41 == arg_125 && arg_41 == arg_92[0] && arg_41 == arg_171[0] && arg_41 == arg_22[0] && arg_41 == arg_18 && arg_41 == arg_154 && arg_41 == arg_101[0] && arg_41 == arg_177 && arg_41 == arg_64[0] && arg_41 == arg_106[0] && arg_41 == arg_40[0] && arg_41 == arg_182[0] && arg_41 == arg_117[0] && arg_93 == arg_183 && arg_93 == arg_54 && arg_93 == arg_113 && arg_93 == arg_17[0] && arg_93 == arg_45[0] && arg_93 == arg_187[0] && arg_93 == arg_82 && arg_93 == arg_136 && arg_93 == arg_12 && arg_93 == arg_129[0] && arg_93 == arg_165 && arg_93 == arg_102 && arg_93 == arg_176[0] && arg_93 == arg_81[0] && arg_93 == arg_140[0] && arg_93 == arg_149 && arg_93 == arg_59 && arg_93 == arg_70 && arg_93 == arg_88 && arg_93 == arg_35 && arg_93 == arg_46 && arg_93 == arg_148[0] && arg_93 == arg_159[0] && arg_93 == arg_29[0] && arg_93 == arg_118 && arg_93 == arg_130 && arg_93 == arg_58[0] && arg_93 == arg_135[0] && arg_93 == arg_6[0] && arg_93 == arg_112[0] && arg_93 == arg_164[0] && arg_93 == arg_125 && arg_93 == arg_92[0] && arg_93 == arg_171[0] && arg_93 == arg_22[0] && arg_93 == arg_18 && arg_93 == arg_154 && arg_93 == arg_101[0] && arg_93 == arg_177 && arg_93 == arg_64[0] && arg_93 == arg_106[0] && arg_93 == arg_40[0] && arg_93 == arg_182[0] && arg_93 == arg_117[0] && arg_183 == arg_54 && arg_183 == arg_113 && arg_183 == arg_17[0] && arg_183 == arg_45[0] && arg_183 == arg_187[0] && arg_183 == arg_82 && arg_183 == arg_136 && arg_183 == arg_12 && arg_183 == arg_129[0] && arg_183 == arg_165 && arg_183 == arg_102 && arg_183 == arg_176[0] && arg_183 == arg_81[0] && arg_183 == arg_140[0] && arg_183 == arg_149 && arg_183 == arg_59 && arg_183 == arg_70 && arg_183 == arg_88 && arg_183 == arg_35 && arg_183 == arg_46 && arg_183 == arg_148[0] && arg_183 == arg_159[0] && arg_183 == arg_29[0] && arg_183 == arg_118 && arg_183 == arg_130 && arg_183 == arg_58[0] && arg_183 == arg_135[0] && arg_183 == arg_6[0] && arg_183 == arg_112[0] && arg_183 == arg_164[0] && arg_183 == arg_125 && arg_183 == arg_92[0] && arg_183 == arg_171[0] && arg_183 == arg_22[0] && arg_183 == arg_18 && arg_183 == arg_154 && arg_183 == arg_101[0] && arg_183 == arg_177 && arg_183 == arg_64[0] && arg_183 == arg_106[0] && arg_183 == arg_40[0] && arg_183 == arg_182[0] && arg_183 == arg_117[0] && arg_54 == arg_113 && arg_54 == arg_17[0] && arg_54 == arg_45[0] && arg_54 == arg_187[0] && arg_54 == arg_82 && arg_54 == arg_136 && arg_54 == arg_12 && arg_54 == arg_129[0] && arg_54 == arg_165 && arg_54 == arg_102 && arg_54 == arg_176[0] && arg_54 == arg_81[0] && arg_54 == arg_140[0] && arg_54 == arg_149 && arg_54 == arg_59 && arg_54 == arg_70 && arg_54 == arg_88 && arg_54 == arg_35 && arg_54 == arg_46 && arg_54 == arg_148[0] && arg_54 == arg_159[0] && arg_54 == arg_29[0] && arg_54 == arg_118 && arg_54 == arg_130 && arg_54 == arg_58[0] && arg_54 == arg_135[0] && arg_54 == arg_6[0] && arg_54 == arg_112[0] && arg_54 == arg_164[0] && arg_54 == arg_125 && arg_54 == arg_92[0] && arg_54 == arg_171[0] && arg_54 == arg_22[0] && arg_54 == arg_18 && arg_54 == arg_154 && arg_54 == arg_101[0] && arg_54 == arg_177 && arg_54 == arg_64[0] && arg_54 == arg_106[0] && arg_54 == arg_40[0] && arg_54 == arg_182[0] && arg_54 == arg_117[0] && arg_113 == arg_17[0] && arg_113 == arg_45[0] && arg_113 == arg_187[0] && arg_113 == arg_82 && arg_113 == arg_136 && arg_113 == arg_12 && arg_113 == arg_129[0] && arg_113 == arg_165 && arg_113 == arg_102 && arg_113 == arg_176[0] && arg_113 == arg_81[0] && arg_113 == arg_140[0] && arg_113 == arg_149 && arg_113 == arg_59 && arg_113 == arg_70 && arg_113 == arg_88 && arg_113 == arg_35 && arg_113 == arg_46 && arg_113 == arg_148[0] && arg_113 == arg_159[0] && arg_113 == arg_29[0] && arg_113 == arg_118 && arg_113 == arg_130 && arg_113 == arg_58[0] && arg_113 == arg_135[0] && arg_113 == arg_6[0] && arg_113 == arg_112[0] && arg_113 == arg_164[0] && arg_113 == arg_125 && arg_113 == arg_92[0] && arg_113 == arg_171[0] && arg_113 == arg_22[0] && arg_113 == arg_18 && arg_113 == arg_154 && arg_113 == arg_101[0] && arg_113 == arg_177 && arg_113 == arg_64[0] && arg_113 == arg_106[0] && arg_113 == arg_40[0] && arg_113 == arg_182[0] && arg_113 == arg_117[0] && arg_17[0] == arg_45[0] && arg_17[0] == arg_187[0] && arg_17[0] == arg_82 && arg_17[0] == arg_136 && arg_17[0] == arg_12 && arg_17[0] == arg_129[0] && arg_17[0] == arg_165 && arg_17[0] == arg_102 && arg_17[0] == arg_176[0] && arg_17[0] == arg_81[0] && arg_17[0] == arg_140[0] && arg_17[0] == arg_149 && arg_17[0] == arg_59 && arg_17[0] == arg_70 && arg_17[0] == arg_88 && arg_17[0] == arg_35 && arg_17[0] == arg_46 && arg_17[0] == arg_148[0] && arg_17[0] == arg_159[0] && arg_17[0] == arg_29[0] && arg_17[0] == arg_118 && arg_17[0] == arg_130 && arg_17[0] == arg_58[0] && arg_17[0] == arg_135[0] && arg_17[0] == arg_6[0] && arg_17[0] == arg_112[0] && arg_17[0] == arg_164[0] && arg_17[0] == arg_125 && arg_17[0] == arg_92[0] && arg_17[0] == arg_171[0] && arg_17[0] == arg_22[0] && arg_17[0] == arg_18 && arg_17[0] == arg_154 && arg_17[0] == arg_101[0] && arg_17[0] == arg_177 && arg_17[0] == arg_64[0] && arg_17[0] == arg_106[0] && arg_17[0] == arg_40[0] && arg_17[0] == arg_182[0] && arg_17[0] == arg_117[0] && arg_45[0] == arg_187[0] && arg_45[0] == arg_82 && arg_45[0] == arg_136 && arg_45[0] == arg_12 && arg_45[0] == arg_129[0] && arg_45[0] == arg_165 && arg_45[0] == arg_102 && arg_45[0] == arg_176[0] && arg_45[0] == arg_81[0] && arg_45[0] == arg_140[0] && arg_45[0] == arg_149 && arg_45[0] == arg_59 && arg_45[0] == arg_70 && arg_45[0] == arg_88 && arg_45[0] == arg_35 && arg_45[0] == arg_46 && arg_45[0] == arg_148[0] && arg_45[0] == arg_159[0] && arg_45[0] == arg_29[0] && arg_45[0] == arg_118 && arg_45[0] == arg_130 && arg_45[0] == arg_58[0] && arg_45[0] == arg_135[0] && arg_45[0] == arg_6[0] && arg_45[0] == arg_112[0] && arg_45[0] == arg_164[0] && arg_45[0] == arg_125 && arg_45[0] == arg_92[0] && arg_45[0] == arg_171[0] && arg_45[0] == arg_22[0] && arg_45[0] == arg_18 && arg_45[0] == arg_154 && arg_45[0] == arg_101[0] && arg_45[0] == arg_177 && arg_45[0] == arg_64[0] && arg_45[0] == arg_106[0] && arg_45[0] == arg_40[0] && arg_45[0] == arg_182[0] && arg_45[0] == arg_117[0] && arg_187[0] == arg_82 && arg_187[0] == arg_136 && arg_187[0] == arg_12 && arg_187[0] == arg_129[0] && arg_187[0] == arg_165 && arg_187[0] == arg_102 && arg_187[0] == arg_176[0] && arg_187[0] == arg_81[0] && arg_187[0] == arg_140[0] && arg_187[0] == arg_149 && arg_187[0] == arg_59 && arg_187[0] == arg_70 && arg_187[0] == arg_88 && arg_187[0] == arg_35 && arg_187[0] == arg_46 && arg_187[0] == arg_148[0] && arg_187[0] == arg_159[0] && arg_187[0] == arg_29[0] && arg_187[0] == arg_118 && arg_187[0] == arg_130 && arg_187[0] == arg_58[0] && arg_187[0] == arg_135[0] && arg_187[0] == arg_6[0] && arg_187[0] == arg_112[0] && arg_187[0] == arg_164[0] && arg_187[0] == arg_125 && arg_187[0] == arg_92[0] && arg_187[0] == arg_171[0] && arg_187[0] == arg_22[0] && arg_187[0] == arg_18 && arg_187[0] == arg_154 && arg_187[0] == arg_101[0] && arg_187[0] == arg_177 && arg_187[0] == arg_64[0] && arg_187[0] == arg_106[0] && arg_187[0] == arg_40[0] && arg_187[0] == arg_182[0] && arg_187[0] == arg_117[0] && arg_82 == arg_136 && arg_82 == arg_12 && arg_82 == arg_129[0] && arg_82 == arg_165 && arg_82 == arg_102 && arg_82 == arg_176[0] && arg_82 == arg_81[0] && arg_82 == arg_140[0] && arg_82 == arg_149 && arg_82 == arg_59 && arg_82 == arg_70 && arg_82 == arg_88 && arg_82 == arg_35 && arg_82 == arg_46 && arg_82 == arg_148[0] && arg_82 == arg_159[0] && arg_82 == arg_29[0] && arg_82 == arg_118 && arg_82 == arg_130 && arg_82 == arg_58[0] && arg_82 == arg_135[0] && arg_82 == arg_6[0] && arg_82 == arg_112[0] && arg_82 == arg_164[0] && arg_82 == arg_125 && arg_82 == arg_92[0] && arg_82 == arg_171[0] && arg_82 == arg_22[0] && arg_82 == arg_18 && arg_82 == arg_154 && arg_82 == arg_101[0] && arg_82 == arg_177 && arg_82 == arg_64[0] && arg_82 == arg_106[0] && arg_82 == arg_40[0] && arg_82 == arg_182[0] && arg_82 == arg_117[0] && arg_136 == arg_12 && arg_136 == arg_129[0] && arg_136 == arg_165 && arg_136 == arg_102 && arg_136 == arg_176[0] && arg_136 == arg_81[0] && arg_136 == arg_140[0] && arg_136 == arg_149 && arg_136 == arg_59 && arg_136 == arg_70 && arg_136 == arg_88 && arg_136 == arg_35 && arg_136 == arg_46 && arg_136 == arg_148[0] && arg_136 == arg_159[0] && arg_136 == arg_29[0] && arg_136 == arg_118 && arg_136 == arg_130 && arg_136 == arg_58[0] && arg_136 == arg_135[0] && arg_136 == arg_6[0] && arg_136 == arg_112[0] && arg_136 == arg_164[0] && arg_136 == arg_125 && arg_136 == arg_92[0] && arg_136 == arg_171[0] && arg_136 == arg_22[0] && arg_136 == arg_18 && arg_136 == arg_154 && arg_136 == arg_101[0] && arg_136 == arg_177 && arg_136 == arg_64[0] && arg_136 == arg_106[0] && arg_136 == arg_40[0] && arg_136 == arg_182[0] && arg_136 == arg_117[0] && arg_12 == arg_129[0] && arg_12 == arg_165 && arg_12 == arg_102 && arg_12 == arg_176[0] && arg_12 == arg_81[0] && arg_12 == arg_140[0] && arg_12 == arg_149 && arg_12 == arg_59 && arg_12 == arg_70 && arg_12 == arg_88 && arg_12 == arg_35 && arg_12 == arg_46 && arg_12 == arg_148[0] && arg_12 == arg_159[0] && arg_12 == arg_29[0] && arg_12 == arg_118 && arg_12 == arg_130 && arg_12 == arg_58[0] && arg_12 == arg_135[0] && arg_12 == arg_6[0] && arg_12 == arg_112[0] && arg_12 == arg_164[0] && arg_12 == arg_125 && arg_12 == arg_92[0] && arg_12 == arg_171[0] && arg_12 == arg_22[0] && arg_12 == arg_18 && arg_12 == arg_154 && arg_12 == arg_101[0] && arg_12 == arg_177 && arg_12 == arg_64[0] && arg_12 == arg_106[0] && arg_12 == arg_40[0] && arg_12 == arg_182[0] && arg_12 == arg_117[0] && arg_129[0] == arg_165 && arg_129[0] == arg_102 && arg_129[0] == arg_176[0] && arg_129[0] == arg_81[0] && arg_129[0] == arg_140[0] && arg_129[0] == arg_149 && arg_129[0] == arg_59 && arg_129[0] == arg_70 && arg_129[0] == arg_88 && arg_129[0] == arg_35 && arg_129[0] == arg_46 && arg_129[0] == arg_148[0] && arg_129[0] == arg_159[0] && arg_129[0] == arg_29[0] && arg_129[0] == arg_118 && arg_129[0] == arg_130 && arg_129[0] == arg_58[0] && arg_129[0] == arg_135[0] && arg_129[0] == arg_6[0] && arg_129[0] == arg_112[0] && arg_129[0] == arg_164[0] && arg_129[0] == arg_125 && arg_129[0] == arg_92[0] && arg_129[0] == arg_171[0] && arg_129[0] == arg_22[0] && arg_129[0] == arg_18 && arg_129[0] == arg_154 && arg_129[0] == arg_101[0] && arg_129[0] == arg_177 && arg_129[0] == arg_64[0] && arg_129[0] == arg_106[0] && arg_129[0] == arg_40[0] && arg_129[0] == arg_182[0] && arg_129[0] == arg_117[0] && arg_165 == arg_102 && arg_165 == arg_176[0] && arg_165 == arg_81[0] && arg_165 == arg_140[0] && arg_165 == arg_149 && arg_165 == arg_59 && arg_165 == arg_70 && arg_165 == arg_88 && arg_165 == arg_35 && arg_165 == arg_46 && arg_165 == arg_148[0] && arg_165 == arg_159[0] && arg_165 == arg_29[0] && arg_165 == arg_118 && arg_165 == arg_130 && arg_165 == arg_58[0] && arg_165 == arg_135[0] && arg_165 == arg_6[0] && arg_165 == arg_112[0] && arg_165 == arg_164[0] && arg_165 == arg_125 && arg_165 == arg_92[0] && arg_165 == arg_171[0] && arg_165 == arg_22[0] && arg_165 == arg_18 && arg_165 == arg_154 && arg_165 == arg_101[0] && arg_165 == arg_177 && arg_165 == arg_64[0] && arg_165 == arg_106[0] && arg_165 == arg_40[0] && arg_165 == arg_182[0] && arg_165 == arg_117[0] && arg_102 == arg_176[0] && arg_102 == arg_81[0] && arg_102 == arg_140[0] && arg_102 == arg_149 && arg_102 == arg_59 && arg_102 == arg_70 && arg_102 == arg_88 && arg_102 == arg_35 && arg_102 == arg_46 && arg_102 == arg_148[0] && arg_102 == arg_159[0] && arg_102 == arg_29[0] && arg_102 == arg_118 && arg_102 == arg_130 && arg_102 == arg_58[0] && arg_102 == arg_135[0] && arg_102 == arg_6[0] && arg_102 == arg_112[0] && arg_102 == arg_164[0] && arg_102 == arg_125 && arg_102 == arg_92[0] && arg_102 == arg_171[0] && arg_102 == arg_22[0] && arg_102 == arg_18 && arg_102 == arg_154 && arg_102 == arg_101[0] && arg_102 == arg_177 && arg_102 == arg_64[0] && arg_102 == arg_106[0] && arg_102 == arg_40[0] && arg_102 == arg_182[0] && arg_102 == arg_117[0] && arg_176[0] == arg_81[0] && arg_176[0] == arg_140[0] && arg_176[0] == arg_149 && arg_176[0] == arg_59 && arg_176[0] == arg_70 && arg_176[0] == arg_88 && arg_176[0] == arg_35 && arg_176[0] == arg_46 && arg_176[0] == arg_148[0] && arg_176[0] == arg_159[0] && arg_176[0] == arg_29[0] && arg_176[0] == arg_118 && arg_176[0] == arg_130 && arg_176[0] == arg_58[0] && arg_176[0] == arg_135[0] && arg_176[0] == arg_6[0] && arg_176[0] == arg_112[0] && arg_176[0] == arg_164[0] && arg_176[0] == arg_125 && arg_176[0] == arg_92[0] && arg_176[0] == arg_171[0] && arg_176[0] == arg_22[0] && arg_176[0] == arg_18 && arg_176[0] == arg_154 && arg_176[0] == arg_101[0] && arg_176[0] == arg_177 && arg_176[0] == arg_64[0] && arg_176[0] == arg_106[0] && arg_176[0] == arg_40[0] && arg_176[0] == arg_182[0] && arg_176[0] == arg_117[0] && arg_81[0] == arg_140[0] && arg_81[0] == arg_149 && arg_81[0] == arg_59 && arg_81[0] == arg_70 && arg_81[0] == arg_88 && arg_81[0] == arg_35 && arg_81[0] == arg_46 && arg_81[0] == arg_148[0] && arg_81[0] == arg_159[0] && arg_81[0] == arg_29[0] && arg_81[0] == arg_118 && arg_81[0] == arg_130 && arg_81[0] == arg_58[0] && arg_81[0] == arg_135[0] && arg_81[0] == arg_6[0] && arg_81[0] == arg_112[0] && arg_81[0] == arg_164[0] && arg_81[0] == arg_125 && arg_81[0] == arg_92[0] && arg_81[0] == arg_171[0] && arg_81[0] == arg_22[0] && arg_81[0] == arg_18 && arg_81[0] == arg_154 && arg_81[0] == arg_101[0] && arg_81[0] == arg_177 && arg_81[0] == arg_64[0] && arg_81[0] == arg_106[0] && arg_81[0] == arg_40[0] && arg_81[0] == arg_182[0] && arg_81[0] == arg_117[0] && arg_140[0] == arg_149 && arg_140[0] == arg_59 && arg_140[0] == arg_70 && arg_140[0] == arg_88 && arg_140[0] == arg_35 && arg_140[0] == arg_46 && arg_140[0] == arg_148[0] && arg_140[0] == arg_159[0] && arg_140[0] == arg_29[0] && arg_140[0] == arg_118 && arg_140[0] == arg_130 && arg_140[0] == arg_58[0] && arg_140[0] == arg_135[0] && arg_140[0] == arg_6[0] && arg_140[0] == arg_112[0] && arg_140[0] == arg_164[0] && arg_140[0] == arg_125 && arg_140[0] == arg_92[0] && arg_140[0] == arg_171[0] && arg_140[0] == arg_22[0] && arg_140[0] == arg_18 && arg_140[0] == arg_154 && arg_140[0] == arg_101[0] && arg_140[0] == arg_177 && arg_140[0] == arg_64[0] && arg_140[0] == arg_106[0] && arg_140[0] == arg_40[0] && arg_140[0] == arg_182[0] && arg_140[0] == arg_117[0] && arg_149 == arg_59 && arg_149 == arg_70 && arg_149 == arg_88 && arg_149 == arg_35 && arg_149 == arg_46 && arg_149 == arg_148[0] && arg_149 == arg_159[0] && arg_149 == arg_29[0] && arg_149 == arg_118 && arg_149 == arg_130 && arg_149 == arg_58[0] && arg_149 == arg_135[0] && arg_149 == arg_6[0] && arg_149 == arg_112[0] && arg_149 == arg_164[0] && arg_149 == arg_125 && arg_149 == arg_92[0] && arg_149 == arg_171[0] && arg_149 == arg_22[0] && arg_149 == arg_18 && arg_149 == arg_154 && arg_149 == arg_101[0] && arg_149 == arg_177 && arg_149 == arg_64[0] && arg_149 == arg_106[0] && arg_149 == arg_40[0] && arg_149 == arg_182[0] && arg_149 == arg_117[0] && arg_59 == arg_70 && arg_59 == arg_88 && arg_59 == arg_35 && arg_59 == arg_46 && arg_59 == arg_148[0] && arg_59 == arg_159[0] && arg_59 == arg_29[0] && arg_59 == arg_118 && arg_59 == arg_130 && arg_59 == arg_58[0] && arg_59 == arg_135[0] && arg_59 == arg_6[0] && arg_59 == arg_112[0] && arg_59 == arg_164[0] && arg_59 == arg_125 && arg_59 == arg_92[0] && arg_59 == arg_171[0] && arg_59 == arg_22[0] && arg_59 == arg_18 && arg_59 == arg_154 && arg_59 == arg_101[0] && arg_59 == arg_177 && arg_59 == arg_64[0] && arg_59 == arg_106[0] && arg_59 == arg_40[0] && arg_59 == arg_182[0] && arg_59 == arg_117[0] && arg_70 == arg_88 && arg_70 == arg_35 && arg_70 == arg_46 && arg_70 == arg_148[0] && arg_70 == arg_159[0] && arg_70 == arg_29[0] && arg_70 == arg_118 && arg_70 == arg_130 && arg_70 == arg_58[0] && arg_70 == arg_135[0] && arg_70 == arg_6[0] && arg_70 == arg_112[0] && arg_70 == arg_164[0] && arg_70 == arg_125 && arg_70 == arg_92[0] && arg_70 == arg_171[0] && arg_70 == arg_22[0] && arg_70 == arg_18 && arg_70 == arg_154 && arg_70 == arg_101[0] && arg_70 == arg_177 && arg_70 == arg_64[0] && arg_70 == arg_106[0] && arg_70 == arg_40[0] && arg_70 == arg_182[0] && arg_70 == arg_117[0] && arg_88 == arg_35 && arg_88 == arg_46 && arg_88 == arg_148[0] && arg_88 == arg_159[0] && arg_88 == arg_29[0] && arg_88 == arg_118 && arg_88 == arg_130 && arg_88 == arg_58[0] && arg_88 == arg_135[0] && arg_88 == arg_6[0] && arg_88 == arg_112[0] && arg_88 == arg_164[0] && arg_88 == arg_125 && arg_88 == arg_92[0] && arg_88 == arg_171[0] && arg_88 == arg_22[0] && arg_88 == arg_18 && arg_88 == arg_154 && arg_88 == arg_101[0] && arg_88 == arg_177 && arg_88 == arg_64[0] && arg_88 == arg_106[0] && arg_88 == arg_40[0] && arg_88 == arg_182[0] && arg_88 == arg_117[0] && arg_35 == arg_46 && arg_35 == arg_148[0] && arg_35 == arg_159[0] && arg_35 == arg_29[0] && arg_35 == arg_118 && arg_35 == arg_130 && arg_35 == arg_58[0] && arg_35 == arg_135[0] && arg_35 == arg_6[0] && arg_35 == arg_112[0] && arg_35 == arg_164[0] && arg_35 == arg_125 && arg_35 == arg_92[0] && arg_35 == arg_171[0] && arg_35 == arg_22[0] && arg_35 == arg_18 && arg_35 == arg_154 && arg_35 == arg_101[0] && arg_35 == arg_177 && arg_35 == arg_64[0] && arg_35 == arg_106[0] && arg_35 == arg_40[0] && arg_35 == arg_182[0] && arg_35 == arg_117[0] && arg_46 == arg_148[0] && arg_46 == arg_159[0] && arg_46 == arg_29[0] && arg_46 == arg_118 && arg_46 == arg_130 && arg_46 == arg_58[0] && arg_46 == arg_135[0] && arg_46 == arg_6[0] && arg_46 == arg_112[0] && arg_46 == arg_164[0] && arg_46 == arg_125 && arg_46 == arg_92[0] && arg_46 == arg_171[0] && arg_46 == arg_22[0] && arg_46 == arg_18 && arg_46 == arg_154 && arg_46 == arg_101[0] && arg_46 == arg_177 && arg_46 == arg_64[0] && arg_46 == arg_106[0] && arg_46 == arg_40[0] && arg_46 == arg_182[0] && arg_46 == arg_117[0] && arg_148[0] == arg_159[0] && arg_148[0] == arg_29[0] && arg_148[0] == arg_118 && arg_148[0] == arg_130 && arg_148[0] == arg_58[0] && arg_148[0] == arg_135[0] && arg_148[0] == arg_6[0] && arg_148[0] == arg_112[0] && arg_148[0] == arg_164[0] && arg_148[0] == arg_125 && arg_148[0] == arg_92[0] && arg_148[0] == arg_171[0] && arg_148[0] == arg_22[0] && arg_148[0] == arg_18 && arg_148[0] == arg_154 && arg_148[0] == arg_101[0] && arg_148[0] == arg_177 && arg_148[0] == arg_64[0] && arg_148[0] == arg_106[0] && arg_148[0] == arg_40[0] && arg_148[0] == arg_182[0] && arg_148[0] == arg_117[0] && arg_159[0] == arg_29[0] && arg_159[0] == arg_118 && arg_159[0] == arg_130 && arg_159[0] == arg_58[0] && arg_159[0] == arg_135[0] && arg_159[0] == arg_6[0] && arg_159[0] == arg_112[0] && arg_159[0] == arg_164[0] && arg_159[0] == arg_125 && arg_159[0] == arg_92[0] && arg_159[0] == arg_171[0] && arg_159[0] == arg_22[0] && arg_159[0] == arg_18 && arg_159[0] == arg_154 && arg_159[0] == arg_101[0] && arg_159[0] == arg_177 && arg_159[0] == arg_64[0] && arg_159[0] == arg_106[0] && arg_159[0] == arg_40[0] && arg_159[0] == arg_182[0] && arg_159[0] == arg_117[0] && arg_29[0] == arg_118 && arg_29[0] == arg_130 && arg_29[0] == arg_58[0] && arg_29[0] == arg_135[0] && arg_29[0] == arg_6[0] && arg_29[0] == arg_112[0] && arg_29[0] == arg_164[0] && arg_29[0] == arg_125 && arg_29[0] == arg_92[0] && arg_29[0] == arg_171[0] && arg_29[0] == arg_22[0] && arg_29[0] == arg_18 && arg_29[0] == arg_154 && arg_29[0] == arg_101[0] && arg_29[0] == arg_177 && arg_29[0] == arg_64[0] && arg_29[0] == arg_106[0] && arg_29[0] == arg_40[0] && arg_29[0] == arg_182[0] && arg_29[0] == arg_117[0] && arg_118 == arg_130 && arg_118 == arg_58[0] && arg_118 == arg_135[0] && arg_118 == arg_6[0] && arg_118 == arg_112[0] && arg_118 == arg_164[0] && arg_118 == arg_125 && arg_118 == arg_92[0] && arg_118 == arg_171[0] && arg_118 == arg_22[0] && arg_118 == arg_18 && arg_118 == arg_154 && arg_118 == arg_101[0] && arg_118 == arg_177 && arg_118 == arg_64[0] && arg_118 == arg_106[0] && arg_118 == arg_40[0] && arg_118 == arg_182[0] && arg_118 == arg_117[0] && arg_130 == arg_58[0] && arg_130 == arg_135[0] && arg_130 == arg_6[0] && arg_130 == arg_112[0] && arg_130 == arg_164[0] && arg_130 == arg_125 && arg_130 == arg_92[0] && arg_130 == arg_171[0] && arg_130 == arg_22[0] && arg_130 == arg_18 && arg_130 == arg_154 && arg_130 == arg_101[0] && arg_130 == arg_177 && arg_130 == arg_64[0] && arg_130 == arg_106[0] && arg_130 == arg_40[0] && arg_130 == arg_182[0] && arg_130 == arg_117[0] && arg_58[0] == arg_135[0] && arg_58[0] == arg_6[0] && arg_58[0] == arg_112[0] && arg_58[0] == arg_164[0] && arg_58[0] == arg_125 && arg_58[0] == arg_92[0] && arg_58[0] == arg_171[0] && arg_58[0] == arg_22[0] && arg_58[0] == arg_18 && arg_58[0] == arg_154 && arg_58[0] == arg_101[0] && arg_58[0] == arg_177 && arg_58[0] == arg_64[0] && arg_58[0] == arg_106[0] && arg_58[0] == arg_40[0] && arg_58[0] == arg_182[0] && arg_58[0] == arg_117[0] && arg_135[0] == arg_6[0] && arg_135[0] == arg_112[0] && arg_135[0] == arg_164[0] && arg_135[0] == arg_125 && arg_135[0] == arg_92[0] && arg_135[0] == arg_171[0] && arg_135[0] == arg_22[0] && arg_135[0] == arg_18 && arg_135[0] == arg_154 && arg_135[0] == arg_101[0] && arg_135[0] == arg_177 && arg_135[0] == arg_64[0] && arg_135[0] == arg_106[0] && arg_135[0] == arg_40[0] && arg_135[0] == arg_182[0] && arg_135[0] == arg_117[0] && arg_6[0] == arg_112[0] && arg_6[0] == arg_164[0] && arg_6[0] == arg_125 && arg_6[0] == arg_92[0] && arg_6[0] == arg_171[0] && arg_6[0] == arg_22[0] && arg_6[0] == arg_18 && arg_6[0] == arg_154 && arg_6[0] == arg_101[0] && arg_6[0] == arg_177 && arg_6[0] == arg_64[0] && arg_6[0] == arg_106[0] && arg_6[0] == arg_40[0] && arg_6[0] == arg_182[0] && arg_6[0] == arg_117[0] && arg_112[0] == arg_164[0] && arg_112[0] == arg_125 && arg_112[0] == arg_92[0] && arg_112[0] == arg_171[0] && arg_112[0] == arg_22[0] && arg_112[0] == arg_18 && arg_112[0] == arg_154 && arg_112[0] == arg_101[0] && arg_112[0] == arg_177 && arg_112[0] == arg_64[0] && arg_112[0] == arg_106[0] && arg_112[0] == arg_40[0] && arg_112[0] == arg_182[0] && arg_112[0] == arg_117[0] && arg_164[0] == arg_125 && arg_164[0] == arg_92[0] && arg_164[0] == arg_171[0] && arg_164[0] == arg_22[0] && arg_164[0] == arg_18 && arg_164[0] == arg_154 && arg_164[0] == arg_101[0] && arg_164[0] == arg_177 && arg_164[0] == arg_64[0] && arg_164[0] == arg_106[0] && arg_164[0] == arg_40[0] && arg_164[0] == arg_182[0] && arg_164[0] == arg_117[0] && arg_125 == arg_92[0] && arg_125 == arg_171[0] && arg_125 == arg_22[0] && arg_125 == arg_18 && arg_125 == arg_154 && arg_125 == arg_101[0] && arg_125 == arg_177 && arg_125 == arg_64[0] && arg_125 == arg_106[0] && arg_125 == arg_40[0] && arg_125 == arg_182[0] && arg_125 == arg_117[0] && arg_92[0] == arg_171[0] && arg_92[0] == arg_22[0] && arg_92[0] == arg_18 && arg_92[0] == arg_154 && arg_92[0] == arg_101[0] && arg_92[0] == arg_177 && arg_92[0] == arg_64[0] && arg_92[0] == arg_106[0] && arg_92[0] == arg_40[0] && arg_92[0] == arg_182[0] && arg_92[0] == arg_117[0] && arg_171[0] == arg_22[0] && arg_171[0] == arg_18 && arg_171[0] == arg_154 && arg_171[0] == arg_101[0] && arg_171[0] == arg_177 && arg_171[0] == arg_64[0] && arg_171[0] == arg_106[0] && arg_171[0] == arg_40[0] && arg_171[0] == arg_182[0] && arg_171[0] == arg_117[0] && arg_22[0] == arg_18 && arg_22[0] == arg_154 && arg_22[0] == arg_101[0] && arg_22[0] == arg_177 && arg_22[0] == arg_64[0] && arg_22[0] == arg_106[0] && arg_22[0] == arg_40[0] && arg_22[0] == arg_182[0] && arg_22[0] == arg_117[0] && arg_18 == arg_154 && arg_18 == arg_101[0] && arg_18 == arg_177 && arg_18 == arg_64[0] && arg_18 == arg_106[0] && arg_18 == arg_40[0] && arg_18 == arg_182[0] && arg_18 == arg_117[0] && arg_154 == arg_101[0] && arg_154 == arg_177 && arg_154 == arg_64[0] && arg_154 == arg_106[0] && arg_154 == arg_40[0] && arg_154 == arg_182[0] && arg_154 == arg_117[0] && arg_101[0] == arg_177 && arg_101[0] == arg_64[0] && arg_101[0] == arg_106[0] && arg_101[0] == arg_40[0] && arg_101[0] == arg_182[0] && arg_101[0] == arg_117[0] && arg_177 == arg_64[0] && arg_177 == arg_106[0] && arg_177 == arg_40[0] && arg_177 == arg_182[0] && arg_177 == arg_117[0] && arg_64[0] == arg_106[0] && arg_64[0] == arg_40[0] && arg_64[0] == arg_182[0] && arg_64[0] == arg_117[0] && arg_106[0] == arg_40[0] && arg_106[0] == arg_182[0] && arg_106[0] == arg_117[0] && arg_40[0] == arg_182[0] && arg_40[0] == arg_117[0] && arg_182[0] == arg_117[0] {
let rhs_inst_20 = pos.ins().band_imm(arg_167, 1);
let rhs_inst_21 = pos.ins().bnot(rhs_inst_20);
pos.func.dfg.replace(inst).bnot(rhs_inst_21);

}
}
}
}
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}

/// The main superoptimization baseline pass.
pub fn do_superopt_baseline(func: &mut Function) {
    let _tt = timing::superopt_baseline();
    let mut pos = FuncCursor::new(func);

    while let Some(_block) = pos.next_block() {
        while let Some(inst) = pos.next_inst() {
            superopt_1(&mut pos, inst);
            superopt_2(&mut pos, inst);
            superopt_3(&mut pos, inst);
            superopt_4(&mut pos, inst);
            superopt_5(&mut pos, inst);
            superopt_6(&mut pos, inst);
            superopt_7(&mut pos, inst);
            superopt_8(&mut pos, inst);
            superopt_9(&mut pos, inst);
            superopt_10(&mut pos, inst);
            superopt_11(&mut pos, inst);
            superopt_12(&mut pos, inst);
            superopt_13(&mut pos, inst);
            superopt_14(&mut pos, inst);
            superopt_15(&mut pos, inst);
            superopt_16(&mut pos, inst);
            superopt_17(&mut pos, inst);
            superopt_18(&mut pos, inst);
            superopt_19(&mut pos, inst);
            superopt_20(&mut pos, inst);
            superopt_21(&mut pos, inst);
            superopt_22(&mut pos, inst);
            superopt_23(&mut pos, inst);
            superopt_24(&mut pos, inst);
            superopt_25(&mut pos, inst);
            superopt_26(&mut pos, inst);
            superopt_27(&mut pos, inst);
            superopt_28(&mut pos, inst);
            superopt_29(&mut pos, inst);
            superopt_30(&mut pos, inst);
            superopt_31(&mut pos, inst);
            superopt_32(&mut pos, inst);
            superopt_33(&mut pos, inst);
            superopt_34(&mut pos, inst);
            superopt_35(&mut pos, inst);
            superopt_36(&mut pos, inst);
            superopt_37(&mut pos, inst);
            superopt_38(&mut pos, inst);
            superopt_39(&mut pos, inst);
            superopt_40(&mut pos, inst);
            superopt_41(&mut pos, inst);
            superopt_42(&mut pos, inst);
            superopt_43(&mut pos, inst);
            superopt_44(&mut pos, inst);
            superopt_45(&mut pos, inst);
            superopt_46(&mut pos, inst);
            superopt_47(&mut pos, inst);
            superopt_48(&mut pos, inst);
            superopt_49(&mut pos, inst);
            superopt_50(&mut pos, inst);
            superopt_51(&mut pos, inst);
            superopt_52(&mut pos, inst);
            superopt_53(&mut pos, inst);
            superopt_54(&mut pos, inst);
            superopt_55(&mut pos, inst);
            superopt_56(&mut pos, inst);
            superopt_57(&mut pos, inst);
            superopt_58(&mut pos, inst);
            superopt_59(&mut pos, inst);
            superopt_60(&mut pos, inst);
            superopt_61(&mut pos, inst);
            superopt_62(&mut pos, inst);
            superopt_63(&mut pos, inst);
            superopt_64(&mut pos, inst);
            superopt_65(&mut pos, inst);
            superopt_66(&mut pos, inst);
            superopt_67(&mut pos, inst);
            superopt_68(&mut pos, inst);
            superopt_69(&mut pos, inst);
            superopt_70(&mut pos, inst);
            superopt_71(&mut pos, inst);
            superopt_72(&mut pos, inst);
            superopt_73(&mut pos, inst);
            superopt_74(&mut pos, inst);
            superopt_75(&mut pos, inst);
            superopt_76(&mut pos, inst);
            superopt_77(&mut pos, inst);
            superopt_78(&mut pos, inst);
            superopt_79(&mut pos, inst);
            superopt_80(&mut pos, inst);
            superopt_81(&mut pos, inst);
            superopt_82(&mut pos, inst);
            superopt_83(&mut pos, inst);
            superopt_84(&mut pos, inst);
            superopt_85(&mut pos, inst);
            superopt_86(&mut pos, inst);
            superopt_87(&mut pos, inst);
            superopt_88(&mut pos, inst);
            superopt_89(&mut pos, inst);
            superopt_90(&mut pos, inst);
            superopt_91(&mut pos, inst);
            superopt_92(&mut pos, inst);
            superopt_93(&mut pos, inst);
            superopt_94(&mut pos, inst);
            superopt_95(&mut pos, inst);
            superopt_96(&mut pos, inst);
            superopt_97(&mut pos, inst);
            superopt_98(&mut pos, inst);
            superopt_99(&mut pos, inst);
            superopt_100(&mut pos, inst);
            superopt_101(&mut pos, inst);
            superopt_102(&mut pos, inst);
            superopt_103(&mut pos, inst);
            superopt_104(&mut pos, inst);
            superopt_105(&mut pos, inst);
            superopt_106(&mut pos, inst);
            superopt_107(&mut pos, inst);
            superopt_108(&mut pos, inst);
            superopt_109(&mut pos, inst);
        }
    }
}

