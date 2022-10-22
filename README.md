# ICE

```bash
cd quack
cargo rustdoc
```

```console
$ cargo --version
cargo 1.66.0-nightly (3ff044334 2022-10-17)
```

```text
warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
 --> moo/src/lib.rs:1:12
  |
1 | #![feature(async_fn_in_trait)]
  |            ^^^^^^^^^^^^^^^^^
  |
  = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
  = note: `#[warn(incomplete_features)]` on by default

warning: `moo` (lib) generated 1 warning
 Documenting quack v0.1.0 (/home/me/git/ice/quack)
warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
 --> quack/src/lib.rs:1:12
  |
1 | #![feature(async_fn_in_trait)]
  |            ^^^^^^^^^^^^^^^^^
  |
  = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
  = note: `#[warn(incomplete_features)]` on by default

thread 'rustc' panicked at 'no encoded ident for item', compiler/rustc_metadata/src/rmeta/decoder.rs:780:9
stack backtrace:
   0:     0x7f882c29a670 - std::backtrace_rs::backtrace::libunwind::trace::h7437781aa85dca60
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f882c29a670 - std::backtrace_rs::backtrace::trace_unsynchronized::hb84419b9fcbab27c
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f882c29a670 - std::sys_common::backtrace::_print_fmt::h9007eb4ce0fa4ee4
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f882c29a670 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h907649500d38afba
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f882c2f658e - core::fmt::write::hb8299136d300b292
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f882c28a8e5 - std::io::Write::write_fmt::hebd34bdde63d754d
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/io/mod.rs:1682:15
   6:     0x7f882c29a435 - std::sys_common::backtrace::_print::h840d14c241033cf4
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f882c29a435 - std::sys_common::backtrace::print::h20a48f724eef8553
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f882c29d23f - std::panicking::default_hook::{{closure}}::hcc5a4d7d2c9355d1
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/panicking.rs:267:22
   9:     0x7f882c29cf7a - std::panicking::default_hook::haf4608272aae7458
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/panicking.rs:286:9
  10:     0x7f882aabfeb4 - rustc_driver[7240838df62c6c6a]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f882c29da69 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h583650acfaf68785
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/alloc/src/boxed.rs:2001:9
  12:     0x7f882c29da69 - std::panicking::rust_panic_with_hook::h3a705e9366d5a595
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/panicking.rs:692:13
  13:     0x7f882c29d7e7 - std::panicking::begin_panic_handler::{{closure}}::hcbadfa286450495e
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/panicking.rs:579:13
  14:     0x7f882c29ab1c - std::sys_common::backtrace::__rust_end_short_backtrace::h8bb0eada007dcf6a
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7f882c29d502 - rust_begin_unwind
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/panicking.rs:575:5
  16:     0x7f882c2f2f73 - core::panicking::panic_fmt::h7b1097799bef8de0
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/core/src/panicking.rs:65:14
  17:     0x7f882c2f30c1 - core::panicking::panic_display::ha954564600d863b5
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/core/src/panicking.rs:139:5
  18:     0x7f882c2f306b - core::panicking::panic_str::h3e2be859b3d8d1f9
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/core/src/panicking.rs:123:5
  19:     0x7f882c2f2ce6 - core::option::expect_failed::h9ff5e77425bcb875
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/core/src/option.rs:1876:5
  20:     0x7f882a9e4b4f - rustc_metadata[9b6c65370b9c350f]::rmeta::decoder::cstore_impl::provide_extern::associated_item
  21:     0x7f882a3a5ffe - <rustc_query_impl[42c2fa874da0ba3c]::Queries as rustc_middle[5bf93a4b064c7070]::ty::query::QueryEngine>::associated_item
  22:     0x55940360c5a8 - rustdoc[af270831dbb49fe9]::clean::projection_to_path_segment
  23:     0x55940360c09b - rustdoc[af270831dbb49fe9]::clean::clean_projection
  24:     0x5594036149c4 - rustdoc[af270831dbb49fe9]::clean::clean_middle_ty
  25:     0x55940360f834 - rustdoc[af270831dbb49fe9]::clean::clean_fn_decl_from_did_and_sig
  26:     0x55940361079f - rustdoc[af270831dbb49fe9]::clean::clean_middle_assoc_item
  27:     0x559403431291 - <alloc[1cd710ac49af97b8]::vec::Vec<rustdoc[af270831dbb49fe9]::clean::types::Item> as alloc[1cd710ac49af97b8]::vec::spec_from_iter::SpecFromIter<rustdoc[af270831dbb49fe9]::clean::types::Item, core[4c69c3a2f73b376]::iter::adapters::map::Map<core[4c69c3a2f73b376]::iter::adapters::map::Map<core[4c69c3a2f73b376]::iter::adapters::map::Map<core[4c69c3a2f73b376]::slice::iter::Iter<(rustc_span[9b0423c38f2996a]::symbol::Symbol, &rustc_middle[5bf93a4b064c7070]::ty::assoc::AssocItem)>, <rustc_data_structures[155a559c9b9e6f13]::sorted_map::index_map::SortedIndexMultiMap<u32, rustc_span[9b0423c38f2996a]::symbol::Symbol, &rustc_middle[5bf93a4b064c7070]::ty::assoc::AssocItem>>::iter::{closure#0}>, <rustc_middle[5bf93a4b064c7070]::ty::assoc::AssocItems>::in_definition_order::{closure#0}>, rustdoc[af270831dbb49fe9]::clean::inline::build_external_trait::{closure#0}>>>::from_iter
  28:     0x5594033f31ad - rustdoc[af270831dbb49fe9]::clean::inline::build_external_trait
  29:     0x5594033fa910 - rustdoc[af270831dbb49fe9]::clean::inline::record_extern_trait
  30:     0x5594033f7ee0 - rustdoc[af270831dbb49fe9]::clean::inline::build_impl
  31:     0x559403510761 - <rustdoc[af270831dbb49fe9]::core::DocContext>::with_all_trait_impls::<rustdoc[af270831dbb49fe9]::passes::collect_trait_impls::collect_trait_impls::{closure#4}>
  32:     0x5594035de025 - rustdoc[af270831dbb49fe9]::passes::collect_trait_impls::collect_trait_impls
  33:     0x5594035412ad - <rustc_session[136ed805c785b0fc]::session::Session>::time::<rustdoc[af270831dbb49fe9]::clean::types::Crate, rustdoc[af270831dbb49fe9]::core::run_global_ctxt::{closure#7}>
  34:     0x5594035145f3 - rustdoc[af270831dbb49fe9]::core::run_global_ctxt
  35:     0x55940354158f - <rustc_session[136ed805c785b0fc]::session::Session>::time::<(rustdoc[af270831dbb49fe9]::clean::types::Crate, rustdoc[af270831dbb49fe9]::config::RenderOptions, rustdoc[af270831dbb49fe9]::formats::cache::Cache), rustdoc[af270831dbb49fe9]::main_args::{closure#1}::{closure#0}::{closure#1}::{closure#0}>
  36:     0x5594035e5611 - <rustc_interface[193c2632633c4bd2]::passes::QueryContext>::enter::<rustdoc[af270831dbb49fe9]::main_args::{closure#1}::{closure#0}::{closure#1}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>
  37:     0x55940335b8f4 - <rustc_interface[193c2632633c4bd2]::interface::Compiler>::enter::<rustdoc[af270831dbb49fe9]::main_args::{closure#1}::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>
  38:     0x5594034dfaf1 - <scoped_tls[ff9cb7d73db555fe]::ScopedKey<rustc_span[9b0423c38f2996a]::SessionGlobals>>::set::<rustc_interface[193c2632633c4bd2]::interface::run_compiler<core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>, rustdoc[af270831dbb49fe9]::main_args::{closure#1}>::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>
  39:     0x5594035526b0 - std[631fbe326a1fa8e7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[193c2632633c4bd2]::util::run_in_thread_pool_with_globals<rustc_interface[193c2632633c4bd2]::interface::run_compiler<core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>, rustdoc[af270831dbb49fe9]::main_args::{closure#1}>::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>
  40:     0x55940365c37a - <<std[631fbe326a1fa8e7]::thread::Builder>::spawn_unchecked_<rustc_interface[193c2632633c4bd2]::util::run_in_thread_pool_with_globals<rustc_interface[193c2632633c4bd2]::interface::run_compiler<core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>, rustdoc[af270831dbb49fe9]::main_args::{closure#1}>::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4c69c3a2f73b376]::result::Result<(), rustc_errors[79b484a79fe74c4b]::ErrorGuaranteed>>::{closure#1} as core[4c69c3a2f73b376]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f882c2a7583 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hcfc5d500cd06cbe0
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/alloc/src/boxed.rs:1987:9
  42:     0x7f882c2a7583 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h98c94487353f1fd4
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/alloc/src/boxed.rs:1987:9
  43:     0x7f882c2a7583 - std::sys::unix::thread::Thread::new::thread_start::h0b76d4916e03e8a3
                               at /rustc/5c8bff74bc1c52bef0c79f3689bb227f51f3e82d/library/std/src/sys/unix/thread.rs:108:17
  44:     0x7f8827e88e86 - start_thread
  45:     0x7f8827f0fc60 - __clone3
  46:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (5c8bff74b 2022-10-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [associated_item] computing associated item data for `moo::Meow::woof::{opaque#0}`
end of query stack
warning: `quack` (lib doc) generated 1 warning
error: could not document `quack`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type lib --crate-name quack quack/src/lib.rs -o /home/me/git/ice/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=68c4a09bbbec8646 -L dependency=/home/me/git/ice/target/debug/deps --extern moo=/home/me/git/ice/target/debug/deps/libmoo-60bfae4501a3bb3e.rmeta --crate-version 0.1.0` (exit status: 101)
```
