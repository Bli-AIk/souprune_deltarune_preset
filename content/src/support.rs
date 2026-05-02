//! Shared helper modules for this content guest.
//!
//! 当前内容 guest 的共享辅助模块。

pub mod dark_menu;

#[cfg(test)]
mod tests {
    use super::dark_menu;
    use souprune_schema::fre::{ActionEventKind, FactValueDef, RuleActionDef, RuleEventDef};
    use souprune_schema::view::{CoordinateExtentDef, RotationDirectionDef, YAxisDirectionDef};

    #[test]
    fn dark_menu_facts_define_party_inventory_and_layers() {
        let asset = dark_menu::fre_asset();

        assert!(matches!(
            asset.facts.get("dr.menu.layer"),
            Some(FactValueDef::Enum(layer)) if layer == "top_menu"
        ));
        assert!(matches!(
            asset.facts.get("dr.party.count"),
            Some(FactValueDef::Int(3))
        ));
        assert!(matches!(
            asset.facts.get("dr.inventory.items"),
            Some(FactValueDef::StringList(items)) if items.len() >= 3
        ));
        assert!(matches!(
            asset.facts.get("dr.key_items.usable"),
            Some(FactValueDef::Bool(false))
        ));
    }

    #[test]
    fn dark_menu_rules_enter_categories_and_return_to_normal() {
        let asset = dark_menu::fre_asset();

        let has_open_rule = asset.rules.iter().any(|rule| {
            rule.id == "dr_open_dark_menu"
                && matches!(
                    rule.event,
                    RuleEventDef::ActionEvent {
                        ref action,
                        kind: ActionEventKind::JustPressed,
                    } if action == "Menu"
                )
                && rule.actions.iter().any(|action| {
                    matches!(action, RuleActionDef::Custom { action_type, params } if action_type == "SetSubState" && params.get("state").is_some_and(|state| state == "DarkMenu"))
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
    fn dark_menu_view_uses_deltarune_coordinate_space_and_party_chunks() {
        let layout = dark_menu::view_asset();
        let space = layout
            .coordinate_space
            .expect("DR menu should define an imported coordinate space");

        assert_eq!(space.y_axis, YAxisDirectionDef::Down);
        assert_eq!(space.rotation, RotationDirectionDef::CounterClockwise);
        assert_eq!(space.extent, CoordinateExtentDef::Explicit((640.0, 480.0)));

        for chunk in [0.0, 212.0, 424.0] {
            let node_name = format!("PartyBox{}", (chunk / 212.0) as usize);
            let node = layout
                .roots
                .iter()
                .find(|node| node.name == node_name)
                .expect("party node should exist");
            let transform = node.transform.as_ref().expect("party node transform");
            let translation = transform.translation.as_ref().expect("party translation");
            assert_eq!(translation.0.as_static(), Some(&chunk));
        }
    }
}
