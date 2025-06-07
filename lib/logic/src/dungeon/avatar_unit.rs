use std::{collections::HashMap, sync::LazyLock};

use config::TemplateCollection;

use crate::item::{AvatarItem, EAvatarSkillType, EquipItem, WeaponItem};

use super::property::EPropertyType;

#[derive(Debug, Clone)]
pub struct AvatarUnit {
    pub avatar_id: u32,
    pub properties: HashMap<EPropertyType, i32>,
}

pub trait EquipmentRepository {
    fn get_avatar(&self, id: u32) -> Option<&AvatarItem>;
    fn get_weapon(&self, uid: u32) -> Option<&WeaponItem>;
    fn get_equip(&self, uid: u32) -> Option<&EquipItem>;

    fn get_avatar_weapon(&self, id: u32) -> Option<&WeaponItem> {
        let avatar = self.get_avatar(id)?;
        (avatar.weapon_uid != 0).then(|| self.get_weapon(avatar.weapon_uid))?
    }

    fn get_avatar_equips(&self, id: u32) -> Option<Vec<&EquipItem>> {
        self.get_avatar(id).map(|avatar| {
            avatar
                .dressed_equip_map
                .iter()
                .filter_map(|(&uid, _)| self.get_equip(uid))
                .collect()
        })
    }
}

impl AvatarUnit {
    pub fn new(
        avatar_id: u32,
        templates: &TemplateCollection,
        equipment: &dyn EquipmentRepository,
    ) -> Self {
        let mut unit = AvatarUnit {
            avatar_id,
            properties: HashMap::new(),
        };

        let Some(avatar_item) = equipment.get_avatar(avatar_id) else {
            return unit;
        };

        unit.init_default_properties(templates);
        unit.init_level_advance_properties(avatar_item.rank, templates);
        unit.apply_property_growth_by_level(avatar_item.level);
        unit.apply_core_skill_properties(
            avatar_item.get_skill_level(EAvatarSkillType::CoreSkill),
            templates,
        );

        if let Some(weapon) = equipment.get_avatar_weapon(avatar_id) {
            unit.init_weapon_properties(weapon.id, weapon.level, weapon.star, templates);
        }

        if let Some(equips) = equipment.get_avatar_equips(avatar_id) {
            unit.init_equip_properties(&equips, templates);
            unit.init_equip_suit_properties(&equips, templates);
        }

        unit.set_dynamic_properties();
        unit.apply_core_skill_bonus(avatar_item.get_skill_level(EAvatarSkillType::CoreSkill));
        unit.clear_custom_properties();
        unit.set_battle_properties();

        unit
    }

    pub fn as_proto(&self) -> yixuan_proto::common::AvatarUnitInfo {
        yixuan_proto::common::AvatarUnitInfo {
            avatar_id: self.avatar_id,
            properties: self
                .properties
                .iter()
                .map(|(&ty, &value)| (ty.into(), value))
                .collect(),
        }
    }

    fn set_battle_properties(&mut self) {
        macro_rules! assign {
            ($($name:ident),*) => {
                ::paste::paste!($(self.properties.insert(EPropertyType::[<$name Battle>], self.get_property(EPropertyType::$name));)*);
            };
        }

        self.add_property(
            EPropertyType::SkipDefAtk,
            self.get_property(EPropertyType::Atk) * 30 / 100,
        );

        assign!(
            HpMax,
            Atk,
            BreakStun,
            SkipDefAtk,
            Def,
            Crit,
            CritDmg,
            SpRecover,
            ElementMystery,
            ElementAbnormalPower,
            AddedDamageRatio,
            AddedDamageRatioPhysics,
            AddedDamageRatioFire,
            AddedDamageRatioIce,
            AddedDamageRatioElec,
            AddedDamageRatioEther,
            RpRecover,
            SkipDefDamageRatio
        );

        self.add_property(EPropertyType::Hp, self.get_property(EPropertyType::HpMax));
        self.add_property(
            EPropertyType::PenRatioBattle,
            self.get_property(EPropertyType::Pen),
        );
        self.add_property(
            EPropertyType::PenDeltaBattle,
            self.get_property(EPropertyType::PenValue),
        );
    }

