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
use rustc::ty::{self, Ty, TyCtxt};
use rustc::ty::layout::{TyLayout, LayoutOf, LayoutError, HasTyCtxt, HasDataLayout,
    TargetDataLayout};
use rustc_target::abi::call::{FnType, Reg, CastTarget};
use super::context::{CraneliftContext, CrType, CrValue};
use rustc_data_structures::fx::FxHashMap;

use std::cell::RefCell;

impl<'ll, 'tcx: 'll> BaseTypeMethods<'ll, 'tcx> for CraneliftContext<'tcx> {
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

impl<'ll, 'tcx: 'll> LayoutTypeMethods<'ll, 'tcx> for CraneliftContext<'tcx> {
    fn backend_type(&self, _ty: &TyLayout<'tcx>) -> CrType {
        unimplemented!()
    }
    fn cast_backend_type(&self, _ty: &CastTarget) -> CrType {
        unimplemented!()
    }
    fn fn_backend_type(&self, _ty: &FnType<'tcx, Ty<'tcx>>) -> CrType {
        unimplemented!()
    }
    fn reg_backend_type(&self, _ty: &Reg) -> CrType {
        unimplemented!()
    }
    fn immediate_backend_type(&self, _ty: &TyLayout<'tcx>) -> CrType {
        unimplemented!()
    }
    fn is_backend_immediate(&self, _ty: &TyLayout<'tcx>) -> bool {
        unimplemented!()
    }
    fn is_backend_scalar_pair(&self, _ty: &TyLayout<'tcx>) -> bool {
        unimplemented!()
    }
    fn backend_field_index(&self, _ty: &TyLayout<'tcx>, _index: usize) -> u64 {
        unimplemented!()
    }
    fn scalar_pair_element_backend_type<'a>(
        &self,
        _ty: &TyLayout<'tcx>,
        _index: usize,
        _immediate: bool
    ) -> CrType{
        unimplemented!()
    }
}

impl<'a, 'tcx: 'a> HasDataLayout for &'a CraneliftContext<'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        &self.tcx().data_layout
    }
}

impl<'a, 'tcx: 'a> HasTyCtxt<'tcx> for &'a CraneliftContext<'tcx> {
    fn tcx<'b>(&'b self) -> TyCtxt<'b, 'tcx, 'tcx> {
        unimplemented!()
    }
}

impl<'a, 'tcx: 'a> LayoutOf for &'a CraneliftContext<'tcx> {
    type Ty = Ty<'tcx>;
    type TyLayout = TyLayout<'tcx>;

    fn layout_of(self, ty: Ty<'tcx>) -> Self::TyLayout {
        self.tcx().layout_of(ty::ParamEnv::reveal_all().and(ty))
            .unwrap_or_else(|e| match e {
                LayoutError::SizeOverflow(_) => self.sess().fatal(&e.to_string()),
                _ => bug!("failed to get layout for `{}`: {}", ty, e)
            })
    }
}

impl<'a, 'll: 'a, 'tcx: 'll> DerivedTypeMethods<'a, 'll, 'tcx> for CraneliftContext<'tcx> {}
