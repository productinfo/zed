use gpui::actions;

// If the zed binary doesn't use anything in this crate, it will be optimized out
// and the actions won't initialize. So we just call an empty initialization function.
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

actions!(
    Cancel,
    Confirm,
    SecondaryConfirm,
    SelectPrev,
    SelectNext,
    SelectFirst,
    SelectLast,
    ShowContextMenu
);
