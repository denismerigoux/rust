// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::ty::{Ty, Instance};
use rustc_codegen_ssa::interfaces::*;
use rustc::hir::def_id::DefId;
use rustc::mir::mono::{Linkage, Visibility};
use super::context::{CrContext, CrValue, CrType};

impl<'ll, 'tcx: 'll> DeclareMethods<'ll, 'tcx> for CrContext<'ll, 'tcx> {

    fn declare_global(
        &self,
        _name: &str, _ty: CrType
    ) -> CrValue {
        unimplemented!()
    }

    fn declare_cfn(
        &self,
        _name: &str,
        _fn_type: CrType
    ) -> CrValue {
        unimplemented!()
    }

    fn declare_fn(
        &self,
        _name: &str,
        _fn_type: Ty<'tcx>,
    ) -> CrValue {
        unimplemented!()
    }

    fn define_global(
        &self,
        _name: &str,
        _ty: CrType
    ) -> Option<CrValue> {
        unimplemented!()
    }

    fn define_private_global(&self, _ty: CrType) -> CrValue {
        unimplemented!()
    }

    fn define_fn(
        &self,
        _name: &str,
        _fn_type: Ty<'tcx>,
    ) -> CrValue {
        unimplemented!()
    }

    fn define_internal_fn(
        &self,
        _name: &str,
        _fn_type: Ty<'tcx>,
    ) -> CrValue {
        unimplemented!()
    }

    fn get_declared_value(&self, _name: &str) -> Option<CrValue> {
        unimplemented!()
    }

    fn get_defined_value(&self, _name: &str) -> Option<CrValue> {
        unimplemented!()
    }
}

impl<'ll, 'tcx: 'll> PreDefineMethods<'ll, 'tcx> for CrContext<'ll, 'tcx> {
    fn predefine_static(
        &self,
        _def_id: DefId,
        _linkage: Linkage,
        _visibility: Visibility,
        _symbol_name: &str
    ) {
        unimplemented!()
    }
    fn predefine_fn(
        &self,
        _instance: Instance<'tcx>,
        _linkage: Linkage,
        _visibility: Visibility,
        _symbol_name: &str
    ) {
        unimplemented!()
    }
}
