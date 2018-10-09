// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc_codegen_ssa::interfaces::{Backend, CodegenObject};
use cranelift;

use std::marker::PhantomData;

#[allow(dead_code)]
pub struct CraneliftContext<'tcx> {
    phantom: PhantomData<&'tcx ()>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrValue(cranelift::prelude::Value);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrType(cranelift::prelude::Type);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrEbb(cranelift::prelude::Type);

impl<'ll, 'tcx: 'll> Backend<'ll> for CraneliftContext<'tcx> {
    type Value = CrValue;
    type BasicBlock = CrEbb;
    type Type = CrType;
    type Context = ();
}

impl<'ll> CodegenObject for CrValue {}

impl<'ll> CodegenObject for CrType {}
