// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc_codegen_ssa::interfaces::*;
use rustc_codegen_ssa::mir::operand::OperandRef;
use syntax_pos::Span;
use super::context::{CrContext, CrValue};
use super::builder::CrBuilder;
use rustc::ty::Ty;
use rustc_target::abi::call::FnType;

impl<'ll, 'tcx: 'll> IntrinsicDeclarationMethods<'ll> for CrContext<'ll, 'tcx> {
    fn get_intrinsic(&self, _key: &str) -> CrValue {
        unimplemented!()
    }

    fn declare_intrinsic(
        &self,
        _key: &str
    ) -> Option<CrValue> {
        unimplemented!()
    }
}

impl<'a, 'll: 'a, 'tcx: 'll> IntrinsicCallMethods<'a, 'll, 'tcx> for CrBuilder<'a, 'll, 'tcx> {
    /// Remember to add all intrinsics here, in librustc_typeck/check/mod.rs,
    /// and in libcore/intrinsics.rs; if you need access to any llvm intrinsics,
    /// add them to librustc_codegen_llvm/context.rs
    fn codegen_intrinsic_call(
        &mut self,
        _callee_ty: Ty<'tcx>,
        _fn_ty: &FnType<'tcx, Ty<'tcx>>,
        _args: &[OperandRef<'tcx, CrValue>],
        _llresult: CrValue,
        _span: Span,
    ) {
        unimplemented!()
    }
}
