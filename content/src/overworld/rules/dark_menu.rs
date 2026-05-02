//! Code representation of `overworld/rules/dark_menu.fre.ron`.
//!
//! `overworld/rules/dark_menu.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> souprune_schema::fre::FreAsset {
    crate::support::dark_menu::fre_asset()
}
