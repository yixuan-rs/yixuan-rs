#[derive(thiserror::Error, Debug)]
pub enum TemplateCollectionError {
    #[error("failed to read asset file at {0}, cause: {1}")]
    Read(String, std::io::Error),
    #[error("TemplateTb at {0} is invalid, cause: {1}")]
    InvalidFlatbuffer(String, flatbuffers::InvalidFlatbuffer),
}

file_cfg! {
    AvatarBaseTemplate;
    AvatarBattleTemplate;
    AvatarLevelTemplate;
    AvatarLevelAdvanceTemplate;
    AvatarPassiveSkillTemplate;
    AvatarSkinBaseTemplate;
    ItemTemplate;
    WeaponTemplate;
    WeaponLevelTemplate;
    WeaponStarTemplate;
    EquipmentTemplate;
    EquipmentSuitTemplate;
    EquipmentLevelTemplate;
    UnlockConfigTemplate;
    SectionConfigTemplate;
    HollowConfigTemplate;
    MainCityObjectTemplate;
    QuestConfigTemplate;
    MainCityQuestTemplate;
    HollowQuestTemplate;
    TraningQuestTemplate;
    BattleGroupConfigTemplate;
    OnceRewardTemplate;
    ConditionConfigTemplate;
    NewbieGroupTemplate;
    YorozuyaLevelTemplate;
    PostGirlConfigTemplate;
    HollowEventTemplate;
    HollowChallengeTemplate;
}

macro_rules! file_cfg {
    ($($name:ident;)*) => {
        ::paste::paste!($(
            #[allow(dead_code, unused_imports, unsafe_op_in_unsafe_fn, non_snake_case)]
            mod [<$name:snake>] {
                include!(concat!(
                        "../gen_flatbuffers/",
                        stringify!([<$name:snake _generated>]),
                        ".rs"
                ));
            }

            #[allow(ambiguous_glob_reexports)]
            pub use [<$name:snake>]::*;
        )*);

        ::paste::paste!{
            pub struct TemplateCollection {
                $([<$name:snake _data>]: Vec<u8>,)*
        }}

        impl TemplateCollection {
            ::paste::paste!{
                pub fn load(filecfg_path: &str) -> Result<Self, TemplateCollectionError> {
                    use ::flatbuffers::Verifiable;

                    let verifier_options = ::flatbuffers::VerifierOptions::default();
                    Ok(Self {
                        $(
                            [<$name:snake _data>]: {
                                let filename_hash = ::xxhash_rust::const_xxh64::xxh64(stringify!([<$name:lower tb>]).as_bytes(), 0);
                                let file_path = format!("{filecfg_path}/{filename_hash}");

                                let data = match std::fs::read(&file_path) {
                                    Ok(data) => data,
                                    Err(err) => return Err(TemplateCollectionError::Read(file_path, err)),
                                };

                               let mut verifier = ::flatbuffers::Verifier::new(&verifier_options, &data);
                               match ::flatbuffers::ForwardsUOffset::<[<$name Tb>]>::run_verifier(&mut verifier, 0) {
                                   Ok(()) => data,
                                   Err(err) => return Err(TemplateCollectionError::InvalidFlatbuffer(file_path, err)),
                               }
                            },
                        )*
                    })
                }

                $(
                    pub fn [<$name:snake _tb>](&self) -> impl Iterator<Item = $name<'_>> {
                        // SAFETY: flatbuffer integrity is already verified
                        unsafe { ::flatbuffers::root_unchecked::<[<$name Tb>]>(&self.[<$name:snake _data>]).data().unwrap().iter() }
                    }
                )*
            }
        }
    };
}

use file_cfg;
