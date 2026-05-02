//! Shared builders for Deltarune overworld menu assets.
//!
//! Deltarune overworld 菜单资源的共享构建器。

use souprune_schema::fre::*;
use souprune_schema::view::*;
use std::collections::HashMap;

const NORMAL_STATE: &str = "Normal";
const DARK_MENU_STATE: &str = "DarkMenu";

pub fn fre_asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![(
            "dr.menu.layer".into(),
            vec![
                "top_menu".into(),
                "item_category".into(),
                "item_list".into(),
                "storage_list".into(),
                "key_item_list".into(),
            ],
        )]
        .into_iter()
        .collect(),
        facts: menu_facts(),
        rules: menu_rules(),
    }
}

pub fn global_facts_asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Global,
        enums: HashMap::new(),
        facts: menu_facts(),
        rules: Vec::new(),
    }
}

fn menu_facts() -> HashMap<String, FactValueDef> {
    vec![
        (
            "dr.menu.layer".into(),
            FactValueDef::Enum("top_menu".into()),
        ),
        (
            "dr_menu_layer".into(),
            FactValueDef::Enum("top_menu".into()),
        ),
        ("dr.menu.top_index".into(), FactValueDef::Int(0)),
        ("dr_menu_top_index".into(), FactValueDef::Int(0)),
        ("dr.menu.category_index".into(), FactValueDef::Int(0)),
        ("dr_menu_category_index".into(), FactValueDef::Int(0)),
        ("dr.menu.item_cursor".into(), FactValueDef::Int(0)),
        ("dr_menu_item_cursor".into(), FactValueDef::Int(0)),
        ("dr.menu.storage_cursor".into(), FactValueDef::Int(0)),
        ("dr_menu_storage_cursor".into(), FactValueDef::Int(0)),
        ("dr.menu.key_item_cursor".into(), FactValueDef::Int(0)),
        ("dr_menu_key_item_cursor".into(), FactValueDef::Int(0)),
        (
            "dr.menu.feedback".into(),
            FactValueDef::String(String::new()),
        ),
        (
            "dr_menu_feedback".into(),
            FactValueDef::String(String::new()),
        ),
        ("dr.money".into(), FactValueDef::Int(42)),
        ("dr_money".into(), FactValueDef::Int(42)),
        ("dr.party.count".into(), FactValueDef::Int(3)),
        ("dr_party_count".into(), FactValueDef::Int(3)),
        (
            "dr.party.0.name".into(),
            FactValueDef::String("Kris".into()),
        ),
        (
            "dr_party_0_name".into(),
            FactValueDef::String("Kris".into()),
        ),
        ("dr.party.0.hp".into(), FactValueDef::Int(90)),
        ("dr_party_0_hp".into(), FactValueDef::Int(90)),
        ("dr.party.0.max_hp".into(), FactValueDef::Int(90)),
        ("dr_party_0_max_hp".into(), FactValueDef::Int(90)),
        (
            "dr.party.0.status".into(),
            FactValueDef::String("OK".into()),
        ),
        (
            "dr.party.1.name".into(),
            FactValueDef::String("Susie".into()),
        ),
        (
            "dr_party_1_name".into(),
            FactValueDef::String("Susie".into()),
        ),
        ("dr.party.1.hp".into(), FactValueDef::Int(110)),
        ("dr_party_1_hp".into(), FactValueDef::Int(110)),
        ("dr.party.1.max_hp".into(), FactValueDef::Int(110)),
        ("dr_party_1_max_hp".into(), FactValueDef::Int(110)),
        (
            "dr.party.1.status".into(),
            FactValueDef::String("OK".into()),
        ),
        (
            "dr.party.2.name".into(),
            FactValueDef::String("Ralsei".into()),
        ),
        (
            "dr_party_2_name".into(),
            FactValueDef::String("Ralsei".into()),
        ),
        ("dr.party.2.hp".into(), FactValueDef::Int(70)),
        ("dr_party_2_hp".into(), FactValueDef::Int(70)),
        ("dr.party.2.max_hp".into(), FactValueDef::Int(70)),
        ("dr_party_2_max_hp".into(), FactValueDef::Int(70)),
        (
            "dr.party.2.status".into(),
            FactValueDef::String("OK".into()),
        ),
        (
            "dr.inventory.items".into(),
            FactValueDef::StringList(vec![
                "Dark Candy".into(),
                "ReviveMint".into(),
                "Glowshard".into(),
            ]),
        ),
        (
            "dr_inventory_items".into(),
            FactValueDef::StringList(vec![
                "Dark Candy".into(),
                "ReviveMint".into(),
                "Glowshard".into(),
            ]),
        ),
        ("dr.inventory.count".into(), FactValueDef::Int(3)),
        ("dr_inventory_count".into(), FactValueDef::Int(3)),
        (
            "dr.storage.items".into(),
            FactValueDef::StringList(vec!["Manual".into()]),
        ),
        (
            "dr_storage_items".into(),
            FactValueDef::StringList(vec!["Manual".into()]),
        ),
        ("dr.storage.count".into(), FactValueDef::Int(1)),
        ("dr_storage_count".into(), FactValueDef::Int(1)),
        (
            "dr.key_items.items".into(),
            FactValueDef::StringList(vec!["Cell Phone".into()]),
        ),
        (
            "dr_key_items_items".into(),
            FactValueDef::StringList(vec!["Cell Phone".into()]),
        ),
        ("dr.key_items.count".into(), FactValueDef::Int(1)),
        ("dr_key_items_count".into(), FactValueDef::Int(1)),
        ("dr.key_items.usable".into(), FactValueDef::Bool(false)),
        ("dr_key_items_usable".into(), FactValueDef::Bool(false)),
    ]
    .into_iter()
    .collect()
}

