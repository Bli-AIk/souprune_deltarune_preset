//! View asset for `overworld/view/dark_menu.view.ron`.
//!
//! `overworld/view/dark_menu.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> souprune_schema::view::ViewLayoutAsset {
    crate::support::dark_menu::view_asset()
}
