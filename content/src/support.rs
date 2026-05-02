//! Shared helper modules for this content guest.
//!
//! 当前内容 guest 的共享辅助模块。

pub mod dark_menu;
mod dark_menu_open;

#[cfg(test)]
mod tests {
    use super::dark_menu;
    use souprune_schema::fre::{ActionEventKind, FactValueDef, RuleActionDef, RuleEventDef};
    use souprune_schema::sequence::{Chapter, ElementSelector, TweenTarget};
    use souprune_schema::view::{
        CoordinateExtentDef, RotationDirectionDef, ViewNodeDef, YAxisDirectionDef,
    };
    use souprune_schema::{Val, Vec3Tuple};

    #[test]
    fn dark_menu_facts_define_party_inventory_and_layers() {
        let asset = dark_menu::fre_asset();

        assert!(matches!(
            asset.facts.get("dr.menu.layer"),
            Some(FactValueDef::Enum(layer)) if layer == "top_menu"
        ));
        assert!(
            asset
                .enums
                .get("dr.menu.layer")
                .is_some_and(|variants| variants.iter().any(|variant| variant == "top_menu"))
        );
        assert!(
            asset
                .enums
                .get("dr_menu_layer")
                .is_some_and(|variants| variants.iter().any(|variant| variant == "top_menu"))
        );
        let global_asset = dark_menu::global_facts_asset();
        assert!(
            global_asset
                .enums
                .get("dr.menu.layer")
                .is_some_and(|variants| variants.iter().any(|variant| variant == "top_menu"))
        );
        assert!(
            global_asset
                .enums
                .get("dr_menu_layer")
                .is_some_and(|variants| variants.iter().any(|variant| variant == "top_menu"))
        );
        assert!(matches!(
            asset.facts.get("dr.party.count"),
            Some(FactValueDef::Int(3))
        ));
        assert!(matches!(
            asset.facts.get("dr.inventory.items"),
            Some(FactValueDef::StringList(items)) if items.len() >= 12
        ));
        assert!(matches!(
            asset.facts.get("dr_inventory_items"),
            Some(FactValueDef::StringList(items)) if items.len() >= 12
        ));
        assert!(matches!(
            asset.facts.get("dr_storage_items"),
            Some(FactValueDef::StringList(items)) if items.len() >= 12
        ));
        assert!(matches!(
            asset.facts.get("dr_key_items_items"),
            Some(FactValueDef::StringList(items)) if items.len() >= 12
        ));
        assert!(matches!(
            asset.facts.get("dr.key_items.usable"),
            Some(FactValueDef::Bool(false))
        ));
    }

