//! Test command for testing the superopt baseline pass.
//!
//! The resulting function is sent to `filecheck`.

use crate::subtest::{run_filecheck, Context, SubTest, SubtestResult};
use cranelift_codegen;
use cranelift_codegen::ir::Function;
use cranelift_codegen::print_errors::pretty_error;
use cranelift_reader::TestCommand;
use std::borrow::Cow;

struct TestSuperoptBaseline;

pub fn subtest(parsed: &TestCommand) -> SubtestResult<Box<dyn SubTest>> {
    assert_eq!(parsed.command, "superopt_baseline");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestSuperoptBaseline))
    }
}

impl SubTest for TestSuperoptBaseline {
    fn name(&self) -> &'static str {
        "superopt_baseline"
    }

    fn is_mutating(&self) -> bool {
        true
    }

    fn run(&self, func: Cow<Function>, context: &Context) -> SubtestResult<()> {
        let mut comp_ctx = cranelift_codegen::Context::for_function(func.into_owned());
        let isa = context.isa.expect("superopt baseline needs an ISA");

        comp_ctx.compute_cfg();
        comp_ctx
            .superopt_baseline(isa)
            .map_err(|e| pretty_error(&comp_ctx.func, context.isa, Into::into(e)))?;
        let text = &comp_ctx.func.display(isa).to_string();
        log::debug!("After superopt_baseline:\n{}", text);

        // Only actually run the filecheck if peepmatic is *not* enabled,
        // because it can generate slightly different code (alias a result vs
        // replace an instruction) than the non-peepmatic versions of peephole
        // optimizations. Note that the `peepmatic`-based results can be tested
        // with the `test peepmatic` subtest.
        // if cfg!(feature = "enable-peepmatic") {
        //     Ok(())
        // } else {
        //     run_filecheck(&text, context)
        // }
        run_filecheck(&text, context)
    }
}
