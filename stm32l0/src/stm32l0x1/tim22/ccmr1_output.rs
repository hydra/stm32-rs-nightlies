///Register `CCMR1_Output` reader
pub struct R(crate::R<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Output` writer
pub struct W(crate::W<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1S` reader - Capture/Compare 1 selection
pub type CC1S_R = crate::FieldReader<u8, CC1S_A>;
///Capture/Compare 1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CCx channel is configured as output
    Output = 0,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1S_A> {
        match self.bits {
            0 => Some(CC1S_A::Output),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S_A::Output
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, CC1S_A, 2, O>;
impl<'a, const O: u8> CC1S_W<'a, O> {
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1S_A::Output)
    }
}
///Field `OC1FE` reader - Output Compare 1 fast enable
pub type OC1FE_R = crate::BitReader<bool>;
///Field `OC1FE` writer - Output Compare 1 fast enable
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1PE` reader - Output Compare 1 preload enable
pub type OC1PE_R = crate::BitReader<OC1PE_A>;
///Output Compare 1 preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1PE_A {
    ///0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC1PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1PE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC1PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1PE_A {
        match self.bits {
            false => OC1PE_A::Disabled,
            true => OC1PE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC1PE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC1PE_A::Enabled
    }
}
///Field `OC1PE` writer - Output Compare 1 preload enable
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, OC1PE_A, O>;
impl<'a, const O: u8> OC1PE_W<'a, O> {
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC1PE_A::Disabled)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC1PE_A::Enabled)
    }
}
///Field `OC1M` reader - Output Compare 1 mode
pub type OC1M_R = crate::FieldReader<u8, OC1M_A>;
///Output Compare 1 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M_A {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    Frozen = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    ActiveOnMatch = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    InactiveOnMatch = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy
    Toggle = 3,
    ///4: OCyREF is forced low
    ForceInactive = 4,
    ///5: OCyREF is forced high
    ForceActive = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    PwmMode1 = 6,
    ///7: Inversely to PwmMode1
    PwmMode2 = 7,
}
impl From<OC1M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC1M_A) -> Self {
        variant as _
    }
}
impl OC1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1M_A {
        match self.bits {
            0 => OC1M_A::Frozen,
            1 => OC1M_A::ActiveOnMatch,
            2 => OC1M_A::InactiveOnMatch,
            3 => OC1M_A::Toggle,
            4 => OC1M_A::ForceInactive,
            5 => OC1M_A::ForceActive,
            6 => OC1M_A::PwmMode1,
            7 => OC1M_A::PwmMode2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC1M_A::Frozen
    }
    ///Checks if the value of the field is `ActiveOnMatch`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC1M_A::ActiveOnMatch
    }
    ///Checks if the value of the field is `InactiveOnMatch`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC1M_A::InactiveOnMatch
    }
    ///Checks if the value of the field is `Toggle`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC1M_A::Toggle
    }
    ///Checks if the value of the field is `ForceInactive`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1M_A::ForceInactive
    }
    ///Checks if the value of the field is `ForceActive`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC1M_A::ForceActive
    }
    ///Checks if the value of the field is `PwmMode1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC1M_A::PwmMode1
    }
    ///Checks if the value of the field is `PwmMode2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC1M_A::PwmMode2
    }
}
///Field `OC1M` writer - Output Compare 1 mode
pub type OC1M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_OUTPUT_SPEC, u8, OC1M_A, 3, O>;
impl<'a, const O: u8> OC1M_W<'a, O> {
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC1M_A::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC1M_A::Toggle)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC1M_A::ForceInactive)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC1M_A::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC1M_A::PwmMode1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC1M_A::PwmMode2)
    }
}
///Field `CC2S` reader - Capture/Compare 2 selection
pub use CC1S_R as CC2S_R;
///Field `CC2S` writer - Capture/Compare 2 selection
pub use CC1S_W as CC2S_W;
///Field `OC2FE` reader - Output Compare 2 fast enable
pub type OC2FE_R = crate::BitReader<bool>;
///Field `OC2FE` writer - Output Compare 2 fast enable
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2M` reader - Output Compare 2 mode
pub use OC1M_R as OC2M_R;
///Field `OC2M` writer - Output Compare 2 mode
pub use OC1M_W as OC2M_W;
///Field `OC2PE` reader - Output Compare 2 preload enable
pub use OC1PE_R as OC2PE_R;
///Field `OC2PE` writer - Output Compare 2 preload enable
pub use OC1PE_W as OC2PE_W;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<4> {
        OC1M_W::new(self)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<10> {
        OC2FE_W::new(self)
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<11> {
        OC2PE_W::new(self)
    }
    ///Bits 12:14 - Output Compare 2 mode
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<12> {
        OC2M_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_output](index.html) module
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_output::R](R) reader structure
impl crate::Readable for CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_output::W](W) writer structure
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