    fn clear_custom_properties(&mut self) {
        self.properties.remove(&EPropertyType::HpMaxGrowth);
        self.properties.remove(&EPropertyType::AtkGrowth);
        self.properties.remove(&EPropertyType::DefGrowth);
        self.properties.remove(&EPropertyType::HpMaxAdvance);
        self.properties.remove(&EPropertyType::AtkAdvance);
        self.properties.remove(&EPropertyType::DefAdvance);
    }

    fn apply_core_skill_bonus(&mut self, core_skill_level: u32) {
        // TODO: find out where this is actually configured

        type PropertyBonusConfig = (EPropertyType, EPropertyType, [i32; 7]);
        static CORE_SKILL_SPECIALS: LazyLock<HashMap<u32, PropertyBonusConfig>> =
            LazyLock::new(|| {
                HashMap::from([
                    (
                        1121,
                        (
                            EPropertyType::Atk,
                            EPropertyType::Def,
                            [40, 46, 52, 60, 66, 72, 80],
                        ),
                    ),
                    (
                        1371,
                        (
                            EPropertyType::SkipDefAtk,
                            EPropertyType::HpMax,
                            [10, 10, 10, 10, 10, 10, 10],
                        ),
                    ),
                ])
            });

        if let Some((bonus_property, scale_property, percentage_per_level)) =
            CORE_SKILL_SPECIALS.get(&self.avatar_id)
        {
            if (1..=7).contains(&core_skill_level) {
                let bonus_value = self.get_property(*scale_property)
                    * percentage_per_level[(core_skill_level - 1) as usize]
                    / 100;

                self.add_property(*bonus_property, bonus_value);
            }
        }
    }

    fn set_dynamic_properties(&mut self) {
        use EPropertyType::*;

        self.set_dynamic_property(HpMax, HpMaxBase, HpMaxRatio, HpMaxDelta);
        self.set_dynamic_property(SpMax, SpMaxBase, None, SpMaxDelta);
        self.set_dynamic_property(Atk, AtkBase, AtkRatio, AtkDelta);
        self.set_dynamic_property(BreakStun, BreakStunBase, BreakStunRatio, BreakStunDelta);
        self.set_dynamic_property(SkipDefAtk, SkipDefAtkBase, None, SkipDefAtkDelta);
        self.set_dynamic_property(Def, DefBase, DefRatio, DefDelta);
        self.set_dynamic_property(Crit, CritBase, None, CritDelta);
        self.set_dynamic_property(CritDmg, CritDmgBase, None, CritDmgDelta);
        self.set_dynamic_property(Pen, PenBase, None, PenDelta);
        self.set_dynamic_property(PenValue, PenValueBase, None, PenValueDelta);
        self.set_dynamic_property(SpRecover, SpRecoverBase, SpRecoverRatio, SpRecoverDelta);
        self.set_dynamic_property(RpRecover, RpRecoverBase, RpRecoverRatio, RpRecoverDelta);
        self.set_dynamic_property(
            ElementMystery,
            ElementMysteryBase,
            None,
            ElementMysteryDelta,
        );
        self.set_dynamic_property(
            ElementAbnormalPower,
            ElementAbnormalPowerBase,
            ElementAbnormalPowerRatio,
            ElementAbnormalPowerDelta,
        );
        self.set_dynamic_property(AddedDamageRatio, AddedDamageRatio1, None, AddedDamageRatio3);
        self.set_dynamic_property(
            AddedDamageRatioPhysics,
            AddedDamageRatioPhysics1,
            None,
            AddedDamageRatioPhysics3,
        );
        self.set_dynamic_property(
            AddedDamageRatioFire,
            AddedDamageRatioFire1,
            None,
            AddedDamageRatioFire3,
        );
        self.set_dynamic_property(
            AddedDamageRatioIce,
            AddedDamageRatioIce1,
            None,
            AddedDamageRatioIce3,
        );
        self.set_dynamic_property(
            AddedDamageRatioElec,
            AddedDamageRatioElec1,
            None,
            AddedDamageRatioElec3,
        );
        self.set_dynamic_property(
            AddedDamageRatioEther,
            AddedDamageRatioEther1,
            None,
            AddedDamageRatioEther3,
        );
        self.set_dynamic_property(
            SkipDefDamageRatio,
            SkipDefDamageRatio1,
            None,
            SkipDefDamageRatio3,
        );
    }

