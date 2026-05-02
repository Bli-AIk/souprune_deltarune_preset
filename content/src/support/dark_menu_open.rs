//! Deltarune dark menu opening keyframe sequence.
//!
//! Deltarune 暗色菜单打开逐帧关键帧序列。

use souprune_schema::sequence::{
    Chapter, EaseKindRepr, ElementSelector, SequenceAsset, TweenTarget,
};
use std::collections::HashMap;

const FRAME_SECONDS: f32 = 1.0 / 30.0;
const OPEN_KEYFRAMES: &[(f32, f32)] = &[
    (30.0, 30.0),
    (60.0, 42.0),
    (68.0, 49.0),
    (73.0, 53.0),
    (76.0, 56.0),
    (78.0, 58.0),
    (79.0, 59.0),
    (80.0, 60.0),
];

pub fn asset() -> SequenceAsset {
    let mut chapters = Vec::from([Chapter::AwaitFact {
        condition: "$state:sequence_sub_state == 'DarkMenu'".into(),
        local: false,
    }, Chapter::Wait(FRAME_SECONDS)]);

    for (index, (tp, bp)) in OPEN_KEYFRAMES.iter().copied().enumerate() {
        chapters.push(keyframe(tp, bp));
        if index + 1 < OPEN_KEYFRAMES.len() {
            chapters.push(Chapter::Wait(FRAME_SECONDS));
        }
    }

    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: HashMap::new(),
        chapters,
    }
}

fn keyframe(tp: f32, bp: f32) -> Chapter {
    Chapter::Parallel(vec![
        set_position("DarkMenuTopGroup", 0.0, 80.0 - tp, 0.0),
        set_scale("DarkMenuTopMask", 660.0, tp + 10.0, 1.0),
        set_position("DarkMenuBottomMask", -10.0, bp - 479.0, 0.5),
        set_scale("DarkMenuBottomMask", 660.0, bp + 21.0, 1.0),
        set_position("DarkMenuPartyGroup", 0.0, bp - 60.0, 0.0),
    ])
}

fn set_position(name: &str, x: f32, y: f32, z: f32) -> Chapter {
    Chapter::SetViewElement {
        selector: ElementSelector::local(name),
        target: TweenTarget::position((x, y, z)),
        duration: None,
        easing: EaseKindRepr::Linear,
        wait_for_completion: true,
    }
}

fn set_scale(name: &str, x: f32, y: f32, z: f32) -> Chapter {
    Chapter::SetViewElement {
        selector: ElementSelector::local(name),
        target: TweenTarget::scale((x, y, z)),
        duration: None,
        easing: EaseKindRepr::Linear,
        wait_for_completion: true,
    }
}
