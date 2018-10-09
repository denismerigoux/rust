// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::context::{CraneliftContext, CrValue};
use rustc_codegen_ssa::interfaces::*;
use rustc::mir;
use syntax_pos;
use rustc::hir::def_id::CrateNum;
use rustc::ty::{Ty, Instance, FnSig};
use rustc_codegen_ssa::debuginfo::{MirDebugScope, FunctionDebugContext};
use rustc_data_structures::indexed_vec::IndexVec;


impl<'ll, 'tcx: 'll> DebugInfoMethods<'ll, 'tcx>  for CraneliftContext<'tcx> {
    type DIScope = ();

    fn create_vtable_metadata(
        &self,
        _ty: Ty<'tcx>,
        _vtable: Self::Value,
    ) {
        unimplemented!()
    }

    fn create_function_debug_context(
        &self,
        _instance: Instance<'tcx>,
        _sig: FnSig<'tcx>,
        _llfn: CrValue,
        _mir: &mir::Mir,
    ) -> FunctionDebugContext<Self::DIScope> {
        unimplemented!()
    }

    fn create_mir_scopes(
        &self,
        _mir: &mir::Mir,
        _debug_context: &FunctionDebugContext<Self::DIScope>,
    ) -> IndexVec<mir::SourceScope, MirDebugScope<Self::DIScope>> {
        unimplemented!()
    }
    fn extend_scope_to_file(
        &self,
        _scope_metadata: Self::DIScope,
        _file: &syntax_pos::SourceFile,
        _defining_crate: CrateNum,
    ) -> Self::DIScope {
        unimplemented!()
    }
    fn debuginfo_finalize(&self) {
        unimplemented!()
    }
    fn debuginfo_upvar_decls_ops_sequence(&self, _byte_offset_of_var_in_env: u64) -> [i64; 4] {
        unimplemented!()
    }
}
