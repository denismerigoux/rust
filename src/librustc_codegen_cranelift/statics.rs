// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::ty::layout::Align;
use rustc::hir::def_id::DefId;
use super::context::{CrContext, CrValue, CrType};
use rustc_codegen_ssa::interfaces::*;

impl<'ll, 'tcx: 'll> StaticMethods<'ll> for CrContext<'tcx> {
    fn static_ptrcast(&self, _val: CrValue, _ty: CrType) -> CrValue {
        unimplemented!()
    }
    fn static_bitcast(&self, _val: CrValue, _ty: CrType) -> CrValue {
        unimplemented!()
    }
    fn static_addr_of_mut(
        &self,
        _cv: CrValue,
        _align: Align,
        _kind: Option<&str>,
    ) -> CrValue {
        unimplemented!()
    }
    fn static_addr_of(
        &self,
        _cv: CrValue,
        _align: Align,
        _kind: Option<&str>,
    ) -> CrValue {
        unimplemented!()
    }
    fn get_static(&self, _def_id: DefId) -> CrValue {
        unimplemented!()
    }
    fn codegen_static(
        &self,
        _def_id: DefId,
        _is_mutable: bool,
    ) {
        unimplemented!()
    }
    fn static_replace_all_uses(&self, _old_g: CrValue, _new_g: CrValue) {
        unimplemented!()
    }
}
