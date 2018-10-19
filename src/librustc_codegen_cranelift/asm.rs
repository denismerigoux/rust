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
use rustc_codegen_ssa::mir::place::PlaceRef;
use super::context::{CrContext, CrValue};
use super::builder::CrBuilder;
use rustc::hir::{GlobalAsm, InlineAsm};

impl<'ll, 'tcx: 'll> AsmMethods for CrContext<'ll, 'tcx> {
    fn codegen_global_asm(&self, _ga: &GlobalAsm) {
        unimplemented!()
    }
}

impl<'a, 'll: 'a, 'tcx: 'll> AsmBuilderMethods<'a, 'll, 'tcx>
    for CrBuilder<'a, 'll, 'tcx>
{
    // Take an inline assembly expression and splat it out via LLVM
    fn codegen_inline_asm(
        &mut self,
        _ia: &InlineAsm,
        _outputs: Vec<PlaceRef<'tcx, CrValue>>,
        _inputs: Vec<CrValue>
    ) -> bool {
        unimplemented!()
    }
}
