// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc_codegen_ssa::back::write::{ModuleConfig, CodegenContext, DiagnosticHandlers};
use rustc_codegen_ssa::back::lto::{SerializedModule, LtoModuleCodegen, ThinModule};
use rustc_codegen_ssa::interfaces::*;
use rustc_codegen_ssa::{ModuleCodegen, CompiledModule};
use CraneliftCodegenBackend;

use rustc::dep_graph::WorkProduct;
use rustc::util::time_graph::Timeline;
use rustc_errors::{FatalError, Handler};


pub struct ThinBuffer();

impl ThinBufferMethods for ThinBuffer {
    fn data(&self) -> &[u8] {
        unimplemented!()
    }
}

pub struct ModuleBuffer();

impl ModuleBufferMethods for ModuleBuffer {
    fn data(&self) -> &[u8] {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Module();
#[derive(Clone)]
pub struct TargetMachine();

impl WriteBackendMethods for CraneliftCodegenBackend {
    type Module = Module;
    type ModuleBuffer = ModuleBuffer;
    type TargetMachine = TargetMachine;
    type Context = ();
    type ThinData = ();
    type ThinBuffer = ThinBuffer;

    /// Performs LTO, which in the case of full LTO means merging all modules into
    /// a single one and returning it for further optimizing. For ThinLTO, it will
    /// do the global analysis necessary and return two lists, one of the modules
    /// the need optimization and another for modules that can simply be copied over
    /// from the incr. comp. cache.
    fn run_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<ModuleCodegen<Self::Module>>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
        _timeline: &mut Timeline
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError> {
        unimplemented!()
    }
    fn new_diagnostic_handlers<'a>(
        _cgcx: &CodegenContext<Self>,
        _handler: &'a Handler,
        _llcx: &Self::Context
    ) -> DiagnosticHandlers<'a, Self> {
        unimplemented!()
    }
    fn drop_diagnostic_handlers<'a>(_diag: &mut DiagnosticHandlers<'a, Self>) {
        unimplemented!()
    }
    fn print_pass_timings(&self) {
        unimplemented!()
    }
    unsafe fn optimize(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _module: &ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
        _timeline: &mut Timeline
    ) -> Result<(), FatalError> {
        unimplemented!()
    }
    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        _thin: &mut ThinModule<Self>,
        _timeline: &mut Timeline
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        unimplemented!()
    }
    unsafe fn codegen(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
        _timeline: &mut Timeline
    ) -> Result<CompiledModule, FatalError> {
        unimplemented!()
    }
    fn run_lto_pass_manager(
        _cgcx: &CodegenContext<Self>,
        _llmod: &Self::Module,
        _config: &ModuleConfig,
        _thin: bool
    ) {
        unimplemented!()
    }
}
