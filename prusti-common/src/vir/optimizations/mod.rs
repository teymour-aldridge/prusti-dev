// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A module that contains various VIR optimizations.

use vir::Program;
use crate::config::{self, Optimizations};

pub mod folding;
pub mod functions;
pub mod methods;
pub mod predicates;

impl Program {
    pub fn optimized(mut self) -> Self {
        let optimizations = config::optimizations();
        info!("Enabled optimisations: {:?}", optimizations);

        // can't borrow self because we need to move fields
        if optimizations.inline_constant_functions {
            let (new_methods, new_functions) =
                functions::inline_constant_functions(self.methods, self.functions);
            
            self.methods = new_methods
                .into_iter()
                .map(|m| {
                    let purified = methods::purify_vars(m);
                    folding::FoldingOptimizer::optimize(purified)
                })
                .collect();
            self.functions = new_functions
                .into_iter()
                .map(|f| folding::FoldingOptimizer::optimize(f))
                .collect();
        }
        if optimizations.delete_unused_predicates {
            self.viper_predicates = predicates::delete_unused_predicates(
                &self.methods,
                &self.functions,
                self.viper_predicates,
            );
        }

        self
    }
}
