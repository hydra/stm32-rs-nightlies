///Register `EEFAR2` reader
pub struct R(crate::R<EEFAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFAR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EEFAR2` writer
pub struct W(crate::W<EEFAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EEFAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFAR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE6LTCH` reader - External Event 6 latch
pub type EE6LTCH_R = crate::BitReader<EE6LTCH_A>;
///External Event 6 latch
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE6LTCH_A {
    ///0: Event is ignored if it happens during a blank, or passed through during a window
    Disabled = 0,
    ///1: Event is latched and delayed till the end of the blanking or windowing period
    Enabled = 1,
}
impl From<EE6LTCH_A> for bool {
    #[inline(always)]
    fn from(variant: EE6LTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl EE6LTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE6LTCH_A {
        match self.bits {
            false => EE6LTCH_A::Disabled,
            true => EE6LTCH_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6LTCH_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE6LTCH_A::Enabled
    }
}
///Field `EE6LTCH` writer - External Event 6 latch
pub type EE6LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, EE6LTCH_A, O>;
impl<'a, const O: u8> EE6LTCH_W<'a, O> {
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6LTCH_A::Disabled)
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE6LTCH_A::Enabled)
    }
}
///Field `EE6FLTR` reader - External Event 6 filter
pub type EE6FLTR_R = crate::FieldReader<u8, EE6FLTR_A>;
///External Event 6 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6FLTR_A {
    ///0: No filtering
    Disabled = 0,
    ///1: Blanking from counter reset/roll-over to Compare 1
    BlankResetToCompare1 = 1,
    ///2: Blanking from counter reset/roll-over to Compare 2
    BlankResetToCompare2 = 2,
    ///3: Blanking from counter reset/roll-over to Compare 3
    BlankResetToCompare3 = 3,
    ///4: Blanking from counter reset/roll-over to Compare 4
    BlankResetToCompare4 = 4,
    ///5: Blanking from another timing unit: TIMFLTR1 source
    BlankTimfltr1 = 5,
    ///6: Blanking from another timing unit: TIMFLTR2 source
    BlankTimfltr2 = 6,
    ///7: Blanking from another timing unit: TIMFLTR3 source
    BlankTimfltr3 = 7,
    ///8: Blanking from another timing unit: TIMFLTR4 source
    BlankTimfltr4 = 8,
    ///9: Blanking from another timing unit: TIMFLTR5 source
    BlankTimfltr5 = 9,
    ///10: Blanking from another timing unit: TIMFLTR6 source
    BlankTimfltr6 = 10,
    ///11: Blanking from another timing unit: TIMFLTR7 source
    BlankTimfltr7 = 11,
    ///12: Blanking from another timing unit: TIMFLTR8 source
    BlankTimfltr8 = 12,
    ///13: Windowing from counter reset/roll-over to compare 2
    WindowResetToCompare2 = 13,
    ///14: Windowing from counter reset/roll-over to compare 3
    WindowResetToCompare3 = 14,
    ///15: Windowing from another timing unit: TIMWIN source
    WindowTimwin = 15,
}
impl From<EE6FLTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6FLTR_A) -> Self {
        variant as _
    }
}
impl EE6FLTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE6FLTR_A {
        match self.bits {
            0 => EE6FLTR_A::Disabled,
            1 => EE6FLTR_A::BlankResetToCompare1,
            2 => EE6FLTR_A::BlankResetToCompare2,
            3 => EE6FLTR_A::BlankResetToCompare3,
            4 => EE6FLTR_A::BlankResetToCompare4,
            5 => EE6FLTR_A::BlankTimfltr1,
            6 => EE6FLTR_A::BlankTimfltr2,
            7 => EE6FLTR_A::BlankTimfltr3,
            8 => EE6FLTR_A::BlankTimfltr4,
            9 => EE6FLTR_A::BlankTimfltr5,
            10 => EE6FLTR_A::BlankTimfltr6,
            11 => EE6FLTR_A::BlankTimfltr7,
            12 => EE6FLTR_A::BlankTimfltr8,
            13 => EE6FLTR_A::WindowResetToCompare2,
            14 => EE6FLTR_A::WindowResetToCompare3,
            15 => EE6FLTR_A::WindowTimwin,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6FLTR_A::Disabled
    }
    ///Checks if the value of the field is `BlankResetToCompare1`
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare1
    }
    ///Checks if the value of the field is `BlankResetToCompare2`
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare2
    }
    ///Checks if the value of the field is `BlankResetToCompare3`
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare3
    }
    ///Checks if the value of the field is `BlankResetToCompare4`
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare4
    }
    ///Checks if the value of the field is `BlankTimfltr1`
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr1
    }
    ///Checks if the value of the field is `BlankTimfltr2`
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr2
    }
    ///Checks if the value of the field is `BlankTimfltr3`
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr3
    }
    ///Checks if the value of the field is `BlankTimfltr4`
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr4
    }
    ///Checks if the value of the field is `BlankTimfltr5`
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr5
    }
    ///Checks if the value of the field is `BlankTimfltr6`
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr6
    }
    ///Checks if the value of the field is `BlankTimfltr7`
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr7
    }
    ///Checks if the value of the field is `BlankTimfltr8`
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr8
    }
    ///Checks if the value of the field is `WindowResetToCompare2`
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR_A::WindowResetToCompare2
    }
    ///Checks if the value of the field is `WindowResetToCompare3`
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR_A::WindowResetToCompare3
    }
    ///Checks if the value of the field is `WindowTimwin`
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE6FLTR_A::WindowTimwin
    }
}
///Field `EE6FLTR` writer - External Event 6 filter
pub type EE6FLTR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EEFAR2_SPEC, u8, EE6FLTR_A, 4, O>;
impl<'a, const O: u8> EE6FLTR_W<'a, O> {
    ///No filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6FLTR_A::Disabled)
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare1)
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare2)
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare3)
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare4)
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr1)
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr2)
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr3)
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr4)
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr5)
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr6)
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr7)
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr8)
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowResetToCompare2)
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowResetToCompare3)
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowTimwin)
    }
}
///Field `EE7FLTR` reader - External Event 7 filter
pub use EE6FLTR_R as EE7FLTR_R;
///Field `EE8FLTR` reader - External Event 8 filter
pub use EE6FLTR_R as EE8FLTR_R;
///Field `EE9FLTR` reader - External Event 9 filter
pub use EE6FLTR_R as EE9FLTR_R;
///Field `EE10FLTR` reader - External Event 10 filter
pub use EE6FLTR_R as EE10FLTR_R;
///Field `EE7FLTR` writer - External Event 7 filter
pub use EE6FLTR_W as EE7FLTR_W;
///Field `EE8FLTR` writer - External Event 8 filter
pub use EE6FLTR_W as EE8FLTR_W;
///Field `EE9FLTR` writer - External Event 9 filter
pub use EE6FLTR_W as EE9FLTR_W;
///Field `EE10FLTR` writer - External Event 10 filter
pub use EE6FLTR_W as EE10FLTR_W;
///Field `EE7LTCH` reader - External Event 7 latch
pub use EE6LTCH_R as EE7LTCH_R;
///Field `EE8LTCH` reader - External Event 8 latch
pub use EE6LTCH_R as EE8LTCH_R;
///Field `EE9LTCH` reader - External Event 9 latch
pub use EE6LTCH_R as EE9LTCH_R;
///Field `EE10LTCH` reader - External Event 10 latch
pub use EE6LTCH_R as EE10LTCH_R;
///Field `EE7LTCH` writer - External Event 7 latch
pub use EE6LTCH_W as EE7LTCH_W;
///Field `EE8LTCH` writer - External Event 8 latch
pub use EE6LTCH_W as EE8LTCH_W;
///Field `EE9LTCH` writer - External Event 9 latch
pub use EE6LTCH_W as EE9LTCH_W;
///Field `EE10LTCH` writer - External Event 10 latch
pub use EE6LTCH_W as EE10LTCH_W;
impl R {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<0> {
        EE6LTCH_W::new(self)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<1> {
        EE6FLTR_W::new(self)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<6> {
        EE7LTCH_W::new(self)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<7> {
        EE7FLTR_W::new(self)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<12> {
        EE8LTCH_W::new(self)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<13> {
        EE8FLTR_W::new(self)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<18> {
        EE9LTCH_W::new(self)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<19> {
        EE9FLTR_W::new(self)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<24> {
        EE10LTCH_W::new(self)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<25> {
        EE10FLTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefar2](index.html) module
pub struct EEFAR2_SPEC;
impl crate::RegisterSpec for EEFAR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [eefar2::R](R) reader structure
impl crate::Readable for EEFAR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eefar2::W](W) writer structure
impl crate::Writable for EEFAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EEFAR2 to value 0
impl crate::Resettable for EEFAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
