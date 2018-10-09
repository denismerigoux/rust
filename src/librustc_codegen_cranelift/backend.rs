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
use rustc::util::time_graph::TimeGraph;
use rustc::mir::mono::Stats;
use rustc_codegen_ssa::interfaces::ExtraBackendMethods;
use rustc_codegen_ssa::{ModuleCodegen, CachedModuleCodegen};
use CraneliftCodegenBackend;

use std::any::Any;
use std::sync::mpsc::Receiver;
use syntax_pos::symbol::InternedString;

impl ExtraBackendMethods for CraneliftCodegenBackend {
    type Metadata = ();
    type OngoingCodegen = ();

    fn thin_lto_available(&self) -> bool {
        unimplemented!()
    }
    fn pgo_available(&self) -> bool {
        unimplemented!()
    }
    fn new_metadata(&self, _sess: &Session, _mod_name: &str) -> Self::Metadata {
        unimplemented!()
    }
    fn write_metadata<'b, 'gcx>(
        &self,
        _tcx: TyCtxt<'b, 'gcx, 'gcx>,
        _metadata: &Self::Metadata
    ) -> EncodedMetadata {
        unimplemented!()
    }
    fn codegen_allocator(&self, _tcx: TyCtxt, _mods: &Self::Metadata, _kind: AllocatorKind) {
        unimplemented!()
    }

    fn start_async_codegen(
        &self,
        _tcx: TyCtxt,
        _time_graph: Option<TimeGraph>,
        _metadata: EncodedMetadata,
        _coordinator_receive: Receiver<Box<dyn Any + Send>>,
        _total_cgus: usize
    ) -> Self::OngoingCodegen {
        unimplemented!()
    }
    fn submit_pre_codegened_module_to_llvm(
        &self,
        _codegen: &Self::OngoingCodegen,
        _tcx: TyCtxt,
        _module: ModuleCodegen<Self::Metadata>
    ) {
        unimplemented!()
    }
    fn submit_pre_lto_module_to_llvm(&self, _tcx: TyCtxt, _module: CachedModuleCodegen) {
        unimplemented!()
    }
    fn submit_post_lto_module_to_llvm(&self, _tcx: TyCtxt, _module: CachedModuleCodegen) {
        unimplemented!()
    }
    fn codegen_finished(&self, _codegen: &Self::OngoingCodegen, _tcx: TyCtxt) {
        unimplemented!()
    }
    fn check_for_errors(&self, _codegen: &Self::OngoingCodegen, _sess: &Session) {
        unimplemented!()
    }
    fn wait_for_signal_to_codegen_item(&self, _codegen: &Self::OngoingCodegen) {
        unimplemented!()
    }
    fn compile_codegen_unit<'ll, 'tcx: 'll>(
        &self,
        _tcx: TyCtxt<'ll, 'tcx, 'tcx>,
        _cgu_name: InternedString
    ) -> Stats {
        unimplemented!()
    }
}
