//! A peephole optimization (baseline) pass.

use crate::cursor::{Cursor, FuncCursor};
// use crate::isa::TargetIsa;
use crate::ir::instructions::Opcode;
use crate::ir::Inst;
use crate::timing;
//use crate::ir::condcodes::IntCC;
use crate::ir::dfg::ValueDef;
use crate::ir::types;
use crate::ir::{Function, InstBuilder, InstructionData};
use cranelift_codegen_shared::condcodes::IntCC;

fn superopt_1(pos: &mut FuncCursor, inst: Inst) {
    //println!("\n\nSuperopt begin \n\n");
    match pos.func.dfg[inst] {
        InstructionData::Binary { opcode, args } => {
            let arg_0 = args;
            match opcode {
                Opcode::Iadd => match pos.func.dfg.value_def(arg_0[0]) {
                    ValueDef::Result(arg_ty, _) => match pos.func.dfg[arg_ty] {
                        InstructionData::Binary { opcode, args } => {
                            let arg_1 = args;
                            match opcode {
                                Opcode::Iadd => {
                                    if arg_1[0] == arg_1[1]
                                        && arg_1[0] == arg_0[1]
                                        && arg_1[1] == arg_0[1]
                                    {
                                        pos.func.dfg.replace(inst).imul_imm(arg_0[1], 3);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
    }
}

fn superopt_2(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        InstructionData::IntCompare { opcode, cond, args } => {
            let arg_0 = args;
            match opcode {
                Opcode::Icmp => match cond {
                    IntCC::Equal => match pos.func.dfg.value_def(arg_0[0]) {
                        ValueDef::Result(arg_ty, _) => match pos.func.dfg[arg_ty] {
                            InstructionData::UnaryImm { opcode, imm } => {
                                let rhs_1: i64 = imm.into();
                                match opcode {
                                    Opcode::Iconst => {
                                        if rhs_1 == 0 {
                                            match pos.func.dfg.value_def(arg_0[1]) {
                                                ValueDef::Result(arg_ty, _) => {
                                                    match pos.func.dfg[arg_ty] {
                                                        InstructionData::UnaryImm {
                                                            opcode,
                                                            imm,
                                                        } => {
                                                            let rhs_3: i64 = imm.into();
                                                            match opcode {
                                                                Opcode::Iconst => {
                                                                    if rhs_3 == 0 {
                                                                        pos.func
                                                                            .dfg
                                                                            .replace(inst)
                                                                            .bconst(
                                                                                types::B1,
                                                                                true,
                                                                            );
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
    }
}

fn superopt_3(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        InstructionData::IntCompare { opcode, cond, args } => {
            let arg_0 = args;
            match opcode {
                Opcode::Icmp => match cond {
                    IntCC::UnsignedLessThanOrEqual => {
                        if arg_0[1] == arg_0[0] {
                            pos.func.dfg.replace(inst).bconst(types::B1, true);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
    }
}

fn superopt_4(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        InstructionData::IntCompareImm {
            opcode,
            cond,
            arg,
            imm,
        } => {
            let args_1 = arg;
            let rhs_1: i64 = imm.into();
            match opcode {
                Opcode::IcmpImm => match cond {
                    IntCC::Equal => match pos.func.dfg.value_def(args_1) {
                        ValueDef::Result(arg_ty, _) => match pos.func.dfg[arg_ty] {
                            InstructionData::BinaryImm64 { opcode, arg, imm } => {
                                let args_2 = arg;
                                let rhs_2: i64 = imm.into();
                                match opcode {
                                    Opcode::BxorImm => {
                                        if rhs_2 == 1 {
                                            if rhs_1 == 0 {
                                                pos.func.dfg.replace(inst).icmp_imm(
                                                    IntCC::Equal,
                                                    args_2,
                                                    1,
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
    }
}

fn superopt_5(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        InstructionData::Binary { opcode, args } => {
            let args_1 = args;
            match opcode {
                Opcode::Ishl => match pos.func.dfg.value_def(args_1[0]) {
                    ValueDef::Result(arg_ty, _) => match pos.func.dfg[arg_ty] {
                        InstructionData::UnaryImm { opcode, imm } => {
                            let rhs_1: i64 = imm.into();
                            match opcode {
                                Opcode::Iconst => {
                                    if rhs_1 == 1 {
                                        match pos.func.dfg.value_def(args_1[1]) {
                                            ValueDef::Result(arg_ty, _) => {
                                                match pos.func.dfg[arg_ty] {
                                                    InstructionData::BinaryImm64 {
                                                        opcode,
                                                        arg,
                                                        imm,
                                                    } => {
                                                        let args_2 = arg;
                                                        let rhs_3: i64 = imm.into();
                                                        match opcode {
                                                            Opcode::BandImm => {
                                                                if rhs_3 == 31 {
                                                                    let inst0 = pos
                                                                        .ins()
                                                                        .iconst(types::I32, 1);
                                                                    pos.func
                                                                        .dfg
                                                                        .replace(inst)
                                                                        .ishl(inst0, args_2);
                                                                }
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
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
        }
    }
}
