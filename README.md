Muda segfault reproduction.

Run with: `cargo run`
Platform reproduced on: MacOS ARM

Rust version:
```
stable-aarch64-apple-darwin (default)
rustc 1.75.0 (82e1608df 2023-12-21)
```

Error: `trace trap` or `segmentation fault`

LLDB Backtrace:
```
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=1, address=0x8008001e)
  * frame #0: 0x000000018b789bbc libobjc.A.dylib`objc_opt_isKindOfClass + 56
    frame #1: 0x000000018f392968 AppKit`+[NSMenuItem(NSLocalizedKeyboardShortcuts) updateKeyboardAwareShortcutsForMenu:ofCurrentSource:withLanguageID:] + 352
    frame #2: 0x000000018f394894 AppKit`UpdateKeyboardAwareShortcutsRecursively + 128
    frame #3: 0x000000018f392338 AppKit`UpdateKeyboardAwareShortcutsForCurrentInputSource + 612
    frame #4: 0x000000018f393e6c AppKit`__35-[_NSMenuShortcutUpdater schedule:]_block_invoke + 96
    frame #5: 0x000000018bbee1d4 CoreFoundation`__CFRUNLOOP_IS_CALLING_OUT_TO_A_BLOCK__ + 28
    frame #6: 0x000000018bbee0e8 CoreFoundation`__CFRunLoopDoBlocks + 364
    frame #7: 0x000000018bbecf38 CoreFoundation`__CFRunLoopRun + 812
    frame #8: 0x000000018bbec4b8 CoreFoundation`CFRunLoopRunSpecific + 612
    frame #9: 0x000000019543edf0 HIToolbox`RunCurrentEventLoopInMode + 292
    frame #10: 0x000000019543ea80 HIToolbox`ReceiveNextEventCommon + 220
    frame #11: 0x000000019543e984 HIToolbox`_BlockUntilNextEventMatchingListInModeWithFilter + 76
    frame #12: 0x000000018ee1397c AppKit`_DPSNextEvent + 636
    frame #13: 0x000000018ee12b18 AppKit`-[NSApplication(NSEvent) _nextEventMatchingEventMask:untilDate:inMode:dequeue:] + 716
    frame #14: 0x000000018ee06f7c AppKit`-[NSApplication run] + 464
    frame #15: 0x00000001001306fc muda-segfault`_$LT$$LP$$RP$$u20$as$u20$objc..message..MessageArguments$GT$::invoke::h306ed89528c1e5d7(imp=(libobjc.A.dylib`objc_msgSend), obj=0x0000000100515a50, sel=Sel @ 0x000000016fdfe308, (null)=<unavailable>) at mod.rs:128:17
    frame #16: 0x000000010012fb3c muda-segfault`objc::message::platform::send_unverified::h2b62a875d2a0438b(obj=0x0000000100515a50, sel=Sel @ 0x000000016fdfe370, args=<unavailable>) at mod.rs:27:9
    frame #17: 0x000000010000aaf0 muda-segfault`tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run_return::h00cd4916ed95df38 [inlined] objc::message::send_message::hccc5ba6dea84d390(obj=0x0000000100515a50, sel=Sel @ 0x000000016fdfe680, args=<unavailable>) at mod.rs:178:5
    frame #18: 0x000000010000aadc muda-segfault`tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run_return::h00cd4916ed95df38(self=0x000000016fdfe758, callback={closure_env#0} @ 0x000000016fdfe5e0) at event_loop.rs:223:16
    frame #19: 0x000000010000ada0 muda-segfault`tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h259be471f46fcc82(self=<unavailable>, callback={closure_env#0} @ 0x000000016fdfe6f8) at event_loop.rs:190:21
    frame #20: 0x000000010000e0a4 muda-segfault`tao::event_loop::EventLoop$LT$T$GT$::run::h2c3c5f912075dd35(self=<unavailable>, event_handler={closure_env#0} @ 0x000000016fdfe780) at event_loop.rs:211:5
    frame #21: 0x0000000100005900 muda-segfault`muda_segfault::main::hbe12da8fbc5ab04b at main.rs:13:5
    frame #22: 0x00000001000035fc muda-segfault`core::ops::function::FnOnce::call_once::h63fe431bde0c2af1((null)=(muda-segfault`muda_segfault::main::hbe12da8fbc5ab04b at main.rs:8), (null)=<unavailable>) at function.rs:250:5
    frame #23: 0x000000010000e1d8 muda-segfault`std::sys_common::backtrace::__rust_begin_short_backtrace::heec387946f909432(f=(muda-segfault`muda_segfault::main::hbe12da8fbc5ab04b at main.rs:8)) at backtrace.rs:154:18
    frame #24: 0x000000010000e3fc muda-segfault`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb96c29d746d4f36d at rt.rs:167:18
    frame #25: 0x000000010015dcd4 muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h1a7c0e059d971da5 at function.rs:284:13 [opt]
    frame #26: 0x000000010015dccc muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panicking::try::do_call::h07a34a23e615022b at panicking.rs:552:40 [opt]
    frame #27: 0x000000010015dccc muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panicking::try::h1111644420b4cc09 at panicking.rs:516:19 [opt]
    frame #28: 0x000000010015dccc muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panic::catch_unwind::h31a3b9d6e2ef9973 at panic.rs:142:14 [opt]
    frame #29: 0x000000010015dccc muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h63c3452500a36531 at rt.rs:148:48 [opt]
    frame #30: 0x000000010015dccc muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panicking::try::do_call::h9c5c8a2a0a297bb7 at panicking.rs:552:40 [opt]
    frame #31: 0x000000010015dcc8 muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panicking::try::h424cfcafca1bde97 at panicking.rs:516:19 [opt]
    frame #32: 0x000000010015dcc8 muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 [inlined] std::panic::catch_unwind::h345d3d448041017f at panic.rs:142:14 [opt]
    frame #33: 0x000000010015dcc8 muda-segfault`std::rt::lang_start_internal::h5b246d44f1526226 at rt.rs:148:20 [opt]
    frame #34: 0x000000010000e3c8 muda-segfault`std::rt::lang_start::h4c410538bde619d6(main=(muda-segfault`muda_segfault::main::hbe12da8fbc5ab04b at main.rs:8), argc=1, argv=0x000000016fdfee40, sigpipe='\0') at rt.rs:166:17
    frame #35: 0x0000000100005950 muda-segfault`main + 36
    frame #36: 0x000000018b7b7f28 dyld`start + 2236
```