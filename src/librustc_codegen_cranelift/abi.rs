// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc_target::abi::call::FnType;
use rustc::ty::{FnSig, Ty, Instance};
use rustc_codegen_ssa::interfaces::*;
use super::context::{CrContext, CrValue};
use super::builder::CrBuilder;

#[allow(unreachable_code, unused_variables)]
impl<'ll, 'tcx: 'll> AbiMethods<'tcx> for CrContext<'ll, 'tcx> {
    fn new_fn_type(&self, sig: FnSig<'tcx>, _extra_args: &[Ty<'tcx>]) -> FnType<'tcx, Ty<'tcx>> {
        FnType {
            // /// The LLVM types of each argument.
            // pub args: Vec<ArgType<'a, Ty>>,
            //
            // /// LLVM return type.
            // pub ret: ArgType<'a, Ty>,
            //
            // pub variadic: bool,
            //
            // pub conv: Conv,
            args: unimplemented!(),
            ret: unimplemented!(),
            variadic: sig.variadic,
            conv: unimplemented!()
        }
    }
    fn new_vtable(
        &self,
        _sig: FnSig<'tcx>,
        _extra_args: &[Ty<'tcx>]
    ) -> FnType<'tcx, Ty<'tcx>> {
        unimplemented!()
    }
    fn fn_type_of_instance(&self, _instance: &Instance<'tcx>) -> FnType<'tcx, Ty<'tcx>> {
        unimplemented!()
    }
}

impl<'a, 'll: 'a, 'tcx: 'll> AbiBuilderMethods<'a, 'll, 'tcx> for CrBuilder<'a, 'll, 'tcx> {
    fn apply_attrs_callsite(
        &mut self,
        _ty: &FnType<'tcx, Ty<'tcx>>,
        _callsite: CrValue
    ) {
        unimplemented!()
    }
}
