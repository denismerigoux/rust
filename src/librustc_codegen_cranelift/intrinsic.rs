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
use super::context::{CraneliftContext, CrValue};

impl<'ll, 'tcx: 'll> IntrinsicDeclarationMethods<'ll> for CraneliftContext<'tcx> {
    fn get_intrinsic(&self, _key: &str) -> CrValue {
        unimplemented!()
    }

    fn declare_intrinsic(
        &self,
        _key: &str
    ) -> Option<CrValue> {
        unimplemented!()
    }
}
