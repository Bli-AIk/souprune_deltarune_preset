//! Shared builders for Deltarune overworld menu assets.
//!
//! Deltarune overworld 菜单资源的共享构建器。

use souprune_schema::fre::*;
use souprune_schema::sequence::SequenceAsset;
use souprune_schema::view::*;
use std::collections::HashMap;

const NORMAL_STATE: &str = "Normal";
const DARK_MENU_STATE: &str = "DarkMenu";
const MAINBIG_TEXT_SCALE: f32 = 26.5;
const MENU_SLOT_COUNT: usize = 12;
pub const OPEN_TWEEN_SEQUENCE_PATH: &str = "overworld/sequences/dark_menu_open.sequence.ron";

pub fn fre_asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: menu_enums(),
        facts: menu_facts(),
        rules: menu_rules(),
    }
}

pub fn global_facts_asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Global,
        enums: menu_enums(),
        facts: menu_facts(),
        rules: Vec::new(),
    }
}

pub fn open_tween_sequence_asset() -> SequenceAsset {
    super::dark_menu_open::asset()
}

pub fn run_open_tween_sequence_action() -> RuleActionDef {
    RuleActionDef::Custom {
        action_type: "RunSequence".into(),
        params: vec![("path".into(), OPEN_TWEEN_SEQUENCE_PATH.into())]
            .into_iter()
            .collect(),
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
        ("dr.menu.layer_id".into(), FactValueDef::Int(0)),
        ("dr_menu_layer_id".into(), FactValueDef::Int(0)),
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
        ("dr.menu.has_feedback".into(), FactValueDef::Bool(false)),
        ("dr_menu_has_feedback".into(), FactValueDef::Bool(false)),
        ("dr.money".into(), FactValueDef::Int(375)),
        ("dr_money".into(), FactValueDef::Int(375)),
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
            FactValueDef::StringList(menu_slots(&["Dark Candy", "ReviveMint", "Glowshard"])),
        ),
        (
            "dr_inventory_items".into(),
            FactValueDef::StringList(menu_slots(&["Dark Candy", "ReviveMint", "Glowshard"])),
        ),
        ("dr.inventory.count".into(), FactValueDef::Int(3)),
        ("dr_inventory_count".into(), FactValueDef::Int(3)),
        (
            "dr.storage.items".into(),
            FactValueDef::StringList(menu_slots(&["Manual"])),
        ),
        (
            "dr_storage_items".into(),
            FactValueDef::StringList(menu_slots(&["Manual"])),
        ),
        ("dr.storage.count".into(), FactValueDef::Int(1)),
        ("dr_storage_count".into(), FactValueDef::Int(1)),
        (
            "dr.key_items.items".into(),
            FactValueDef::StringList(menu_slots(&["Cell Phone"])),
        ),
        (
            "dr_key_items_items".into(),
            FactValueDef::StringList(menu_slots(&["Cell Phone"])),
        ),
        ("dr.key_items.count".into(), FactValueDef::Int(1)),
        ("dr_key_items_count".into(), FactValueDef::Int(1)),
        ("dr.key_items.usable".into(), FactValueDef::Bool(false)),
        ("dr_key_items_usable".into(), FactValueDef::Bool(false)),
    ]
    .into_iter()
    .collect()
}

fn menu_enums() -> HashMap<String, Vec<String>> {
    let variants = vec![
        "top_menu".into(),
        "item_category".into(),
        "item_list".into(),
        "storage_list".into(),
        "key_item_list".into(),
    ];
    vec![
        ("dr.menu.layer".into(), variants.clone()),
        ("dr_menu_layer".into(), variants),
    ]
    .into_iter()
    .collect()
}

fn menu_slots(items: &[&str]) -> Vec<String> {
    let mut slots = items
        .iter()
        .map(|item| (*item).to_string())
        .collect::<Vec<_>>();
    slots.resize(MENU_SLOT_COUNT, String::new());
    slots
}

