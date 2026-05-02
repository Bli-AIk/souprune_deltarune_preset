//! Code representation of `narrative/dialogue.ron`.
//!
//! `narrative/dialogue.ron` 的代码表示。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;
use souprune_schema::dialogue::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> DialogueConfig {
    DialogueConfig {
        auto_pause: AutoPauseConfig {
            default_preset: "normal".into(),
            presets: vec![(
                "normal".into(),
                vec![
                    (".".into(), 0.333),
                    (",".into(), 0.08),
                    ("!".into(), 0.333),
                    ("?".into(), 0.333),
                    ("\n".into(), 0.15),
                    ("。".into(), 0.333),
                    ("，".into(), 0.08),
                    ("！".into(), 0.333),
                    ("？".into(), 0.333),
                    ("…".into(), 0.333),
                ]
                .into_iter()
                .collect(),
            )]
            .into_iter()
            .collect(),
        },
        voice: VoiceConfig {
            default_preset: "normal".into(),
            presets: vec![(
                "normal".into(),
                vec![
                    (" ".into(), false),
                    ("\n".into(), false),
                    (".".into(), false),
                    (",".into(), false),
                    ("!".into(), false),
                    ("?".into(), false),
                    ("。".into(), false),
                    ("，".into(), false),
                    ("！".into(), false),
                    ("？".into(), false),
                    ("…".into(), false),
                ]
                .into_iter()
                .collect(),
            )]
            .into_iter()
            .collect(),
        },
    }
}
