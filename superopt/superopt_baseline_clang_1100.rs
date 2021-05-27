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
if arg_0[1] == arg_1[1] && arg_0[1] != arg_2[0] && arg_0[1] == arg_2[1] && arg_1[1] != arg_2[0] && arg_1[1] == arg_2[1] && arg_2[0] != arg_2[1] {
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
if arg_1[0] == arg_0[1] && arg_1[0] != arg_1[1] && arg_0[1] != arg_1[1] {
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
if arg_1[0] == arg_0[1] && arg_1[0] != arg_1[1] && arg_0[1] != arg_1[1] {
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
if arg_1[1] != arg_1[0] && arg_1[1] == arg_0[1] && arg_1[0] != arg_0[1] {
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
if arg_1 != arg_0[1] {
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
if arg_1 != arg_0[1] {
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
if arg_0[0] != arg_1 {
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
fn superopt_101(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 12 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967284_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
if rhs_1 == 4224 {
let rhs_inst_4 = pos.ins().iconst(I64, 4224_u64 as i64);
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
fn superopt_103(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 28 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 28_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
Opcode::Band => {
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
pos.func.dfg.replace(inst).band_not(arg_1, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -65536 {
if rhs_1 == 1023 {
pos.func.dfg.replace(inst).band_imm(arg_1, 1023_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1073741823 {
if rhs_1 == 80 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 80_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 12 {
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
fn superopt_108(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 40 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 40_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_1 != arg_0[0] {
pos.func.dfg.replace(inst).band_not(arg_1, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_110(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 24 {
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
fn superopt_111(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == -2 {
if rhs_1 == -3 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 4294967295_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_112(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == -7 {
pos.func.dfg.replace(inst).irsub_imm(arg_1, 4294967288_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_113(pos: &mut FuncCursor, inst: Inst) {
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
fn superopt_114(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 255 {
if arg_0[0] != arg_1 {
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
fn superopt_115(pos: &mut FuncCursor, inst: Inst) {
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
if arg_0[1] != arg_1 {
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
fn superopt_116(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 127 {
if rhs_1 == -128 {
pos.func.dfg.replace(inst).bor_imm(arg_1, 4294967168_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_117(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -1 {
if rhs_1 == -8 {
let rhs_inst_4 = pos.ins().iconst(I32, 4294967288_u64 as i64);
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
fn superopt_118(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -1073741824 {
if rhs_1 == 31 {
pos.func.dfg.replace(inst).band_imm(arg_1, 31_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_119(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4352 {
if rhs_1 == 4352 {
let rhs_inst_4 = pos.ins().iconst(I64, 4352_u64 as i64);
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
fn superopt_120(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 65535 {
let rhs_inst_4 = pos.ins().iconst(I32, 65535_u64 as i64);
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
fn superopt_121(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == 24 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4278190080_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_122(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32767 {
if rhs_1 == -1 {
let rhs_inst_4 = pos.ins().iconst(I32, 4294934528_u64 as i64);
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
fn superopt_123(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 65535 {
if rhs_1 == -1 {
let rhs_inst_4 = pos.ins().iconst(I32, 4294901760_u64 as i64);
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
fn superopt_124(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2097280 {
let rhs_inst_4 = pos.ins().iconst(I64, 2097280_u64 as i64);
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
fn superopt_125(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Ishl => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_1 == 4294967295 {
if arg_1 != arg_0[0] {
pos.func.dfg.replace(inst).ishl(arg_1, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_126(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2097168 {
let rhs_inst_4 = pos.ins().iconst(I64, 2097168_u64 as i64);
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
fn superopt_127(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 134217727 {
if arg_0[0] != arg_1 {
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
fn superopt_128(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1 {
if rhs_1 == 0 {
if arg_0[1] == arg_2 {
pos.func.dfg.replace(inst).imul(arg_2, arg_1);

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
fn superopt_129(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1 {
if arg_1 == arg_2 {
pos.func.dfg.replace(inst).isub(arg_2, arg_0[0]);

}
}
},
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
fn superopt_130(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1 {
if arg_1 == arg_2 {
pos.func.dfg.replace(inst).isub(arg_2, arg_0[0]);

}
}
},
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
fn superopt_131(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 67108992 {
let rhs_inst_4 = pos.ins().iconst(I64, 67108992_u64 as i64);
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
fn superopt_132(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4294967295 {
if rhs_1 == 32 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967296_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_133(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).imul_imm(arg_1, 18446744073709551608_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_134(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).imul_imm(arg_1, 18446744073709551614_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_135(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_1 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1 {
if arg_1 == arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 3_u64 as i64);

}
}
},
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
fn superopt_136(pos: &mut FuncCursor, inst: Inst) {
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
pos.func.dfg.replace(inst).band_imm(arg_1, 18446744073709551608_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_137(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 134217727 {
if rhs_1 == -1 {
let rhs_inst_4 = pos.ins().iconst(I32, 4160749568_u64 as i64);
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
fn superopt_138(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 0 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 18446744069414584320_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_139(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_1 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if arg_1 == arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 12_u64 as i64);

}
}
},
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
fn superopt_140(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -1 {
if rhs_1 == -2147483648 {
let rhs_inst_4 = pos.ins().iconst(I32, 2147483648_u64 as i64);
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
fn superopt_141(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 3_u64 as i64);

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
fn superopt_142(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Unary { opcode, arg } => {
let arg_3 = arg;
match opcode {
Opcode::Clz => {
if rhs_3 == 1 {
if rhs_2 == 64 {
if rhs_1 == 31870 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_143(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 2_u64 as i64);

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
fn superopt_144(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 255 {
if rhs_1 == 8 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_5 == 255 {
if arg_2 == arg_3 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 257_u64 as i64);

}
}
},
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
fn superopt_145(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 255 {
if rhs_1 == -1 {
let rhs_inst_4 = pos.ins().iconst(I64, 18446744073709551360_u64 as i64);
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
fn superopt_146(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 16 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 17_u64 as i64);

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
fn superopt_147(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 63 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 64_u64 as i64);

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
fn superopt_148(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 4 {
if rhs_2 == 3 {
if rhs_1 == 4 {
pos.func.dfg.replace(inst).band_imm(arg_2, 48_u64 as i64);

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
fn superopt_149(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_2, 24_u64 as i64);

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
fn superopt_150(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_3 == -4 {
if rhs_2 == 2 {
if rhs_1 == 2 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1, rep_insts_0);
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
fn superopt_151(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 6 {
if rhs_2 == 7 {
if rhs_1 == 6 {
pos.func.dfg.replace(inst).band_imm(arg_2, 448_u64 as i64);

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
fn superopt_152(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
if rhs_2 == 4294967296 {
if rhs_1 == 4294967295 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967295_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_153(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 4 {
if rhs_3 == -32 {
if rhs_2 == 4 {
if rhs_1 == 4 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1, rep_insts_0);
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
fn superopt_154(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 549755818112 {
let rhs_inst_4 = pos.ins().iconst(I64, 549755818112_u64 as i64);
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
fn superopt_155(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
if rhs_2 == 42949672960 {
if rhs_1 == 4294967295 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967295_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_156(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
if rhs_2 == 25769803776 {
if rhs_1 == 4294967295 {
pos.func.dfg.replace(inst).band_imm(arg_1, 4294967295_u64 as i64);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_157(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2748779069440 {
let rhs_inst_4 = pos.ins().iconst(I64, 2748779069440_u64 as i64);
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
fn superopt_158(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2199023255808 {
let rhs_inst_4 = pos.ins().iconst(I64, 2199023255808_u64 as i64);
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
fn superopt_159(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 28 {
if rhs_2 == 28 {
if rhs_1 == 2 {
pos.func.dfg.replace(inst).sshr_imm(arg_1, 26_u64 as i64);

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
fn superopt_160(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_1 == -8 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 4 {
if arg_1 == arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 4294967292_u64 as i64);

}
}
},
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
fn superopt_161(pos: &mut FuncCursor, inst: Inst) {
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
if arg_1 == arg_2 {
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
fn superopt_162(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 32 {
if rhs_2 == 2 {
if rhs_1 == 32 {
pos.func.dfg.replace(inst).ushr_imm(arg_2, 62_u64 as i64);

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
fn superopt_163(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 32 {
if rhs_2 == 3 {
if rhs_1 == 32 {
pos.func.dfg.replace(inst).ushr_imm(arg_2, 61_u64 as i64);

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
fn superopt_164(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 17 {
if rhs_3 == 63 {
if rhs_2 == 17 {
if rhs_1 == 255 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_165(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 37383395344384 {
let rhs_inst_4 = pos.ins().iconst(I64, 37383395344384_u64 as i64);
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
fn superopt_166(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2147483647 {
if rhs_1 == 2147483647 {
let rhs_inst_4 = pos.ins().iconst(I32, 2147483647_u64 as i64);
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
fn superopt_167(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1 {
if rhs_1 == 0 {
if arg_2 != arg_0[0] {
pos.func.dfg.replace(inst).imul(arg_2, arg_1);

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
fn superopt_168(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1 {
if rhs_1 == 0 {
if arg_0[1] != arg_2 {
pos.func.dfg.replace(inst).imul(arg_0[1], arg_1);

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
fn superopt_169(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 13 {
if rhs_2 == 3 {
if rhs_1 == 13 {
pos.func.dfg.replace(inst).band_imm(arg_2, 24576_u64 as i64);

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
fn superopt_170(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == 8191 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_2, 65528_u64 as i64);

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
fn superopt_171(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
if rhs_1 == 8 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 0 {
if arg_1 != arg_2 {
pos.func.dfg.replace(inst).isub(arg_0[0], arg_2);

}
}
},
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
fn superopt_172(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 4 {
if rhs_2 == 1023 {
if rhs_1 == 4 {
pos.func.dfg.replace(inst).band_imm(arg_2, 16368_u64 as i64);

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
fn superopt_173(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
if rhs_1 == 8 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_174(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == -8 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_175(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 127 {
if rhs_2 == -128 {
if rhs_1 == 255 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 128_u64 as i64);

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
fn superopt_176(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

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
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_3 && arg_1 == arg_2[0] && arg_3 != arg_2[0] {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_3, rep_insts_0);
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
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
fn superopt_177(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
if arg_2 != arg_1[1] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_1[1]);

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
fn superopt_178(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -4 {
if rhs_1 == 4 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_179(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -1 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_180(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 422212465065984 {
let rhs_inst_4 = pos.ins().iconst(I64, 422212465065984_u64 as i64);
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
fn superopt_181(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -1 {
if rhs_1 == -1 {
if arg_2 != arg_0[1] {
pos.func.dfg.replace(inst).isub(arg_0[1], arg_2);

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
fn superopt_182(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 5 {
if rhs_1 == -1 {
if arg_2 != arg_0[0] {
pos.func.dfg.replace(inst).bor_not(arg_2, arg_1);

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
fn superopt_183(pos: &mut FuncCursor, inst: Inst) {
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
if arg_2 != arg_0[0] {
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
fn superopt_184(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 8 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 8_u64 as i64);

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
fn superopt_185(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -96 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_186(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 4095 {
if rhs_1 == 8 {
pos.func.dfg.replace(inst).band_imm(arg_2, 1048320_u64 as i64);

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
fn superopt_187(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_1 == 63 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 63 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_4[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_5 == 63 {
match pos.func.dfg.value_def(arg_4[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_6 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_7 == 63 {
if arg_3 != arg_5 && arg_3 != arg_2 && arg_3 == arg_6 && arg_5 == arg_2 && arg_5 != arg_6 && arg_2 != arg_6 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0[0], rep_insts_0);
}
}
},
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
}
},
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
fn superopt_188(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 92 {
if rhs_1 == -92 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_189(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -12 {
if rhs_1 == 12 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_190(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 8 {
if rhs_1 == 7 {
if arg_2[0] != arg_2[1] {
pos.func.dfg.replace(inst).band_imm(arg_1, 7_u64 as i64);

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
fn superopt_191(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 1 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 1_u64 as i64);

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
fn superopt_192(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == -2 {
if arg_3 != arg_2[1] {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_193(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 40 {
if rhs_1 == -40 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_194(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
if rhs_1 == 3 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
pos.func.dfg.replace(inst).bor_not(arg_0[0], arg_2);

}
}
},
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
fn superopt_195(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == -16 {
if arg_2 != arg_1[1] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_1[1]);

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
fn superopt_196(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -16 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_197(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 88 {
if rhs_1 == -88 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_198(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_3 == 1 {
if rhs_2 == 0 {
if rhs_1 == 12 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967284_u64 as i64);

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
fn superopt_199(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -56 {
if rhs_1 == 56 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_200(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 28 {
if rhs_1 == -28 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_201(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == -48 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_202(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
if rhs_1 == 16 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_203(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 2 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).band_imm(arg_2, 2_u64 as i64);

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
fn superopt_204(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 32 {
if rhs_1 == 1 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 1_u64 as i64);

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
fn superopt_205(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == -24 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).iadd(arg_2, arg_2);

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
fn superopt_206(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 23 {
if rhs_1 == -1 {
if arg_2 != arg_0[1] {
pos.func.dfg.replace(inst).bor_not(arg_0[1], arg_1);

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
fn superopt_207(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 6755399441055744 {
let rhs_inst_4 = pos.ins().iconst(I64, 6755399441055744_u64 as i64);
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
fn superopt_208(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 32 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).band_imm(arg_2, 32_u64 as i64);

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
fn superopt_209(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 7 {
if rhs_1 == -1 {
if arg_0[1] != arg_2 {
pos.func.dfg.replace(inst).band_not(arg_0[1], arg_1);

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
fn superopt_210(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_2 == 5 {
if rhs_1 == 31 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 31_u64 as i64);

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
fn superopt_211(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 223 {
if rhs_2 == 32 {
if rhs_1 == -97 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 4294967231_u64 as i64);

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
fn superopt_212(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Ushr => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_1 == 255 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if arg_1 != arg_2 {
pos.func.dfg.replace(inst).sshr(arg_0[0], arg_2);

}
}
},
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
fn superopt_213(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 5 {
if rhs_2 == -1 {
if rhs_1 == 5 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967264_u64 as i64);
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
fn superopt_214(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 512 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 512_u64 as i64);

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
fn superopt_215(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 6 {
if rhs_2 == -1 {
if rhs_1 == 6 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967232_u64 as i64);
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
fn superopt_216(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 128 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 128_u64 as i64);

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
fn superopt_217(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 256 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 256_u64 as i64);

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
fn superopt_218(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == -8 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967288_u64 as i64);
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
fn superopt_219(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 4 {
if rhs_2 == -1 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967280_u64 as i64);
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
fn superopt_220(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967292_u64 as i64);
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
fn superopt_221(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967294_u64 as i64);
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
fn superopt_222(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 39 {
if rhs_1 == 7 {
if arg_2[0] != arg_2[1] {
let rhs_inst_6 = pos.ins().iconst(I32, 7_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_6, arg_1);

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
fn superopt_223(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967288_u64 as i64);
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
fn superopt_224(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 8 {
if rhs_1 == 31 {
if arg_2[1] != arg_2[0] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_225(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 6 {
if rhs_1 == 31 {
if arg_2[0] != arg_2[1] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_226(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 4 {
if rhs_1 == 31 {
if arg_2[0] != arg_2[1] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_227(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 5 {
if rhs_1 == 31 {
if arg_2[1] != arg_2[0] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_228(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 4294967295 {
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_5 == 4294967295 {
if arg_3 == arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967297_u64 as i64);

}
}
},
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
fn superopt_229(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 3 {
if rhs_1 == 1073741823 {
if arg_0[1] == arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 1073741820_u64 as i64);

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
fn superopt_230(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 2 {
if rhs_1 == 31 {
if arg_2[1] != arg_2[0] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_231(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 16 {
if rhs_2 == 32767 {
if rhs_1 == 16 {
pos.func.dfg.replace(inst).band_imm(arg_2, 2147418112_u64 as i64);

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
fn superopt_232(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
if arg_1[1] == arg_0[1] && arg_1[1] != arg_2[1] && arg_1[1] != arg_2[0] && arg_0[1] != arg_2[1] && arg_0[1] != arg_2[0] && arg_2[1] != arg_2[0] {
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
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_233(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 15 {
if rhs_1 == 31 {
if arg_2[1] != arg_2[0] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_234(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 11 {
if rhs_1 == 31 {
if arg_2[0] != arg_2[1] {
pos.func.dfg.replace(inst).ushr_imm(arg_1, 31_u64 as i64);

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
fn superopt_235(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 134217727 {
if rhs_1 == 1 {
pos.func.dfg.replace(inst).band_imm(arg_2, 268435454_u64 as i64);

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
fn superopt_236(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
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
Opcode::IaddImm => {
if rhs_4 == -1 {
if rhs_3 == -4 {
if rhs_2 == 3 {
if rhs_1 == 4 {
pos.func.dfg.replace(inst).bnot(arg_1);

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
fn superopt_237(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 65031 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).band_imm(arg_2, 65031_u64 as i64);

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
fn superopt_238(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 268435455 {
if rhs_2 == 0 {
if rhs_1 == 12 {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967284_u64 as i64);

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
fn superopt_239(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 65535 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 65535_u64 as i64);

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
fn superopt_240(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 16384 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).band_imm(arg_2, 16384_u64 as i64);

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
fn superopt_241(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_1 == -1 {
if arg_2[0] != arg_2[1] && arg_2[0] != arg_0[0] && arg_2[1] != arg_0[0] {
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
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_242(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_2 == 0 {
if rhs_1 == 12 {
if arg_2[0] != arg_2[1] {
pos.func.dfg.replace(inst).imul_imm(arg_1, 4294967284_u64 as i64);

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
fn superopt_243(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 196608 {
if rhs_1 == 1073741823 {
if arg_0[1] == arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 1073545215_u64 as i64);

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
fn superopt_244(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 32 {
if rhs_1 == 32 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 4294967296_u64 as i64);

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
fn superopt_245(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_1 == -1 {
if arg_2[0] != arg_2[1] && arg_2[0] != arg_0[0] && arg_2[1] != arg_0[0] {
pos.func.dfg.replace(inst).band_not(arg_1, arg_1);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_246(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 32 {
if rhs_1 == 4294967295 {
if arg_3 != arg_2 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_1[0], rep_insts_0);
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
fn superopt_247(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_1 == -1 {
if arg_2[1] != arg_0[1] && arg_2[1] != arg_2[0] && arg_0[1] != arg_2[0] {
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
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_248(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_249(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_3 == 37 {
if rhs_2 == 4294967295 {
if rhs_1 == 32 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 158913789952_u64 as i64);

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
fn superopt_250(pos: &mut FuncCursor, inst: Inst) {
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
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_251(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 8_u64 as i64);

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
fn superopt_252(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 7 {
if rhs_2 == 3 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 8_u64 as i64);

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
fn superopt_253(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == 7 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 1_u64 as i64);
let rhs_inst_6 = pos.ins().iconst(I32, 7_u64 as i64);
pos.func.dfg.replace(inst).sshr(rhs_inst_6, rhs_inst_5);

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
fn superopt_254(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 49 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 562949953421312_u64 as i64);

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
fn superopt_255(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 24_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_256(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 28_u64 as i64);

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
fn superopt_257(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 48 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 281474976710656_u64 as i64);

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
fn superopt_258(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_1 == 7 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == -8 {
if arg_2 != arg_1[1] && arg_2 == arg_3 && arg_1[1] != arg_3 {
pos.func.dfg.replace(inst).bor(arg_3, arg_1[1]);

}
}
},
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
fn superopt_259(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_260(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 2_u64 as i64);

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
fn superopt_261(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 1_u64 as i64);
let rhs_inst_6 = pos.ins().iconst(I32, 7_u64 as i64);
pos.func.dfg.replace(inst).sshr(rhs_inst_6, rhs_inst_5);

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
fn superopt_262(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 53 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 9007199254740992_u64 as i64);

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
fn superopt_263(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 51 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 2251799813685248_u64 as i64);

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
fn superopt_264(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 4 {
if rhs_2 == 3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_265(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 52 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 4503599627370496_u64 as i64);

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
fn superopt_266(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 6 {
if rhs_2 == 7 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 28_u64 as i64);

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
fn superopt_267(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 5 {
if rhs_2 == 3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_268(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 255 {
if rhs_2 == 2 {
if rhs_1 == 28 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 7_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_269(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 7 {
if rhs_2 == 7 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 5_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 28_u64 as i64);

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
fn superopt_270(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_271(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 6 {
if rhs_2 == 3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_272(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 11 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 9_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_273(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 3 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 24_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 36_u64 as i64);

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
fn superopt_274(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 3 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_5, 3_u64 as i64);

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
fn superopt_275(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 5 {
if rhs_2 == 7 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 56_u64 as i64);

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
fn superopt_276(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 24_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 24_u64 as i64);

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
fn superopt_277(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 80 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 20_u64 as i64);

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
fn superopt_278(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 50 {
if arg_2 != arg_1[0] {
pos.func.dfg.replace(inst).imul_imm(arg_2, 1125899906842624_u64 as i64);

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
fn superopt_279(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == 64 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().ushr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 8_u64 as i64);

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
fn superopt_280(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 6 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_281(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 4 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_282(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 4_u64 as i64);

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
fn superopt_283(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 8_u64 as i64);

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
fn superopt_284(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if rhs_2 == 128 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_285(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 254 {
if rhs_2 == 1 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_286(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 56 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 72057594037927936_u64 as i64);

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
fn superopt_287(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 17 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 15_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_288(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 254 {
if rhs_2 == 1 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 8_u64 as i64);

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
fn superopt_289(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 2 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 2_u64 as i64);

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
fn superopt_290(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 55 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 36028797018963968_u64 as i64);

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
fn superopt_291(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 54 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 18014398509481984_u64 as i64);

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
fn superopt_292(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 10 {
if rhs_2 == 3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_293(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 15 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 60_u64 as i64);

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
fn superopt_294(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 5 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 5_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_295(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_1 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
if arg_3 == arg_2 && arg_3 != arg_1[0] && arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_3, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_6);

}
}
},
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
fn superopt_296(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 7 {
if rhs_1 == 7 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 7_u64 as i64);

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
fn superopt_297(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_1 == 7 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 7 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 7_u64 as i64);

}
}
},
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
fn superopt_298(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 3 {
if rhs_1 == 2 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_1[1]);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 2_u64 as i64);

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
fn superopt_299(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 7 {
if rhs_1 == 1 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_300(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 13 {
if rhs_2 == 7 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 11_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 28_u64 as i64);

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
fn superopt_301(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 2 {
if rhs_2 == 5 {
if rhs_1 == 64 {
let rhs_inst_5 = pos.ins().iconst(I32, 2_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 32_u64 as i64);

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
fn superopt_302(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 15 {
if rhs_2 == 1 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 7_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 256_u64 as i64);

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
fn superopt_303(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 1 {
if rhs_1 == 13 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 256_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 32_u64 as i64);

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
fn superopt_304(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

}
}
},
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
fn superopt_305(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 252 {
if rhs_2 == 5 {
if rhs_1 == 237 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 232_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 5_u64 as i64);

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
fn superopt_306(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 32 {
if rhs_1 == 4294966271 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).band_imm(arg_2, 4294966271_u64 as i64);

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
fn superopt_307(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 57 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 144115188075855872_u64 as i64);

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
fn superopt_308(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 15 {
if rhs_2 == 1 {
if rhs_1 == 9 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 512_u64 as i64);

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
fn superopt_309(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 7 {
if rhs_1 == 2 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().bor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 2_u64 as i64);

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
fn superopt_310(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 16 {
if rhs_1 == 63 {
if arg_1[0] != arg_2 {
pos.func.dfg.replace(inst).imul_imm(arg_2, 9223372036854775808_u64 as i64);

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
fn superopt_311(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 64 {
let rhs_inst_5 = pos.ins().iconst(I32, 16_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 4_u64 as i64);

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
fn superopt_312(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 2 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 16_u64 as i64);

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
fn superopt_313(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().bxor_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_5);

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
fn superopt_314(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 124_u64 as i64);

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
fn superopt_315(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().bxor_imm(arg_2, 4294967288_u64 as i64);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_5);

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
fn superopt_316(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 254 {
if rhs_2 == 1 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 16_u64 as i64);

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
fn superopt_317(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 254 {
if rhs_2 == 1 {
if rhs_1 == 32 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 32_u64 as i64);

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
fn superopt_318(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 65535 {
if rhs_2 == 9 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 9_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_319(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 65535 {
if rhs_2 == 8 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_320(pos: &mut FuncCursor, inst: Inst) {
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
if arg_0[0] == arg_2 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_321(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 9_u64 as i64);
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
fn superopt_322(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 2 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 3_u64 as i64);
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
fn superopt_323(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 127 {
if rhs_2 == -16 {
if rhs_1 == 31 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 16_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 31_u64 as i64);

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
fn superopt_324(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 3_u64 as i64);
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
fn superopt_325(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 16 {
if rhs_2 == 4 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 65535_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_326(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 7_u64 as i64);
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
fn superopt_327(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1048575 {
if rhs_2 == 1072693248 {
if rhs_1 == -1048576 {
pos.func.dfg.replace(inst).iadd_imm(arg_1, 1071644672_u64 as i64);

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
fn superopt_328(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().iconst(I32, 33_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 3_u64 as i64);

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
fn superopt_329(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 9_u64 as i64);
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
fn superopt_330(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 5 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 45_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 5_u64 as i64);

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
fn superopt_331(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 17 {
if rhs_2 == 31 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 14_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 248_u64 as i64);

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
fn superopt_332(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2_u64 as i64);
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
fn superopt_333(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 12 {
if rhs_2 == 63 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 10_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 252_u64 as i64);

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
fn superopt_334(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 25 {
if rhs_2 == 63 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 23_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 252_u64 as i64);

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
fn superopt_335(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 2 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
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
fn superopt_336(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 4 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 65535_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_337(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == 1 {
if rhs_1 == 4 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 3_u64 as i64);
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
fn superopt_338(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 5_u64 as i64);
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
fn superopt_339(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_1 == 3 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 8_u64 as i64);

}
}
},
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
fn superopt_340(pos: &mut FuncCursor, inst: Inst) {
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
if arg_1[0] == arg_2 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 4294967295_u64 as i64);
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
fn superopt_341(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 6_u64 as i64);
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
fn superopt_342(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 12_u64 as i64);
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
fn superopt_343(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 0 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4294967294_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_344(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 558_u64 as i64);
let rhs_inst_6 = pos.ins().iconst(I32, 12_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_6, rhs_inst_5);

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
fn superopt_345(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == 1 {
if rhs_1 == 124 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 1_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 124_u64 as i64);

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
fn superopt_346(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 5 {
if rhs_2 == 1023 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4092_u64 as i64);

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
fn superopt_347(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 65535 {
if rhs_2 == 10 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 10_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_348(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 9 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 721_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 9_u64 as i64);

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
fn superopt_349(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == -2 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 6_u64 as i64);
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
fn superopt_350(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 10_u64 as i64);
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
fn superopt_351(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().bor_not(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_352(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 8 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 349_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 8_u64 as i64);

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
fn superopt_353(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == -4 {
if arg_2 != arg_3 {
pos.func.dfg.replace(inst).iadd(arg_3, arg_1[1]);

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
fn superopt_354(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == 3 {
if arg_0[0] != arg_3 {
pos.func.dfg.replace(inst).iadd(arg_3, arg_2);

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
fn superopt_355(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 65535 {
if rhs_2 == 11 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 11_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_356(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2 == arg_1[0] {
let rhs_inst_5 = pos.ins().iconst(I32, 2290649224_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_357(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 1 {
if rhs_1 == -1 {
if arg_2[1] != arg_2[0] {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2[1], 0_u64 as i64);
pos.func.dfg.replace(inst).isub(rhs_inst_6, arg_2[0]);

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
fn superopt_358(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 2147483646 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_6 == 30 {
if rhs_5 == 31 {
if arg_2 == arg_4 {
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
}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_359(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 11_u64 as i64);
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
fn superopt_360(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 18 {
if rhs_2 == 127 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 16_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 508_u64 as i64);

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
fn superopt_361(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 4 {
if rhs_2 == 1023 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4092_u64 as i64);

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
fn superopt_362(pos: &mut FuncCursor, inst: Inst) {
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
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 18_u64 as i64);
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
fn superopt_363(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 16 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_364(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 72 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 80_u64 as i64);
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
fn superopt_365(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 32_u64 as i64);
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
fn superopt_366(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 4 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 36_u64 as i64);
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
fn superopt_367(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == 1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17_u64 as i64);
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
fn superopt_368(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_369(pos: &mut FuncCursor, inst: Inst) {
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
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17_u64 as i64);
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
fn superopt_370(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 16 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 18_u64 as i64);
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
fn superopt_371(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 71 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 79_u64 as i64);
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
fn superopt_372(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 20 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_373(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 5 {
if rhs_1 == -1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_5 == 1 {
if arg_2 != arg_3 {
pos.func.dfg.replace(inst).iadd(arg_3, arg_1);

}
}
},
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
fn superopt_374(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 24 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 27_u64 as i64);
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
fn superopt_375(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 59 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 67_u64 as i64);
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
fn superopt_376(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 128 {
if rhs_2 == 7 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 198_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 7_u64 as i64);

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
fn superopt_377(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 48 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_378(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 36_u64 as i64);
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
fn superopt_379(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 20 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 23_u64 as i64);
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
fn superopt_380(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 40_u64 as i64);
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
fn superopt_381(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 33 {
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_6 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_6 == 1 {
if rhs_5 == 4294967295 {
if arg_2 == arg_4 {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_3, rep_insts_0);
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_382(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 36_u64 as i64);
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
fn superopt_383(pos: &mut FuncCursor, inst: Inst) {
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
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17_u64 as i64);
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
fn superopt_384(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 8 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 40_u64 as i64);
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
fn superopt_385(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_386(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 28 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 31_u64 as i64);
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
fn superopt_387(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if arg_1[0] != arg_0[1] && arg_1[0] != arg_2[0] && arg_1[0] != arg_2[1] && arg_0[1] != arg_2[0] && arg_0[1] == arg_2[1] && arg_2[0] != arg_2[1] {
let rhs_inst_7 = pos.ins().bor(arg_1[0], arg_2[0]);
pos.func.dfg.replace(inst).band(arg_2[1], rhs_inst_7);

}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_388(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 17 {
if rhs_2 == 255 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 15_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1020_u64 as i64);

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
fn superopt_389(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 4095 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 6_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 16380_u64 as i64);

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
fn superopt_390(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 84_u64 as i64);
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
fn superopt_391(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2 {
if rhs_1 == 12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 14_u64 as i64);
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
fn superopt_392(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 1 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 16 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 17_u64 as i64);
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
fn superopt_393(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 84 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 92_u64 as i64);
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
fn superopt_394(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 15_u64 as i64);
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
fn superopt_395(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 12 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 20_u64 as i64);
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
fn superopt_396(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 3 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 16 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 19_u64 as i64);
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
fn superopt_397(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == -8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 8_u64 as i64);
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
fn superopt_398(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 2 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 34_u64 as i64);
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
fn superopt_399(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 3 {
if rhs_1 == 16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 19_u64 as i64);
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
fn superopt_400(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 28 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 36_u64 as i64);
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
fn superopt_401(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -4 {
if rhs_1 == 12 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 8_u64 as i64);
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
fn superopt_402(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 83 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 91_u64 as i64);
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
fn superopt_403(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 1 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().bor_not(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_404(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == 12 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 24_u64 as i64);
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
fn superopt_405(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 60 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 68_u64 as i64);
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
fn superopt_406(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().bor_not(arg_1[1], arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_407(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1 {
if rhs_1 == 12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 13_u64 as i64);
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
fn superopt_408(pos: &mut FuncCursor, inst: Inst) {
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
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 19_u64 as i64);
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
fn superopt_409(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 72_u64 as i64);
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
fn superopt_410(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 24 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_411(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == -8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_412(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 40 {
if rhs_1 == 16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_413(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_414(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 40_u64 as i64);
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
fn superopt_415(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 80 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 92_u64 as i64);

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
fn superopt_416(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 64_u64 as i64);
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
fn superopt_417(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == -3 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967284_u64 as i64);

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
fn superopt_418(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 53_u64 as i64);
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
fn superopt_419(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 48_u64 as i64);
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
fn superopt_420(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 858993459_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_421(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 32 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 44_u64 as i64);

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
fn superopt_422(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 88_u64 as i64);
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
fn superopt_423(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 92_u64 as i64);
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
fn superopt_424(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == -5 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 27_u64 as i64);
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
fn superopt_425(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
if rhs_1 == 24 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 16_u64 as i64);
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
fn superopt_426(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == -4 {
let rhs_inst_5 = pos.ins().bor_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967289_u64 as i64);

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
fn superopt_427(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == -4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 20_u64 as i64);
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
fn superopt_428(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 96 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 104_u64 as i64);
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
fn superopt_429(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == -39 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 9_u64 as i64);
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
fn superopt_430(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 4 {
if rhs_2 == 2 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 252645135_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_431(pos: &mut FuncCursor, inst: Inst) {
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
fn superopt_432(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 69_u64 as i64);
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
fn superopt_433(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 20 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 68_u64 as i64);
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
fn superopt_434(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iconst(I32, 33423870_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 2_u64 as i64);

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
fn superopt_435(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
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
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 24_u64 as i64);

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
fn superopt_436(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == 17 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 97_u64 as i64);
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
fn superopt_437(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 40 {
if rhs_1 == -8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 32_u64 as i64);
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
fn superopt_438(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 40 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 64_u64 as i64);
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
fn superopt_439(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == 28 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 76_u64 as i64);
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
fn superopt_440(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 24 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 36_u64 as i64);

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
fn superopt_441(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == 16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 96_u64 as i64);
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
fn superopt_442(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 92 {
if rhs_1 == -8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 84_u64 as i64);
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
fn superopt_443(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 16711935_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_444(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 37_u64 as i64);
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
fn superopt_445(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 28 {
if rhs_1 == -4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_446(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 40 {
if rhs_1 == 32 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 72_u64 as i64);
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
fn superopt_447(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 64 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 85_u64 as i64);
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
fn superopt_448(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4_u64 as i64);

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
fn superopt_449(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 56 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 63_u64 as i64);

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
fn superopt_450(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 64 {
if rhs_1 == -8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_451(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 260136 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 260132_u64 as i64);

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
fn superopt_452(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 32 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 52_u64 as i64);

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
fn superopt_453(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 11 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 3345_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 11_u64 as i64);

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
fn superopt_454(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 180 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 181_u64 as i64);
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
fn superopt_455(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 914536 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 914532_u64 as i64);

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
fn superopt_456(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 9 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 441_u64 as i64);
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
fn superopt_457(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 262796 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 262792_u64 as i64);

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
fn superopt_458(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -56 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 40_u64 as i64);
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
fn superopt_459(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 228 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 236_u64 as i64);
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
fn superopt_460(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_1 == 52 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_3 == -52 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().isub(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 52_u64 as i64);

}
}
},
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
fn superopt_461(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -12 {
if rhs_1 == 36 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_462(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 24 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 44_u64 as i64);

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
fn superopt_463(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 180 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 188_u64 as i64);
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
fn superopt_464(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 92 {
if rhs_1 == -16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 76_u64 as i64);
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
fn superopt_465(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
if rhs_1 == -12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 12_u64 as i64);
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
fn superopt_466(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -48 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 48_u64 as i64);
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
fn superopt_467(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 464 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 468_u64 as i64);
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
fn superopt_468(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 12 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 4207_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 12_u64 as i64);

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
fn superopt_469(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 84_u64 as i64);
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
fn superopt_470(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 32767 {
if rhs_1 == 20 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 65534_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 10_u64 as i64);

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
fn superopt_471(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 131 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 139_u64 as i64);
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
fn superopt_472(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 167 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 175_u64 as i64);
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
fn superopt_473(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -40 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 56_u64 as i64);
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
fn superopt_474(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -3 {
if rhs_2 == 2 {
if rhs_1 == 104976 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 104964_u64 as i64);

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
fn superopt_475(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 16_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_476(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 914524 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 914520_u64 as i64);

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
fn superopt_477(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 155 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 163_u64 as i64);
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
fn superopt_478(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 288 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 296_u64 as i64);
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
fn superopt_479(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 216 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 224_u64 as i64);
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
fn superopt_480(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 192 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 200_u64 as i64);
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
fn superopt_481(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 308_u64 as i64);
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
fn superopt_482(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 144 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 152_u64 as i64);
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
fn superopt_483(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 107 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 115_u64 as i64);
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
fn superopt_484(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 179 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 187_u64 as i64);
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
fn superopt_485(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 7 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 7_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

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
fn superopt_486(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 177 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 178_u64 as i64);
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
fn superopt_487(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(arg_2, rhs_inst_6);
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
fn superopt_488(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 4 {
if rhs_2 == 1 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967280_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 16_u64 as i64);

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
fn superopt_489(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 168 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 176_u64 as i64);
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
fn superopt_490(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 592 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 600_u64 as i64);
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
fn superopt_491(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 120 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 128_u64 as i64);
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
fn superopt_492(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 440_u64 as i64);
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
fn superopt_493(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 117_u64 as i64);
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
fn superopt_494(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 2 {
if rhs_1 == 2 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 3_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(arg_2, rhs_inst_6);
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
fn superopt_495(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 272 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 280_u64 as i64);
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
fn superopt_496(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 24 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 52_u64 as i64);

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
fn superopt_497(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 761472 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 761468_u64 as i64);

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
fn superopt_498(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 56 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 72_u64 as i64);

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
fn superopt_499(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 32 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 60_u64 as i64);

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
fn superopt_500(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 88 {
if rhs_1 == 76 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 164_u64 as i64);
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
fn superopt_501(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 3 {
if rhs_1 == 3 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 7_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(arg_2, rhs_inst_6);
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
fn superopt_502(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -6 {
if rhs_2 == 2 {
if rhs_1 == 914548 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 914524_u64 as i64);

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
fn superopt_503(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 156 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 164_u64 as i64);
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
fn superopt_504(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 608 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 612_u64 as i64);
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
fn superopt_505(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 260124 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 260120_u64 as i64);

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
fn superopt_506(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 252645135_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_507(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 2 {
if rhs_1 == 2 {
if arg_0[1] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 3_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(arg_2, rhs_inst_6);
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
fn superopt_508(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 608 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 4 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 612_u64 as i64);
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
fn superopt_509(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 101_u64 as i64);
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
fn superopt_510(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 1 {
if rhs_1 == -1 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_7);

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
fn superopt_511(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 203 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 211_u64 as i64);
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
fn superopt_512(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().band_not(arg_1[1], arg_2);
pos.func.dfg.replace(inst).bxor_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_513(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 7 {
if rhs_2 == -8 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 15_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_514(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_1 == 24 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_3 == -24 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().isub(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 24_u64 as i64);

}
}
},
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
fn superopt_515(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 132_u64 as i64);
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
fn superopt_516(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 28 {
if rhs_1 == -12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 16_u64 as i64);
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
fn superopt_517(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 24 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 40_u64 as i64);

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
fn superopt_518(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -276 {
if rhs_2 == 2 {
if rhs_1 == 21804 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 20700_u64 as i64);

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
fn superopt_519(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 875544 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 875540_u64 as i64);

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
fn superopt_520(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 88 {
if rhs_1 == 84 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 172_u64 as i64);
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
fn superopt_521(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 312_u64 as i64);
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
fn superopt_522(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -20 {
if rhs_1 == 60 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 40_u64 as i64);
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
fn superopt_523(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 132 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 140_u64 as i64);
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
fn superopt_524(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 108 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 116_u64 as i64);
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
fn superopt_525(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
if rhs_1 == -24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 24_u64 as i64);
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
fn superopt_526(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
if rhs_1 == 72 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 48_u64 as i64);
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
fn superopt_527(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iconst(I32, 505290270_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 2_u64 as i64);

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
fn superopt_528(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
if rhs_1 == -16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 16_u64 as i64);
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
fn superopt_529(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 72_u64 as i64);
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
fn superopt_530(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_1 == 12 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_3 == -12 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().isub(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 12_u64 as i64);

}
}
},
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
fn superopt_531(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 40 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 68_u64 as i64);

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
fn superopt_532(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -32 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 64_u64 as i64);
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
fn superopt_533(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 119 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 127_u64 as i64);
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
fn superopt_534(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 177 {
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
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 180_u64 as i64);
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
fn superopt_535(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 215 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 223_u64 as i64);
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
fn superopt_536(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 177 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 178_u64 as i64);
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
fn superopt_537(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 227 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 235_u64 as i64);
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
fn superopt_538(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 858993459_u64 as i64);
let rhs_inst_6 = pos.ins().sshr(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 1_u64 as i64);

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
fn superopt_539(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 40 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 56_u64 as i64);

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
fn superopt_540(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 336 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 344_u64 as i64);
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
fn superopt_541(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
if rhs_1 == 204 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 212_u64 as i64);
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
fn superopt_542(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 288 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 292_u64 as i64);
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
fn superopt_543(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 64 {
if rhs_1 == -16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 48_u64 as i64);
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
fn superopt_544(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 64 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 80_u64 as i64);

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
fn superopt_545(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 177 {
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
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 179_u64 as i64);
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
fn superopt_546(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
if rhs_1 == 48 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 32_u64 as i64);
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
fn superopt_547(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 96 {
if rhs_1 == -16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 80_u64 as i64);
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
fn superopt_548(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 10 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 1785_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 10_u64 as i64);

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
fn superopt_549(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 13 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 14185_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 13_u64 as i64);

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
fn superopt_550(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 120 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 127_u64 as i64);

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
fn superopt_551(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 368 {
if rhs_1 == 12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 380_u64 as i64);
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
fn superopt_552(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 208 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 229_u64 as i64);
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
fn superopt_553(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 624 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 631_u64 as i64);

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
fn superopt_554(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 8 {
if rhs_1 == 65280 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 10_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 65280_u64 as i64);

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
fn superopt_555(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 880 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 901_u64 as i64);
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
fn superopt_556(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 324 {
if rhs_1 == 11 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 335_u64 as i64);
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
fn superopt_557(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 416 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 437_u64 as i64);
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
fn superopt_558(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 368 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 389_u64 as i64);
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
fn superopt_559(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 480 {
if rhs_1 == -8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 472_u64 as i64);
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
fn superopt_560(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_2 == 371 {
if rhs_1 == -2 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2, 369_u64 as i64);
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
fn superopt_561(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 272 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 293_u64 as i64);
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
fn superopt_562(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 35 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 467_u64 as i64);
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
fn superopt_563(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 8 {
if rhs_2 == 1 {
if rhs_1 == 14 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 1006633216_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 64_u64 as i64);

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
fn superopt_564(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 149_u64 as i64);
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
fn superopt_565(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 128 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 127_u64 as i64);
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
fn superopt_566(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 17 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 449_u64 as i64);
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
fn superopt_567(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -48 {
if rhs_1 == 144 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 96_u64 as i64);
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
fn superopt_568(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 336 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 335_u64 as i64);
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
fn superopt_569(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 31 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 463_u64 as i64);
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
fn superopt_570(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bxor => {
if arg_2[1] != arg_2[0] && arg_2[1] != arg_1[0] && arg_2[1] == arg_0[1] && arg_2[0] != arg_1[0] && arg_2[0] != arg_0[1] && arg_1[0] != arg_0[1] {
let rhs_inst_7 = pos.ins().bor_not(arg_1[0], arg_2[0]);
pos.func.dfg.replace(inst).band(arg_2[1], rhs_inst_7);

}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_571(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == 20 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 148_u64 as i64);
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
fn superopt_572(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 112 {
if rhs_1 == -8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 104_u64 as i64);
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
fn superopt_573(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == -5 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 427_u64 as i64);
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
fn superopt_574(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 65535 {
if rhs_2 == 3 {
if rhs_1 == 8188 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 8188_u64 as i64);

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
fn superopt_575(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 112 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 119_u64 as i64);

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
fn superopt_576(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == 11 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 139_u64 as i64);
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
fn superopt_577(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 488 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 500_u64 as i64);

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
fn superopt_578(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 80 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 108_u64 as i64);

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
fn superopt_579(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 640 {
if rhs_1 == 20 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 660_u64 as i64);
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
fn superopt_580(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 320_u64 as i64);
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
fn superopt_581(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 640 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 639_u64 as i64);
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
fn superopt_582(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 80 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 100_u64 as i64);

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
fn superopt_583(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 14 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iconst(I32, 35354_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 14_u64 as i64);

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
fn superopt_584(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 512 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 524_u64 as i64);

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
fn superopt_585(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 896 {
if rhs_1 == 17 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 913_u64 as i64);
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
fn superopt_586(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 456_u64 as i64);
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
fn superopt_587(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == -1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 127_u64 as i64);
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
fn superopt_588(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 720 {
if rhs_1 == 12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 732_u64 as i64);
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
fn superopt_589(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 32 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 464_u64 as i64);
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
fn superopt_590(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 464 {
if rhs_1 == -1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 463_u64 as i64);
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
fn superopt_591(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 392 {
if rhs_2 == 4 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 403_u64 as i64);

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
fn superopt_592(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 15 {
if rhs_2 == -16 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 0_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280_u64 as i64);

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
fn superopt_593(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 520 {
if rhs_1 == -1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 519_u64 as i64);
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
fn superopt_594(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == 28 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 460_u64 as i64);
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
fn superopt_595(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 324 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 323_u64 as i64);
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
fn superopt_596(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 160 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 172_u64 as i64);

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
fn superopt_597(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 248 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 247_u64 as i64);
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
fn superopt_598(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 480 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 487_u64 as i64);

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
fn superopt_599(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 640 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 661_u64 as i64);
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
fn superopt_600(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2[1] != arg_2[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2[1]);
pos.func.dfg.replace(inst).band_not(rhs_inst_7, arg_2[0]);

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
fn superopt_601(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 328_u64 as i64);
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
fn superopt_602(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 464 {
if rhs_1 == 11 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 475_u64 as i64);
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
fn superopt_603(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == 12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 316_u64 as i64);
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
fn superopt_604(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 512 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 533_u64 as i64);
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
fn superopt_605(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 768 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 775_u64 as i64);

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
fn superopt_606(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 720 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 741_u64 as i64);
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
fn superopt_607(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 152 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 168_u64 as i64);

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
fn superopt_608(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 464 {
if rhs_1 == -12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 452_u64 as i64);
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
fn superopt_609(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 256 {
if rhs_1 == -13 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 243_u64 as i64);
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
fn superopt_610(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2492784 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2492780_u64 as i64);

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
fn superopt_611(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == -1 {
let rhs_inst_5 = pos.ins().iconst(I32, 3435973836_u64 as i64);
let rhs_inst_6 = pos.ins().ishl(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 31_u64 as i64);

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
fn superopt_612(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2600640 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2600636_u64 as i64);

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
fn superopt_613(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 504 {
if rhs_1 == -28 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 476_u64 as i64);
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
fn superopt_614(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == 63 {
if rhs_1 == 63 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 3758096192_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 63_u64 as i64);

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
fn superopt_615(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == -76 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 228_u64 as i64);
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
fn superopt_616(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 96 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 96_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

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
fn superopt_617(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -9 {
if rhs_2 == 2 {
if rhs_1 == 1876564 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1876528_u64 as i64);

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
fn superopt_618(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 16 {
if rhs_2 == 32767 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 14_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 131068_u64 as i64);

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
fn superopt_619(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 296 {
if rhs_1 == -16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 280_u64 as i64);
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
fn superopt_620(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -151 {
if rhs_2 == 2 {
if rhs_1 == 244496 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 243892_u64 as i64);

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
fn superopt_621(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 256 {
if rhs_1 == -24 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 232_u64 as i64);
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
fn superopt_622(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 392 {
if rhs_1 == -16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 376_u64 as i64);
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
fn superopt_623(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2600528 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2600524_u64 as i64);

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
fn superopt_624(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 464 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 480_u64 as i64);

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
fn superopt_625(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 456 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 472_u64 as i64);

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
fn superopt_626(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -2 {
if rhs_2 == 2 {
if rhs_1 == 2388432 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2388424_u64 as i64);

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
fn superopt_627(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1536 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1540_u64 as i64);
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
fn superopt_628(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1487000 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1486996_u64 as i64);

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
fn superopt_629(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 178 {
if rhs_1 == 134 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 312_u64 as i64);
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
fn superopt_630(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_1 == 512 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 512 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().bxor(arg_2, arg_2);
pos.func.dfg.replace(inst).band_imm(rhs_inst_6, 512_u64 as i64);

}
}
},
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
fn superopt_631(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -8 {
if rhs_2 == 2 {
if rhs_1 == 2585872 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2585840_u64 as i64);

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
fn superopt_632(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2596936 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2596932_u64 as i64);

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
fn superopt_633(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 480 {
if rhs_1 == -24 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 456_u64 as i64);
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
fn superopt_634(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 179 {
if rhs_1 == 137 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 316_u64 as i64);
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
fn superopt_635(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 512 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 532_u64 as i64);

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
fn superopt_636(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == -1 {
if rhs_1 == 63 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 2147483456_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 63_u64 as i64);

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
fn superopt_637(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 392 {
if rhs_1 == -28 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 364_u64 as i64);
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
fn superopt_638(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 2 {
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 18_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280_u64 as i64);

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
fn superopt_639(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -2 {
if rhs_2 == 2 {
if rhs_1 == 2593092 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2593084_u64 as i64);

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
fn superopt_640(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -88 {
if rhs_1 == 240 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 152_u64 as i64);
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
fn superopt_641(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 432 {
if rhs_1 == -16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 416_u64 as i64);
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
fn superopt_642(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -3 {
if rhs_2 == 2 {
if rhs_1 == 1490380 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1490368_u64 as i64);

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
fn superopt_643(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 176 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 192_u64 as i64);

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
fn superopt_644(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1499452 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1499448_u64 as i64);

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
fn superopt_645(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 400 {
if rhs_1 == -76 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 324_u64 as i64);
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
fn superopt_646(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -88 {
if rhs_1 == 208 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 120_u64 as i64);
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
fn superopt_647(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1780628 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1780624_u64 as i64);

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
fn superopt_648(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -92 {
if rhs_1 == 276 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 184_u64 as i64);
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
fn superopt_649(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -4 {
if rhs_2 == 2 {
if rhs_1 == 1557704 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1557688_u64 as i64);

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
fn superopt_650(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1806756 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1806752_u64 as i64);

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
fn superopt_651(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2531884 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2531880_u64 as i64);

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
fn superopt_652(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2369392 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2369388_u64 as i64);

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
fn superopt_653(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 13 {
if rhs_2 == 1 {
if rhs_1 == 19 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 2080382976_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 64_u64 as i64);

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
fn superopt_654(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 480 {
if rhs_1 == -40 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 440_u64 as i64);
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
fn superopt_655(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2561992 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2561988_u64 as i64);

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
fn superopt_656(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 3 {
if rhs_1 == 2388120 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2388112_u64 as i64);

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
fn superopt_657(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1888964 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1888960_u64 as i64);

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
fn superopt_658(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1497924 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1497920_u64 as i64);

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
fn superopt_659(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 31 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 31_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_1[1], rhs_inst_7);

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
fn superopt_660(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -56 {
if rhs_1 == 168 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 112_u64 as i64);
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
fn superopt_661(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 400 {
if rhs_1 == -64 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 336_u64 as i64);
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
fn superopt_662(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
if rhs_1 == -64 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 240_u64 as i64);
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
fn superopt_663(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == -1 {
if rhs_1 == 31 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 4026531840_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 31_u64 as i64);

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
fn superopt_664(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -48 {
if rhs_1 == 177 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 129_u64 as i64);
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
fn superopt_665(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 1526752 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1526748_u64 as i64);

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
fn superopt_666(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 296 {
if rhs_1 == -28 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 268_u64 as i64);
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
fn superopt_667(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -2 {
if rhs_2 == 2 {
if rhs_1 == 1497908 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1497900_u64 as i64);

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
fn superopt_668(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 520 {
if rhs_1 == -12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 508_u64 as i64);
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
fn superopt_669(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 17 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 240269_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 17_u64 as i64);

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
fn superopt_670(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -5 {
if rhs_2 == 2 {
if rhs_1 == 1457612 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1457592_u64 as i64);

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
fn superopt_671(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 12 {
if rhs_2 == -1 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().iconst(I32, 69465_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 12_u64 as i64);

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
fn superopt_672(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 63 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().isub(arg_2, arg_2);
let rhs_inst_7 = pos.ins().iconst(I32, 63_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_7, rhs_inst_6);

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
fn superopt_673(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2595580 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2595576_u64 as i64);

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
fn superopt_674(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2595564 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2595560_u64 as i64);

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
fn superopt_675(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == 6 {
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 22_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280_u64 as i64);

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
fn superopt_676(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 3 {
if rhs_1 == 1457656 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1457648_u64 as i64);

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
fn superopt_677(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 3 {
if rhs_1 == 1495552 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1495544_u64 as i64);

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
fn superopt_678(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -88 {
if rhs_1 == 192 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 104_u64 as i64);
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
fn superopt_679(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2386720 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2386716_u64 as i64);

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
fn superopt_680(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -88 {
if rhs_1 == 224 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 136_u64 as i64);
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
fn superopt_681(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 504 {
if rhs_1 == -16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 488_u64 as i64);
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
fn superopt_682(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -4 {
if rhs_2 == 2 {
if rhs_1 == 1557772 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1557756_u64 as i64);

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
fn superopt_683(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -5 {
if rhs_2 == 2 {
if rhs_1 == 2589852 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2589832_u64 as i64);

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
fn superopt_684(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 2604716 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2604712_u64 as i64);

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
fn superopt_685(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 184 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 200_u64 as i64);

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
fn superopt_686(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 16 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 100948_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 16_u64 as i64);

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
fn superopt_687(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -88 {
if rhs_1 == 256 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 168_u64 as i64);
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
fn superopt_688(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
if rhs_1 == -12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 116_u64 as i64);
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
fn superopt_689(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 240 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1200_u64 as i64);
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
fn superopt_690(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 264 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1224_u64 as i64);
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
fn superopt_691(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Imul => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_1 == 0 {
if arg_1[1] != arg_0[0] && arg_1[1] != arg_2 && arg_0[0] != arg_2 {
let rhs_inst_7 = pos.ins().imul(arg_2, arg_1[1]);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_692(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -20 {
if rhs_2 == 2 {
if rhs_1 == 2677880 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2677800_u64 as i64);

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
fn superopt_693(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 3696 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 3703_u64 as i64);

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
fn superopt_694(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 275 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1235_u64 as i64);
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
fn superopt_695(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1456 {
if rhs_1 == 12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1468_u64 as i64);
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
fn superopt_696(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_2 != arg_0[1] && arg_2 != arg_1[0] && arg_0[1] != arg_1[0] {
let rhs_inst_7 = pos.ins().bor_not(arg_2, arg_2);
pos.func.dfg.replace(inst).bor(arg_0[1], rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_697(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 252 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1212_u64 as i64);
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
fn superopt_698(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1536 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1543_u64 as i64);

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
fn superopt_699(pos: &mut FuncCursor, inst: Inst) {
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
if arg_2 != arg_0[1] && arg_2 != arg_1[0] && arg_0[1] != arg_1[0] {
let rhs_inst_7 = pos.ins().isub(arg_2, arg_2);
pos.func.dfg.replace(inst).iadd(arg_0[1], rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_700(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == 17 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 18_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280_u64 as i64);

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
fn superopt_701(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 251 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1211_u64 as i64);
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
fn superopt_702(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -16 {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967294_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 18_u64 as i64);

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
fn superopt_703(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 29 {
if rhs_2 == -1 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 3758096384_u64 as i64);
pos.func.dfg.replace(inst).ushr_imm(rhs_inst_5, 29_u64 as i64);

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
fn superopt_704(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 24 {
if rhs_2 == 1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 18252902_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 24_u64 as i64);

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
fn superopt_705(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -68 {
if rhs_2 == 2 {
if rhs_1 == 2472352 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2472080_u64 as i64);

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
fn superopt_706(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -48 {
if rhs_2 == 2 {
if rhs_1 == 2594888 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2594696_u64 as i64);

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
fn superopt_707(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1712 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1733_u64 as i64);
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
fn superopt_708(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -44 {
if rhs_2 == 2 {
if rhs_1 == 2597956 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2597780_u64 as i64);

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
fn superopt_709(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 31 {
if rhs_2 == -32 {
if rhs_1 == 32 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 63_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967264_u64 as i64);

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
fn superopt_710(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 255 {
if rhs_2 == -24 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 254_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967273_u64 as i64);

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
fn superopt_711(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 276 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1236_u64 as i64);
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
fn superopt_712(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -26 {
if rhs_2 == 2 {
if rhs_1 == 2472276 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2472172_u64 as i64);

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
fn superopt_713(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 5 {
if rhs_2 == 5 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967264_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 32_u64 as i64);

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
fn superopt_714(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if arg_2[0] != arg_1[0] && arg_2[0] != arg_2[1] && arg_2[0] != arg_0[0] && arg_1[0] != arg_2[1] && arg_1[0] != arg_0[0] && arg_2[1] == arg_0[0] {
let rhs_inst_7 = pos.ins().band(arg_1[0], arg_2[0]);
pos.func.dfg.replace(inst).band_not(arg_2[1], rhs_inst_7);

}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_715(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2720 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2741_u64 as i64);
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
fn superopt_716(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 4 {
if rhs_2 == 4 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967280_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 16_u64 as i64);

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
fn superopt_717(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 960 {
if rhs_1 == 263 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1223_u64 as i64);
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
fn superopt_718(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 17_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967280_u64 as i64);

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
fn superopt_719(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_1[0] != arg_0[0] && arg_1[0] != arg_2 && arg_0[0] != arg_2 {
let rhs_inst_7 = pos.ins().bor_not(arg_1[0], arg_2);
pos.func.dfg.replace(inst).bor(arg_2, rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_720(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 360 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2248_u64 as i64);
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
fn superopt_721(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 564 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1924_u64 as i64);
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
fn superopt_722(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 563 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1923_u64 as i64);
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
fn superopt_723(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -452 {
if rhs_2 == 2 {
if rhs_1 == 2382484 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2380676_u64 as i64);

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
fn superopt_724(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 539 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1899_u64 as i64);
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
fn superopt_725(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 540 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1900_u64 as i64);
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
fn superopt_726(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_4 = args;
match opcode {
Opcode::Iadd => {
if rhs_3 == -1 {
if rhs_2 == 64 {
if rhs_1 == 4294967295 {
if arg_4[1] != arg_4[0] {
let rep_insts = pos.func.dfg.inst_results(inst);
let rep_insts_0 = rep_insts[0];
pos.func.dfg.change_to_alias(arg_0, rep_insts_0);
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
fn superopt_727(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -194 {
if rhs_2 == 2 {
if rhs_1 == 2746000 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2745224_u64 as i64);

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
fn superopt_728(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 7 {
if rhs_1 == 8 {
if arg_2 != arg_0[1] {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967288_u64 as i64);
pos.func.dfg.replace(inst).isub(arg_0[1], rhs_inst_6);

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
fn superopt_729(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 348 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2236_u64 as i64);
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
fn superopt_730(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -360 {
if rhs_2 == 2 {
if rhs_1 == 2472516 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2471076_u64 as i64);

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
fn superopt_731(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 359 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2247_u64 as i64);
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
fn superopt_732(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -395 {
if rhs_2 == 2 {
if rhs_1 == 2382360 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2380780_u64 as i64);

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
fn superopt_733(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 30 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 31_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 4294967293_u64 as i64);

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
fn superopt_734(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 552 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1912_u64 as i64);
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
fn superopt_735(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 528 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1888_u64 as i64);
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
fn superopt_736(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967168_u64 as i64);
let rhs_inst_6 = pos.ins().bor_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 3_u64 as i64);

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
fn superopt_737(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bxor => {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_1 == 7 {
if arg_2[1] != arg_0[1] && arg_2[1] != arg_2[0] && arg_0[1] == arg_2[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967288_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2[1]);
pos.func.dfg.replace(inst).band(arg_2[0], rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_738(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 371 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2259_u64 as i64);
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
fn superopt_739(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -570 {
if rhs_2 == 2 {
if rhs_1 == 2382512 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2380232_u64 as i64);

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
fn superopt_740(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 372 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2260_u64 as i64);
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
fn superopt_741(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -229 {
if rhs_2 == 2 {
if rhs_1 == 2472416 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2471500_u64 as i64);

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
fn superopt_742(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 347 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2235_u64 as i64);
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
fn superopt_743(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -422 {
if rhs_2 == 2 {
if rhs_1 == 2382444 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2380756_u64 as i64);

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
fn superopt_744(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 25 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 33554433_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 25_u64 as i64);

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
fn superopt_745(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1888 {
if rhs_1 == 336 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2224_u64 as i64);
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
fn superopt_746(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 24 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 19996672_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 24_u64 as i64);

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
fn superopt_747(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 3 {
if rhs_2 == 1 {
if rhs_1 == 30 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 31_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 4294967293_u64 as i64);

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
fn superopt_748(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -209 {
if rhs_2 == 2 {
if rhs_1 == 2472376 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2471540_u64 as i64);

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
fn superopt_749(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 1360 {
if rhs_1 == 551 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 1911_u64 as i64);
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
fn superopt_750(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 4 {
if rhs_2 == 1 {
if rhs_1 == 30 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 31_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 4294967293_u64 as i64);

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
fn superopt_751(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 16384 {
if rhs_2 == 14 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 27088_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 14_u64 as i64);

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
fn superopt_752(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 3 {
if rhs_1 == 0 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_753(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_754(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 2 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
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
fn superopt_755(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1008 {
if rhs_2 == 2 {
if rhs_1 == 2382560 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2378528_u64 as i64);

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
fn superopt_756(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -7296 {
if rhs_2 == 2 {
if rhs_1 == 2382604 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2353420_u64 as i64);

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
fn superopt_757(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1103 {
if rhs_2 == 2 {
if rhs_1 == 1486960 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1482548_u64 as i64);

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
fn superopt_758(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_0[0] != arg_2 && arg_0[0] != arg_1[1] && arg_2 != arg_1[1] {
let rhs_inst_7 = pos.ins().band_not(arg_1[1], arg_2);
pos.func.dfg.replace(inst).bor(arg_2, rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_759(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -1101 {
if rhs_2 == 2 {
if rhs_1 == 1486920 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1482516_u64 as i64);

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
fn superopt_760(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_2 != arg_1[0] && arg_2 != arg_0[0] && arg_1[0] != arg_0[0] {
let rhs_inst_7 = pos.ins().bor_not(arg_1[0], arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_761(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 27 {
if rhs_2 == 16 {
if rhs_1 == -18 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 27_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967294_u64 as i64);

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
fn superopt_762(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 26 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I32, 133799032_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 26_u64 as i64);

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
fn superopt_763(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 3 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
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
fn superopt_764(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -7301 {
if rhs_2 == 2 {
if rhs_1 == 2382616 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2353412_u64 as i64);

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
fn superopt_765(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_3 == -1 {
if rhs_2 == 1 {
if rhs_1 == 2743873 {
let rhs_inst_5 = pos.ins().iconst(I32, 1_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 2743873_u64 as i64);

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
fn superopt_766(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_0[0]) {

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
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_1[0] != arg_2 && arg_1[0] != arg_0[1] && arg_2 != arg_0[1] {
let rhs_inst_7 = pos.ins().band_not(arg_2, arg_2);
pos.func.dfg.replace(inst).bor(arg_0[1], rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_767(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 30 {
if rhs_2 == -1 {
if rhs_1 == -2 {
let rhs_inst_5 = pos.ins().iconst(I32, 2147483648_u64 as i64);
let rhs_inst_6 = pos.ins().bor_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 30_u64 as i64);

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
fn superopt_768(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -2051 {
if rhs_2 == 3 {
if rhs_1 == 1487024 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1470616_u64 as i64);

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
fn superopt_769(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 2 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967292_u64 as i64);
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
fn superopt_770(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1073741823 {
if rhs_1 == 2 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
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
fn superopt_771(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 7 {
if rhs_2 == 8 {
if rhs_1 == 6 {
let rhs_inst_5 = pos.ins().bor_imm(arg_2, 2147483640_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 4294967290_u64 as i64);

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
fn superopt_772(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -5532 {
if rhs_2 == 2 {
if rhs_1 == 21864 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967032_u64 as i64);

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
fn superopt_773(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 5 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 32_u64 as i64);
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
fn superopt_774(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -7 {
if rhs_1 == 6 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967295_u64 as i64);
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
fn superopt_775(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
if rhs_1 == 4 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967292_u64 as i64);
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
fn superopt_776(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 6 {
if rhs_1 == 6 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().band_imm(arg_2, 4294967232_u64 as i64);
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
fn superopt_777(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 1 {
if rhs_1 == 1 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().band_imm(arg_2, 4294967294_u64 as i64);
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
fn superopt_778(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -8 {
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
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967289_u64 as i64);
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
fn superopt_779(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -11360 {
if rhs_2 == 2 {
if rhs_1 == 2382632 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2337192_u64 as i64);

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
fn superopt_780(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 134217727 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 268435454_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 2_u64 as i64);

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
fn superopt_781(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_2 == 0 {
if rhs_1 == 12 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967284_u64 as i64);
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
fn superopt_782(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_3 == 256 {
if rhs_2 == -1537 {
if rhs_1 == 512 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294965503_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 768_u64 as i64);

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
fn superopt_783(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -5532 {
if rhs_2 == 2 {
if rhs_1 == 21852 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967020_u64 as i64);

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
fn superopt_784(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == -11373 {
if rhs_2 == 2 {
if rhs_1 == 2382652 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2337160_u64 as i64);

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
fn superopt_785(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_2 == -1073741824 {
if rhs_1 == 2 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
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
fn superopt_786(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 63 {
if rhs_1 == 64 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967232_u64 as i64);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_6);

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
fn superopt_787(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 5 {
if rhs_1 == 5 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().band_imm(arg_2, 4294967264_u64 as i64);
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
fn superopt_788(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 8 {
if rhs_1 == 8 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().band_imm(arg_2, 4294967040_u64 as i64);
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
fn superopt_789(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_2 == 4 {
if rhs_1 == 4 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().band_imm(arg_2, 4294967280_u64 as i64);
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
fn superopt_790(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2382668 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2210980_u64 as i64);

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
fn superopt_791(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
if rhs_1 == 2 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967290_u64 as i64);
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
fn superopt_792(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == 127 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 1182269948_u64 as i64);
let rhs_inst_6 = pos.ins().iconst(I32, 508_u64 as i64);
pos.func.dfg.replace(inst).band_not(rhs_inst_6, rhs_inst_5);

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
fn superopt_793(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
if rhs_1 == 1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967281_u64 as i64);
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
fn superopt_794(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967272_u64 as i64);
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
fn superopt_795(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -64 {
if rhs_1 == 4 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967236_u64 as i64);
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
fn superopt_796(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
if rhs_1 == 2 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967282_u64 as i64);
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
fn superopt_797(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -65536 {
if rhs_2 == 1023 {
if rhs_1 == 56320 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 1023_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 56320_u64 as i64);

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
fn superopt_798(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -16 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967281_u64 as i64);
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
fn superopt_799(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 12 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 12_u64 as i64);
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
fn superopt_800(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 28 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 28_u64 as i64);
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
fn superopt_801(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -12 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967292_u64 as i64);
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
fn superopt_802(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
if rhs_1 == -8 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967280_u64 as i64);
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
fn superopt_803(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -28 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967276_u64 as i64);
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
fn superopt_804(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 134217727 {
if rhs_1 == 10 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 268435454_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 5_u64 as i64);

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
fn superopt_805(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_2 != arg_1[1] && arg_2 != arg_1[0] && arg_1[1] != arg_1[0] {
let rhs_inst_7 = pos.ins().band_not(arg_1[1], arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_806(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 24 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 24_u64 as i64);
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
fn superopt_807(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967280_u64 as i64);
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
fn superopt_808(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -4 {
if rhs_1 == -4 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_809(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_3 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 1 {
if rhs_1 == -1 {
if arg_3[1] != arg_3[0] && arg_3[1] != arg_1[1] && arg_3[0] != arg_1[1] {
pos.func.dfg.replace(inst).iadd(arg_1[1], arg_2);

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
fn superopt_810(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 0 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 18446744073709551614_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 1_u64 as i64);

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
fn superopt_811(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_1 == -1 {
if arg_0[1] != arg_2 && arg_0[1] != arg_1[0] && arg_2 != arg_1[0] {
let rhs_inst_7 = pos.ins().band_not(arg_2, arg_2);
pos.func.dfg.replace(inst).band(arg_0[1], rhs_inst_7);

}
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_812(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_813(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 1073741823 {
if rhs_1 == 80 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 80_u64 as i64);
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
fn superopt_814(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 2147483647 {
if rhs_1 == 40 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 40_u64 as i64);
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
fn superopt_815(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 34 {
if rhs_2 == -1 {
if rhs_1 == 1 {
let rhs_inst_5 = pos.ins().iconst(I64, 26913404097_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 34_u64 as i64);

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
fn superopt_816(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 21 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967285_u64 as i64);
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
fn superopt_817(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -464 {
if rhs_1 == 4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966836_u64 as i64);
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
fn superopt_818(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967280_u64 as i64);
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
fn superopt_819(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_3 == 1073741823 {
if rhs_2 == 2 {
if rhs_1 == -4 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_820(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -256 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967048_u64 as i64);
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
fn superopt_821(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -40 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967263_u64 as i64);

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
fn superopt_822(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -96 {
if rhs_2 == 4 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967206_u64 as i64);

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
fn superopt_823(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -56 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967252_u64 as i64);

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
fn superopt_824(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -80 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967228_u64 as i64);

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
fn superopt_825(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 30 {
if rhs_2 == 31 {
if rhs_1 == 536870944 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 2_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 268435472_u64 as i64);

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
fn superopt_826(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967263_u64 as i64);
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
fn superopt_827(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 134217727 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 268435454_u64 as i64);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_5, 12_u64 as i64);

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
fn superopt_828(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 2904321 {
if rhs_1 == 1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 2904322_u64 as i64);
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
fn superopt_829(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -40 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967268_u64 as i64);

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
fn superopt_830(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -360 {
if rhs_1 == 8 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966944_u64 as i64);
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
fn superopt_831(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -28 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967292_u64 as i64);
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
fn superopt_832(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 63 {
if rhs_1 == 1 {
if arg_2 != arg_0[1] {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967232_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).isub(arg_0[1], rhs_inst_7);

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
fn superopt_833(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 20 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967284_u64 as i64);
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
fn superopt_834(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_835(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 17 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967281_u64 as i64);
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
fn superopt_836(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -16 {
if rhs_2 == 4 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967291_u64 as i64);

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
fn superopt_837(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -16 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_838(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
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
if rhs_2 == -1073741824 {
if rhs_1 == 40 {
if arg_2 != arg_1[0] {
let rhs_inst_6 = pos.ins().iadd(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 40_u64 as i64);

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
fn superopt_839(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -168 {
if rhs_1 == 8 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967136_u64 as i64);
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
fn superopt_840(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -64 {
if rhs_1 == 16 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967248_u64 as i64);
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
fn superopt_841(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
if rhs_1 == -100 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967200_u64 as i64);
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
fn superopt_842(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_1 == -28 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_3 == 28 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().isub(arg_2, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 4294967268_u64 as i64);

}
}
},
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
fn superopt_843(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -64 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967256_u64 as i64);
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
fn superopt_844(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
if rhs_1 == 24 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967288_u64 as i64);
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
fn superopt_845(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -32 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967271_u64 as i64);

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
fn superopt_846(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -64 {
if rhs_1 == -1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967231_u64 as i64);
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
fn superopt_847(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 63 {
if rhs_1 == 1 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967232_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_7);

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
fn superopt_848(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::Binary { opcode, args } => {
let arg_0 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_1 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == -8 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967288_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

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
fn superopt_849(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
if rhs_1 == -1 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967271_u64 as i64);
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
fn superopt_850(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -24 {
if rhs_2 == 4 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967283_u64 as i64);

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
fn superopt_851(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -32 {
if rhs_2 == 4 {
if rhs_1 == 8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967276_u64 as i64);

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
fn superopt_852(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -28 {
if rhs_1 == 16 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967284_u64 as i64);
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
fn superopt_853(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -368 {
if rhs_1 == 20 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966948_u64 as i64);
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
fn superopt_854(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 4 {
if rhs_2 == 4 {
if rhs_1 == -4 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967280_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_855(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -72 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967240_u64 as i64);

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
fn superopt_856(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_857(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967288_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_858(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == -8 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967288_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_859(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -40 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967276_u64 as i64);

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
fn superopt_860(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -56 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967268_u64 as i64);

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
fn superopt_861(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -992 {
if rhs_2 == 4 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294966315_u64 as i64);

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
fn superopt_862(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -48 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967268_u64 as i64);

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
fn superopt_863(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iconst(I32, 1073741822_u64 as i64);
let rhs_inst_6 = pos.ins().bor_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 4294967292_u64 as i64);

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
fn superopt_864(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -256 {
if rhs_1 == -1 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967039_u64 as i64);
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
fn superopt_865(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -48 {
if rhs_2 == 4 {
if rhs_1 == -4 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967248_u64 as i64);

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
fn superopt_866(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
if rhs_1 == -12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967260_u64 as i64);
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
fn superopt_867(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -56 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967256_u64 as i64);

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
fn superopt_868(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -64 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 63 {
if rhs_3 == 3 {
if arg_1 == arg_3 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 7_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_6);

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
fn superopt_869(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == -144 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967232_u64 as i64);
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
fn superopt_870(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -104 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967199_u64 as i64);

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
fn superopt_871(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_2 == 1 {
if rhs_1 == -1 {
if arg_0[1] != arg_2 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967294_u64 as i64);
pos.func.dfg.replace(inst).band_not(arg_0[1], rhs_inst_6);

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
fn superopt_872(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -112 {
if rhs_1 == 21 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967205_u64 as i64);
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
fn superopt_873(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -464 {
if rhs_1 == 12 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966844_u64 as i64);
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
fn superopt_874(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -416 {
if rhs_1 == 17 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966897_u64 as i64);
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
fn superopt_875(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 2 {
if rhs_2 == -4 {
if rhs_1 == 2 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 2147483647_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 1073741823_u64 as i64);

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
fn superopt_876(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -88 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967224_u64 as i64);

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
fn superopt_877(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -100 {
if rhs_1 == -4 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967192_u64 as i64);
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
fn superopt_878(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -56 {
if rhs_2 == 4 {
if rhs_1 == 16 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967260_u64 as i64);

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
fn superopt_879(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -96 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967216_u64 as i64);

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
fn superopt_880(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -368 {
if rhs_1 == -4 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966924_u64 as i64);
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
fn superopt_881(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
if rhs_1 == -128 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967248_u64 as i64);
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
fn superopt_882(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -40 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967272_u64 as i64);

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
fn superopt_883(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 255 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967040_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_7);

}
}
},
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
fn superopt_884(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -368 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if arg_1 != arg_2 {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294966927_u64 as i64);
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
fn superopt_885(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -64 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 63 {
if rhs_3 == 2 {
if arg_1 == arg_3 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_6);

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
fn superopt_886(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 11 {
if rhs_2 == -8 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_887(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 7 {
if rhs_2 == -8 {
if rhs_1 == -8 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 4294967295_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 4294967288_u64 as i64);

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
fn superopt_888(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
if rhs_3 == -2 {
if rhs_2 == 1 {
if rhs_1 == 20 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4294967276_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967276_u64 as i64);

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
fn superopt_889(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -40 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967284_u64 as i64);

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
fn superopt_890(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 2 {
if rhs_2 == 2 {
if rhs_1 == -4 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967292_u64 as i64);

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
fn superopt_891(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -360 {
if rhs_2 == 4 {
if rhs_1 == 7 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294966947_u64 as i64);

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
fn superopt_892(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -48 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967264_u64 as i64);

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
fn superopt_893(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_3 == 8 {
if rhs_2 == 2 {
if rhs_1 == 2744544 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4294967292_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2744576_u64 as i64);

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
fn superopt_894(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
if rhs_2 == 7 {
if rhs_1 == -1 {
if arg_2[1] != arg_2[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967288_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2[1]);
pos.func.dfg.replace(inst).bor_not(rhs_inst_7, arg_2[0]);

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
fn superopt_895(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2750708 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4294967294_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2750710_u64 as i64);

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
fn superopt_896(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -184 {
if rhs_2 == 4 {
if rhs_1 == 21 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967137_u64 as i64);

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
fn superopt_897(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2750704 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4294967294_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2750706_u64 as i64);

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
fn superopt_898(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -136 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967176_u64 as i64);

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
fn superopt_899(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -8 {
if rhs_3 == 3 {
if rhs_2 == 1 {
if rhs_1 == 3 {
pos.func.dfg.replace(inst).band_imm(arg_3, 18446744073709551608_u64 as i64);

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
fn superopt_900(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_3 == 1073741823 {
if rhs_2 == 2 {
if rhs_1 == 2598256 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2598256_u64 as i64);

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
fn superopt_901(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == -64 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 63 {
if rhs_3 == 4 {
if arg_1 == arg_3 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 15_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_3, rhs_inst_6);

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
fn superopt_902(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -104 {
if rhs_2 == 4 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294967208_u64 as i64);

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
fn superopt_903(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_3 == 21 {
if rhs_2 == -1 {
if rhs_1 == 268435456 {
let rhs_inst_5 = pos.ins().iconst(I64, 128_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 2097152_u64 as i64);

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
fn superopt_904(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -256 {
if rhs_1 == -12 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iadd_imm(arg_2, 4294967028_u64 as i64);
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
fn superopt_905(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1664 {
if rhs_2 == 4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294965639_u64 as i64);

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
fn superopt_906(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1168 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294966156_u64 as i64);

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
fn superopt_907(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 32767 {
if rhs_2 == -1 {
if rhs_1 == 16383 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 4294688767_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 16383_u64 as i64);

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
fn superopt_908(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1160 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294966164_u64 as i64);

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
fn superopt_909(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1264 {
if rhs_2 == 4 {
if rhs_1 == 24 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4294966060_u64 as i64);

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
fn superopt_910(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 2902360 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4294967288_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2902360_u64 as i64);

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
fn superopt_911(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 4 {
let rhs_inst_5 = pos.ins().iconst(I32, 2147483646_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 2147483646_u64 as i64);

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
fn superopt_912(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
if rhs_3 == 2 {
if rhs_2 == 1 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().iconst(I32, 4294967292_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 1073741822_u64 as i64);

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
fn superopt_913(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == 3 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_7 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_7 == 32 {
if arg_4 == arg_3 {
pos.func.dfg.replace(inst).band_imm(arg_4, 56_u64 as i64);

}
}
},
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
fn superopt_914(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().iconst(I32, 2147483640_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 536870910_u64 as i64);

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
fn superopt_915(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 65535 {
if rhs_1 == -1 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 4294901760_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
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
fn superopt_916(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 32 {
if rhs_2 == 4 {
if rhs_1 == 4294967295 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 17179869184_u64 as i64);
pos.func.dfg.replace(inst).ushr_imm(rhs_inst_5, 32_u64 as i64);

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
fn superopt_917(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == -1 {
if rhs_1 == -65536 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 1317011456_u64 as i64);
pos.func.dfg.replace(inst).bor_imm(rhs_inst_5, 4294901760_u64 as i64);

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
fn superopt_918(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 12 {
let rhs_inst_5 = pos.ins().iconst(I32, 2147483646_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 2147483642_u64 as i64);

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
fn superopt_919(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_5 = pos.ins().iconst(I32, 1073741820_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 1073741820_u64 as i64);

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
fn superopt_920(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
if rhs_2 == 4294967295 {
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_5 == 4294967295 {
if arg_2 == arg_3 {
pos.func.dfg.replace(inst).imul_imm(arg_0[1], 4294967297_u64 as i64);

}
}
},
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
fn superopt_921(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 32 {
if rhs_2 == 0 {
if rhs_1 == 32 {
let rhs_inst_5 = pos.ins().imul_imm(arg_2, 18446744069414584320_u64 as i64);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_5, 32_u64 as i64);

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
fn superopt_922(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 30 {
if rhs_2 == -1 {
if rhs_1 == 1073741822 {
let rhs_inst_5 = pos.ins().iconst(I32, 2147483648_u64 as i64);
let rhs_inst_6 = pos.ins().bor_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).sshr_imm(rhs_inst_6, 30_u64 as i64);

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
fn superopt_923(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
if rhs_2 == 1 {
if rhs_1 == 2147483647 {
if arg_2[1] != arg_2[0] {
let rhs_inst_6 = pos.ins().irsub_imm(arg_2[1], 2147483646_u64 as i64);
pos.func.dfg.replace(inst).isub(rhs_inst_6, arg_2[0]);

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
fn superopt_924(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_4 == 2 {
if rhs_3 == 7 {
if rhs_2 == 7 {
if rhs_1 == -3 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_925(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_0) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Binary { opcode, args } => {
let arg_1 = args;
match opcode {
Opcode::Imul => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 4294967295 {
if rhs_1 == 32 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4294967296_u64 as i64);
pos.func.dfg.replace(inst).imul(arg_1[1], rhs_inst_6);

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
fn superopt_926(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 134217727 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 1 {
if arg_2 != arg_1 {
let rhs_inst_6 = pos.ins().iconst(I32, 4160749568_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).isub(arg_2, rhs_inst_7);

}
}
},
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
fn superopt_927(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_3 == -1 {
if rhs_2 == -16 {
if rhs_1 == 1 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967294_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_928(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_3 == -1 {
if rhs_2 == -16 {
if rhs_1 == 8 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967287_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_929(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 2147483647 {
if rhs_2 == 3 {
if rhs_1 == 268435452 {
let rhs_inst_5 = pos.ins().sshr_imm(arg_2, 3_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 268435452_u64 as i64);

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
fn superopt_930(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == -512 {
if rhs_2 == 2 {
if rhs_1 == -524286 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 18446744073709027328_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 2_u64 as i64);

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
fn superopt_931(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_3 == -1 {
if rhs_2 == -16 {
if rhs_1 == 2 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967293_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_932(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_2 == 134217727 {
if rhs_1 == -1 {
if arg_0[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I32, 4160749568_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
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
fn superopt_933(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_3 == -1 {
if rhs_2 == -16 {
if rhs_1 == 4 {
let rhs_inst_6 = pos.ins().bor_imm(arg_2, 4294967291_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_934(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == -2147483648 {
if arg_2 != arg_0[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 2147483648_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_2);
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
fn superopt_935(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 1073741823 {
if arg_2 != arg_1[1] {
let rhs_inst_6 = pos.ins().iconst(I32, 1073741823_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_1[1], rhs_inst_7);

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
fn superopt_936(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 8 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 16_u64 as i64);
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
fn superopt_937(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_3 == 6 {
if rhs_2 == -1 {
if rhs_1 == 134217728 {
let rhs_inst_5 = pos.ins().iconst(I64, 1152921504608944128_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).imul_imm(rhs_inst_6, 64_u64 as i64);

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
fn superopt_938(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 12 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == -4 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 8_u64 as i64);
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
fn superopt_939(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_3 == 524288 {
if rhs_2 == -13631489 {
if rhs_1 == 9437184 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4280811519_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 9961472_u64 as i64);

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
fn superopt_940(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 92 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 96_u64 as i64);
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
fn superopt_941(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 4 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 20 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 24_u64 as i64);
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
fn superopt_942(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_3 == 524288 {
if rhs_2 == -12582913 {
if rhs_1 == 4194304 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4281860095_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 4718592_u64 as i64);

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
fn superopt_943(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
if rhs_3 == 524288 {
if rhs_2 == -12582913 {
if rhs_1 == 8388608 {
let rhs_inst_5 = pos.ins().band_imm(arg_2, 4281860095_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_5, 8912896_u64 as i64);

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
fn superopt_944(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 4 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 36_u64 as i64);
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
fn superopt_945(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 4 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 20_u64 as i64);
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
fn superopt_946(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 3 {
if rhs_1 == 40 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 48_u64 as i64);
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
fn superopt_947(pos: &mut FuncCursor, inst: Inst) {
match pos.func.dfg[inst] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_0 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
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
if rhs_2 == 2147483647 {
if rhs_1 == -2147483648 {
if arg_1[1] != arg_2 {
let rhs_inst_6 = pos.ins().iadd(arg_2, arg_1[1]);
pos.func.dfg.replace(inst).bor_imm(rhs_inst_6, 2147483648_u64 as i64);

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
fn superopt_948(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 63 {
if rhs_2 == 1 {
if rhs_1 == 24 {
if arg_1[0] != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_2, 25_u64 as i64);
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
fn superopt_949(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == -8 {
if rhs_1 == 16 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 8_u64 as i64);
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
fn superopt_950(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 268435455 {
if rhs_2 == 2 {
if rhs_1 == 268435455 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 4026531842_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 268435455_u64 as i64);

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
fn superopt_951(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == -4 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 12_u64 as i64);
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
fn superopt_952(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 128 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 8 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 136_u64 as i64);
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
fn superopt_953(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == -8 {
if rhs_1 == 20 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 12_u64 as i64);
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
fn superopt_954(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 48 {
if rhs_1 == 24 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 72_u64 as i64);
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
fn superopt_955(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 35 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 67_u64 as i64);
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
fn superopt_956(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 24 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 56_u64 as i64);
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
fn superopt_957(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 14 {
if rhs_1 == 7 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 55_u64 as i64);
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
fn superopt_958(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 960 {
if rhs_3 == 6 {
if rhs_2 == 1 {
if rhs_1 == 16 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 1024_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 65536_u64 as i64);

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
fn superopt_959(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 48 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 48 {
if rhs_1 == 16 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 64_u64 as i64);
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
fn superopt_960(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 31 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 63_u64 as i64);
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
fn superopt_961(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 32 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 64_u64 as i64);
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
fn superopt_962(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 24 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 20 {
if rhs_1 == 20 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 44_u64 as i64);
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
fn superopt_963(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 960 {
if rhs_3 == 6 {
if rhs_2 == 1 {
if rhs_1 == 16 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 1024_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 65536_u64 as i64);

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
fn superopt_964(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 28 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 60_u64 as i64);
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
fn superopt_965(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 40 {
if rhs_1 == 17 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 49_u64 as i64);
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
fn superopt_966(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 80 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 144 {
if rhs_1 == 16 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 96_u64 as i64);
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
fn superopt_967(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 560 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 20 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 580_u64 as i64);
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
fn superopt_968(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 160 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 3 {
if rhs_1 == -8 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 152_u64 as i64);
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
fn superopt_969(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 320 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 3 {
if rhs_1 == -8 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 312_u64 as i64);
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
fn superopt_970(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 480 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == -4 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 476_u64 as i64);
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
fn superopt_971(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 14 {
if rhs_2 == -524289 {
if rhs_1 == 524288 {
let rhs_inst_5 = pos.ins().iconst(I64, 32_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
let rhs_inst_7 = pos.ins().iconst(I64, 18446744073709027327_u64 as i64);
pos.func.dfg.replace(inst).sshr(rhs_inst_7, rhs_inst_6);

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
fn superopt_972(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::Unary { opcode, arg } => {
let arg_4 = arg;
match opcode {
Opcode::Clz => {
if rhs_3 == 1 {
if rhs_2 == 64 {
if rhs_1 == 32768 {
if arg_1[0] != arg_4 {
pos.func.dfg.replace(inst).band_imm(arg_3, 32768_u64 as i64);

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
fn superopt_973(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_1 == 48 {
match pos.func.dfg.value_def(arg_0[1]) {

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
Opcode::BandImm => {
if rhs_4 == 255 {
if rhs_3 == -1 {
if arg_1 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_2, 47_u64 as i64);
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
},
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
fn superopt_974(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 28 {
if rhs_1 == 24 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 328_u64 as i64);
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
fn superopt_975(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 304 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 28 {
if rhs_1 == 12 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 316_u64 as i64);
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
fn superopt_976(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 9 {
if rhs_3 == -1 {
if rhs_2 == 3 {
if rhs_1 == 1493552 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 1493544_u64 as i64);

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
fn superopt_977(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Band => {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 34634616274944 {
if arg_1[0] != arg_2 {
let rhs_inst_6 = pos.ins().iconst(I64, 34634616274944_u64 as i64);
let rhs_inst_7 = pos.ins().band_not(rhs_inst_6, arg_2);
pos.func.dfg.replace(inst).band(arg_2, rhs_inst_7);

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
fn superopt_978(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -1 {
if rhs_2 == -4 {
if rhs_1 == 3 {
let rhs_inst_5 = pos.ins().irsub_imm(arg_2, 5082235795319799808_u64 as i64);
pos.func.dfg.replace(inst).bor_imm(rhs_inst_5, 18446744073709551612_u64 as i64);

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
fn superopt_979(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 15 {
if rhs_3 == -2 {
if rhs_2 == 2 {
if rhs_1 == 2531908 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 2531900_u64 as i64);

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
fn superopt_980(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 15 {
if rhs_3 == -2 {
if rhs_2 == 3 {
if rhs_1 == 1519376 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 8_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 1519360_u64 as i64);

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
fn superopt_981(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 32767 {
if rhs_3 == -1 {
if rhs_2 == 2 {
if rhs_1 == 879400 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 879396_u64 as i64);

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
fn superopt_982(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_2 == -1 {
if rhs_1 == 1 {
if arg_3 != arg_2[1] && arg_3 != arg_1[1] && arg_2[1] != arg_1[1] {
let rhs_inst_8 = pos.ins().isub(arg_2[1], arg_3);
pos.func.dfg.replace(inst).iadd(arg_1[1], rhs_inst_8);

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
fn superopt_983(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 37 {
if rhs_3 == 4294967295 {
if rhs_2 == 32 {
if rhs_1 == -1 {
let rhs_inst_6 = pos.ins().imul_imm(arg_3, 158913789952_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_6);

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
fn superopt_984(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 4294967291 {
if rhs_2 == 4 {
if rhs_1 == 0 {
let rhs_inst_5 = pos.ins().iconst(I64, 4294967291_u64 as i64);
let rhs_inst_6 = pos.ins().band_not(rhs_inst_5, arg_2);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 18446744069414584321_u64 as i64);

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
fn superopt_985(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 32767 {
if rhs_3 == -32 {
if rhs_2 == 2 {
if rhs_1 == 257196 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 257068_u64 as i64);

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
fn superopt_986(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 127 {
if rhs_3 == -24 {
if rhs_2 == 2 {
if rhs_1 == 2591040 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 2590944_u64 as i64);

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
fn superopt_987(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == 32767 {
if rhs_3 == -32 {
if rhs_2 == 2 {
if rhs_1 == 257232 {
let rhs_inst_6 = pos.ins().imul_imm(arg_2, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 257104_u64 as i64);

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
fn superopt_988(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_3) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_5 == 9 {
if rhs_4 == 8191 {
if rhs_3 == -27 {
if rhs_2 == 9 {
if rhs_1 == 13824 {
pos.func.dfg.replace(inst).band_imm(arg_4, 4193792_u64 as i64);

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
fn superopt_989(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == 7 {
if rhs_2 == -8 {
if rhs_1 == -8 {
let rhs_inst_5 = pos.ins().iadd_imm(arg_2, 18446744073709551615_u64 as i64);
pos.func.dfg.replace(inst).band_imm(rhs_inst_5, 18446744073709551608_u64 as i64);

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
fn superopt_990(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::SshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 28 {
if rhs_3 == 28 {
if rhs_2 == 2 {
if rhs_1 == 2528056 {
let rhs_inst_6 = pos.ins().sshr_imm(arg_2, 26_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_6, 2528056_u64 as i64);

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
fn superopt_991(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BxorImm => {
if rhs_3 == 1 {
if rhs_2 == 0 {
if rhs_1 == 12 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().imul_imm(arg_2, 4294967284_u64 as i64);
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
fn superopt_992(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_4 == -8 {
if rhs_1 == 4 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 4294967292_u64 as i64);
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
fn superopt_993(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 31 {
if rhs_1 == -32 {
if arg_1[0] != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_2, 4294967264_u64 as i64);
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
fn superopt_994(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -24 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 12 {
if rhs_1 == 4 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 4294967276_u64 as i64);
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
fn superopt_995(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -12 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::ImulImm => {
if rhs_4 == 12 {
if rhs_1 == 4 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[1], 4294967288_u64 as i64);
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
fn superopt_996(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Iadd => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_1 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_1 == 2 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_5 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 2 {
if arg_2[0] != arg_4 && arg_2[0] != arg_3 && arg_2[0] != arg_5 && arg_4 == arg_3 && arg_4 == arg_5 && arg_3 == arg_5 {
let rhs_inst_7 = pos.ins().imul_imm(arg_5, 11_u64 as i64);
let rhs_inst_8 = pos.ins().iadd(arg_5, rhs_inst_7);
pos.func.dfg.replace(inst).iadd(arg_5, rhs_inst_8);

}
}
},
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_997(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
if rhs_2 == 58 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -65 {
if rhs_1 == 2743936 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().iadd_imm(arg_1[0], 2743871_u64 as i64);
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
fn superopt_998(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == 16 {
if rhs_1 == -16 {
let rhs_inst_6 = pos.ins().bxor_imm(arg_3, 7_u64 as i64);
let rhs_inst_7 = pos.ins().iadd_imm(rhs_inst_6, 9_u64 as i64);
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
fn superopt_999(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 1 {
if rhs_2 == -1 {
if rhs_1 == 1 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 1_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1000(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 2 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 3_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1001(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 3 {
if arg_3 != arg_0[0] {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 7_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1002(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -8 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == 1 {
if rhs_1 == 1 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 5_u64 as i64);
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
fn superopt_1003(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

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
Opcode::SshrImm => {
if rhs_3 == 3 {
if rhs_2 == 3 {
if rhs_1 == -8 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 7_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1004(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Band => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 0 {
if arg_3 == arg_2[0] {
let rhs_inst_6 = pos.ins().iconst(I32, 4008636142_u64 as i64);
let rhs_inst_7 = pos.ins().ishl(rhs_inst_6, arg_3);
let rhs_inst_8 = pos.ins().sshr_imm(rhs_inst_7, 31_u64 as i64);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1005(pos: &mut FuncCursor, inst: Inst) {
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
InstructionData::Binary { opcode, args } => {
let arg_2 = args;
match opcode {
Opcode::Bor => {
match pos.func.dfg.value_def(arg_2[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 32 {
if rhs_2 == 1 {
if rhs_1 == 0 {
if arg_2[0] != arg_3 {
let rhs_inst_7 = pos.ins().iconst(I64, 1_u64 as i64);
let rhs_inst_8 = pos.ins().bor_not(rhs_inst_7, arg_3);
let rhs_inst_9 = pos.ins().iadd(arg_3, rhs_inst_8);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

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
fn superopt_1006(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 4 {
if rhs_2 == -1 {
if rhs_1 == 4 {
if arg_3 != arg_0[0] {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 15_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1007(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 6 {
if rhs_2 == -1 {
if rhs_1 == 6 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 63_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1008(pos: &mut FuncCursor, inst: Inst) {
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
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 8 {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_6 = pos.ins().sshr_imm(arg_3, 6_u64 as i64);
let rhs_inst_7 = pos.ins().band_imm(rhs_inst_6, 4_u64 as i64);
pos.func.dfg.replace(inst).iadd_imm(rhs_inst_7, 4_u64 as i64);

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
fn superopt_1009(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
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
if rhs_3 == 5 {
if rhs_2 == -1 {
if rhs_1 == 5 {
if arg_3 != arg_0[0] {
let rhs_inst_7 = pos.ins().bor_imm(arg_3, 31_u64 as i64);
let rhs_inst_8 = pos.ins().isub(rhs_inst_7, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1010(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::ImulImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::IrsubImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
if rhs_3 == 268435455 {
if rhs_2 == 0 {
if rhs_1 == 12 {
if arg_3 != arg_0[0] {
let rhs_inst_7 = pos.ins().imul_imm(arg_2, 4294967284_u64 as i64);
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
fn superopt_1011(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == 1 {
if rhs_1 == 2 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 12_u64 as i64);
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
fn superopt_1012(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
let rhs_inst_6 = pos.ins().sshr_imm(arg_3, 2_u64 as i64);
let rhs_inst_7 = pos.ins().bor_imm(rhs_inst_6, 4294967293_u64 as i64);
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
fn superopt_1013(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -16 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == 1 {
if rhs_1 == 1 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 13_u64 as i64);
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
fn superopt_1014(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -4 {
if rhs_3 == 2 {
if rhs_2 == -1 {
if rhs_1 == 2 {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967292_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_3);
let rhs_inst_8 = pos.ins().iadd(arg_3, rhs_inst_7);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1015(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_3 == -4 {
if rhs_2 == -4 {
if rhs_1 == 4 {
if arg_3 != arg_1[0] {
let rhs_inst_7 = pos.ins().iconst(I32, 3_u64 as i64);
let rhs_inst_8 = pos.ins().bor_not(rhs_inst_7, arg_3);
let rhs_inst_9 = pos.ins().isub(rhs_inst_8, arg_3);
pos.func.dfg.replace(inst).bnot(rhs_inst_9);

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
fn superopt_1016(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BxorImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -8 {
if rhs_3 == 3 {
if rhs_2 == -1 {
if rhs_1 == 3 {
let rhs_inst_6 = pos.ins().iconst(I32, 4294967288_u64 as i64);
let rhs_inst_7 = pos.ins().bor_not(rhs_inst_6, arg_3);
let rhs_inst_8 = pos.ins().iadd(arg_3, rhs_inst_7);
pos.func.dfg.replace(inst).bnot(rhs_inst_8);

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
fn superopt_1017(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -1 {
if rhs_1 == 21 {
if arg_2 != arg_3 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 11_u64 as i64);
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
fn superopt_1018(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::Bor => {
match pos.func.dfg.value_def(arg_1[0]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_2 == 2 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::UshrImm => {
if rhs_4 == 3 {
if rhs_1 == 1 {
if arg_2 == arg_3 {
let rhs_inst_6 = pos.ins().iconst(I32, 983055_u64 as i64);
let rhs_inst_7 = pos.ins().sshr(rhs_inst_6, arg_3);
let rhs_inst_8 = pos.ins().bor_imm(rhs_inst_7, 4294967294_u64 as i64);
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
fn superopt_1019(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::UshrImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::BandImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 3 {
if rhs_3 == 128 {
if rhs_2 == 7 {
if rhs_1 == 1 {
let rhs_inst_6 = pos.ins().sshr_imm(arg_3, 4_u64 as i64);
let rhs_inst_7 = pos.ins().bor_imm(rhs_inst_6, 4294967294_u64 as i64);
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
fn superopt_1020(pos: &mut FuncCursor, inst: Inst) {
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
if rhs_2 == -32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IaddImm => {
if rhs_4 == -1 {
if rhs_1 == 20 {
if arg_3 != arg_2 {
let rhs_inst_7 = pos.ins().irsub_imm(arg_3, 12_u64 as i64);
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
fn superopt_1021(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_1 == 44 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 2 {
if arg_2 != arg_4 && arg_2 != arg_3 && arg_4 == arg_3 {
let rhs_inst_7 = pos.ins().imul_imm(arg_4, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 44_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_4, rhs_inst_8);

}
}
},
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_1022(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
match pos.func.dfg.value_def(arg_1) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_2 = arg;
let rhs_2 : i64 = imm.into();
match opcode {
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_2) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 1 {
if rhs_1 == 2 {
if arg_3 != arg_0[0] {
let rhs_inst_7 = pos.ins().imul_imm(arg_3, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 4_u64 as i64);
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
fn superopt_1023(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_1 == 60 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 2 {
if arg_3 != arg_2 && arg_3 == arg_4 && arg_2 != arg_4 {
let rhs_inst_7 = pos.ins().imul_imm(arg_4, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 60_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_4, rhs_inst_8);

}
}
},
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_1024(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_1 == 28 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 2 {
if arg_2 != arg_4 && arg_2 != arg_3 && arg_4 == arg_3 {
let rhs_inst_7 = pos.ins().imul_imm(arg_4, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 28_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_4, rhs_inst_8);

}
}
},
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_1025(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::BorImm => {
match pos.func.dfg.value_def(arg_1) {

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
Opcode::IshlImm => {
if rhs_3 == 1 {
if rhs_2 == 2 {
if rhs_1 == 4 {
if arg_0[0] != arg_3 {
let rhs_inst_7 = pos.ins().imul_imm(arg_3, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 4_u64 as i64);
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
fn superopt_1026(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_2 == 2 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 28 {
if arg_2[0] != arg_3 && arg_2[0] != arg_4 && arg_3 == arg_4 {
let rhs_inst_7 = pos.ins().imul_imm(arg_4, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 28_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_4, rhs_inst_8);

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
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_1027(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IaddImm => {
if rhs_1 == 32 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_3 = arg;
let rhs_3 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_3 == 2 {
match pos.func.dfg.value_def(arg_0[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_5 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_5 == 2 {
if arg_3 == arg_4 && arg_3 != arg_2 && arg_4 != arg_2 {
let rhs_inst_7 = pos.ins().imul_imm(arg_4, 8_u64 as i64);
let rhs_inst_8 = pos.ins().iadd_imm(rhs_inst_7, 32_u64 as i64);
pos.func.dfg.replace(inst).iadd(arg_4, rhs_inst_8);

}
}
},
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
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
},
_ => {},
}
}
fn superopt_1028(pos: &mut FuncCursor, inst: Inst) {
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
Opcode::IshlImm => {
if rhs_2 == 2 {
match pos.func.dfg.value_def(arg_1[1]) {

ValueDef::Result(arg_ty, _) => {
match pos.func.dfg[arg_ty] {
InstructionData::BinaryImm64 { opcode, arg, imm } => {
let arg_4 = arg;
let rhs_4 : i64 = imm.into();
match opcode {
Opcode::IshlImm => {
if rhs_4 == 2 {
if rhs_1 == 44 {
if arg_2[0] != arg_3 && arg_2[0] != arg_4 && arg_3 == arg_4 {

}
}
}
},
_ => {},
}
},
}
}