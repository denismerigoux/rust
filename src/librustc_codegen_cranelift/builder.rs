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
use rustc_codegen_ssa::common::{IntPredicate, RealPredicate, AtomicOrdering,
    SynchronizationScope, AtomicRmwBinOp, OperandBundleDef};
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::mir::operand::OperandRef;
use rustc_codegen_ssa::MemFlags;
use super::context::{CrContext, CrValue, CrType, CrEbb};
use rustc::ty::TyCtxt;
use rustc::ty::layout::{Align, Size};

use libc::c_char;
use syntax::ast::AsmDialect;
use std::borrow::Cow;
use std::ops::Range;


#[allow(dead_code)]
pub struct CrBuilder<'a, 'll: 'a, 'tcx: 'll> {
    cx: &'a CrContext<'ll, 'tcx>,
}

impl<'a, 'll: 'a, 'tcx: 'll> HasCodegen<'a, 'll, 'tcx> for CrBuilder<'a, 'll, 'tcx> {
    type CodegenCx = CrContext<'ll, 'tcx>;
}


impl<'a, 'll :'a, 'tcx: 'll> BuilderMethods<'a, 'll, 'tcx> for CrBuilder<'a, 'll, 'tcx> {
    fn new_block<'b>(
        _cx: &'a CrContext<'ll, 'tcx>,
        _crfn: CrValue,
        _name: &'b str
    ) -> Self {
        unimplemented!()
    }
    fn with_cx(_cx: &'a CrContext<'ll, 'tcx>) -> Self {
        unimplemented!()
    }
    fn build_sibling_block<'c>(&self, _name: &'c str) -> Self {
        unimplemented!()
    }
    fn cx(&self) -> &'a CrContext<'ll, 'tcx> {
        &self.cx
    }
    fn tcx(&self) -> TyCtxt<'a, 'tcx, 'tcx> {
        unimplemented!()
    }
    fn llfn(&self) -> CrValue {
        unimplemented!()
    }
    fn llbb(&self) -> CrEbb {
        unimplemented!()
    }
    fn count_insn(&self, _category: &str) {
        unimplemented!()
    }

    fn set_value_name(&mut self, _value: CrValue, _name: &str) {
        unimplemented!()
    }
    fn position_at_end(&mut self, _llbb: CrEbb) {
        unimplemented!()
    }
    fn position_at_start(&mut self, _llbb: CrEbb) {
        unimplemented!()
    }
    fn ret_void(&mut self) {
        unimplemented!()
    }
    fn ret(&mut self, _v: CrValue) {
        unimplemented!()
    }
    fn br(&mut self, _dest: CrEbb) {
        unimplemented!()
    }
    fn cond_br(
        &mut self,
        _cond: CrValue,
        _then_llbb: CrEbb,
        _else_llbb: CrEbb,
    ) {
        unimplemented!()
    }
    fn switch(
        &mut self,
        _v: CrValue,
        _else_llbb: CrEbb,
        _num_cases: usize,
    ) -> CrValue {
        unimplemented!()
    }
    fn invoke(
        &mut self,
        _llfn: CrValue,
        _args: &[CrValue],
        _then: CrEbb,
        _catch: CrEbb,
        _bundle: Option<&OperandBundleDef<'ll, CrValue>>
    ) -> CrValue {
        unimplemented!()
    }
    fn unreachable(&mut self) {
        unimplemented!()
    }
    fn add(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fadd(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fadd_fast(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn sub(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fsub(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fsub_fast(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn mul(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fmul(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fmul_fast(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn udiv(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn exactudiv(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn sdiv(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn exactsdiv(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fdiv(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fdiv_fast(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn urem(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn srem(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn frem(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn frem_fast(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn shl(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn lshr(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn ashr(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn and(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn or(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn xor(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn neg(
        &mut self,
        _v: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fneg(
        &mut self,
        _v: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn not(
        &mut self,
        _v: CrValue
    ) -> CrValue {
        unimplemented!()
    }

    fn alloca(
        &mut self,
        _ty: CrType,
        _name: &str, _align: Align
    ) -> CrValue {
        unimplemented!()
    }
    fn dynamic_alloca(
        &mut self,
        _ty: CrType,
        _name: &str, _align: Align
    ) -> CrValue {
        unimplemented!()
    }
    fn array_alloca(
        &mut self,
        _ty: CrType,
        _len: CrValue,
        _name: &str,
        _align: Align
    ) -> CrValue {
        unimplemented!()
    }

    fn load(
        &mut self,
        _ptr: CrValue,
        _align: Align
    ) -> CrValue {
        unimplemented!()
    }
    fn volatile_load(
        &mut self,
        _ptr: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn atomic_load(
        &mut self,
        _ptr: CrValue,
        _order: AtomicOrdering, _align: Align
    ) -> CrValue {
        unimplemented!()
    }
    fn load_ref(
        &mut self,
        _ptr: &PlaceRef<'tcx,CrValue>
    ) -> OperandRef<'tcx, CrValue> {
        unimplemented!()
    }

    fn range_metadata(
        &mut self,
        _load: CrValue,
        _range: Range<u128>
    ) {
        unimplemented!()
    }
    fn nonnull_metadata(&mut self, _load: CrValue) {
        unimplemented!()
    }

    fn store(
        &mut self,
        _val: CrValue,
        _ptr: CrValue,
        _align: Align
    ) -> CrValue {
        unimplemented!()
    }
    fn atomic_store(
        &mut self,
        _val: CrValue,
        _ptr: CrValue,
        _order: AtomicOrdering,
        _align: Align
    ) {
        unimplemented!()
    }
    fn store_with_flags(
        &mut self,
        _val: CrValue,
        _ptr: CrValue,
        _align: Align,
        _flags: MemFlags,
    ) -> CrValue {
        unimplemented!()
    }

    fn gep(
        &mut self,
        _ptr: CrValue,
        _indices: &[CrValue]
    ) -> CrValue {
        unimplemented!()
    }
    fn inbounds_gep(
        &mut self,
        _ptr: CrValue,
        _indices: &[CrValue]
    ) -> CrValue {
        unimplemented!()
    }
    fn struct_gep(
        &mut self,
        _ptr: CrValue,
        _idx: u64
    ) -> CrValue {
        unimplemented!()
    }

    fn trunc(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn sext(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn fptoui(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn fptosi(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn uitofp(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn sitofp(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn fptrunc(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn fpext(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn ptrtoint(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn inttoptr(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn bitcast(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn intcast(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType, _is_signed: bool
    ) -> CrValue {
        unimplemented!()
    }
    fn pointercast(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }

    fn icmp(
        &mut self,
        _op: IntPredicate,
        _lhs: CrValue, _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn fcmp(
        &mut self,
        _op: RealPredicate,
        _lhs: CrValue, _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }

    fn empty_phi(
        &mut self,
        _ty: CrType) -> CrValue {
        unimplemented!()
    }
    fn phi(
        &mut self,
        _ty: CrType,
        _vals: &[CrValue],
        _bbs: &[CrEbb]
    ) -> CrValue {
        unimplemented!()
    }
    fn inline_asm_call(
        &mut self,
        _asm: *const c_char,
        _cons: *const c_char,
        _inputs: &[CrValue],
        _output: CrType,
        _volatile: bool,
        _alignstack: bool,
        _dia: AsmDialect
    ) -> Option<CrValue> {
        unimplemented!()
    }

    fn minnum(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn maxnum(
        &mut self,
        _lhs: CrValue,
        _rhs: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn select(
        &mut self, _cond: CrValue,
        _then_val: CrValue,
        _else_val: CrValue,
    ) -> CrValue {
        unimplemented!()
    }

    fn va_arg(
        &mut self,
        _list: CrValue,
        _ty: CrType
    ) -> CrValue {
        unimplemented!()
    }
    fn extract_element(&mut self,
        _vec: CrValue,
        _idx: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn insert_element(
        &mut self, _vec: CrValue,
        _elt: CrValue,
        _idx: CrValue,
    ) -> CrValue {
        unimplemented!()
    }
    fn shuffle_vector(
        &mut self,
        _v1: CrValue,
        _v2: CrValue,
        _mask: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_splat(
        &mut self,
        _num_elts: usize,
        _elt: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fadd_fast(
        &mut self,
        _acc: CrValue,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fmul_fast(
        &mut self,
        _acc: CrValue,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_add(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_mul(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_and(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_or(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_xor(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fmin(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fmax(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fmin_fast(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_fmax_fast(
        &mut self,
        _src: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_min(
        &mut self,
        _src: CrValue,
        _is_signed: bool
    ) -> CrValue {
        unimplemented!()
    }
    fn vector_reduce_max(
        &mut self,
        _src: CrValue,
        _is_signed: bool
    ) -> CrValue {
        unimplemented!()
    }
    fn extract_value(
        &mut self,
        _agg_val: CrValue,
        _idx: u64
    ) -> CrValue {
        unimplemented!()
    }
    fn insert_value(
        &mut self,
        _agg_val: CrValue,
        _elt: CrValue,
        _idx: u64
    ) -> CrValue {
        unimplemented!()
    }

    fn landing_pad(
        &mut self,
        _ty: CrType,
        _pers_fn: CrValue,
        _num_clauses: usize
    ) -> CrValue {
        unimplemented!()
    }
    fn add_clause(
        &mut self,
        _landing_pad: CrValue,
        _clause: CrValue
    ) {
        unimplemented!()
    }
    fn set_cleanup(
        &mut self,
        _landing_pad: CrValue
    ) {
        unimplemented!()
    }
    fn resume(
        &mut self,
        _exn: CrValue
    ) -> CrValue {
        unimplemented!()
    }
    fn cleanup_pad(
        &mut self,
        _parent: Option<CrValue>,
        _args: &[CrValue]
    ) -> CrValue {
        unimplemented!()
    }
    fn cleanup_ret(
        &mut self, _cleanup: CrValue,
        _unwind: Option<CrEbb>,
    ) -> CrValue {
        unimplemented!()
    }
    fn catch_pad(
        &mut self,
        _parent: CrValue,
        _args: &[CrValue]
    ) -> CrValue {
        unimplemented!()
    }
    fn catch_ret(
        &mut self,
        _pad: CrValue,
        _unwind: CrEbb
    ) -> CrValue {
        unimplemented!()
    }
    fn catch_switch(
        &mut self,
        _parent: Option<CrValue>,
        _unwind: Option<CrEbb>,
        _num_handlers: usize,
    ) -> CrValue {
        unimplemented!()
    }
    fn add_handler(
        &mut self,
        _catch_switch: CrValue,
        _handler: CrEbb
    ) {
        unimplemented!()
    }
    fn set_personality_fn(&mut self, _personality: CrValue) {
        unimplemented!()
    }

    fn atomic_cmpxchg(
        &mut self,
        _dst: CrValue,
        _cmp: CrValue,
        _src: CrValue,
        _order: AtomicOrdering,
        _failure_order: AtomicOrdering,
        _weak: bool,
    ) -> CrValue {
        unimplemented!()
    }
    fn atomic_rmw(
        &mut self,
        _op: AtomicRmwBinOp,
        _dst: CrValue,
        _src: CrValue,
        _order: AtomicOrdering,
    ) -> CrValue {
        unimplemented!()
    }
    fn atomic_fence(&mut self, _order: AtomicOrdering, _scope: SynchronizationScope) {
        unimplemented!()
    }
    fn add_case(
        &mut self,
        _s: CrValue,
        _on_val: CrValue,
        _dest: CrEbb
    ) {
        unimplemented!()
    }
    fn add_incoming_to_phi(
        &mut self,
        _phi: CrValue,
        _val: CrValue,
        _bb: CrEbb
    ) {
        unimplemented!()
    }
    fn set_invariant_load(&mut self, _load: CrValue) {
        unimplemented!()
    }

    /// Returns the ptr value that should be used for storing `val`.
    fn check_store(
        &mut self,
        _val: CrValue,
        _ptr: CrValue
    ) -> CrValue {
        unimplemented!()
    }

    /// Returns the args that should be used for a call to `llfn`.
    fn check_call<'c>(
        &mut self,
        _typ: &str,
        _llfn: CrValue,
        _args: &'c [CrValue]
    ) -> Cow<'c, [CrValue]>
        where [CrValue] : ToOwned {
        unimplemented!()
    }

    fn lifetime_start(&mut self, _ptr: CrValue, _size: Size) {
        unimplemented!()
    }
    fn lifetime_end(&mut self, _ptr: CrValue, _size: Size) {
        unimplemented!()
    }

    /// If LLVM lifetime intrinsic support is enabled (i.e. optimizations
    /// on), and `ptr` is nonzero-sized, then extracts the size of `ptr`
    /// and the intrinsic for `lt` and passes them to `emit`, which is in
    /// charge of generating code to call the passed intrinsic on whatever
    /// block of generated code is targeted for the intrinsic.
    ///
    /// If LLVM lifetime intrinsic support is disabled (i.e.  optimizations
    /// off) or `ptr` is zero-sized, then no-op (does not call `emit`).
    fn call_lifetime_intrinsic(
        &mut self,
        _intrinsic: &str,
        _ptr: CrValue, _size: Size
    ) {
        unimplemented!()
    }

    fn call(
        &mut self,
        _llfn: CrValue,
        _args: &[CrValue],
        _bundle: Option<&OperandBundleDef<'ll, CrValue>>
    ) -> CrValue {
        unimplemented!()
    }

    fn call_memcpy(
        &mut self,
        _dst: CrValue,
        _src: CrValue,
        _n_bytes: CrValue,
        _align: Align,
        _flags: MemFlags,
    ) {
        unimplemented!()
    }

    fn call_memset(
        &mut self,
        _ptr: CrValue,
        _fill_byte: CrValue,
        _size: CrValue,
        _align: CrValue,
        _volatile: bool,
    ) -> CrValue {
        unimplemented!()
    }

    fn zext(
        &mut self,
        _val: CrValue,
        _dest_ty: CrType
    ) -> CrValue {
        unimplemented!()
    }

    fn delete_basic_block(&mut self, _bb: CrEbb) {
        unimplemented!()
    }
    fn do_not_inline(&mut self, _llret: CrValue) {
        unimplemented!()
    }
}
