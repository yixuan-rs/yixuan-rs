use config::ETimePeriodType;
use vivian_proto::common::TimePeriodType;

pub fn logic_to_proto(value: ETimePeriodType) -> TimePeriodType {
    match value {
        ETimePeriodType::Morning => TimePeriodType::Morning,
        ETimePeriodType::Afternoon => TimePeriodType::Afternoon,
        ETimePeriodType::Evening => TimePeriodType::Evening,
        ETimePeriodType::Night => TimePeriodType::Night,
        ETimePeriodType::Now => TimePeriodType::Now,
    }
}

#[expect(dead_code)]
pub fn proto_to_logic(value: TimePeriodType) -> ETimePeriodType {
    match value {
        TimePeriodType::Morning => ETimePeriodType::Morning,
        TimePeriodType::Afternoon => ETimePeriodType::Afternoon,
        TimePeriodType::Evening => ETimePeriodType::Evening,
        TimePeriodType::Night => ETimePeriodType::Night,
        TimePeriodType::Now => ETimePeriodType::Now,
        TimePeriodType::None => ETimePeriodType::Now,
    }
}

pub fn get_time_of_day(period: ETimePeriodType) -> Option<u32> {
    Some(match period {
        ETimePeriodType::Morning => 6 * 60,
        ETimePeriodType::Afternoon => 12 * 60,
        ETimePeriodType::Evening => 18 * 60,
        ETimePeriodType::Night => 0,
        ETimePeriodType::Now => return None,
    })
}
