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
use rustc_mir::monomorphize::partitioning::CodegenUnitExt;
use rustc::session::config::DebugInfo;
use rustc_codegen_ssa::interfaces::*;
use rustc_codegen_ssa::{ModuleCodegen, ModuleKind};
use rustc_codegen_ssa::back::write::submit_codegened_module_to_llvm;
use rustc_codegen_ssa::base::maybe_create_entry_wrapper;
use rustc_codegen_ssa::mono_item::MonoItemExt;
use CraneliftCodegenBackend;
use write::{CrModule, TargetMachine};
use context::CrContext;
use builder::CrBuilder;

use std::sync::Arc;
use std::time::Instant;
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
        CrModule()
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
        tcx: TyCtxt<'ll, 'tcx, 'tcx>,
        cgu_name: InternedString
    ) -> Stats {
        let start_time = Instant::now();

        let dep_node = tcx.codegen_unit(cgu_name).codegen_dep_node(tcx);
        let ((stats, module), _) = tcx.dep_graph.with_task(dep_node,
                                                           tcx,
                                                           cgu_name,
                                                           module_codegen);
        let time_to_codegen = start_time.elapsed();

        // We assume that the cost to run LLVM on a CGU is proportional to
        // the time we needed for codegenning it.
        let cost = time_to_codegen.as_secs() * 1_000_000_000 +
                   time_to_codegen.subsec_nanos() as u64;

        submit_codegened_module_to_llvm(self, tcx, module, cost);
        return stats;

        fn module_codegen<'ll, 'tcx>(
            tcx: TyCtxt<'ll, 'tcx, 'tcx>,
            cgu_name: InternedString)
            -> (Stats, ModuleCodegen<CrModule>)
        {
            let backend = CraneliftCodegenBackend(());
            let cgu = tcx.codegen_unit(cgu_name);
            // Instantiate monomorphizations without filling out definitions yet...
            let cr_module = backend.new_metadata(tcx.sess, &cgu_name.as_str());
            let stats = {
                let cx = CrContext::new(tcx, cgu, &cr_module);
                let mono_items = cx.codegen_unit
                                   .items_in_deterministic_order(cx.tcx);
                for &(mono_item, (linkage, visibility)) in &mono_items {
                    mono_item.predefine::<CrBuilder>(&cx, linkage, visibility);
                }

                // ... and now that we have everything pre-defined, fill out those definitions.
                for &(mono_item, _) in &mono_items {
                    mono_item.define::<CrBuilder>(&cx);
                }

                // If this codegen unit contains the main function, also create the
                // wrapper here
                maybe_create_entry_wrapper::<CrBuilder>(&cx);

                // Run replace-all-uses-with for statics that need it
                for &(old_g, new_g) in cx.statics_to_rauw().borrow().iter() {
                    cx.static_replace_all_uses(old_g, new_g)
                }

                // Finalize debuginfo
                if cx.sess().opts.debuginfo != DebugInfo::None {
                    cx.debuginfo_finalize();
                }

                cx.consume_stats().into_inner()
            };

            (stats, ModuleCodegen {
                name: cgu_name.to_string(),
                module_llvm: cr_module,
                kind: ModuleKind::Regular,
            })
        }
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
