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
use cranelift;
use cranelift::prelude::FunctionBuilderContext;
use cranelift::prelude::codegen::Context;
use cranelift::prelude::codegen::entity::{PrimaryMap, EntityRef};
use cranelift::prelude::codegen::ir::function::Function;
use write::CrModule;
use rustc::ty::{TyCtxt, Instance};
use rustc::mir::mono::{CodegenUnit, Stats};
use rustc_data_structures::fx::FxHashMap;

use std::sync::Arc;
use std::cell::RefCell;

pub struct CrContext<'ll, 'tcx: 'll> {
    pub crcx: Context,
    pub fx_crcx: RefCell<FunctionBuilderContext>,
    pub current_instance: RefCell<Option<CrInstance>>,
    pub tcx : TyCtxt<'ll, 'tcx, 'tcx>,
    pub codegen_unit: Arc<CodegenUnit<'tcx>>,
    pub cr_instances: RefCell<PrimaryMap<CrInstance, RefCell<Function>>>,
    pub instances: RefCell<FxHashMap<Instance<'tcx>, CrValue>>,
    pub stats: RefCell<Stats>
}

impl<'ll, 'tcx: 'll> CrContext<'ll, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'ll, 'tcx, 'tcx>,
        cgu: Arc<CodegenUnit<'tcx>>,
        _cranelift_module: &CrModule
    ) -> Self {
            CrContext {
                crcx: Context::new(),
                fx_crcx: RefCell::new(FunctionBuilderContext::new()),
                tcx,
                codegen_unit: cgu,
                current_instance: RefCell::new(None),
                cr_instances: RefCell::new(PrimaryMap::new()),
                instances: RefCell::new(FxHashMap()),
                stats: RefCell::new(Stats::default())
            }
        }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrInstance(usize);

impl EntityRef for CrInstance {
    fn new(x : usize) -> Self {
        CrInstance(x)
    }
    fn index(self) -> usize {
        let CrInstance(v) = self;
        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrValue {
    Value(cranelift::prelude::Value),
    Instance(CrInstance)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrType(cranelift::prelude::Type);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrEbb(cranelift::prelude::Type);

impl<'ll, 'tcx: 'll> Backend<'ll> for CrContext<'ll, 'tcx> {
    type Value = CrValue;
    type BasicBlock = CrEbb;
    type Type = CrType;
    type Context = ();
}

impl<'a, 'll: 'a, 'tcx: 'll> CodegenMethods<'a, 'll, 'tcx> for CrContext<'ll, 'tcx> {}

impl<'ll> CodegenObject for CrValue {}

impl<'ll> CodegenObject for CrType {}
