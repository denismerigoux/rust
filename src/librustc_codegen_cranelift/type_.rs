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
use rustc_codegen_ssa::common::TypeKind;
use rustc::ty::{Ty, TyCtxt};
use super::context::{CraneliftContext, CrType, CrValue};
use rustc_data_structures::fx::FxHashMap;

use std::cell::RefCell;

impl<'ll, 'tcx: 'll> BaseTypeMethods<'ll, 'tcx> for CraneliftContext {
    fn type_void(&self) -> CrType  {
        unimplemented!()
    }
    fn type_metadata(&self) -> CrType {
        unimplemented!()
    }
    fn type_i1(&self) -> CrType {
        unimplemented!()
    }
    fn type_i8(&self) -> CrType {
        unimplemented!()
    }
    fn type_i16(&self) -> CrType {
        unimplemented!()
    }
    fn type_i32(&self) -> CrType {
        unimplemented!()
    }
    fn type_i64(&self) -> CrType {
        unimplemented!()
    }
    fn type_i128(&self) -> CrType {
        unimplemented!()
    }

    // Creates an integer type with the given number of bits, e.g. i24
    fn type_ix(&self, _num_bits: u64) -> CrType {
        unimplemented!()
    }
    fn type_isize(&self) -> CrType {
        unimplemented!()
    }

    fn type_f32(&self) -> CrType {
        unimplemented!()
    }
    fn type_f64(&self) -> CrType {
        unimplemented!()
    }
    fn type_x86_mmx(&self) -> CrType {
        unimplemented!()
    }

    fn type_func(&self, _args: &[CrType], _ret: CrType) -> CrType {
        unimplemented!()
    }
    fn type_variadic_func(&self, _args: &[CrType], _ret: CrType) -> CrType {
        unimplemented!()
    }
    fn type_struct(&self, _els: &[CrType], _packed: bool) -> CrType {
        unimplemented!()
    }
    fn type_named_struct(&self, _name: &str) -> CrType {
        unimplemented!()
    }
    fn type_array(&self, _ty: CrType, _len: u64) -> CrType {
        unimplemented!()
    }
    fn type_vector(&self, _ty: CrType, _len: u64) -> CrType {
        unimplemented!()
    }
    fn type_kind(&self, _ty: CrType) -> TypeKind {
        unimplemented!()
    }
    fn set_struct_body(&self, _ty: CrType, _els: &[CrType], _packed: bool){
        unimplemented!()
    }
    fn type_ptr_to(&self, _ty: CrType) -> CrType {
        unimplemented!()
    }
    fn element_type(&self, _ty: CrType) -> CrType {
        unimplemented!()
    }

    /// Return the number of elements in `self` if it is a LLVM vector type.
    fn vector_length(&self, _ty: CrType) -> usize {
        unimplemented!()
    }

    fn func_params_types(&self, _ty: CrType) -> Vec<CrType> {
        unimplemented!()
    }
    fn float_width(&self, _ty: CrType) -> usize {
        unimplemented!()
    }

    /// Retrieve the bit width of the integer type `self`.
    fn int_width(&self, _ty: CrType) -> u64 {
        unimplemented!()
    }

    fn val_ty(&self, _v: CrValue) -> CrType {
        unimplemented!()
    }
    fn scalar_lltypes(&self) -> &RefCell<FxHashMap<Ty<'tcx>, CrType>> {
        unimplemented!()
    }
    fn tcx(&self) -> &TyCtxt<'ll, 'tcx, 'tcx> {
        unimplemented!()
    }
}
