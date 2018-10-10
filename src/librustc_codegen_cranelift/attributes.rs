// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::ty::query::Providers;
use rustc::hir::def_id::LOCAL_CRATE;
use rustc_data_structures::sync::Lrc;
use rustc_data_structures::fx::FxHashMap;

pub fn provide(providers: &mut Providers) {
    providers.target_features_whitelist = |_tcx, cnum| {
        assert_eq!(cnum, LOCAL_CRATE);
        //FIXME: fix this dummy implementation
        let ret = FxHashMap();
        Lrc::new(ret)
    };
}
