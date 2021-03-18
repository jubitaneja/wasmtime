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
use crate::ir::types::{I32, I64};

fn superopt_1(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == -2 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).bnot(arg_1);

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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == 4611686018427387904 {
if rhs_1 == 0 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
Opcode::BorImm => {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_4 == 1 {
if rhs_3 == 0 {
if arg_1 == arg_3 {
pos.func.dfg.replace(inst).iconst(I64, 1_u64 as i64); 
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
}
},
_ => {},
}
},
_ => {},
}
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 8_u64 as i64);

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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 4 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 3_u64 as i64);

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
Opcode::BandImm => {
if rhs_2 == -2 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).bor_imm(arg_1, 1_u64 as i64);

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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 8 {
if rhs_1 == 7 {
pos.func.dfg.replace(inst).band_imm(arg_1, 7_u64 as i64);

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
Opcode::IaddImm => {
if rhs_2 == -1 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 1_u64 as i64);

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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_2 == 1 {
if rhs_1 == 0 {
let rhs_inst_4 = pos.ins().iconst(I64, 1_u64 as i64);
pos.func.dfg.replace(inst).bor_not(rhs_inst_4, arg_1);

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
if rhs_1 == 1 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 2_u64 as i64);

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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -8 {
if rhs_1 == 7 {
pos.func.dfg.replace(inst).band_imm(arg_1, 7_u64 as i64);

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
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == 2 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1, rep_insts_0);
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
fn superopt_13(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == 3 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1, rep_insts_0);
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
fn superopt_14(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -2 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 1_u64 as i64);

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
fn superopt_15(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_2 == 3 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 12_u64 as i64);

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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 12 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 24_u64 as i64);

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
Opcode::IaddImm => {
if rhs_2 == -1 {
if rhs_1 == -1 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 0_u64 as i64);

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
Opcode::IrsubImm => {
if rhs_2 == 2 {
if rhs_1 == 16 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 18_u64 as i64);

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
fn superopt_19(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_1 == -512 {
if arg_0[0] == arg_1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 511_u64 as i64);

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
fn superopt_20(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == 6 {
if rhs_1 == 16 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 22_u64 as i64);

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
fn superopt_21(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -68 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_1, 3_u64 as i64);

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
fn superopt_22(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -12 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 1_u64 as i64);

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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 63 {
if rhs_1 == -1 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 62_u64 as i64);

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
fn superopt_24(pos: &mut FuncCursor, inst: Inst) {
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
fn superopt_25(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 17 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 18_u64 as i64);

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
fn superopt_26(pos: &mut FuncCursor, inst: Inst) {
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
fn superopt_27(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -142 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 1_u64 as i64);

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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 64 {
if rhs_1 == -1 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 63_u64 as i64);

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
fn superopt_29(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 4 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).sshr_imm(arg_1, 5_u64 as i64);

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
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 2 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).sshr_imm(arg_1, 3_u64 as i64);

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
fn superopt_31(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 3 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).sshr_imm(arg_1, 4_u64 as i64);

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
fn superopt_32(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 16 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 17_u64 as i64);

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
fn superopt_33(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1900 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_1, 3_u64 as i64);

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
fn superopt_34(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 3 {
let rhs_inst_4 = pos.ins().iconst(I32, 3_u64 as i64);
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
fn superopt_35(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -454 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 1_u64 as i64);

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
fn superopt_36(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Unary { opcode, arg } => {
let arg_2 = arg;
match opcode {
Opcode::Clz => {
if rhs_2 == 64 {
if rhs_1 == 4294967295 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_37(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 2 {
if rhs_1 == 2 {
let rhs_inst_4 = pos.ins().iconst(I32, 3_u64 as i64);
pos.func.dfg.replace(inst).band_not(arg_1, rhs_inst_4);

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
if rhs_1 == 4 {
let rhs_inst_4 = pos.ins().iconst(I32, 4_u64 as i64);
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
fn superopt_39(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
let rhs_inst_4 = pos.ins().iconst(I64, 1_u64 as i64);
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
fn superopt_40(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Unary { opcode, arg } => {
let arg_2 = arg;
match opcode {
Opcode::Ctz => {
if rhs_2 == 64 {
if rhs_1 == 4294967295 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_42(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_4 = pos.ins().iconst(I32, 1_u64 as i64);
pos.func.dfg.replace(inst).band_not(arg_1, rhs_inst_4);

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
fn superopt_43(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_44(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_45(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_46(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -35 {
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
fn superopt_47(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -19 {
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
fn superopt_48(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Ishl => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Sshr => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Ishl => {
if arg_2[1] == arg_1[1] && arg_2[1] == arg_0[1] && arg_2[1] != arg_2[0] && arg_1[1] == arg_0[1] && arg_1[1] != arg_2[0] && arg_0[1] != arg_2[0] {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1[0], rep_insts_0);
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
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
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_50(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 7 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_51(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_52(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 7 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_53(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 6 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_54(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 5 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_55(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_56(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 26 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_57(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 33 {
let rhs_inst_4 = pos.ins().iconst(I64, 33_u64 as i64);
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
fn superopt_58(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 15 {
if rhs_1 == 15 {
let rhs_inst_4 = pos.ins().iconst(I32, 15_u64 as i64);
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
Opcode::IaddImm => {
if rhs_2 == -143 {
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
fn superopt_60(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 15 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_61(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 13 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_62(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 31 {
if rhs_1 == 31 {
let rhs_inst_4 = pos.ins().iconst(I32, 31_u64 as i64);
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
fn superopt_63(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -544 {
if rhs_1 == -2 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 542_u64 as i64);

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
fn superopt_64(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 18 {
if rhs_1 == 63 {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 63_u64 as i64);

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
fn superopt_65(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 63 {
if rhs_1 == 63 {
let rhs_inst_4 = pos.ins().iconst(I32, 63_u64 as i64);
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
fn superopt_66(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
if arg_1[1] != arg_0[1] && arg_1[1] != arg_1[0] && arg_0[1] == arg_1[0] {
pos.func.dfg.replace(inst).band_not(arg_1[0], arg_1[1]);

}
},
_ => {},
}
},
_ => {},
}
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
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
if arg_0[1] == arg_1[1] && arg_0[1] != arg_1[0] && arg_1[1] != arg_1[0] {
pos.func.dfg.replace(inst).band_not(arg_1[1], arg_1[0]);

}
},
_ => {},
}
},
_ => {},
}
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
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
if arg_0[1] == arg_1[0] && arg_0[1] != arg_1[1] && arg_1[0] != arg_1[1] {
pos.func.dfg.replace(inst).band_not(arg_1[0], arg_1[1]);

}
},
_ => {},
}
},
_ => {},
}
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
if arg_1 != arg_0[0] {
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
fn superopt_70(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bxor => {
if arg_1[1] == arg_0[1] && arg_1[1] != arg_1[0] && arg_0[1] != arg_1[0] {
pos.func.dfg.replace(inst).band_not(arg_1[1], arg_1[0]);

}
},
_ => {},
}
},
_ => {},
}
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
Opcode::IrsubImm => {
if rhs_1 == 0 {
if arg_0[1] != arg_1 {
pos.func.dfg.replace(inst).isub(arg_0[1], arg_1);

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
fn superopt_72(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 255 {
let rhs_inst_4 = pos.ins().iconst(I32, 255_u64 as i64);
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
fn superopt_73(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 144 {
let rhs_inst_4 = pos.ins().iconst(I64, 144_u64 as i64);
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
fn superopt_74(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 128 {
let rhs_inst_4 = pos.ins().iconst(I64, 128_u64 as i64);
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
fn superopt_75(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 257 {
let rhs_inst_4 = pos.ins().iconst(I64, 257_u64 as i64);
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
fn superopt_76(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4_u64 as i64);

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
fn superopt_77(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_0[1] != arg_1 {
pos.func.dfg.replace(inst).bor_not(arg_0[1], arg_1);

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
fn superopt_78(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 8_u64 as i64);

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
fn superopt_79(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 1073741823 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4_u64 as i64);

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
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_0[0] != arg_1 {
pos.func.dfg.replace(inst).bor_not(arg_1, arg_1);

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
fn superopt_81(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967288_u64 as i64);

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
fn superopt_82(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967280_u64 as i64);

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
fn superopt_83(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967292_u64 as i64);

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
fn superopt_84(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967292_u64 as i64);

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
fn superopt_85(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967288_u64 as i64);

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
fn superopt_86(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967294_u64 as i64);

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
fn superopt_87(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 31 {
if arg_1 != arg_0[0] {
pos.func.dfg.replace(inst).ushr(arg_1, arg_1);

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
fn superopt_88(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 6 {
if rhs_1 == 6 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967232_u64 as i64);

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
fn superopt_89(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 3 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967288_u64 as i64);

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
fn superopt_90(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_2 == -1073741824 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4_u64 as i64);

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
fn superopt_91(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 4 {
if rhs_1 == 4 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967280_u64 as i64);

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
fn superopt_92(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 5 {
if rhs_1 == 5 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967264_u64 as i64);

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
fn superopt_93(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 5 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 32_u64 as i64);

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
fn superopt_94(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967294_u64 as i64);

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
fn superopt_95(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_2 == 8 {
if rhs_1 == 8 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967040_u64 as i64);

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
fn superopt_96(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 24 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967272_u64 as i64);

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
fn superopt_97(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967292_u64 as i64);

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
fn superopt_98(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 5 {
if rhs_1 == 5 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967264_u64 as i64);

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
fn superopt_99(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 4096 {
let rhs_inst_4 = pos.ins().iconst(I64, 4096_u64 as i64);
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
fn superopt_100(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967288_u64 as i64);

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
        }
    }
}