    fn set_dynamic_property(
        &mut self,
        ty: EPropertyType,
        base: EPropertyType,
        ratio: EPropertyType,
        delta: EPropertyType,
    ) {
        const DIVISOR: f32 = 10_000.0;

        let base = self.get_property(base);
        let ratio = self.get_property(ratio);
        let delta = self.get_property(delta);

        if ty == EPropertyType::HpMax {
            // Only HP has ceil()
            self.properties.insert(
                ty,
                base + (base as f32 * ratio as f32 / DIVISOR).ceil() as i32 + delta,
            );
        } else {
            self.properties.insert(
                ty,
                base + (base as f32 * ratio as f32 / DIVISOR) as i32 + delta,
            );
        }
    }

    fn init_equip_suit_properties(
        &mut self,
        equips: &[&EquipItem],
        templates: &TemplateCollection,
    ) {
        let mut equip_suit_counts = HashMap::<u32, u32>::new();

        equips.iter().for_each(|equip| {
            let suit = equip.id / 100 * 100;
            *equip_suit_counts.entry(suit).or_default() += 1;
        });

        equip_suit_counts.iter().for_each(|(&suit, &count)| {
            if let Some(equipment_suit_template) = templates
                .equipment_suit_template_tb()
                .find(|tmpl| tmpl.id() == suit)
            {
                if equipment_suit_template.primary_condition() <= count {
                    equipment_suit_template
                        .primary_suit_propertys()
                        .unwrap()
                        .iter()
                        .for_each(|prop| {
                            self.add_property(
                                EPropertyType::try_from(prop.property()).unwrap(),
                                prop.value(),
                            );
                        });
                }
            }
        });
    }

    fn init_equip_properties(&mut self, equips: &[&EquipItem], templates: &TemplateCollection) {
        const DIVISOR: f32 = 10_000.0;

        equips.iter().for_each(|equip| {
            let rarity = (equip.id / 10) % 10;
            let property_rate = templates
                .equipment_level_template_tb()
                .find(|tmpl| tmpl.rarity() == rarity && tmpl.level() == equip.level)
                .map(|tmpl| tmpl.property_rate())
                .unwrap_or(1);

            equip.properties.iter().for_each(|(&key, &(base, _))| {
                self.add_property(
                    EPropertyType::try_from(key).unwrap(),
                    base as i32 + (base as f32 * property_rate as f32 / DIVISOR) as i32,
                );
            });

            equip
                .sub_properties
                .iter()
                .for_each(|(&key, &(base, add))| {
                    self.add_property(
                        EPropertyType::try_from(key).unwrap(),
                        (base as f32 * add as f32) as i32,
                    );
                });
        });
    }

    fn init_weapon_properties(
        &mut self,
        weapon_id: u32,
        level: u32,
        star: u32,
        templates: &TemplateCollection,
    ) {
        const DIVISOR: f32 = 10_000.0;

        let weapon_template = templates
            .weapon_template_tb()
            .find(|tmpl| tmpl.item_id() == weapon_id)
            .unwrap();

        let weapon_level_template = templates
            .weapon_level_template_tb()
            .find(|tmpl| tmpl.level() == level)
            .unwrap();

        let weapon_star_template = templates
            .weapon_star_template_tb()
            .find(|tmpl| tmpl.star() == star)
            .unwrap();

        let base_property = weapon_template.base_property().unwrap();
        let base_property_value = base_property.value();
        let base_property_value = base_property_value
            + (base_property_value as f32 * weapon_level_template.rate() as f32 / DIVISOR) as i32
            + (base_property_value as f32 * weapon_star_template.star_rate() as f32 / DIVISOR)
                as i32;

        self.add_property(
            EPropertyType::try_from(base_property.property()).unwrap(),
            base_property_value,
        );

        let rand_property = weapon_template.rand_property().unwrap();
        let rand_property_value = rand_property.value();
        let rand_property_value = rand_property_value
            + (rand_property_value as f32 * weapon_star_template.rand_rate() as f32 / DIVISOR)
                as i32;

        self.add_property(
            EPropertyType::try_from(rand_property.property()).unwrap(),
            rand_property_value,
        );
    }

