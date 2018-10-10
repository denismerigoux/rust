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
use rustc::ty::{self, Ty, Instance};
use rustc::session::Session;
use rustc::mir::mono::{Stats, CodegenUnit};
use rustc_data_structures::fx::FxHashMap;
use super::context::{CrContext, CrValue};
use libc::c_uint;

use std::cell::RefCell;
use std::sync::Arc;

impl<'ll, 'tcx: 'll> MiscMethods<'ll, 'tcx> for CrContext<'tcx> {
    fn vtables(&self) -> &RefCell<FxHashMap<(Ty<'tcx>,
                                Option<ty::PolyExistentialTraitRef<'tcx>>), CrValue>> {
        unimplemented!()
    }
    fn check_overflow(&self) -> bool{
        unimplemented!()
    }
    fn instances(&self) -> &RefCell<FxHashMap<Instance<'tcx>, CrValue>>{
        unimplemented!()
    }
    fn get_fn(&self, _instance: Instance<'tcx>) -> CrValue{
        unimplemented!()
    }
    fn get_param(&self, _llfn: CrValue, _index: c_uint) -> CrValue{
        unimplemented!()
    }
    fn eh_personality(&self) -> CrValue{
        unimplemented!()
    }
    fn eh_unwind_resume(&self) -> CrValue{
        unimplemented!()
    }
    fn sess(&self) -> &Session{
        unimplemented!()
    }
    fn stats(&self) -> &RefCell<Stats>{
        unimplemented!()
    }
    fn consume_stats(self) -> RefCell<Stats>{
        unimplemented!()
    }
    fn codegen_unit(&self) -> &Arc<CodegenUnit<'tcx>>{
        unimplemented!()
    }
    fn statics_to_rauw(&self) -> &RefCell<Vec<(CrValue, CrValue)>>{
        unimplemented!()
    }
    fn env_alloca_allowed(&self) -> bool{
        unimplemented!()
    }
    fn used_statics(&self) -> &RefCell<Vec<CrValue>>{
        unimplemented!()
    }
    fn set_frame_pointer_elimination(&self, _llfn: CrValue){
        unimplemented!()
    }
    fn apply_target_cpu_attr(&self, _llfn: CrValue){
        unimplemented!()
    }
    fn create_used_variable(&self){
        unimplemented!()
    }
}