fn menu_rules() -> Vec<RuleDef> {
    vec![
        rule(
            "dr_open_dark_menu",
            input_event("Menu"),
            vec![
                "$state:sequence_sub_state == 'Normal'".into(),
                "$dialogue:active != true".into(),
            ],
            vec![
                set_layer("top_menu"),
                set_layer_id("top_menu"),
                set_int("dr_menu_top_index", 0),
                set_int("dr_menu_category_index", 0),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                set_sub_state(DARK_MENU_STATE),
                run_open_tween_sequence_action(),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_top_confirm_item",
            input_event("Confirm"),
            vec!["$dr_menu_layer == 'top_menu'".into(), "$dr_menu_top_index == 0".into()],
            vec![
                set_layer("item_category"),
                set_layer_id("item_category"),
                set_int("dr_menu_category_index", 0),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_top_cancel",
            input_event("Cancel"),
            vec!["$dr_menu_layer == 'top_menu'".into()],
            vec![set_sub_state(NORMAL_STATE), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_left_wrap",
            input_event("Left"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index <= 0".into()],
            vec![set_int("dr_menu_category_index", 2), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_left",
            input_event("Left"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index > 0".into()],
            vec![
                set_expr("dr_menu_category_index", "$dr_menu_category_index - 1"),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_category_right_wrap",
            input_event("Right"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index >= 2".into()],
            vec![set_int("dr_menu_category_index", 0), RuleActionDef::PlaySound("choice".into())],
        ),
        rule(
            "dr_category_right",
            input_event("Right"),
            vec!["$dr_menu_layer == 'item_category'".into(), "$dr_menu_category_index < 2".into()],
            vec![
                set_expr("dr_menu_category_index", "$dr_menu_category_index + 1"),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_category_confirm_item",
            input_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 0".into(),
            ],
            vec![
                set_layer("item_list"),
                set_layer_id("item_list"),
                set_int("dr_menu_item_cursor", 0),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_confirm_storage",
            input_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 1".into(),
            ],
            vec![
                set_layer("storage_list"),
                set_layer_id("storage_list"),
                set_int("dr_menu_storage_cursor", 0),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_confirm_key_item",
            input_event("Confirm"),
            vec![
                "$dr_menu_layer == 'item_category'".into(),
                "$dr_menu_category_index == 2".into(),
            ],
            vec![
                set_layer("key_item_list"),
                set_layer_id("key_item_list"),
                set_int("dr_menu_key_item_cursor", 0),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                RuleActionDef::PlaySound("confirm".into()),
            ],
        ),
        rule(
            "dr_category_cancel",
            input_event("Cancel"),
            vec!["$dr_menu_layer == 'item_category'".into()],
            vec![
                set_layer("top_menu"),
                set_layer_id("top_menu"),
                RuleActionDef::PlaySound("choice".into()),
            ],
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
            input_event("Cancel"),
            vec![
                "$dr_menu_layer == 'item_list' || $dr_menu_layer == 'storage_list' || $dr_menu_layer == 'key_item_list'".into(),
            ],
            vec![
                set_layer("item_category"),
                set_layer_id("item_category"),
                set_string("dr_menu_feedback", ""),
                set_bool("dr_menu_has_feedback", false),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
        rule(
            "dr_key_item_unusable_feedback",
            input_event("Confirm"),
            vec!["$dr_menu_layer == 'key_item_list'".into(), "$dr_key_items_usable != true".into()],
            vec![
                set_string("dr.menu.feedback", "You can't use this here."),
                set_string("dr_menu_feedback", "You can't use this here."),
                set_bool("dr.menu.has_feedback", true),
                set_bool("dr_menu_has_feedback", true),
                RuleActionDef::PlaySound("choice".into()),
            ],
        ),
    ]
}

pub fn view_asset() -> ViewLayoutAsset {
    let mut children = vec![top_mask(), bottom_mask()];
    children.push(top_menu_group(top_menu_nodes()));
    children.push(party_group(vec![
        party_box(
            0,
            0.0,
            "common/dark_menu/head_kris.png",
            "common/dark_menu/name_kris.png",
            90,
            90,
            color(0.0, 1.0, 1.0, 1.0),
        ),
        party_box(
            1,
            213.0,
            "common/dark_menu/head_susie.png",
            "common/dark_menu/name_susie.png",
            110,
            110,
            color(1.0, 0.0, 1.0, 1.0),
        ),
        party_box(
            2,
            426.0,
            "common/dark_menu/head_ralsei.png",
            "common/dark_menu/name_ralsei.png",
            70,
            70,
            color(0.0, 1.0, 0.0, 1.0),
        ),
    ]));
    children.extend([
        category_box(),
        item_list_box(
            "ItemListBox",
            2,
            "dr_inventory_items",
            "dr_inventory_count",
            "dr_menu_item_cursor",
        ),
        item_list_box(
            "StorageListBox",
            3,
            "dr_storage_items",
            "dr_storage_count",
            "dr_menu_storage_cursor",
        ),
        item_list_box(
            "KeyItemListBox",
            4,
            "dr_key_items_items",
            "dr_key_items_count",
            "dr_menu_key_item_cursor",
        ),
        feedback_text(),
    ]);

    ViewLayout {
        coordinate_space: Some(gms_coordinate_space()),
        roots: vec![dark_menu_canvas(children)],
        ..Default::default()
    }
}

fn input_event(action: &str) -> RuleEventDef {
    crate::support::input_event(action)
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
        input_event(action),
        vec![format!("$dr_menu_layer == '{layer}'"), condition.into()],
        vec![
            set_expr(cursor, next_value),
            set_string("dr.menu.feedback", ""),
            set_string("dr_menu_feedback", ""),
            set_bool("dr.menu.has_feedback", false),
            set_bool("dr_menu_has_feedback", false),
            RuleActionDef::PlaySound("choice".into()),
        ],
    )
}

fn set_layer(layer: &str) -> RuleActionDef {
    RuleActionDef::SetLocalFact("dr_menu_layer".into(), LocalFactValue::Enum(layer.into()))
}

fn set_layer_id(layer: &str) -> RuleActionDef {
    set_int("dr_menu_layer_id", layer_id(layer))
}

fn layer_id(layer: &str) -> i64 {
    match layer {
        "top_menu" => 0,
        "item_category" => 1,
        "item_list" => 2,
        "storage_list" => 3,
        "key_item_list" => 4,
        _ => 0,
    }
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

fn set_bool(name: &str, value: bool) -> RuleActionDef {
    RuleActionDef::SetLocalFact(name.into(), LocalFactValue::Bool(value))
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

fn transform_scaled(
    x: impl Into<FloatOrExpr>,
    y: impl Into<FloatOrExpr>,
    z: f32,
    scale_x: impl Into<FloatOrExpr>,
    scale_y: impl Into<FloatOrExpr>,
) -> SerializableTransform {
    SerializableTransform {
        translation: Some(vector3(x, y, z)),
        scale: Some(vector3(scale_x, scale_y, 1.0)),
        ..Default::default()
    }
}

fn black() -> SerializableColor {
    color(0.0, 0.0, 0.0, 1.0)
}

fn dark_menu_purple() -> SerializableColor {
    color(0.2, 0.125, 0.2, 1.0)
}

fn dark_menu_canvas(children: Vec<ViewNodeDef>) -> ViewNodeDef {
    ViewNodeDef {
        name: "DarkMenuCanvas".into(),
        transform: Some(transform_scaled(0.0, 0.0, 0.0, 0.5, 0.5)),
        children,
        ..Default::default()
    }
}

fn top_mask() -> ViewNodeDef {
    rect_node("DarkMenuTopMask", -10.0, -10.0, 0.5, 660.0, 10.0, black())
}

fn bottom_mask() -> ViewNodeDef {
    rect_node(
        "DarkMenuBottomMask",
        -10.0,
        479.0,
        0.5,
        660.0,
        21.0,
        black(),
    )
}

fn top_menu_group(children: Vec<ViewNodeDef>) -> ViewNodeDef {
    ViewNodeDef {
        name: "DarkMenuTopGroup".into(),
        transform: Some(transform(0.0, -80.0, 0.0)),
        children,
        ..Default::default()
    }
}

fn party_group(children: Vec<ViewNodeDef>) -> ViewNodeDef {
    ViewNodeDef {
        name: "DarkMenuPartyGroup".into(),
        transform: Some(transform(0.0, 60.0, 0.0)),
        children,
        ..Default::default()
    }
}

fn top_menu_nodes() -> Vec<ViewNodeDef> {
    vec![
        sprite_node(
            "DarkMenuTitle",
            "common/dark_menu/desc_item.png",
            20.0,
            24.0,
            2.0,
            2.0,
            2.0,
            white(),
        ),
        sprite_node(
            "DarkMenuItemButton",
            "common/dark_menu/darkitem_selected.png",
            120.0,
            20.0,
            2.0,
            2.0,
            2.0,
            white(),
        ),
        sprite_node(
            "DarkMenuEquipButton",
            "common/dark_menu/darkequip_inactive.png",
            220.0,
            20.0,
            2.0,
            2.0,
            2.0,
            white(),
        ),
        sprite_node(
            "DarkMenuTechButton",
            "common/dark_menu/darktech_inactive.png",
            320.0,
            20.0,
            2.0,
            2.0,
            2.0,
            white(),
        ),
        sprite_node(
            "DarkMenuConfigButton",
            "common/dark_menu/darkconfig_inactive.png",
            420.0,
            20.0,
            2.0,
            2.0,
            2.0,
            white(),
        ),
        ViewNodeDef {
            name: "DarkMenuMoney".into(),
            texts: vec![TextDef {
                id: "MoneyText".into(),
                content: Some("D$ {$dr_money}".into()),
                font: "DTM-Sans".into(),
                world_scale: vector2(MAINBIG_TEXT_SCALE, MAINBIG_TEXT_SCALE),
                transform: transform(520.0, 19.0, 2.0),
                ..Default::default()
            }],
            ..Default::default()
        },
    ]
}

fn party_box(
    index: usize,
    xchunk: f32,
    head: &str,
    name: &str,
    hp: i64,
    max_hp: i64,
    hp_color: SerializableColor,
) -> ViewNodeDef {
    let mut children = vec![
        rect_node("PartyAccent", 0.0, 0.0, 1.0, 212.0, 3.0, dark_menu_purple()),
        rect_node("PartyPanel", 2.0, 2.0, 1.0, 208.0, 34.0, black()),
        sprite_node("PartyHead", head, 13.0, 13.0, 3.0, 1.0, 1.0, white()),
        sprite_node("PartyName", name, 51.0, 16.0, 3.0, 1.0, 1.0, white()),
        sprite_node(
            "PartyHpLabel",
            "common/dark_menu/hpname.png",
            109.0,
            24.0,
            3.0,
            1.0,
            1.0,
            white(),
        ),
        sprite_node(
            "PartyHpSlash",
            "common/dark_menu/hpslash.png",
            159.0,
            9.0,
            3.0,
            1.0,
            1.0,
            white(),
        ),
    ];
    children.extend(number_sprites("PartyHpCurrent", hp, 160.0, 11.0, 4.0));
    children.extend(number_sprites("PartyHpMax", max_hp, 205.0, 11.0, 4.0));
    children.extend([
        rect_node(
            "PartyHpBarBack",
            128.0,
            24.0,
            2.0,
            75.0,
            8.0,
            color(0.5, 0.0, 0.0, 1.0),
        ),
        rect_node(
            "PartyHpBarFill",
            128.0,
            24.0,
            3.0,
            hp_bar_width(hp, max_hp),
            8.0,
            hp_color,
        ),
    ]);

    ViewNodeDef {
        name: format!("PartyBox{index}"),
        visible_when: Some(format!("$dr_party_count > {index}")),
        transform: Some(transform(xchunk, 417.0, 0.0)),
        children,
        ..Default::default()
    }
}

fn hp_bar_width(hp: i64, max_hp: i64) -> f32 {
    if max_hp <= 0 {
        0.0
    } else {
        75.0 * hp.max(0) as f32 / max_hp as f32
    }
}

fn number_sprites(prefix: &str, value: i64, right_x: f32, y: f32, z: f32) -> Vec<ViewNodeDef> {
    let text = value.max(0).to_string();
    let width = number_text_width(text.len());
    let start_x = right_x - width;
    text.chars()
        .enumerate()
        .map(|(index, digit)| {
            let digit = digit.to_digit(10).unwrap_or(0);
            sprite_node(
                &format!("{prefix}Digit{index}"),
                &format!("common/dark_menu/number_{digit}.png"),
                start_x + (index as f32 * 8.0),
                y,
                z,
                1.0,
                1.0,
                white(),
            )
        })
        .collect()
}

fn number_text_width(len: usize) -> f32 {
    len as f32 * 8.0
}

fn sprite_node(
    name: &str,
    visual: &str,
    x: impl Into<FloatOrExpr>,
    y: impl Into<FloatOrExpr>,
    z: f32,
    scale_x: impl Into<FloatOrExpr>,
    scale_y: impl Into<FloatOrExpr>,
    tint: SerializableColor,
) -> ViewNodeDef {
    ViewNodeDef {
        name: name.into(),
        sprite: Some(SpriteDef {
            visual: Visual(visual.into()),
            color: Some(tint),
            transform: Some(transform_scaled(x, y, z, scale_x, scale_y)),
            pivot: Some(vector2(0.0, 0.0)),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn rect_node(
    name: &str,
    x: impl Into<FloatOrExpr>,
    y: impl Into<FloatOrExpr>,
    z: f32,
    width: impl Into<FloatOrExpr>,
    height: impl Into<FloatOrExpr>,
    tint: SerializableColor,
) -> ViewNodeDef {
    sprite_node(
        name,
        "procedural://white_pixel",
        x,
        y,
        z,
        width,
        height,
        tint,
    )
}

fn category_box() -> ViewNodeDef {
    ViewNodeDef {
        name: "ItemCategoryBox".into(),
        visible_when: Some("$dr_menu_layer_id >= 1 && $dr_menu_layer_id <= 4".into()),
        view_box: Some(ViewBoxLogicDef {
            width: 520.0,
            height: 290.0,
            border_width: 3.0,
            offset: vector3(320.0, 225.0, 0.0),
            structure_file: Some("view/structures/view_box.sdf.ron".into()),
            fill_color: Some(black()),
            ..Default::default()
        }),
        texts: vec![
            category_label("CategoryUse", "USE", 180.0, "$dr_menu_category_index == 0"),
            category_label(
                "CategoryToss",
                "TOSS",
                300.0,
                "$dr_menu_category_index == 1",
            ),
            category_label("CategoryKey", "KEY", 420.0, "$dr_menu_category_index == 2"),
        ],
        children: vec![ViewNodeDef {
            name: "CategoryCursor".into(),
            visible_when: Some("$dr_menu_layer_id == 1".into()),
            sprite: Some(SpriteDef {
                visual: Visual("common/view/dr_heart".into()),
                color: Some(red()),
                transform: Some(transform(
                    expression("155.0 + 120.0 * $dr_menu_category_index"),
                    120.0,
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
        world_scale: vector2(MAINBIG_TEXT_SCALE, MAINBIG_TEXT_SCALE),
        transform: transform(x, 110.0, 2.0),
        conditional_style: Some(ConditionalStyleDef {
            condition: selected.into(),
            color: color(1.0, 0.65, 0.0, 1.0),
        }),
        ..Default::default()
    }
}

fn item_entry_texts(name: &str, source: &str, count: &str) -> Vec<TextDef> {
    (0..12)
        .map(|index| {
            let column = index % 2;
            let row = index / 2;
            TextDef {
                id: format!("{name}Item{index}"),
                content: Some(format!("{{${source}[{index}]}}")),
                font: "DTM-Sans".into(),
                world_scale: vector2(MAINBIG_TEXT_SCALE, MAINBIG_TEXT_SCALE),
                transform: transform(
                    146.0 + (210.0 * column as f32),
                    152.0 + (30.0 * row as f32),
                    2.0,
                ),
                visible_when: Some(format!("${count} > {index}")),
                ..Default::default()
            }
        })
        .collect()
}

fn item_list_box(
    name: &str,
    layer_id: i64,
    source: &str,
    count: &str,
    cursor: &str,
) -> ViewNodeDef {
    ViewNodeDef {
        name: name.into(),
        visible_when: Some(format!("$dr_menu_layer_id == {layer_id}")),
        texts: item_entry_texts(name, source, count),
        children: vec![ViewNodeDef {
            name: format!("{name}Cursor"),
            sprite: Some(SpriteDef {
                visual: Visual("common/view/dr_heart".into()),
                color: Some(red()),
                transform: Some(transform(
                    expression(format!("120.0 + 210.0 * (${cursor} % 2)")),
                    expression(format!("162.0 + 30.0 * floor(${cursor} / 2)")),
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
        visible_when: Some("$dr_menu_has_feedback == true".into()),
        texts: vec![TextDef {
            id: "FeedbackText".into(),
            content: Some("{$dr_menu_feedback}".into()),
            font: "DTM-Sans".into(),
            world_scale: vector2(MAINBIG_TEXT_SCALE, MAINBIG_TEXT_SCALE),
            transform: transform(64.0, 326.0, 5.0),
            color: color(1.0, 1.0, 0.0, 1.0),
            ..Default::default()
        }],
        ..Default::default()
    }
}