    #[test]
    fn dark_menu_rules_enter_categories_and_return_to_normal() {
        let asset = dark_menu::fre_asset();

        let open_rules = asset
            .rules
            .iter()
            .filter(|rule| {
                matches!(
                    rule.event,
                    RuleEventDef::ActionEvent {
                        ref action,
                        kind: ActionEventKind::JustPressed,
                    } if action == "Menu"
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            open_rules.len(),
            1,
            "dark menu should have a single Menu open rule so no same-priority rule consumes the event first"
        );

        let has_open_rule = open_rules.iter().any(|rule| {
            rule.id == "dr_open_dark_menu"
                && rule.actions.iter().any(|action| {
                    matches!(action, RuleActionDef::Custom { action_type, params } if action_type == "SetSubState" && params.get("state").is_some_and(|state| state == "DarkMenu"))
                })
                && rule.actions.iter().any(|action| {
                    matches!(action, RuleActionDef::Custom { action_type, params } if action_type == "RunSequence" && params.get("path").is_some_and(|path| path == dark_menu::OPEN_TWEEN_SEQUENCE_PATH))
                })
        });
        assert!(has_open_rule);

        let has_key_item_feedback = asset.rules.iter().any(|rule| {
            rule.id == "dr_key_item_unusable_feedback"
                && rule.actions.iter().any(|action| {
                    matches!(action, RuleActionDef::SetLocalFact(name, _) if name == "dr.menu.feedback")
                })
        });
        assert!(has_key_item_feedback);
    }

    #[test]
    fn dark_menu_view_uses_deltarune_original_top_menu_and_party_hud_coordinates() {
        let layout = dark_menu::view_asset();
        let space = layout
            .coordinate_space
            .expect("DR menu should define an imported coordinate space");

        assert_eq!(space.y_axis, YAxisDirectionDef::Down);
        assert_eq!(space.rotation, RotationDirectionDef::CounterClockwise);
        assert_eq!(space.extent, CoordinateExtentDef::Explicit((640.0, 480.0)));

        let canvas = find_root(&layout.roots, "DarkMenuCanvas");
        let canvas_transform = canvas.transform.as_ref().expect("canvas transform");
        let canvas_translation = canvas_transform
            .translation
            .as_ref()
            .expect("canvas translation");
        assert_eq!(canvas_translation.0.as_static(), Some(&0.0));
        assert_eq!(canvas_translation.1.as_static(), Some(&0.0));
        let canvas_scale = canvas_transform.scale.as_ref().expect("canvas scale");
        assert_eq!(canvas_scale.0.as_static(), Some(&0.5));
        assert_eq!(canvas_scale.1.as_static(), Some(&0.5));

        let title = find_node(canvas, "DarkMenuTitle");
        assert_sprite(title, "common/dark_menu/desc_item.png", 20.0, 24.0, 2.0);
        assert_sprite(
            find_node(canvas, "DarkMenuItemButton"),
            "common/dark_menu/darkitem_selected.png",
            120.0,
            20.0,
            2.0,
        );
        assert_sprite(
            find_node(canvas, "DarkMenuEquipButton"),
            "common/dark_menu/darkequip_inactive.png",
            220.0,
            20.0,
            2.0,
        );
        assert_sprite(
            find_node(canvas, "DarkMenuTechButton"),
            "common/dark_menu/darktech_inactive.png",
            320.0,
            20.0,
            2.0,
        );
        assert_sprite(
            find_node(canvas, "DarkMenuConfigButton"),
            "common/dark_menu/darkconfig_inactive.png",
            420.0,
            20.0,
            2.0,
        );

        let money = find_node(canvas, "DarkMenuMoney");
        assert_eq!(money.texts[0].content.as_deref(), Some("D$ {$dr_money}"));
        assert_eq!(money.texts[0].world_scale.0.as_static(), Some(&26.5));
        let money_transform = &money.texts[0].transform;
        let money_translation = money_transform
            .translation
            .as_ref()
            .expect("money translation");
        assert_eq!(money_translation.0.as_static(), Some(&520.0));
        assert_eq!(money_translation.1.as_static(), Some(&19.0));

        assert_party_box(
            find_node(canvas, "PartyBox0"),
            0.0,
            "common/dark_menu/head_kris.png",
            "common/dark_menu/name_kris.png",
            "common/dark_menu/number_9.png",
            144.0,
        );
        assert_party_box(
            find_node(canvas, "PartyBox1"),
            213.0,
            "common/dark_menu/head_susie.png",
            "common/dark_menu/name_susie.png",
            "common/dark_menu/number_1.png",
            136.0,
        );
        assert_party_box(
            find_node(canvas, "PartyBox2"),
            426.0,
            "common/dark_menu/head_ralsei.png",
            "common/dark_menu/name_ralsei.png",
            "common/dark_menu/number_7.png",
            144.0,
        );

        let top_group = find_node(canvas, "DarkMenuTopGroup");
        let top_group_transform = top_group.transform.as_ref().expect("top group transform");
        let top_group_translation = top_group_transform
            .translation
            .as_ref()
            .expect("top group translation");
        assert_eq!(top_group_translation.1.as_static(), Some(&-80.0));

        let party_group = find_node(canvas, "DarkMenuPartyGroup");
        let party_group_transform = party_group.transform.as_ref().expect("party group transform");
        let party_group_translation = party_group_transform
            .translation
            .as_ref()
            .expect("party group translation");
        assert_eq!(party_group_translation.1.as_static(), Some(&60.0));
    }

    #[test]
    fn dark_menu_item_cursor_expressions_reference_fre_facts() {
        let layout = dark_menu::view_asset();
        let canvas = find_root(&layout.roots, "DarkMenuCanvas");

        assert_cursor_exprs(
            find_node(canvas, "ItemListBoxCursor"),
            "$dr_menu_item_cursor",
        );
        assert_cursor_exprs(
            find_node(canvas, "StorageListBoxCursor"),
            "$dr_menu_storage_cursor",
        );
        assert_cursor_exprs(
            find_node(canvas, "KeyItemListBoxCursor"),
            "$dr_menu_key_item_cursor",
        );
    }

    #[test]
    fn dark_menu_open_sequence_replays_deltarune_original_tp_bp_keyframes() {
        let sequence = dark_menu::open_tween_sequence_asset();

        assert!(
            matches!(sequence.chapters.get(1), Some(Chapter::Wait(seconds)) if (*seconds - (1.0 / 30.0)).abs() < f32::EPSILON),
            "opening sequence should wait one frame after DarkMenu state so spawned view elements exist"
        );
        assert_eq!(
            first_position_y(&sequence.chapters, "DarkMenuTopGroup"),
            Some(50.0)
        );
        assert_eq!(
            first_position_y(&sequence.chapters, "DarkMenuPartyGroup"),
            Some(-30.0)
        );
        assert_eq!(
            first_scale_y(&sequence.chapters, "DarkMenuTopMask"),
            Some(40.0)
        );
        assert_eq!(
            first_position_y(&sequence.chapters, "DarkMenuBottomMask"),
            Some(-449.0)
        );
        assert_eq!(
            first_scale_y(&sequence.chapters, "DarkMenuBottomMask"),
            Some(51.0)
        );

        assert_eq!(
            last_position_y(&sequence.chapters, "DarkMenuTopGroup"),
            Some(0.0)
        );
        assert_eq!(
            last_position_y(&sequence.chapters, "DarkMenuPartyGroup"),
            Some(0.0)
        );
        assert_eq!(
            last_scale_y(&sequence.chapters, "DarkMenuTopMask"),
            Some(90.0)
        );
        assert_eq!(
            last_position_y(&sequence.chapters, "DarkMenuBottomMask"),
            Some(-419.0)
        );
        assert_eq!(
            last_scale_y(&sequence.chapters, "DarkMenuBottomMask"),
            Some(81.0)
        );
    }

    fn find_root<'a>(roots: &'a [ViewNodeDef], name: &str) -> &'a ViewNodeDef {
        roots
            .iter()
            .find(|node| node.name == name)
            .unwrap_or_else(|| panic!("view node {name} should exist"))
    }

