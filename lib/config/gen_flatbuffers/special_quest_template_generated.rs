// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum SpecialQuestTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SpecialQuestTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SpecialQuestTemplate<'a> {
  type Inner = SpecialQuestTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SpecialQuestTemplate<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_UNK_2: flatbuffers::VOffsetT = 6;
  pub const VT_UNK_3: flatbuffers::VOffsetT = 8;
  pub const VT_UNK_4: flatbuffers::VOffsetT = 10;
  pub const VT_UNK_5: flatbuffers::VOffsetT = 12;
  pub const VT_UNK_6: flatbuffers::VOffsetT = 14;
  pub const VT_UNK_7: flatbuffers::VOffsetT = 16;
  pub const VT_UNK_8: flatbuffers::VOffsetT = 18;
  pub const VT_UNK_9: flatbuffers::VOffsetT = 20;
  pub const VT_UNK_10: flatbuffers::VOffsetT = 22;
  pub const VT_UNK_11: flatbuffers::VOffsetT = 24;
  pub const VT_UNK_12: flatbuffers::VOffsetT = 26;
  pub const VT_UNK_13: flatbuffers::VOffsetT = 28;
  pub const VT_UNK_14: flatbuffers::VOffsetT = 30;
  pub const VT_UNK_15: flatbuffers::VOffsetT = 32;
  pub const VT_UNK_16: flatbuffers::VOffsetT = 34;
  pub const VT_UNK_17: flatbuffers::VOffsetT = 36;
  pub const VT_UNK_18: flatbuffers::VOffsetT = 38;
  pub const VT_UNK_19: flatbuffers::VOffsetT = 40;
  pub const VT_UNK_20: flatbuffers::VOffsetT = 42;
  pub const VT_UNK_21: flatbuffers::VOffsetT = 44;
  pub const VT_QUEST_LISTS: flatbuffers::VOffsetT = 46;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SpecialQuestTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args SpecialQuestTemplateArgs<'args>
  ) -> flatbuffers::WIPOffset<SpecialQuestTemplate<'bldr>> {
    let mut builder = SpecialQuestTemplateBuilder::new(_fbb);
    if let Some(x) = args.quest_lists { builder.add_quest_lists(x); }
    builder.add_unk_21(args.unk_21);
    builder.add_unk_20(args.unk_20);
    if let Some(x) = args.unk_19 { builder.add_unk_19(x); }
    if let Some(x) = args.unk_18 { builder.add_unk_18(x); }
    if let Some(x) = args.unk_17 { builder.add_unk_17(x); }
    builder.add_unk_16(args.unk_16);
    builder.add_unk_15(args.unk_15);
    builder.add_unk_14(args.unk_14);
    builder.add_unk_13(args.unk_13);
    if let Some(x) = args.unk_10 { builder.add_unk_10(x); }
    if let Some(x) = args.unk_9 { builder.add_unk_9(x); }
    if let Some(x) = args.unk_8 { builder.add_unk_8(x); }
    builder.add_unk_7(args.unk_7);
    builder.add_unk_6(args.unk_6);
    if let Some(x) = args.unk_5 { builder.add_unk_5(x); }
    builder.add_unk_4(args.unk_4);
    builder.add_unk_3(args.unk_3);
    builder.add_unk_2(args.unk_2);
    builder.add_id(args.id);
    builder.add_unk_12(args.unk_12);
    builder.add_unk_11(args.unk_11);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_2(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_2, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_3(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_3, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_4(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_4, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_5(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_5, None)}
  }
  #[inline]
  pub fn unk_6(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_6, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_7(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_7, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_8(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_8, None)}
  }
  #[inline]
  pub fn unk_9(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_9, None)}
  }
  #[inline]
  pub fn unk_10(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_10, None)}
  }
  #[inline]
  pub fn unk_11(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(SpecialQuestTemplate::VT_UNK_11, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_12(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(SpecialQuestTemplate::VT_UNK_12, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_13(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_13, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_14(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_14, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_15(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_15, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_16(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_16, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_17(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_17, None)}
  }
  #[inline]
  pub fn unk_18(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_18, None)}
  }
  #[inline]
  pub fn unk_19(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpecialQuestTemplate::VT_UNK_19, None)}
  }
  #[inline]
  pub fn unk_20(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_20, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_21(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(SpecialQuestTemplate::VT_UNK_21, Some(0)).unwrap()}
  }
  #[inline]
  pub fn quest_lists(&self) -> Option<flatbuffers::Vector<'a, u32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(SpecialQuestTemplate::VT_QUEST_LISTS, None)}
  }
}

