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
if arg_1[1] == arg_2[1] && arg_1[1] == arg_0[1] && arg_1[1] != arg_2[0] && arg_2[1] == arg_0[1] && arg_2[1] != arg_2[0] && arg_0[1] != arg_2[0] {
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
if arg_1[0] != arg_1[1] && arg_1[0] == arg_0[1] && arg_1[1] != arg_0[1] {
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
if arg_1[0] != arg_0[1] && arg_1[0] != arg_1[1] && arg_0[1] == arg_1[1] {
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
if arg_1 != arg_0[0] {
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
if arg_0[0] != arg_1 {
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
if arg_2 == arg_0[1] {
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
if arg_2 == arg_1 {
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
if arg_2 == arg_1 {
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
if arg_2 == arg_1 {
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
if arg_2 != arg_0[1] {
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
if arg_2[0] != arg_3 && arg_2[0] == arg_1 && arg_3 != arg_1 {
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
if arg_1[1] != arg_2 {
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
if arg_0[1] != arg_2 {
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
if arg_0[0] != arg_2 {
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
if arg_5 == arg_2 && arg_5 != arg_3 && arg_5 != arg_6 && arg_2 != arg_3 && arg_2 != arg_6 && arg_3 == arg_6 {
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
if arg_2[1] != arg_2[0] {
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
if arg_1[0] != arg_2 {
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
if arg_2 != arg_1[0] {
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
if arg_1[0] != arg_2 {
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
if arg_2 != arg_0[1] {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2 == arg_0[1] {
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
if arg_2[0] != arg_1[1] && arg_2[0] != arg_0[1] && arg_2[0] != arg_2[1] && arg_1[1] == arg_0[1] && arg_1[1] != arg_2[1] && arg_0[1] != arg_2[1] {
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
if arg_1[0] != arg_2 {
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
if arg_2 != arg_1[0] {
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
if arg_1[0] != arg_2 {
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
if arg_0[0] != arg_2[0] && arg_0[0] != arg_2[1] && arg_2[0] != arg_2[1] {
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
if arg_2[1] != arg_2[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2[1] != arg_2[0] && arg_2[1] != arg_0[0] && arg_2[0] != arg_0[0] {
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
if arg_2[1] != arg_2[0] && arg_2[1] != arg_0[1] && arg_2[0] != arg_0[1] {
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
if arg_2 != arg_1[0] {
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
if arg_3 != arg_1[1] && arg_3 == arg_2 && arg_1[1] != arg_2 {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_1[0] != arg_2 && arg_1[0] != arg_3 && arg_2 == arg_3 {
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
if arg_1[0] != arg_2 {
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
if arg_1 != arg_2 {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_2 != arg_1[0] {
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
if arg_1[0] != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_1 {
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
if arg_3 != arg_2 {
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
if arg_3 != arg_0[0] {
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
if arg_2[0] != arg_2[1] {
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
if arg_2 != arg_1[1] {
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
if arg_1[0] != arg_2 {
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
if arg_2 != arg_1[0] {
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
if arg_1 != arg_2 {
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
if arg_2[0] != arg_1[0] && arg_2[0] != arg_2[1] && arg_2[0] != arg_0[1] && arg_1[0] != arg_2[1] && arg_1[0] != arg_0[1] && arg_2[1] == arg_0[1] {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_0[0] {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_1[1] {
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
if arg_1 != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_1 != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_1[1] {
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
if arg_1[1] != arg_2 {
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
if arg_1[1] != arg_2 {
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
if arg_2 != arg_0[0] {
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
superopt_110(&mut pos, inst);
superopt_111(&mut pos, inst);
superopt_112(&mut pos, inst);
superopt_113(&mut pos, inst);
superopt_114(&mut pos, inst);
superopt_115(&mut pos, inst);
superopt_116(&mut pos, inst);
superopt_117(&mut pos, inst);
superopt_118(&mut pos, inst);
superopt_119(&mut pos, inst);
superopt_120(&mut pos, inst);
superopt_121(&mut pos, inst);
superopt_122(&mut pos, inst);
superopt_123(&mut pos, inst);
superopt_124(&mut pos, inst);
superopt_125(&mut pos, inst);
superopt_126(&mut pos, inst);
superopt_127(&mut pos, inst);
superopt_128(&mut pos, inst);
superopt_129(&mut pos, inst);
superopt_130(&mut pos, inst);
superopt_131(&mut pos, inst);
superopt_132(&mut pos, inst);
superopt_133(&mut pos, inst);
superopt_134(&mut pos, inst);
superopt_135(&mut pos, inst);
superopt_136(&mut pos, inst);
superopt_137(&mut pos, inst);
superopt_138(&mut pos, inst);
superopt_139(&mut pos, inst);
superopt_140(&mut pos, inst);
superopt_141(&mut pos, inst);
superopt_142(&mut pos, inst);
superopt_143(&mut pos, inst);
superopt_144(&mut pos, inst);
superopt_145(&mut pos, inst);
superopt_146(&mut pos, inst);
superopt_147(&mut pos, inst);
superopt_148(&mut pos, inst);
superopt_149(&mut pos, inst);
superopt_150(&mut pos, inst);
superopt_151(&mut pos, inst);
superopt_152(&mut pos, inst);
superopt_153(&mut pos, inst);
superopt_154(&mut pos, inst);
superopt_155(&mut pos, inst);
superopt_156(&mut pos, inst);
superopt_157(&mut pos, inst);
superopt_158(&mut pos, inst);
superopt_159(&mut pos, inst);
superopt_160(&mut pos, inst);
superopt_161(&mut pos, inst);
superopt_162(&mut pos, inst);
superopt_163(&mut pos, inst);
superopt_164(&mut pos, inst);
superopt_165(&mut pos, inst);
superopt_166(&mut pos, inst);
superopt_167(&mut pos, inst);
superopt_168(&mut pos, inst);
superopt_169(&mut pos, inst);
superopt_170(&mut pos, inst);
superopt_171(&mut pos, inst);
superopt_172(&mut pos, inst);
superopt_173(&mut pos, inst);
superopt_174(&mut pos, inst);
superopt_175(&mut pos, inst);
superopt_176(&mut pos, inst);
superopt_177(&mut pos, inst);
superopt_178(&mut pos, inst);
superopt_179(&mut pos, inst);
superopt_180(&mut pos, inst);
superopt_181(&mut pos, inst);
superopt_182(&mut pos, inst);
superopt_183(&mut pos, inst);
superopt_184(&mut pos, inst);
superopt_185(&mut pos, inst);
superopt_186(&mut pos, inst);
superopt_187(&mut pos, inst);
superopt_188(&mut pos, inst);
superopt_189(&mut pos, inst);
superopt_190(&mut pos, inst);
superopt_191(&mut pos, inst);
superopt_192(&mut pos, inst);
superopt_193(&mut pos, inst);
superopt_194(&mut pos, inst);
superopt_195(&mut pos, inst);
superopt_196(&mut pos, inst);
superopt_197(&mut pos, inst);
superopt_198(&mut pos, inst);
superopt_199(&mut pos, inst);
superopt_200(&mut pos, inst);
superopt_201(&mut pos, inst);
superopt_202(&mut pos, inst);
superopt_203(&mut pos, inst);
superopt_204(&mut pos, inst);
superopt_205(&mut pos, inst);
superopt_206(&mut pos, inst);
superopt_207(&mut pos, inst);
superopt_208(&mut pos, inst);
superopt_209(&mut pos, inst);
superopt_210(&mut pos, inst);
superopt_211(&mut pos, inst);
superopt_212(&mut pos, inst);
superopt_213(&mut pos, inst);
superopt_214(&mut pos, inst);
superopt_215(&mut pos, inst);
superopt_216(&mut pos, inst);
superopt_217(&mut pos, inst);
superopt_218(&mut pos, inst);
superopt_219(&mut pos, inst);
superopt_220(&mut pos, inst);
superopt_221(&mut pos, inst);
superopt_222(&mut pos, inst);
superopt_223(&mut pos, inst);
superopt_224(&mut pos, inst);
superopt_225(&mut pos, inst);
superopt_226(&mut pos, inst);
superopt_227(&mut pos, inst);
superopt_228(&mut pos, inst);
superopt_229(&mut pos, inst);
superopt_230(&mut pos, inst);
superopt_231(&mut pos, inst);
superopt_232(&mut pos, inst);
superopt_233(&mut pos, inst);
superopt_234(&mut pos, inst);
superopt_235(&mut pos, inst);
superopt_236(&mut pos, inst);
superopt_237(&mut pos, inst);
superopt_238(&mut pos, inst);
superopt_239(&mut pos, inst);
superopt_240(&mut pos, inst);
superopt_241(&mut pos, inst);
superopt_242(&mut pos, inst);
superopt_243(&mut pos, inst);
superopt_244(&mut pos, inst);
superopt_245(&mut pos, inst);
superopt_246(&mut pos, inst);
superopt_247(&mut pos, inst);
superopt_248(&mut pos, inst);
superopt_249(&mut pos, inst);
superopt_250(&mut pos, inst);
superopt_251(&mut pos, inst);
superopt_252(&mut pos, inst);
superopt_253(&mut pos, inst);
superopt_254(&mut pos, inst);
superopt_255(&mut pos, inst);
superopt_256(&mut pos, inst);
superopt_257(&mut pos, inst);
superopt_258(&mut pos, inst);
superopt_259(&mut pos, inst);
superopt_260(&mut pos, inst);
superopt_261(&mut pos, inst);
superopt_262(&mut pos, inst);
superopt_263(&mut pos, inst);
superopt_264(&mut pos, inst);
superopt_265(&mut pos, inst);
superopt_266(&mut pos, inst);
superopt_267(&mut pos, inst);
superopt_268(&mut pos, inst);
superopt_269(&mut pos, inst);
superopt_270(&mut pos, inst);
superopt_271(&mut pos, inst);
superopt_272(&mut pos, inst);
superopt_273(&mut pos, inst);
superopt_274(&mut pos, inst);
superopt_275(&mut pos, inst);
superopt_276(&mut pos, inst);
superopt_277(&mut pos, inst);
superopt_278(&mut pos, inst);
superopt_279(&mut pos, inst);
superopt_280(&mut pos, inst);
superopt_281(&mut pos, inst);
superopt_282(&mut pos, inst);
superopt_283(&mut pos, inst);
superopt_284(&mut pos, inst);
superopt_285(&mut pos, inst);
superopt_286(&mut pos, inst);
superopt_287(&mut pos, inst);
superopt_288(&mut pos, inst);
superopt_289(&mut pos, inst);
superopt_290(&mut pos, inst);
superopt_291(&mut pos, inst);
superopt_292(&mut pos, inst);
superopt_293(&mut pos, inst);
superopt_294(&mut pos, inst);
superopt_295(&mut pos, inst);
superopt_296(&mut pos, inst);
superopt_297(&mut pos, inst);
superopt_298(&mut pos, inst);
superopt_299(&mut pos, inst);
superopt_300(&mut pos, inst);
superopt_301(&mut pos, inst);
superopt_302(&mut pos, inst);
superopt_303(&mut pos, inst);
superopt_304(&mut pos, inst);
superopt_305(&mut pos, inst);
superopt_306(&mut pos, inst);
superopt_307(&mut pos, inst);
superopt_308(&mut pos, inst);
superopt_309(&mut pos, inst);
superopt_310(&mut pos, inst);
superopt_311(&mut pos, inst);
superopt_312(&mut pos, inst);
superopt_313(&mut pos, inst);
superopt_314(&mut pos, inst);
superopt_315(&mut pos, inst);
superopt_316(&mut pos, inst);
superopt_317(&mut pos, inst);
superopt_318(&mut pos, inst);
superopt_319(&mut pos, inst);
superopt_320(&mut pos, inst);
superopt_321(&mut pos, inst);
superopt_322(&mut pos, inst);
superopt_323(&mut pos, inst);
superopt_324(&mut pos, inst);
superopt_325(&mut pos, inst);
superopt_326(&mut pos, inst);
superopt_327(&mut pos, inst);
superopt_328(&mut pos, inst);
superopt_329(&mut pos, inst);
superopt_330(&mut pos, inst);
superopt_331(&mut pos, inst);
superopt_332(&mut pos, inst);
superopt_333(&mut pos, inst);
superopt_334(&mut pos, inst);
superopt_335(&mut pos, inst);
superopt_336(&mut pos, inst);
superopt_337(&mut pos, inst);
superopt_338(&mut pos, inst);
superopt_339(&mut pos, inst);
superopt_340(&mut pos, inst);
superopt_341(&mut pos, inst);
superopt_342(&mut pos, inst);
superopt_343(&mut pos, inst);
superopt_344(&mut pos, inst);
superopt_345(&mut pos, inst);
superopt_346(&mut pos, inst);
superopt_347(&mut pos, inst);
superopt_348(&mut pos, inst);
superopt_349(&mut pos, inst);
superopt_350(&mut pos, inst);
superopt_351(&mut pos, inst);
superopt_352(&mut pos, inst);
superopt_353(&mut pos, inst);
superopt_354(&mut pos, inst);
superopt_355(&mut pos, inst);
superopt_356(&mut pos, inst);
superopt_357(&mut pos, inst);
superopt_358(&mut pos, inst);
superopt_359(&mut pos, inst);
superopt_360(&mut pos, inst);
superopt_361(&mut pos, inst);
superopt_362(&mut pos, inst);
superopt_363(&mut pos, inst);
superopt_364(&mut pos, inst);
superopt_365(&mut pos, inst);
superopt_366(&mut pos, inst);
superopt_367(&mut pos, inst);
superopt_368(&mut pos, inst);
superopt_369(&mut pos, inst);
superopt_370(&mut pos, inst);
superopt_371(&mut pos, inst);
superopt_372(&mut pos, inst);
superopt_373(&mut pos, inst);
superopt_374(&mut pos, inst);
superopt_375(&mut pos, inst);
superopt_376(&mut pos, inst);
superopt_377(&mut pos, inst);
superopt_378(&mut pos, inst);
superopt_379(&mut pos, inst);
superopt_380(&mut pos, inst);
superopt_381(&mut pos, inst);
superopt_382(&mut pos, inst);
superopt_383(&mut pos, inst);
superopt_384(&mut pos, inst);
superopt_385(&mut pos, inst);
superopt_386(&mut pos, inst);
superopt_387(&mut pos, inst);
superopt_388(&mut pos, inst);
superopt_389(&mut pos, inst);
superopt_390(&mut pos, inst);
superopt_391(&mut pos, inst);
superopt_392(&mut pos, inst);
superopt_393(&mut pos, inst);
superopt_394(&mut pos, inst);
superopt_395(&mut pos, inst);
superopt_396(&mut pos, inst);
superopt_397(&mut pos, inst);
superopt_398(&mut pos, inst);
superopt_399(&mut pos, inst);
superopt_400(&mut pos, inst);
superopt_401(&mut pos, inst);
superopt_402(&mut pos, inst);
superopt_403(&mut pos, inst);
superopt_404(&mut pos, inst);
superopt_405(&mut pos, inst);
superopt_406(&mut pos, inst);
superopt_407(&mut pos, inst);
superopt_408(&mut pos, inst);
superopt_409(&mut pos, inst);
superopt_410(&mut pos, inst);
superopt_411(&mut pos, inst);
superopt_412(&mut pos, inst);
superopt_413(&mut pos, inst);
superopt_414(&mut pos, inst);
superopt_415(&mut pos, inst);
superopt_416(&mut pos, inst);
superopt_417(&mut pos, inst);
superopt_418(&mut pos, inst);
superopt_419(&mut pos, inst);
superopt_420(&mut pos, inst);
superopt_421(&mut pos, inst);
superopt_422(&mut pos, inst);
superopt_423(&mut pos, inst);
superopt_424(&mut pos, inst);
superopt_425(&mut pos, inst);
superopt_426(&mut pos, inst);
superopt_427(&mut pos, inst);
superopt_428(&mut pos, inst);
superopt_429(&mut pos, inst);
superopt_430(&mut pos, inst);
superopt_431(&mut pos, inst);
superopt_432(&mut pos, inst);
superopt_433(&mut pos, inst);
superopt_434(&mut pos, inst);
superopt_435(&mut pos, inst);
superopt_436(&mut pos, inst);
superopt_437(&mut pos, inst);
superopt_438(&mut pos, inst);
superopt_439(&mut pos, inst);
superopt_440(&mut pos, inst);
superopt_441(&mut pos, inst);
superopt_442(&mut pos, inst);
superopt_443(&mut pos, inst);
superopt_444(&mut pos, inst);
superopt_445(&mut pos, inst);
superopt_446(&mut pos, inst);
superopt_447(&mut pos, inst);
superopt_448(&mut pos, inst);
superopt_449(&mut pos, inst);
superopt_450(&mut pos, inst);
superopt_451(&mut pos, inst);
superopt_452(&mut pos, inst);
superopt_453(&mut pos, inst);
superopt_454(&mut pos, inst);
superopt_455(&mut pos, inst);
superopt_456(&mut pos, inst);
superopt_457(&mut pos, inst);
superopt_458(&mut pos, inst);
superopt_459(&mut pos, inst);
superopt_460(&mut pos, inst);
superopt_461(&mut pos, inst);
superopt_462(&mut pos, inst);
superopt_463(&mut pos, inst);
superopt_464(&mut pos, inst);
superopt_465(&mut pos, inst);
superopt_466(&mut pos, inst);
superopt_467(&mut pos, inst);
superopt_468(&mut pos, inst);
superopt_469(&mut pos, inst);
superopt_470(&mut pos, inst);
superopt_471(&mut pos, inst);
superopt_472(&mut pos, inst);
superopt_473(&mut pos, inst);
superopt_474(&mut pos, inst);
superopt_475(&mut pos, inst);
superopt_476(&mut pos, inst);
superopt_477(&mut pos, inst);
superopt_478(&mut pos, inst);
superopt_479(&mut pos, inst);
superopt_480(&mut pos, inst);
superopt_481(&mut pos, inst);
superopt_482(&mut pos, inst);
superopt_483(&mut pos, inst);
superopt_484(&mut pos, inst);
superopt_485(&mut pos, inst);
superopt_486(&mut pos, inst);
superopt_487(&mut pos, inst);
superopt_488(&mut pos, inst);
superopt_489(&mut pos, inst);
superopt_490(&mut pos, inst);
superopt_491(&mut pos, inst);
superopt_492(&mut pos, inst);
superopt_493(&mut pos, inst);
superopt_494(&mut pos, inst);
superopt_495(&mut pos, inst);
superopt_496(&mut pos, inst);
superopt_497(&mut pos, inst);
superopt_498(&mut pos, inst);
superopt_499(&mut pos, inst);
superopt_500(&mut pos, inst);
        }
    }
}
