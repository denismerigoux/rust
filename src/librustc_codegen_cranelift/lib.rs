// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]
#![feature(libc)]

#[macro_use] extern crate rustc;
extern crate rustc_codegen_utils;
extern crate cranelift;
extern crate rustc_target;
extern crate rustc_data_structures;
extern crate rustc_codegen_ssa;
extern crate syntax_pos;
extern crate libc;

mod backend;
mod context;
mod consts;
mod type_;
mod misc;
mod declare;
mod intrinsic;
mod statics;
mod debuginfo;

use rustc::ty::{self, TyCtxt};
use rustc::session::{Session, config::{PrintRequest, OutputFilenames}, CompileIncomplete};
use rustc::middle::cstore::MetadataLoader;
use rustc::dep_graph::DepGraph;
use rustc_codegen_utils::codegen_backend::{CodegenBackend, NoLlvmMetadataLoader,
    MetadataOnlyCodegenBackend};


use std::sync::mpsc;
use std::any::Any;

pub struct CraneliftCodegenBackend(());

impl CraneliftCodegenBackend {
    #[allow(dead_code)]
    pub fn new() -> Box<dyn CodegenBackend> {
        box CraneliftCodegenBackend(())
    }
}

impl CodegenBackend for CraneliftCodegenBackend {
    fn init(&self, _sess: &Session) {

    }

    fn print(&self, _req: PrintRequest, _sess: &Session) {
        unimplemented!()
    }

    fn print_passes(&self) {
        unimplemented!()
    }

    fn print_version(&self) {
        unimplemented!()
    }

    fn metadata_loader(&self) -> Box<dyn MetadataLoader + Sync> {
        box NoLlvmMetadataLoader
    }

    fn provide(&self, providers: &mut ty::query::Providers) {
        // FIXME: replace this dummy implementation
        MetadataOnlyCodegenBackend::new().provide(providers)
    }

    fn provide_extern(&self, providers: &mut ty::query::Providers) {
        // FIXME: replace this dummy implementation
        MetadataOnlyCodegenBackend::new().provide_extern(providers)
    }

    fn codegen_crate<'a, 'tcx>(
        &self,
        _tcx: TyCtxt<'a, 'tcx, 'tcx>,
        _rx: mpsc::Receiver<Box<dyn Any + Send>>
    ) -> Box<dyn Any> {
        unimplemented!()
    }

    fn join_codegen_and_link(
        &self,
        _ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _dep_graph: &DepGraph,
        _outputs: &OutputFilenames,
    ) -> Result<(), CompileIncomplete>{
        unimplemented!()
    }
}
