// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::mir;

use super::FunctionCx;
use super::LocalRef;
use super::OperandValue;
use rustc::ty::Ty;
use rustc::ty::layout::{TyLayout, HasTyCtxt, LayoutOf};
use interfaces::*;
use value::Value;

impl<'a, 'f, 'll: 'a + 'f, 'tcx: 'll, Cx: CodegenMethods<'ll, 'tcx>>
    FunctionCx<'a, 'f, 'll, 'tcx, Cx> where
    &'a Cx: LayoutOf<Ty = Ty<'tcx>, TyLayout = TyLayout<'tcx>> + HasTyCtxt<'tcx>
{
    pub fn codegen_statement<Bx: BuilderMethods<'a, 'll, 'tcx, CodegenCx=Cx>>(
        &mut self,
        bx: Bx,
        statement: &mir::Statement<'tcx>
    ) -> Bx {
        debug!("codegen_statement(statement={:?})", statement);

        self.set_debug_loc(&bx, statement.source_info);
        match statement.kind {
            mir::StatementKind::Assign(ref place, ref rvalue) => {
                if let mir::Place::Local(index) = *place {
                    match self.locals[index] {
                        LocalRef::Place(cg_dest) => {
                            self.codegen_rvalue(bx, cg_dest, rvalue)
                        }
                        LocalRef::UnsizedPlace(cg_indirect_dest) => {
                            self.codegen_rvalue_unsized(bx, cg_indirect_dest, rvalue)
                        }
                        LocalRef::Operand(None) => {
                            let (bx, operand) = self.codegen_rvalue_operand(bx, rvalue);
                            self.locals[index] = LocalRef::Operand(Some(operand));
                            bx
                        }
                        LocalRef::Operand(Some(op)) => {
                            if !op.layout.is_zst() {
                                span_bug!(statement.source_info.span,
                                          "operand {:?} already assigned",
                                          rvalue);
                            }

                            // If the type is zero-sized, it's already been set here,
                            // but we still need to make sure we codegen the operand
                            self.codegen_rvalue_operand(bx, rvalue).0
                        }
                    }
                } else {
                    let cg_dest = self.codegen_place(&bx, place);
                    self.codegen_rvalue(bx, cg_dest, rvalue)
                }
            }
            mir::StatementKind::SetDiscriminant{ref place, variant_index} => {
                self.codegen_place(&bx, place)
                    .codegen_set_discr(&bx, variant_index);
                bx
            }
            mir::StatementKind::StorageLive(local) => {
                if let LocalRef::Place(cg_place) = self.locals[local] {
                    cg_place.storage_live(&bx);
                } else if let LocalRef::UnsizedPlace(cg_indirect_place) = self.locals[local] {
                    cg_indirect_place.storage_live(&bx);
                }
                bx
            }
            mir::StatementKind::StorageDead(local) => {
                if let LocalRef::Place(cg_place) = self.locals[local] {
                    cg_place.storage_dead(&bx);
                } else if let LocalRef::UnsizedPlace(cg_indirect_place) = self.locals[local] {
                    cg_indirect_place.storage_dead(&bx);
                }
                bx
            }
            mir::StatementKind::InlineAsm { ref asm, ref outputs, ref inputs } => {
                let outputs = outputs.iter().map(|output| {
                    self.codegen_place(&bx, output)
                }).collect();

                let input_vals = inputs.iter()
                    .try_fold(Vec::with_capacity(inputs.len()), |mut acc, input| {
                        let op = self.codegen_operand(&bx, input);
                        if let OperandValue::Immediate(_) = op.val {
                            acc.push(op.immediate());
                            Ok(acc)
                        } else {
                            Err(op)
                        }
                });

                if input_vals.is_err() {
                   span_err!(bx.cx().sess(), statement.source_info.span, E0669,
                             "invalid value for constraint in inline assembly");
                } else {
                    let input_vals = input_vals.unwrap();
                    let res = xb.codegen_inline_asm(asm, outputs, input_vals);
                    if !res {
                        span_err!(bx.cx().sess(), statement.source_info.span, E0668,
                                  "malformed inline assembly");
                    }
                }
                bx
            }
            mir::StatementKind::FakeRead(..) |
            mir::StatementKind::EndRegion(_) |
            mir::StatementKind::Validate(..) |
            mir::StatementKind::AscribeUserType(..) |
            mir::StatementKind::Nop => bx,
        }
    }
}
