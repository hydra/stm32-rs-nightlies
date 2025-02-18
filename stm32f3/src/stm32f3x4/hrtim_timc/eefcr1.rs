///Register `EEFCR1` reader
pub struct R(crate::R<EEFCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EEFCR1` writer
pub struct W(crate::W<EEFCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFCR1_SPEC>;
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
impl From<crate::W<EEFCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE1LTCH` reader - External Event 1 latch
pub type EE1LTCH_R = crate::BitReader<EE1LTCH_A>;
///External Event 1 latch
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1LTCH_A {
    ///0: Event is ignored if it happens during a blank, or passed through during a window
    Disabled = 0,
    ///1: Event is latched and delayed till the end of the blanking or windowing period
    Enabled = 1,
}
impl From<EE1LTCH_A> for bool {
    #[inline(always)]
    fn from(variant: EE1LTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl EE1LTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1LTCH_A {
        match self.bits {
            false => EE1LTCH_A::Disabled,
            true => EE1LTCH_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1LTCH_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE1LTCH_A::Enabled
    }
}
///Field `EE1LTCH` writer - External Event 1 latch
pub type EE1LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFCR1_SPEC, EE1LTCH_A, O>;
impl<'a, const O: u8> EE1LTCH_W<'a, O> {
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE1LTCH_A::Disabled)
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE1LTCH_A::Enabled)
    }
}
///Field `EE1FLTR` reader - External Event 1 filter
pub type EE1FLTR_R = crate::FieldReader<u8, EE1FLTR_A>;
///External Event 1 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1FLTR_A {
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
impl From<EE1FLTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EE1FLTR_A) -> Self {
        variant as _
    }
}
impl EE1FLTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1FLTR_A {
        match self.bits {
            0 => EE1FLTR_A::Disabled,
            1 => EE1FLTR_A::BlankResetToCompare1,
            2 => EE1FLTR_A::BlankResetToCompare2,
            3 => EE1FLTR_A::BlankResetToCompare3,
            4 => EE1FLTR_A::BlankResetToCompare4,
            5 => EE1FLTR_A::BlankTimfltr1,
            6 => EE1FLTR_A::BlankTimfltr2,
            7 => EE1FLTR_A::BlankTimfltr3,
            8 => EE1FLTR_A::BlankTimfltr4,
            9 => EE1FLTR_A::BlankTimfltr5,
            10 => EE1FLTR_A::BlankTimfltr6,
            11 => EE1FLTR_A::BlankTimfltr7,
            12 => EE1FLTR_A::BlankTimfltr8,
            13 => EE1FLTR_A::WindowResetToCompare2,
            14 => EE1FLTR_A::WindowResetToCompare3,
            15 => EE1FLTR_A::WindowTimwin,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1FLTR_A::Disabled
    }
    ///Checks if the value of the field is `BlankResetToCompare1`
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE1FLTR_A::BlankResetToCompare1
    }
    ///Checks if the value of the field is `BlankResetToCompare2`
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR_A::BlankResetToCompare2
    }
    ///Checks if the value of the field is `BlankResetToCompare3`
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR_A::BlankResetToCompare3
    }
    ///Checks if the value of the field is `BlankResetToCompare4`
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE1FLTR_A::BlankResetToCompare4
    }
    ///Checks if the value of the field is `BlankTimfltr1`
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr1
    }
    ///Checks if the value of the field is `BlankTimfltr2`
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr2
    }
    ///Checks if the value of the field is `BlankTimfltr3`
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr3
    }
    ///Checks if the value of the field is `BlankTimfltr4`
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr4
    }
    ///Checks if the value of the field is `BlankTimfltr5`
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr5
    }
    ///Checks if the value of the field is `BlankTimfltr6`
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr6
    }
    ///Checks if the value of the field is `BlankTimfltr7`
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr7
    }
    ///Checks if the value of the field is `BlankTimfltr8`
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE1FLTR_A::BlankTimfltr8
    }
    ///Checks if the value of the field is `WindowResetToCompare2`
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR_A::WindowResetToCompare2
    }
    ///Checks if the value of the field is `WindowResetToCompare3`
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR_A::WindowResetToCompare3
    }
    ///Checks if the value of the field is `WindowTimwin`
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE1FLTR_A::WindowTimwin
    }
}
///Field `EE1FLTR` writer - External Event 1 filter
pub type EE1FLTR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EEFCR1_SPEC, u8, EE1FLTR_A, 4, O>;
impl<'a, const O: u8> EE1FLTR_W<'a, O> {
    ///No filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE1FLTR_A::Disabled)
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankResetToCompare1)
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankResetToCompare2)
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankResetToCompare3)
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankResetToCompare4)
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr1)
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr2)
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr3)
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr4)
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr5)
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr6)
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr7)
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BlankTimfltr8)
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WindowResetToCompare2)
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WindowResetToCompare3)
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WindowTimwin)
    }
}
///Field `EE2FLTR` reader - External Event 2 filter
pub use EE1FLTR_R as EE2FLTR_R;
///Field `EE3FLTR` reader - External Event 3 filter
pub use EE1FLTR_R as EE3FLTR_R;
///Field `EE4FLTR` reader - External Event 4 filter
pub use EE1FLTR_R as EE4FLTR_R;
///Field `EE5FLTR` reader - External Event 5 filter
pub use EE1FLTR_R as EE5FLTR_R;
///Field `EE2FLTR` writer - External Event 2 filter
pub use EE1FLTR_W as EE2FLTR_W;
///Field `EE3FLTR` writer - External Event 3 filter
pub use EE1FLTR_W as EE3FLTR_W;
///Field `EE4FLTR` writer - External Event 4 filter
pub use EE1FLTR_W as EE4FLTR_W;
///Field `EE5FLTR` writer - External Event 5 filter
pub use EE1FLTR_W as EE5FLTR_W;
///Field `EE2LTCH` reader - External Event 2 latch
pub use EE1LTCH_R as EE2LTCH_R;
///Field `EE3LTCH` reader - External Event 3 latch
pub use EE1LTCH_R as EE3LTCH_R;
///Field `EE4LTCH` reader - External Event 4 latch
pub use EE1LTCH_R as EE4LTCH_R;
///Field `EE5LTCH` reader - External Event 5 latch
pub use EE1LTCH_R as EE5LTCH_R;
///Field `EE2LTCH` writer - External Event 2 latch
pub use EE1LTCH_W as EE2LTCH_W;
///Field `EE3LTCH` writer - External Event 3 latch
pub use EE1LTCH_W as EE3LTCH_W;
///Field `EE4LTCH` writer - External Event 4 latch
pub use EE1LTCH_W as EE4LTCH_W;
///Field `EE5LTCH` writer - External Event 5 latch
pub use EE1LTCH_W as EE5LTCH_W;
impl R {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<0> {
        EE1LTCH_W::new(self)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<1> {
        EE1FLTR_W::new(self)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<6> {
        EE2LTCH_W::new(self)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<7> {
        EE2FLTR_W::new(self)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<12> {
        EE3LTCH_W::new(self)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<13> {
        EE3FLTR_W::new(self)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<18> {
        EE4LTCH_W::new(self)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<19> {
        EE4FLTR_W::new(self)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<24> {
        EE5LTCH_W::new(self)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<25> {
        EE5FLTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefcr1](index.html) module
pub struct EEFCR1_SPEC;
impl crate::RegisterSpec for EEFCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [eefcr1::R](R) reader structure
impl crate::Readable for EEFCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eefcr1::W](W) writer structure
impl crate::Writable for EEFCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EEFCR1 to value 0
impl crate::Resettable for EEFCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
