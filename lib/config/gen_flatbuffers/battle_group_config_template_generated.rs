// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum BattleGroupConfigTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct BattleGroupConfigTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BattleGroupConfigTemplate<'a> {
  type Inner = BattleGroupConfigTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> BattleGroupConfigTemplate<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_QUEST_ID: flatbuffers::VOffsetT = 6;
  pub const VT_UNK_1: flatbuffers::VOffsetT = 8;
  pub const VT_UNK_2: flatbuffers::VOffsetT = 10;
  pub const VT_UNK_3: flatbuffers::VOffsetT = 12;
  pub const VT_UNK_4: flatbuffers::VOffsetT = 14;
  pub const VT_UNK_5: flatbuffers::VOffsetT = 16;
  pub const VT_UNK_6: flatbuffers::VOffsetT = 18;
  pub const VT_BATTLE_EVENT_ID: flatbuffers::VOffsetT = 20;
  pub const VT_UNK_8: flatbuffers::VOffsetT = 22;
  pub const VT_UNK_9: flatbuffers::VOffsetT = 24;
  pub const VT_UNK_10: flatbuffers::VOffsetT = 26;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    BattleGroupConfigTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args BattleGroupConfigTemplateArgs<'args>
  ) -> flatbuffers::WIPOffset<BattleGroupConfigTemplate<'bldr>> {
    let mut builder = BattleGroupConfigTemplateBuilder::new(_fbb);
    if let Some(x) = args.unk_10 { builder.add_unk_10(x); }
    builder.add_battle_event_id(args.battle_event_id);
    if let Some(x) = args.unk_6 { builder.add_unk_6(x); }
    if let Some(x) = args.unk_5 { builder.add_unk_5(x); }
    if let Some(x) = args.unk_4 { builder.add_unk_4(x); }
    builder.add_unk_3(args.unk_3);
    if let Some(x) = args.unk_2 { builder.add_unk_2(x); }
    builder.add_unk_1(args.unk_1);
    builder.add_quest_id(args.quest_id);
    builder.add_id(args.id);
    builder.add_unk_9(args.unk_9);
    builder.add_unk_8(args.unk_8);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(BattleGroupConfigTemplate::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn quest_id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(BattleGroupConfigTemplate::VT_QUEST_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_1(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(BattleGroupConfigTemplate::VT_UNK_1, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_2(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(BattleGroupConfigTemplate::VT_UNK_2, None)}
  }
  #[inline]
  pub fn unk_3(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(BattleGroupConfigTemplate::VT_UNK_3, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_4(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(BattleGroupConfigTemplate::VT_UNK_4, None)}
  }
  #[inline]
  pub fn unk_5(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(BattleGroupConfigTemplate::VT_UNK_5, None)}
  }
  #[inline]
  pub fn unk_6(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(BattleGroupConfigTemplate::VT_UNK_6, None)}
  }
  #[inline]
  pub fn battle_event_id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(BattleGroupConfigTemplate::VT_BATTLE_EVENT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_8(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(BattleGroupConfigTemplate::VT_UNK_8, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_9(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(BattleGroupConfigTemplate::VT_UNK_9, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_10(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(BattleGroupConfigTemplate::VT_UNK_10, None)}
  }
}

impl flatbuffers::Verifiable for BattleGroupConfigTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("id", Self::VT_ID, false)?
     .visit_field::<u32>("quest_id", Self::VT_QUEST_ID, false)?
     .visit_field::<i32>("unk_1", Self::VT_UNK_1, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_2", Self::VT_UNK_2, false)?
     .visit_field::<i32>("unk_3", Self::VT_UNK_3, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_4", Self::VT_UNK_4, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_5", Self::VT_UNK_5, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("unk_6", Self::VT_UNK_6, false)?
     .visit_field::<u32>("battle_event_id", Self::VT_BATTLE_EVENT_ID, false)?
     .visit_field::<bool>("unk_8", Self::VT_UNK_8, false)?
     .visit_field::<bool>("unk_9", Self::VT_UNK_9, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("unk_10", Self::VT_UNK_10, false)?
     .finish();
    Ok(())
  }
}
pub struct BattleGroupConfigTemplateArgs<'a> {
    pub id: u32,
    pub quest_id: u32,
    pub unk_1: i32,
    pub unk_2: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_3: i32,
    pub unk_4: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_5: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_6: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub battle_event_id: u32,
    pub unk_8: bool,
    pub unk_9: bool,
    pub unk_10: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
}
impl<'a> Default for BattleGroupConfigTemplateArgs<'a> {
  #[inline]
  fn default() -> Self {
    BattleGroupConfigTemplateArgs {
      id: 0,
      quest_id: 0,
      unk_1: 0,
      unk_2: None,
      unk_3: 0,
      unk_4: None,
      unk_5: None,
      unk_6: None,
      battle_event_id: 0,
      unk_8: false,
      unk_9: false,
      unk_10: None,
    }
  }
}

pub struct BattleGroupConfigTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> BattleGroupConfigTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: u32) {
    self.fbb_.push_slot::<u32>(BattleGroupConfigTemplate::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_quest_id(&mut self, quest_id: u32) {
    self.fbb_.push_slot::<u32>(BattleGroupConfigTemplate::VT_QUEST_ID, quest_id, 0);
  }
  #[inline]
  pub fn add_unk_1(&mut self, unk_1: i32) {
    self.fbb_.push_slot::<i32>(BattleGroupConfigTemplate::VT_UNK_1, unk_1, 0);
  }
  #[inline]
  pub fn add_unk_2(&mut self, unk_2: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplate::VT_UNK_2, unk_2);
  }
  #[inline]
  pub fn add_unk_3(&mut self, unk_3: i32) {
    self.fbb_.push_slot::<i32>(BattleGroupConfigTemplate::VT_UNK_3, unk_3, 0);
  }
  #[inline]
  pub fn add_unk_4(&mut self, unk_4: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplate::VT_UNK_4, unk_4);
  }
  #[inline]
  pub fn add_unk_5(&mut self, unk_5: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplate::VT_UNK_5, unk_5);
  }
  #[inline]
  pub fn add_unk_6(&mut self, unk_6: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplate::VT_UNK_6, unk_6);
  }
  #[inline]
  pub fn add_battle_event_id(&mut self, battle_event_id: u32) {
    self.fbb_.push_slot::<u32>(BattleGroupConfigTemplate::VT_BATTLE_EVENT_ID, battle_event_id, 0);
  }
  #[inline]
  pub fn add_unk_8(&mut self, unk_8: bool) {
    self.fbb_.push_slot::<bool>(BattleGroupConfigTemplate::VT_UNK_8, unk_8, false);
  }
  #[inline]
  pub fn add_unk_9(&mut self, unk_9: bool) {
    self.fbb_.push_slot::<bool>(BattleGroupConfigTemplate::VT_UNK_9, unk_9, false);
  }
  #[inline]
  pub fn add_unk_10(&mut self, unk_10: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplate::VT_UNK_10, unk_10);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BattleGroupConfigTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    BattleGroupConfigTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BattleGroupConfigTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for BattleGroupConfigTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("BattleGroupConfigTemplate");
      ds.field("id", &self.id());
      ds.field("quest_id", &self.quest_id());
      ds.field("unk_1", &self.unk_1());
      ds.field("unk_2", &self.unk_2());
      ds.field("unk_3", &self.unk_3());
      ds.field("unk_4", &self.unk_4());
      ds.field("unk_5", &self.unk_5());
      ds.field("unk_6", &self.unk_6());
      ds.field("battle_event_id", &self.battle_event_id());
      ds.field("unk_8", &self.unk_8());
      ds.field("unk_9", &self.unk_9());
      ds.field("unk_10", &self.unk_10());
      ds.finish()
  }
}
pub enum BattleGroupConfigTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct BattleGroupConfigTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BattleGroupConfigTemplateTb<'a> {
  type Inner = BattleGroupConfigTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> BattleGroupConfigTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    BattleGroupConfigTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args BattleGroupConfigTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<BattleGroupConfigTemplateTb<'bldr>> {
    let mut builder = BattleGroupConfigTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BattleGroupConfigTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BattleGroupConfigTemplate>>>>(BattleGroupConfigTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for BattleGroupConfigTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<BattleGroupConfigTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct BattleGroupConfigTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BattleGroupConfigTemplate<'a>>>>>,
}
impl<'a> Default for BattleGroupConfigTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    BattleGroupConfigTemplateTbArgs {
      data: None,
    }
  }
}

pub struct BattleGroupConfigTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> BattleGroupConfigTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<BattleGroupConfigTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BattleGroupConfigTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BattleGroupConfigTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    BattleGroupConfigTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BattleGroupConfigTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for BattleGroupConfigTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("BattleGroupConfigTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