fn menu_rules() -> Vec<RuleDef> {
    vec![
        rule(
            "dr_open_dark_menu",
            action_event("Menu"),
            vec![
                "$state:sequence_sub_state == 'Normal'".into(),
                "$dialogue:active != true".into(),
            ],
            vec![
                set_layer("top_menu"),
                set_int("dr_menu_top_index", 0),
                set_int("dr_menu_category_index", 0),
                set_string("dr_menu_feedback", ""),
                set_sub_state(DARK_MENU_STATE),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_top_confirm_item",
            action_event("Confirm"),
            vec!["$dr_menu_layer == 'top_menu'".into(), "$dr_menu_top_index == 0".into()],
            vec![
                set_layer("item_category"),
                set_int("dr_menu_category_index", 0),
                set_string("dr_menu_feedback", ""),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_top_cancel",
            action_event("Cancel"),
            vec!["$dr_menu_layer == 'top_menu'".into()],
            vec![set_sub_state(NORMAL_STATE), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_left_wrap",
            action_event("Left"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index <= 0".into()],
            vec![set_int("dr_menu_category_index", 2), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_left",
            action_event("Left"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index > 0".into()],
            vec![
                set_expr("dr_menu_category_index", "$dr_menu_category_index - 1"),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_category_right_wrap",
            action_event("Right"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index >= 2".into()],
            vec![set_int("dr_menu_category_index", 0), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_right",
            action_event("Right"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index < 2".into()],
            vec![
                set_expr("dr_menu_category_index", "$dr_menu_category_index + 1"),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_category_confirm_item",
            action_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 0".into(),
            ],
            vec![
                set_layer("item_list"),
                set_int("dr_menu_item_cursor", 0),
                set_string("dr_menu_feedback", ""),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_confirm_storage",
            action_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 1".into(),
            ],
            vec![
                set_layer("storage_list"),
                set_int("dr_menu_storage_cursor", 0),
                set_string("dr_menu_feedback", ""),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_confirm_key_item",
            action_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 2".into(),
            ],
            vec![
                set_layer("key_item_list"),
                set_int("dr_menu_key_item_cursor", 0),
                set_string("dr_menu_feedback", ""),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_cancel",
            action_event("Cancel"),
            vec!["$dr_menu_layer == 'item_category'".into()],
            vec![set_layer("top_menu"), RuleActionDef::PlaySound("choice".into())],
        ),
        list_nav_rule("dr_item_left", "Left", "item_list", "dr_menu_item_cursor", "$dr_menu_item_cursor % 2 == 1", "$dr_menu_item_cursor - 1"),
        list_nav_rule("dr_item_right", "Right", "item_list", "dr_menu_item_cursor", "$dr_menu_item_cursor % 2 == 0 && $dr_menu_item_cursor + 1 < $dr_inventory_count", "$dr_menu_item_cursor + 1"),
        list_nav_rule("dr_item_up", "Up", "item_list", "dr_menu_item_cursor", "$dr_menu_item_cursor >= 2", "$dr_menu_item_cursor - 2"),
        list_nav_rule("dr_item_down", "Down", "item_list", "dr_menu_item_cursor", "$dr_menu_item_cursor + 2 < $dr_inventory_count && $dr_menu_item_cursor < 11", "$dr_menu_item_cursor + 2"),
        list_nav_rule("dr_storage_left", "Left", "storage_list", "dr_menu_storage_cursor", "$dr_menu_storage_cursor % 2 == 1", "$dr_menu_storage_cursor - 1"),
        list_nav_rule("dr_storage_right", "Right", "storage_list", "dr_menu_storage_cursor", "$dr_menu_storage_cursor % 2 == 0 && $dr_menu_storage_cursor + 1 < $dr_storage_count", "$dr_menu_storage_cursor + 1"),
        list_nav_rule("dr_storage_up", "Up", "storage_list", "dr_menu_storage_cursor", "$dr_menu_storage_cursor >= 2", "$dr_menu_storage_cursor - 2"),
        list_nav_rule("dr_storage_down", "Down", "storage_list", "dr_menu_storage_cursor", "$dr_menu_storage_cursor + 2 < $dr_storage_count && $dr_menu_storage_cursor < 11", "$dr_menu_storage_cursor + 2"),
        list_nav_rule("dr_key_item_left", "Left", "key_item_list", "dr_menu_key_item_cursor", "$dr_menu_key_item_cursor % 2 == 1", "$dr_menu_key_item_cursor - 1"),
        list_nav_rule("dr_key_item_right", "Right", "key_item_list", "dr_menu_key_item_cursor", "$dr_menu_key_item_cursor % 2 == 0 && $dr_menu_key_item_cursor + 1 < $dr_key_items_count", "$dr_menu_key_item_cursor + 1"),
        list_nav_rule("dr_key_item_up", "Up", "key_item_list", "dr_menu_key_item_cursor", "$dr_menu_key_item_cursor >= 2", "$dr_menu_key_item_cursor - 2"),
        list_nav_rule("dr_key_item_down", "Down", "key_item_list", "dr_menu_key_item_cursor", "$dr_menu_key_item_cursor + 2 < $dr_key_items_count && $dr_menu_key_item_cursor < 11", "$dr_menu_key_item_cursor + 2"),
        rule(
            "dr_list_cancel",
            action_event("Cancel"),
            vec![
                "$dr_menu_layer == 'item_list' || $dr_menu_layer == 'storage_list' || $dr_menu_layer == 'key_item_list'".into(),
            ],
            vec![
                set_layer("item_category"),
                set_string("dr_menu_feedback", ""),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_key_item_unusable_feedback",
            action_event("Confirm"),
            vec!["$dr_menu_layer == 'key_item_list'".into(), "$dr_key_items_usable != true".into()],
            vec![
                set_string("dr.menu.feedback", "You can't use this here."),
                set_string("dr_menu_feedback", "You can't use this here."),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
    ]
}

pub fn view_asset() -> ViewLayoutAsset {
    ViewLayout {
        coordinate_space: Some(gms_coordinate_space()),
        roots: vec![
            top_bar(),
            party_box(
                0,
                0.0,
                "dr_party_0_name",
                "dr_party_0_hp",
                "dr_party_0_max_hp",
            ),
            party_box(
                1,
                212.0,
                "dr_party_1_name",
                "dr_party_1_hp",
                "dr_party_1_max_hp",
            ),
            party_box(
                2,
                424.0,
                "dr_party_2_name",
                "dr_party_2_hp",
                "dr_party_2_max_hp",
            ),
            category_box(),
            item_list_box(
                "ItemListBox",
                "item_list",
                "dr_inventory_items",
                "dr_menu_item_cursor",
            ),
            item_list_box(
                "StorageListBox",
                "storage_list",
                "dr_storage_items",
                "dr_menu_storage_cursor",
            ),
            item_list_box(
                "KeyItemListBox",
                "key_item_list",
                "dr_key_items_items",
                "dr_menu_key_item_cursor",
            ),
            feedback_text(),
        ],
        ..Default::default()
    }
}

fn action_event(action: &str) -> RuleEventDef {
    RuleEventDef::ActionEvent {
        action: action.into(),
        kind: ActionEventKind::JustPressed,
    }
}

fn rule(
    id: &str,
    event: RuleEventDef,
    conditions: Vec<String>,
    actions: Vec<RuleActionDef>,
) -> RuleDef {
    RuleDef {
        id: id.into(),
        event,
        conditions,
        actions,
        modifications: Vec::new(),
        outputs: Vec::new(),
        enabled: true,
        priority: 0,
        consume_event: true,
    }
}

fn list_nav_rule(
    id: &str,
    action: &str,
    layer: &str,
    cursor: &str,
    condition: &str,
    next_value: &str,
) -> RuleDef {
    rule(
        id,
        action_event(action),
        vec![format!("$dr_menu_layer == '{layer}'"), condition.into()],
        vec![
            set_expr(cursor, next_value),
            set_string("dr.menu.feedback", ""),
            RuleActionDef::PlaySound("choice".into()),
        ],
    )
}

fn set_layer(layer: &str) -> RuleActionDef {
    RuleActionDef::SetLocalFact("dr_menu_layer".into(), LocalFactValue::Enum(layer.into()))
}

fn set_int(name: &str, value: i64) -> RuleActionDef {
    RuleActionDef::SetLocalFact(name.into(), LocalFactValue::Int(value))
}

fn set_expr(name: &str, value: &str) -> RuleActionDef {
    RuleActionDef::SetLocalFact(name.into(), LocalFactValue::Expr(value.into()))
}

fn set_string(name: &str, value: &str) -> RuleActionDef {
    RuleActionDef::SetLocalFact(name.into(), LocalFactValue::String(value.into()))
}

fn set_sub_state(state: &str) -> RuleActionDef {
    RuleActionDef::Custom {
        action_type: "SetSubState".into(),
        params: vec![("state".into(), state.into())].into_iter().collect(),
    }
}

fn gms_coordinate_space() -> CoordinateSpaceDef {
    CoordinateSpaceDef {
        axis_origin: vector2(0.0, 0.0),
        y_axis: YAxisDirectionDef::Down,
        rotation: RotationDirectionDef::CounterClockwise,
        extent: CoordinateExtentDef::Explicit((640.0, 480.0)),
    }
}

fn transform(
    x: impl Into<FloatOrExpr>,
    y: impl Into<FloatOrExpr>,
    z: f32,
) -> SerializableTransform {
    SerializableTransform {
        translation: Some(vector3(x, y, z)),
        ..Default::default()
    }
}

fn top_bar() -> ViewNodeDef {
    ViewNodeDef {
        name: "TopMenuBar".into(),
        transform: Some(transform(0.0, 0.0, 0.0)),
        view_box: Some(ViewBoxLogicDef {
            width: 640.0,
            height: 72.0,
            border_width: 3.0,
            offset: vector3(320.0, 36.0, 0.0),
            structure_file: Some("view/structures/view_box.sdf.ron".into()),
            fill_color: Some(color(0.0, 0.0, 0.0, 1.0)),
            ..Default::default()
        }),
        texts: vec![
            top_button("TopItem", "ITEM", 120.0),
            top_button("TopEquip", "EQUIP", 220.0),
            top_button("TopTalk", "TALK", 320.0),
            top_button("TopTech", "TECH", 420.0),
            top_button("TopConfig", "CONFIG", 520.0),
            TextDef {
                id: "MoneyText".into(),
                content: Some("${dr_money}".into()),
                font: "DTM-Sans".into(),
                world_scale: vector2(12.0, 12.0),
                transform: transform(520.0, 18.0, 2.0),
                ..Default::default()
            },
        ],
        children: vec![ViewNodeDef {
            name: "TopMenuCursor".into(),
            visible_when: Some("$dr_menu_layer == 'top_menu'".into()),
            sprite: Some(SpriteDef {
                visual: Visual("common/view/dr_heart".into()),
                color: Some(red()),
                transform: Some(transform(94.0, 28.0, 4.0)),
                pivot: Some(vector2(0.0, 0.0)),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn top_button(id: &str, label: &str, x: f32) -> TextDef {
    TextDef {
        id: id.into(),
        content: Some(label.into()),
        font: "DTM-Sans".into(),
        world_scale: vector2(12.0, 12.0),
        color: if label == "ITEM" {
            white()
        } else {
            color(0.45, 0.45, 0.45, 1.0)
        },
        transform: transform(x, 18.0, 2.0),
        ..Default::default()
    }
}

fn party_box(index: usize, xchunk: f32, name: &str, hp: &str, max_hp: &str) -> ViewNodeDef {
    ViewNodeDef {
        name: format!("PartyBox{index}"),
        visible_when: Some(format!("$dr_party_count > {index}")),
        transform: Some(transform(xchunk, 350.0, 0.0)),
        view_box: Some(ViewBoxLogicDef {
            width: 212.0,
            height: 82.0,
            border_width: 3.0,
            offset: vector3(106.0, 41.0, 0.0),
            structure_file: Some("view/structures/view_box.sdf.ron".into()),
            fill_color: Some(color(0.0, 0.0, 0.0, 1.0)),
            ..Default::default()
        }),
        texts: vec![
            TextDef {
                id: format!("PartyName{index}"),
                content: Some(format!("${{{name}}}")),
                font: "DTM-Sans".into(),
                world_scale: vector2(11.0, 11.0),
                transform: transform(51.0, 18.0, 2.0),
                ..Default::default()
            },
            TextDef {
                id: format!("PartyHp{index}"),
                content: Some(format!("HP  ${{{hp}}}/${{{max_hp}}}")),
                font: "DTM-Sans".into(),
                world_scale: vector2(9.0, 9.0),
                transform: transform(109.0, 36.0, 2.0),
                ..Default::default()
            },
        ],
        children: vec![ViewNodeDef {
            name: format!("PartyHpBar{index}"),
            transform: Some(transform(128.0, 55.0, 1.0)),
            view_box: Some(ViewBoxLogicDef {
                width: 75.0,
                height: 9.0,
                border_width: 0.0,
                offset: vector3(37.5, 4.5, 0.0),
                fill_color: Some(color(1.0, 1.0, 0.0, 1.0)),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn category_box() -> ViewNodeDef {
    ViewNodeDef {
        name: "ItemCategoryBox".into(),
        visible_when: Some(
            "$dr_menu_layer == 'item_category' || $dr_menu_layer == 'item_list' || $dr_menu_layer == 'storage_list' || $dr_menu_layer == 'key_item_list'"
                .into(),
        ),
        transform: Some(transform(20.0, 88.0, 0.0)),
        texts: vec![
            category_label("CategoryItem", "ITEM", 88.0, "$dr_menu_category_index == 0"),
            category_label("CategoryStorage", "STORAGE", 240.0, "$dr_menu_category_index == 1"),
            category_label("CategoryKeyItem", "KEYITEM", 430.0, "$dr_menu_category_index == 2"),
        ],
        children: vec![ViewNodeDef {
            name: "CategoryCursor".into(),
            visible_when: Some("$dr_menu_layer == 'item_category'".into()),
            sprite: Some(SpriteDef {
                visual: Visual("common/view/dr_heart".into()),
                color: Some(red()),
                transform: Some(transform(
                    expression("60.0 + 170.0 * $dr_menu_category_index"),
                    18.0,
                    4.0,
                )),
                pivot: Some(vector2(0.0, 0.0)),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn category_label(id: &str, label: &str, x: f32, selected: &str) -> TextDef {
    TextDef {
        id: id.into(),
        content: Some(label.into()),
        font: "DTM-Sans".into(),
        world_scale: vector2(12.0, 12.0),
        transform: transform(x, 12.0, 2.0),
        conditional_style: Some(ConditionalStyleDef {
            condition: selected.into(),
            color: color(1.0, 1.0, 0.0, 1.0),
        }),
        ..Default::default()
    }
}

fn item_list_box(name: &str, layer: &str, source: &str, cursor: &str) -> ViewNodeDef {
    ViewNodeDef {
        name: name.into(),
        visible_when: Some(format!("$dr_menu_layer == '{layer}'")),
        transform: Some(transform(54.0, 132.0, 0.0)),
        view_box: Some(ViewBoxLogicDef {
            width: 532.0,
            height: 184.0,
            border_width: 3.0,
            offset: vector3(266.0, 92.0, 0.0),
            structure_file: Some("view/structures/view_box.sdf.ron".into()),
            fill_color: Some(color(0.0, 0.0, 0.0, 1.0)),
            ..Default::default()
        }),
        texts: vec![TextDef {
            id: format!("{name}Items"),
            content: Some(format!("{{{{data:{source}}}}}")),
            font: "DTM-Sans".into(),
            world_scale: vector2(12.0, 12.0),
            transform: transform(42.0, 28.0, 2.0),
            line_height: Some(1.5),
            ..Default::default()
        }],
        children: vec![ViewNodeDef {
            name: format!("{name}Cursor"),
            sprite: Some(SpriteDef {
                visual: Visual("common/view/dr_heart".into()),
                color: Some(red()),
                transform: Some(transform(
                    expression(format!("22.0 + 240.0 * ({cursor} % 2)")),
                    expression(format!("28.0 + 24.0 * floor({cursor} / 2)")),
                    4.0,
                )),
                pivot: Some(vector2(0.0, 0.0)),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn feedback_text() -> ViewNodeDef {
    ViewNodeDef {
        name: "DarkMenuFeedback".into(),
        visible_when: Some("$dr_menu_feedback != ''".into()),
        texts: vec![TextDef {
            id: "FeedbackText".into(),
            content: Some("${dr_menu_feedback}".into()),
            font: "DTM-Sans".into(),
            world_scale: vector2(11.0, 11.0),
            transform: transform(64.0, 326.0, 5.0),
            color: color(1.0, 1.0, 0.0, 1.0),
            ..Default::default()
        }],
        ..Default::default()
    }
}
