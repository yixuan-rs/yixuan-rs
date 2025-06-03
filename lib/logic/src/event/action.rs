use config::{
    ActionDownloadFullResource, ActionEnterHollowQuest, ActionOpenUI, ActionPerform,
    ActionShowTeleportUi, ActionShowTip, ActionSwitchSection, ActionTriggerInteract,
    ConfigBreakDialogAnim, ConfigCameraMoveV2, ConfigCameraStretch, ConfigCloseDialog,
    ConfigEventAction, ConfigLogText, ConfigMakeDialogChoice, ConfigOpenDialog, ConfigPlayAnim,
    ConfigShowPopWindow, ConfigShowTip, ConfigStopAnim, ConfigWaitSeconds, ConfigWaitTipsEnd,
};
use yixuan_proto::{ActionType, Message};

pub trait ActionBase {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo>;
}

impl ActionBase for ActionOpenUI {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::OpenUi.into(),
            body: yixuan_proto::action::ActionOpenUi {
                ui: self.ui.clone(),
                args: self.args,
                store_template_id: self.store_template_id,
                npc_id: 0,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionSwitchSection {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::SwitchSection.into(),
            body: yixuan_proto::action::ActionSwitchSection {
                section: self.section_id,
                transform_id: self.transform.clone(),
                camera_x: self.camera_x,
                camera_y: self.camera_y,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionPerform {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::Perform.into(),
            body:
                yixuan_proto::action::ActionPerform {
                    perform_id: self.perform_id,
                    perform_id_2: self.perform_id_2,
                    perform_id_3: self.perform_id_3,
                    black_mask: self.black_mask,
                    black_mask_fade_out: self.black_mask_fade_out,
                    black_mask_fade_out_2: self.black_mask_fade_out_2,
                    avatar_id: self.avatar_id,
                    npc_id: self.npc_id,
                    unk_perform_field: Some(
                        yixuan_proto::action::action_perform::UnkPerform::default(),
                    ),
                    participators: self.participators.clone(),
                    ..Default::default()
                }
                .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionShowTip {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::ShowTip.into(),
            body: yixuan_proto::action::ActionShowTip {
                tip_id: self.tip_id,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionTriggerInteract {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::TriggerInteract.into(),
            body: yixuan_proto::action::ActionTriggerInteract {
                tag_id: self.tag_id,
                interact_id: self.interact_id,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionDownloadFullResource {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::DownloadFullResource.into(),
            body: yixuan_proto::action::ActionDownloadFullResource {}.encode_to_vec(),
        })
    }
}

impl ActionBase for ActionShowTeleportUi {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::ShowTeleportUi.into(),
            body: yixuan_proto::action::ActionShowTeleportUi {
                black_mask: self.black_mask,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigOpenDialog {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::OpenDialogHollow.into(),
            body: yixuan_proto::action::ConfigOpenDialog {
                open_event: self.open_event,
                camera_move: self.camera_move,
                unk_open_dialog: self.unk_open_dialog,
                open_param: Some(yixuan_proto::action::OpenDialogParam {
                    unk_open_dialog_flag_1: self.open_param.unk_open_dialog_flag_1,
                    unk_open_dialog_flag_2: self.open_param.unk_open_dialog_flag_2,
                    unk_open_dialog_flag_3: self.open_param.unk_open_dialog_flag_3,
                    start_texture_sheet: self.open_param.start_texture_sheet.clone(),
                    loop_texture_sheet: self.open_param.loop_texture_sheet.clone(),
                    end_texture_sheet: self.open_param.end_texture_sheet.clone(),
                }),
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigLogText {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::LogText.into(),
            body: yixuan_proto::action::ConfigLogText {
                messages: self.messages.clone(),
                log_title: self.log_title.clone(),
                voicelines: self.voicelines.clone(),
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigCloseDialog {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::CloseDialogHollow.into(),
            body: yixuan_proto::action::ConfigCloseDialog {
                camera_move: self.camera_move,
                need_reset_center: self.need_reset_center,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigCameraMoveV2 {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::CameraMoveV2.into(),
            body: yixuan_proto::action::ConfigCameraMoveV2 {
                stretch_key: self.stretch_key.clone(),
                r#type: self.r#type.into(),
                position_offset_type: self.position_offset_type.into(),
                position_offset_x: self.position_offset_x.clone(),
                position_offset_y: self.position_offset_y.clone(),
                radius_x: self.radius_x.clone(),
                radius_y: self.radius_y.clone(),
                bound_index_x: self
                    .bound_index_x
                    .as_ref()
                    .map(|v| yixuan_proto::action::ConfigVector2Int { x: v.x, y: v.y }),
                bound_index_y: self
                    .bound_index_y
                    .as_ref()
                    .map(|v| yixuan_proto::action::ConfigVector2Int { x: v.x, y: v.y }),
                freeze_z: self.freeze_z,
                parallel: self.parallel,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigWaitSeconds {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::WaitSeconds.into(),
            body: yixuan_proto::action::ConfigWaitSeconds {
                wait_time: self.wait_time,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigShowTip {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::ShowTipHollow.into(),
            body: yixuan_proto::action::ConfigShowTip {
                tip_id: self.tip_id,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigShowPopWindow {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::ShowPopWindowHollow.into(),
            body: yixuan_proto::action::ConfigShowPopWindow {
                pop_id: self.pop_id,
                show_directly: self.show_directly,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigCameraStretch {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::CameraStretch.into(),
            body: yixuan_proto::action::ConfigCameraStretch {
                stretch_key: self.stretch_key.clone(),
                shake_key: self.shake_key.clone(),
                r#type: self.r#type.into(),
                parallel: self.parallel,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigPlayAnim {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::PlayAnimHollow.into(),
            body: yixuan_proto::action::ConfigPlayAnim {
                anim_id: self.anim_id,
                indexes: self
                    .indexes
                    .iter()
                    .map(|i| yixuan_proto::action::ConfigVector2Int { x: i.x, y: i.y })
                    .collect(),
                looping: self.looping,
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigStopAnim {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::StopAnim.into(),
            body: yixuan_proto::action::ConfigStopAnim {
                indexes: self
                    .indexes
                    .iter()
                    .map(|i| yixuan_proto::action::ConfigVector2Int { x: i.x, y: i.y })
                    .collect(),
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigWaitTipsEnd {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::WaitTipsEnd.into(),
            body: yixuan_proto::action::ConfigWaitTipsEnd {
                tips_id: self.tips_id.clone(),
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigBreakDialogAnim {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::BreakDialogAnim.into(),
            body: yixuan_proto::action::ConfigBreakDialogAnim {}.encode_to_vec(),
        })
    }
}

impl ActionBase for ConfigMakeDialogChoice {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::MakeDialogChoice.into(),
            body: yixuan_proto::action::ConfigMakeDialogChoice {
                title: self.title.clone(),
                description: self.description.clone(),
                question_description: self.question_description.clone(),
                choices: self
                    .choice_details
                    .iter()
                    .map(|detail| yixuan_proto::action::ConfigDialogChoiceDetail {
                        option_text: detail.option_text.clone(),
                        option_id: detail.option_id.clone(),
                        option_text_2: detail.option_text_2.clone(),
                        choice_id: detail.choice_id,
                        uid: detail.uid,
                    })
                    .collect(),
            }
            .encode_to_vec(),
        })
    }
}

impl ActionBase for ActionEnterHollowQuest {
    fn to_proto(&self) -> Option<yixuan_proto::ActionInfo> {
        Some(yixuan_proto::ActionInfo {
            action_id: self.id,
            action_type: ActionType::EnterHollowQuest.into(),
            body: yixuan_proto::action::ActionEnterHollowQuest {
                hollow_id: self.hollow_id as i32,
                r#type: 1,
                close_black_mask: true,
            }
            .encode_to_vec(),
        })
    }
}

macro_rules! client_actions {
    ($($name:ident),*) => {
        pub fn action_to_proto(config: &ConfigEventAction) -> Option<yixuan_proto::ActionInfo> {
            match config {
                $(ConfigEventAction::$name(cfg) => cfg.to_proto(),)*
                _ => None,
            }
        }
    };
}

client_actions! {
    ActionOpenUI,
    ActionSwitchSection,
    ActionPerform,
    ActionShowTip,
    ActionTriggerInteract,
    ActionDownloadFullResource,
    ActionShowTeleportUi,
    ConfigOpenDialog,
    ConfigLogText,
    ConfigCloseDialog,
    ConfigCameraMoveV2,
    ConfigWaitSeconds,
    ConfigShowTip,
    ConfigShowPopWindow,
    ConfigCameraStretch,
    ConfigPlayAnim,
    ConfigStopAnim,
    ConfigWaitTipsEnd,
    ConfigBreakDialogAnim,
    ConfigMakeDialogChoice,
    ActionEnterHollowQuest
}

pub fn requires_client_input(config: &ConfigEventAction) -> bool {
    matches!(
        config,
        ConfigEventAction::ConfigLogText(_) | ConfigEventAction::ConfigMakeDialogChoice(_)
    )
}