impl flatbuffers::Verifiable for SpecialQuestTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("id", Self::VT_ID, false)?
     .visit_field::<u32>("unk_2", Self::VT_UNK_2, false)?
     .visit_field::<u32>("unk_3", Self::VT_UNK_3, false)?
     .visit_field::<u32>("unk_4", Self::VT_UNK_4, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_5", Self::VT_UNK_5, false)?
     .visit_field::<u32>("unk_6", Self::VT_UNK_6, false)?
     .visit_field::<u32>("unk_7", Self::VT_UNK_7, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_8", Self::VT_UNK_8, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_9", Self::VT_UNK_9, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_10", Self::VT_UNK_10, false)?
     .visit_field::<bool>("unk_11", Self::VT_UNK_11, false)?
     .visit_field::<bool>("unk_12", Self::VT_UNK_12, false)?
     .visit_field::<u32>("unk_13", Self::VT_UNK_13, false)?
     .visit_field::<u32>("unk_14", Self::VT_UNK_14, false)?
     .visit_field::<u32>("unk_15", Self::VT_UNK_15, false)?
     .visit_field::<u32>("unk_16", Self::VT_UNK_16, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_17", Self::VT_UNK_17, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_18", Self::VT_UNK_18, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_19", Self::VT_UNK_19, false)?
     .visit_field::<u32>("unk_20", Self::VT_UNK_20, false)?
     .visit_field::<u32>("unk_21", Self::VT_UNK_21, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>("quest_lists", Self::VT_QUEST_LISTS, false)?
     .finish();
    Ok(())
  }
}
pub struct SpecialQuestTemplateArgs<'a> {
    pub id: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_9: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_10: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_11: bool,
    pub unk_12: bool,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_18: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_19: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_20: u32,
    pub unk_21: u32,
    pub quest_lists: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
}
impl<'a> Default for SpecialQuestTemplateArgs<'a> {
  #[inline]
  fn default() -> Self {
    SpecialQuestTemplateArgs {
      id: 0,
      unk_2: 0,
      unk_3: 0,
      unk_4: 0,
      unk_5: None,
      unk_6: 0,
      unk_7: 0,
      unk_8: None,
      unk_9: None,
      unk_10: None,
      unk_11: false,
      unk_12: false,
      unk_13: 0,
      unk_14: 0,
      unk_15: 0,
      unk_16: 0,
      unk_17: None,
      unk_18: None,
      unk_19: None,
      unk_20: 0,
      unk_21: 0,
      quest_lists: None,
    }
  }
}

pub struct SpecialQuestTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SpecialQuestTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_unk_2(&mut self, unk_2: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_2, unk_2, 0);
  }
  #[inline]
  pub fn add_unk_3(&mut self, unk_3: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_3, unk_3, 0);
  }
  #[inline]
  pub fn add_unk_4(&mut self, unk_4: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_4, unk_4, 0);
  }
  #[inline]
  pub fn add_unk_5(&mut self, unk_5: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_5, unk_5);
  }
  #[inline]
  pub fn add_unk_6(&mut self, unk_6: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_6, unk_6, 0);
  }
  #[inline]
  pub fn add_unk_7(&mut self, unk_7: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_7, unk_7, 0);
  }
  #[inline]
  pub fn add_unk_8(&mut self, unk_8: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_8, unk_8);
  }
  #[inline]
  pub fn add_unk_9(&mut self, unk_9: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_9, unk_9);
  }
  #[inline]
  pub fn add_unk_10(&mut self, unk_10: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_10, unk_10);
  }
  #[inline]
  pub fn add_unk_11(&mut self, unk_11: bool) {
    self.fbb_.push_slot::<bool>(SpecialQuestTemplate::VT_UNK_11, unk_11, false);
  }
  #[inline]
  pub fn add_unk_12(&mut self, unk_12: bool) {
    self.fbb_.push_slot::<bool>(SpecialQuestTemplate::VT_UNK_12, unk_12, false);
  }
  #[inline]
  pub fn add_unk_13(&mut self, unk_13: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_13, unk_13, 0);
  }
  #[inline]
  pub fn add_unk_14(&mut self, unk_14: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_14, unk_14, 0);
  }
  #[inline]
  pub fn add_unk_15(&mut self, unk_15: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_15, unk_15, 0);
  }
  #[inline]
  pub fn add_unk_16(&mut self, unk_16: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_16, unk_16, 0);
  }
  #[inline]
  pub fn add_unk_17(&mut self, unk_17: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_17, unk_17);
  }
  #[inline]
  pub fn add_unk_18(&mut self, unk_18: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_18, unk_18);
  }
  #[inline]
  pub fn add_unk_19(&mut self, unk_19: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_UNK_19, unk_19);
  }
  #[inline]
  pub fn add_unk_20(&mut self, unk_20: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_20, unk_20, 0);
  }
  #[inline]
  pub fn add_unk_21(&mut self, unk_21: u32) {
    self.fbb_.push_slot::<u32>(SpecialQuestTemplate::VT_UNK_21, unk_21, 0);
  }
  #[inline]
  pub fn add_quest_lists(&mut self, quest_lists: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplate::VT_QUEST_LISTS, quest_lists);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SpecialQuestTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    SpecialQuestTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SpecialQuestTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SpecialQuestTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SpecialQuestTemplate");
      ds.field("id", &self.id());
      ds.field("unk_2", &self.unk_2());
      ds.field("unk_3", &self.unk_3());
      ds.field("unk_4", &self.unk_4());
      ds.field("unk_5", &self.unk_5());
      ds.field("unk_6", &self.unk_6());
      ds.field("unk_7", &self.unk_7());
      ds.field("unk_8", &self.unk_8());
      ds.field("unk_9", &self.unk_9());
      ds.field("unk_10", &self.unk_10());
      ds.field("unk_11", &self.unk_11());
      ds.field("unk_12", &self.unk_12());
      ds.field("unk_13", &self.unk_13());
      ds.field("unk_14", &self.unk_14());
      ds.field("unk_15", &self.unk_15());
      ds.field("unk_16", &self.unk_16());
      ds.field("unk_17", &self.unk_17());
      ds.field("unk_18", &self.unk_18());
      ds.field("unk_19", &self.unk_19());
      ds.field("unk_20", &self.unk_20());
      ds.field("unk_21", &self.unk_21());
      ds.field("quest_lists", &self.quest_lists());
      ds.finish()
  }
}
pub enum SpecialQuestTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SpecialQuestTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SpecialQuestTemplateTb<'a> {
  type Inner = SpecialQuestTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SpecialQuestTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SpecialQuestTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args SpecialQuestTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<SpecialQuestTemplateTb<'bldr>> {
    let mut builder = SpecialQuestTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SpecialQuestTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SpecialQuestTemplate>>>>(SpecialQuestTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for SpecialQuestTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<SpecialQuestTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct SpecialQuestTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SpecialQuestTemplate<'a>>>>>,
}
impl<'a> Default for SpecialQuestTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    SpecialQuestTemplateTbArgs {
      data: None,
    }
  }
}

pub struct SpecialQuestTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SpecialQuestTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<SpecialQuestTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpecialQuestTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SpecialQuestTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    SpecialQuestTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SpecialQuestTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SpecialQuestTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SpecialQuestTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
