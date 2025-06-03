use super::*;
use property::{PrimitiveProperty, Property, PropertyTransform};
use yixuan_logic::{hall::HallPosition, math::Transform};

#[derive(Model)]
pub struct MainCityModel {
    pub pos_in_main_city: PropertyTransform,
    pub transform_id: PrimitiveProperty<String>,
    pub bgm_id: PrimitiveProperty<u32>,
    pub time_of_day: PrimitiveProperty<u32>,
    pub day_of_week: PrimitiveProperty<u32>,
    pub time_period: PrimitiveProperty<i32>,
}

impl MainCityModel {
    pub fn load_from_pb(pb: MainCityData) -> Self {
        use yixuan_proto::server_only::main_city_data::Pos;

        let (pos_in_main_city, transform_id) = match pb.pos {
            Some(Pos::Position(transform)) => (
                PropertyTransform::from(Transform::from_proto(&transform)),
                PrimitiveProperty::default(),
            ),
            Some(Pos::TransformId(id)) => (PropertyTransform::default(), id.into()),
            None => (PropertyTransform::default(), PrimitiveProperty::default()),
        };

        Self {
            pos_in_main_city,
            transform_id,
            bgm_id: pb.bgm_id.into(),
            time_of_day: pb.time_of_day.into(),
            day_of_week: pb.day_of_week.into(),
            time_period: pb.time_period.into(),
        }
    }

    pub fn get_hall_position(&self) -> Option<HallPosition> {
        self.transform_id
            .get()
            .is_empty()
            .then(|| {
                self.pos_in_main_city
                    .is_zero()
                    .then_some(None)
                    .unwrap_or_else(|| {
                        Some(HallPosition::Transform(
                            self.pos_in_main_city.clone().into(),
                        ))
                    })
            })
            .unwrap_or_else(|| Some(HallPosition::Id(self.transform_id.get().to_string())))
    }
}

impl Saveable for MainCityModel {
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
        use yixuan_proto::server_only::main_city_data::Pos;

        root.main_city = Some(MainCityData {
            pos: Some(
                self.transform_id
                    .get()
                    .is_empty()
                    .then(|| {
                        Pos::Position(Transform::from(self.pos_in_main_city.clone()).to_proto())
                    })
                    .unwrap_or_else(|| Pos::TransformId(self.transform_id.get().to_string())),
            ),
            bgm_id: self.bgm_id.get(),
            time_of_day: self.time_of_day.get(),
            day_of_week: self.day_of_week.get(),
            time_period: self.time_period.get(),
        });
    }
}
