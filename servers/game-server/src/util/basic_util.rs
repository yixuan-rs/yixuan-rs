use crate::{player::Player, resources::NapResources};

const MAX_YOROZUYA_LEVEL: u32 = 60;

pub fn add_experience(player: &mut Player, amount: u32) {
    let model = &mut player.basic_model;
    if model.level.get() == MAX_YOROZUYA_LEVEL {
        return;
    }

    model.exp.set(model.exp.get() + amount);

    while let Some(exp_cost) =
        should_increase_level(model.level.get(), model.exp.get(), player.resources)
    {
        model.exp.set(model.exp.get() - exp_cost);
        model.level.set(model.level.get() + 1);

        if model.level.get() == MAX_YOROZUYA_LEVEL {
            model.exp.set(0);
        }
    }
}

fn should_increase_level(level: u32, exp: u32, res: &NapResources) -> Option<u32> {
    if level < MAX_YOROZUYA_LEVEL {
        let yorozuya_level_template = res
            .templates
            .yorozuya_level_template_tb()
            .find(|tmpl| tmpl.level() == level)
            .unwrap();

        (yorozuya_level_template.experience() <= exp)
            .then_some(yorozuya_level_template.experience())
    } else {
        None
    }
}
