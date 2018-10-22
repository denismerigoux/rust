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
use rustc_target::abi::LayoutOf;
use rustc_target::spec::abi::Abi;
pub use rustc_target::abi::call::*;
use rustc_codegen_ssa::interfaces::*;
use super::context::{CrContext, CrValue};
use super::builder::CrBuilder;
use cranelift::prelude::{CallConv, AbiParam};

impl<'ll, 'tcx: 'll> CrContext<'ll, 'tcx> {
    pub(crate) fn rustc_conv_to_cr(&self, abi: Abi) -> CallConv {
        //FIXME: improve this dummy impl
        match abi {
            Abi::Rust => CallConv::Fast,
            _ => panic!()
        }
    }

    pub(crate) fn rustc_ty_to_cr_abi_param(&self, ty: Ty<'tcx>) -> AbiParam {
        //FIXME: improve this dummy impl
        let ty_cr = self.rustc_ty_to_cr_ty(ty);
        AbiParam::new(ty_cr)
    }
}

#[allow(unreachable_code, unused_variables)]
impl<'ll, 'tcx: 'll> AbiMethods<'tcx> for CrContext<'ll, 'tcx> {
    fn new_fn_type(&self, sig: FnSig<'tcx>, _extra_args: &[Ty<'tcx>]) -> FnType<'tcx, Ty<'tcx>> {
        //FIXME: improve this dummy impl
        use rustc_target::spec::abi::Abi::*;
        let args = sig.inputs().iter().map(
            |x| ArgType::new(self.layout_of(*x))
        ).collect::<Vec<_>>();
        let conv = match self.sess().target.target.adjust_abi(sig.abi) {
            RustIntrinsic | PlatformIntrinsic |
            Rust | RustCall => Conv::C,

            // It's the ABI's job to select this, not ours.
            System => bug!("system abi should be selected elsewhere"),

            Stdcall => Conv::X86Stdcall,
            Fastcall => Conv::X86Fastcall,
            Vectorcall => Conv::X86VectorCall,
            Thiscall => Conv::X86ThisCall,
            C => Conv::C,
            Unadjusted => Conv::C,
            Win64 => Conv::X86_64Win64,
            SysV64 => Conv::X86_64SysV,
            Aapcs => Conv::ArmAapcs,
            PtxKernel => Conv::PtxKernel,
            Msp430Interrupt => Conv::Msp430Intr,
            X86Interrupt => Conv::X86Intr,
            AmdGpuKernel => Conv::AmdGpuKernel,

            // These API constants ought to be more specific...
            Cdecl => Conv::C,
        };
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
            args,
            ret: ArgType::new(self.layout_of(sig.output())),
            variadic: sig.variadic,
            conv
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

impl<'a, 'll: 'a, 'tcx: 'll> AbiBuilderMethods<'a, 'll, 'tcx>
    for CrBuilder<'a, 'll, 'tcx>
{
    fn apply_attrs_callsite(
        &mut self,
        _ty: &FnType<'tcx, Ty<'tcx>>,
        _callsite: CrValue
    ) {
        unimplemented!()
    }
}