    fn find_node<'a>(root: &'a ViewNodeDef, name: &str) -> &'a ViewNodeDef {
        if root.name == name {
            return root;
        }
        root.children
            .iter()
            .find_map(|child| find_node_optional(child, name))
            .unwrap_or_else(|| panic!("view node {name} should exist"))
    }

    fn find_node_optional<'a>(root: &'a ViewNodeDef, name: &str) -> Option<&'a ViewNodeDef> {
        if root.name == name {
            return Some(root);
        }
        root.children
            .iter()
            .find_map(|child| find_node_optional(child, name))
    }

    fn assert_sprite(node: &ViewNodeDef, visual: &str, x: f32, y: f32, scale: f32) {
        let sprite = node.sprite.as_ref().expect("node should have sprite");
        assert_eq!(sprite.visual.0, visual);
        let transform = sprite.transform.as_ref().expect("sprite transform");
        let translation = transform.translation.as_ref().expect("sprite translation");
        assert_eq!(translation.0.as_static(), Some(&x));
        assert_eq!(translation.1.as_static(), Some(&y));
        let actual_scale = transform.scale.as_ref().expect("sprite scale");
        assert_eq!(actual_scale.0.as_static(), Some(&scale));
        assert_eq!(actual_scale.1.as_static(), Some(&scale));
        assert_eq!(sprite.pivot.as_ref().and_then(|pivot| pivot.0.as_static()), Some(&0.0));
        assert_eq!(sprite.pivot.as_ref().and_then(|pivot| pivot.1.as_static()), Some(&0.0));
    }

    fn assert_party_box(
        node: &ViewNodeDef,
        x: f32,
        head_visual: &str,
        name_visual: &str,
        first_hp_digit: &str,
        current_digits_x: f32,
    ) {
        let transform = node.transform.as_ref().expect("party node transform");
        let translation = transform.translation.as_ref().expect("party translation");
        assert_eq!(translation.0.as_static(), Some(&x));
        assert_eq!(translation.1.as_static(), Some(&417.0));
        assert!(node.children.iter().any(|child| {
            child
                .sprite
                .as_ref()
                .is_some_and(|sprite| sprite.visual.0 == head_visual)
        }));
        assert!(node.children.iter().any(|child| {
            child
                .sprite
                .as_ref()
                .is_some_and(|sprite| sprite.visual.0 == name_visual)
        }));
        let first_digit = find_node(node, "PartyHpCurrentDigit0");
        assert_sprite(first_digit, first_hp_digit, current_digits_x, 11.0, 1.0);
        assert!(node.children.iter().any(|child| {
            child.name == "PartyHpBarFill"
                && child
                    .sprite
                    .as_ref()
                    .and_then(|sprite| sprite.transform.as_ref())
                    .and_then(|transform| transform.scale.as_ref())
                    .and_then(|scale| scale.0.as_static())
                    == Some(&75.0)
        }));
    }

    fn assert_cursor_exprs(node: &ViewNodeDef, cursor_fact: &str) {
        let sprite = node.sprite.as_ref().expect("cursor sprite");
        let transform = sprite.transform.as_ref().expect("cursor transform");
        let translation = transform.translation.as_ref().expect("cursor translation");

        assert!(
            val_expr(&translation.0).is_some_and(|expr| expr.contains(cursor_fact)),
            "cursor x expression should reference {cursor_fact}"
        );
        assert!(
            val_expr(&translation.1).is_some_and(|expr| expr.contains(cursor_fact)),
            "cursor y expression should reference {cursor_fact}"
        );
    }

    fn val_expr(value: &Val<f32>) -> Option<&str> {
        match value {
            Val::Expr(expr) => Some(expr.as_str()),
            Val::Static(_) => None,
        }
    }

    fn first_position_y(chapters: &[Chapter], local_name: &str) -> Option<f32> {
        chapters.iter().find_map(|chapter| position_y(chapter, local_name))
    }

    fn last_position_y(chapters: &[Chapter], local_name: &str) -> Option<f32> {
        chapters
            .iter()
            .rev()
            .find_map(|chapter| position_y(chapter, local_name))
    }

    fn first_scale_y(chapters: &[Chapter], local_name: &str) -> Option<f32> {
        chapters.iter().find_map(|chapter| scale_y(chapter, local_name))
    }

    fn last_scale_y(chapters: &[Chapter], local_name: &str) -> Option<f32> {
        chapters
            .iter()
            .rev()
            .find_map(|chapter| scale_y(chapter, local_name))
    }

    fn position_y(chapter: &Chapter, local_name: &str) -> Option<f32> {
        let Chapter::Parallel(children) = chapter else {
            return None;
        };

        children.iter().find_map(|child| {
            let Chapter::SetViewElement {
                selector,
                target: TweenTarget::Position { to, .. },
                ..
            } = child
            else {
                return None;
            };
            if !selector_is_local(selector, local_name) {
                return None;
            }
            vec3_y(to)
        })
    }

    fn scale_y(chapter: &Chapter, local_name: &str) -> Option<f32> {
        let Chapter::Parallel(children) = chapter else {
            return None;
        };

        children.iter().find_map(|child| {
            let Chapter::SetViewElement {
                selector,
                target: TweenTarget::Scale { to, .. },
                ..
            } = child
            else {
                return None;
            };
            if !selector_is_local(selector, local_name) {
                return None;
            }
            vec3_y(to)
        })
    }

    fn selector_is_local(selector: &ElementSelector, local_name: &str) -> bool {
        matches!(selector, ElementSelector::LocalName(name) if name == local_name)
    }

    fn vec3_y(value: &Vec3Tuple) -> Option<f32> {
        match value {
            Vec3Tuple::Named { y, .. } | Vec3Tuple::Positional(_, y, _) => y.as_static().copied(),
        }
    }
}
