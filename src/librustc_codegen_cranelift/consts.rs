// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::ty::layout;
use rustc_codegen_ssa::interfaces::*;
use rustc_codegen_ssa::mir::place::PlaceRef;
use super::context::{CrValue, CrType, CrContext};
use syntax_pos::symbol::LocalInternedString;
use rustc::mir::interpret::{Scalar, Allocation};

impl<'ll, 'tcx: 'll> ConstMethods<'ll, 'tcx> for CrContext<'tcx> {
    // Constant constructors

    fn const_null(&self, _t: CrType) -> CrValue {
        unimplemented!()
    }
    fn const_undef(&self, _t: CrType) -> CrValue {
        unimplemented!()
    }
    fn const_int(&self, _t: CrType, _i: i64) -> CrValue {
        unimplemented!()
    }
    fn const_uint(&self, _t: CrType, _i: u64) -> CrValue{
     unimplemented!()
    }
    fn const_uint_big(&self, _t: CrType, _u: u128) -> CrValue{
     unimplemented!()
    }
    fn const_bool(&self, _val: bool) -> CrValue{
     unimplemented!()
    }
    fn const_i32(&self, _i: i32) -> CrValue{
     unimplemented!()
    }
    fn const_u32(&self, _i: u32) -> CrValue{
     unimplemented!()
    }
    fn const_u64(&self, _i: u64) -> CrValue{
     unimplemented!()
    }
    fn const_usize(&self, _i: u64) -> CrValue{
     unimplemented!()
    }
    fn const_u8(&self, _i: u8) -> CrValue{
     unimplemented!()
    }

    // This is a 'c-like' raw string, which differs from
    // our boxed-and-length-annotated strings.
    fn const_cstr(
        &self,
        _s: LocalInternedString,
        _null_terminated: bool,
    ) -> CrValue{
     unimplemented!()
    }

    // NB: Do not use `do_spill_noroot` to make this into a constant string, or
    // you will be kicked off fast isel. See issue #4352 for an example of this.
    fn const_str_slice(&self, _s: LocalInternedString) -> CrValue{
     unimplemented!()
    }

    fn const_fat_ptr(
        &self,
        _ptr: CrValue,
        _meta: CrValue
    ) -> CrValue{
     unimplemented!()
    }
    fn const_struct(
        &self,
        _elts: &[CrValue],
        _packed: bool
    ) -> CrValue{
     unimplemented!()
    }
    fn const_array(&self, _ty: CrType, _elts: &[CrValue]) -> CrValue{
     unimplemented!()
    }
    fn const_vector(&self, _elts: &[CrValue]) -> CrValue{
     unimplemented!()
    }
    fn const_bytes(&self, _bytes: &[u8]) -> CrValue{
     unimplemented!()
    }

    fn const_get_elt(&self, _v: CrValue, _idx: u64) -> CrValue{
     unimplemented!()
    }
    fn const_get_real(&self, _v: CrValue) -> Option<(f64, bool)>{
     unimplemented!()
    }
    fn const_to_uint(&self, _v: CrValue) -> u64{
     unimplemented!()
    }
    fn const_to_opt_u128(&self, _v: CrValue, _sign_ext: bool) -> Option<u128>{
     unimplemented!()
    }

    fn is_const_integral(&self, _v: CrValue) -> bool{
     unimplemented!()
    }
    fn is_const_real(&self, _v: CrValue) -> bool{
     unimplemented!()
    }

    fn scalar_to_backend(
        &self,
        _cv: Scalar,
        _layout: &layout::Scalar,
        _llty: CrType,
    ) -> CrValue{
     unimplemented!()
    }
    fn from_const_alloc(
        &self,
        _layout: layout::TyLayout<'tcx>,
        _alloc: &Allocation,
        _offset: layout::Size,
    ) -> PlaceRef<'tcx, CrValue>{
     unimplemented!()
    }
}
