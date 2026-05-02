//! Code representation of `overworld/sequences/dark_menu_open.sequence.ron`.
//!
//! `overworld/sequences/dark_menu_open.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;
use souprune_schema::sequence::SequenceAsset;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> SequenceAsset {
    crate::support::dark_menu::open_tween_sequence_asset()
}
