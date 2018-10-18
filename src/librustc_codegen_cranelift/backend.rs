// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::session::Session;
use rustc::ty::TyCtxt;
use rustc::middle::allocator::AllocatorKind;
use rustc::middle::cstore::EncodedMetadata;
use rustc::mir::mono::Stats;
use rustc_codegen_ssa::interfaces::ExtraBackendMethods;
use CraneliftCodegenBackend;
use write::{Module, TargetMachine};

use std::sync::Arc;
use syntax_pos::symbol::InternedString;

impl ExtraBackendMethods for CraneliftCodegenBackend {
    fn thin_lto_available(&self) -> bool {
        //FIXME: replace this dummy implementation
        false
    }
    fn pgo_available(&self) -> bool {
        //FIXME: replace this dummy implementation
        false
    }
    fn new_metadata(&self, _sess: &Session, _mod_name: &str) -> Self::Module {
        //FIXME: replace this dummy implementation
        Module()
    }
    fn write_metadata<'b, 'gcx>(
        &self,
        tcx: TyCtxt<'b, 'gcx, 'gcx>,
        _metadata: &Self::Module
    ) -> EncodedMetadata {
        //FIXME: replace this dummy implementation
        tcx.encode_metadata()
    }
    fn codegen_allocator(&self, _tcx: TyCtxt, _mods: &Self::Module, _kind: AllocatorKind) {
        unimplemented!()
    }
    fn compile_codegen_unit<'ll, 'tcx: 'll>(
        &self,
        _tcx: TyCtxt<'ll, 'tcx, 'tcx>,
        _cgu_name: InternedString
    ) -> Stats {
        unimplemented!()
    }
    fn target_machine_factory(
        &self,
        _sess: &Session,
        _find_features: bool
    ) -> Arc<dyn Fn() ->
        Result<TargetMachine, String> + Send + Sync>
    {
        //FIXME: replace this dummy implementation
        Arc::new(|| { Ok(TargetMachine()) })
    }
    fn target_cpu<'b>(&self, _sess: &'b Session) -> &'b str {
        unimplemented!()
    }
}