    fn apply_core_skill_properties(
        &mut self,
        core_skill_level: u32,
        templates: &TemplateCollection,
    ) {
        if let Some(passive_skill_template) =
            templates.avatar_passive_skill_template_tb().find(|tmpl| {
                tmpl.avatar_id() == self.avatar_id
                    && tmpl.unlock_passive_skill_level() == core_skill_level
            })
        {
            passive_skill_template
                .propertys()
                .unwrap()
                .iter()
                .for_each(|prop| {
                    let ty = EPropertyType::try_from(prop.property()).unwrap();
                    self.add_property(ty, prop.value());
                });
        }
    }

    fn apply_property_growth_by_level(&mut self, level: u32) {
        self.grow_property_by_level(
            level,
            EPropertyType::HpMaxBase,
            EPropertyType::HpMaxGrowth,
            EPropertyType::HpMaxAdvance,
        );

        self.grow_property_by_level(
            level,
            EPropertyType::AtkBase,
            EPropertyType::AtkGrowth,
            EPropertyType::AtkAdvance,
        );

        self.grow_property_by_level(
            level,
            EPropertyType::DefBase,
            EPropertyType::DefGrowth,
            EPropertyType::DefAdvance,
        );
    }

    fn grow_property_by_level(
        &mut self,
        level: u32,
        base: EPropertyType,
        growth: EPropertyType,
        advance: EPropertyType,
    ) {
        const DIVISOR: f32 = 10_000.0;

        let base_value = *self.properties.get(&base).unwrap();
        let growth = *self.properties.get(&growth).unwrap();
        let advance = *self.properties.get(&advance).unwrap();

        self.properties.insert(
            base,
            base_value + (((level - 1) as f32 * growth as f32) / DIVISOR) as i32 + advance,
        );
    }

    fn init_level_advance_properties(&mut self, rank: u32, templates: &TemplateCollection) {
        let (hp_max, attack, defence) = match templates
            .avatar_level_advance_template_tb()
            .find(|tmpl| tmpl.avatar_id() == self.avatar_id && tmpl.id() == rank)
        {
            Some(tmpl) => (tmpl.hp_max(), tmpl.attack(), tmpl.defence()),
            None => (0, 0, 0),
        };

        self.properties.insert(EPropertyType::HpMaxAdvance, hp_max);
        self.properties.insert(EPropertyType::AtkAdvance, attack);
        self.properties.insert(EPropertyType::DefAdvance, defence);
    }

    fn init_default_properties(&mut self, templates: &TemplateCollection) {
        macro_rules! assign {
            ($unit:ident, $config:ident, $($property:ident: $config_field:ident),*) => {
                $($unit.properties.insert(EPropertyType::$property, $config.$config_field());)*
            };
        }

        let battle_template = templates
            .avatar_battle_template_tb()
            .find(|tmpl| tmpl.id() == self.avatar_id)
            .unwrap();

        assign!(self, battle_template,
            HpMaxBase: hp_max,
            HpMaxGrowth: health_growth,
            AtkBase: attack,
            AtkGrowth: attack_growth,
            BreakStunBase: break_stun,
            DefBase: defence,
            DefGrowth: defence_growth,
            CritBase: crit,
            CritDmgBase: crit_damage,
            PenBase: pen_rate,
            PenValueBase: pen_delta,
            SpMaxBase: sp_bar_point,
            SpRecoverBase: sp_recover,
            ElementMysteryBase: element_mystery,
            ElementAbnormalPowerBase: element_abnormal_power,
            RpMax: rp_max,
            RpRecoverBase: rp_recover
        );
    }

    fn add_property(&mut self, ty: EPropertyType, value: i32) {
        *self.properties.entry(ty).or_default() += value;
    }

    fn get_property(&self, ty: EPropertyType) -> i32 {
        self.properties.get(&ty).copied().unwrap_or(0)
    }
}

impl From<yixuan_proto::common::AvatarUnitInfo> for AvatarUnit {
    fn from(value: yixuan_proto::common::AvatarUnitInfo) -> Self {
        Self {
            avatar_id: value.avatar_id,
            properties: value
                .properties
                .into_iter()
                .filter_map(|(ty, value)| Some((EPropertyType::try_from(ty).ok()?, value)))
                .collect(),
        }
    }
}
