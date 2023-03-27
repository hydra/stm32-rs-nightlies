///Register `CCMR2_Output` reader
pub struct R(crate::R<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR2_Output` writer
pub struct W(crate::W<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR2_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC3S` reader - Capture/Compare 3 selection
pub type CC3S_R = crate::FieldReader<u8, CC3S_A>;
///Capture/Compare 3 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S_A {
    ///0: CC3 channel is configured as output
    Output = 0,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
impl CC3S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC3S_A> {
        match self.bits {
            0 => Some(CC3S_A::Output),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC3S_A::Output
    }
}
///Field `CC3S` writer - Capture/Compare 3 selection
pub type CC3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, CC3S_A, 2, O>;
impl<'a, const O: u8> CC3S_W<'a, O> {
    ///CC3 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC3S_A::Output)
    }
}
///Field `OC3FE` reader - Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader<bool>;
///Field `OC3FE` writer - Output compare 3 fast enable
pub type OC3FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader<OC3PE_A>;
///Output compare 3 preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC3PE_A {
    ///0: Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCR3 enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC3PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC3PE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC3PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC3PE_A {
        match self.bits {
            false => OC3PE_A::Disabled,
            true => OC3PE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC3PE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC3PE_A::Enabled
    }
}
///Field `OC3PE` writer - Output compare 3 preload enable
pub type OC3PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, OC3PE_A, O>;
impl<'a, const O: u8> OC3PE_W<'a, O> {
    ///Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC3PE_A::Disabled)
    }
    ///Preload register on CCR3 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC3PE_A::Enabled)
    }
}
///Field `OC3M` reader - Output compare 3 mode
pub type OC3M_R = crate::FieldReader<u8, OC3M_A>;
///Output compare 3 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC3M_A {
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
impl From<OC3M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC3M_A) -> Self {
        variant as _
    }
}
impl OC3M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC3M_A {
        match self.bits {
            0 => OC3M_A::Frozen,
            1 => OC3M_A::ActiveOnMatch,
            2 => OC3M_A::InactiveOnMatch,
            3 => OC3M_A::Toggle,
            4 => OC3M_A::ForceInactive,
            5 => OC3M_A::ForceActive,
            6 => OC3M_A::PwmMode1,
            7 => OC3M_A::PwmMode2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC3M_A::Frozen
    }
    ///Checks if the value of the field is `ActiveOnMatch`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC3M_A::ActiveOnMatch
    }
    ///Checks if the value of the field is `InactiveOnMatch`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC3M_A::InactiveOnMatch
    }
    ///Checks if the value of the field is `Toggle`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC3M_A::Toggle
    }
    ///Checks if the value of the field is `ForceInactive`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC3M_A::ForceInactive
    }
    ///Checks if the value of the field is `ForceActive`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC3M_A::ForceActive
    }
    ///Checks if the value of the field is `PwmMode1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC3M_A::PwmMode1
    }
    ///Checks if the value of the field is `PwmMode2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC3M_A::PwmMode2
    }
}
///Field `OC3M` writer - Output compare 3 mode
pub type OC3M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_OUTPUT_SPEC, u8, OC3M_A, 3, O>;
impl<'a, const O: u8> OC3M_W<'a, O> {
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC3M_A::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC3M_A::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC3M_A::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC3M_A::Toggle)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC3M_A::ForceInactive)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC3M_A::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC3M_A::PwmMode1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC3M_A::PwmMode2)
    }
}
///Field `OC3CE` reader - Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader<bool>;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type OC3CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::BitReader<CC4S_A>;
///Capture/Compare 4 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC4S_A {
    ///0: CC4 channel is configured as output
    Output = 0,
}
impl From<CC4S_A> for bool {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as u8 != 0
    }
}
impl CC4S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC4S_A> {
        match self.bits {
            false => Some(CC4S_A::Output),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC4S_A::Output
    }
}
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, CC4S_A, O>;
impl<'a, const O: u8> CC4S_W<'a, O> {
    ///CC4 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC4S_A::Output)
    }
}
///Field `OC4FE` reader - Output compare 4 fast enable
pub type OC4FE_R = crate::BitReader<bool>;
///Field `OC4FE` writer - Output compare 4 fast enable
pub type OC4FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type OC4PE_R = crate::BitReader<OC4PE_A>;
///Output compare 4 preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC4PE_A {
    ///0: Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCR4 enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC4PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC4PE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC4PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC4PE_A {
        match self.bits {
            false => OC4PE_A::Disabled,
            true => OC4PE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC4PE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC4PE_A::Enabled
    }
}
///Field `OC4PE` writer - Output compare 4 preload enable
pub type OC4PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, OC4PE_A, O>;
impl<'a, const O: u8> OC4PE_W<'a, O> {
    ///Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC4PE_A::Disabled)
    }
    ///Preload register on CCR4 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC4PE_A::Enabled)
    }
}
///Field `OC4M` reader - Output compare 4 mode
pub use OC3M_R as OC4M_R;
///Field `OC4M` writer - Output compare 4 mode
pub use OC3M_W as OC4M_W;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type OC4CE_R = crate::BitReader<bool>;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type OC4CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<2> {
        OC3FE_W::new(self)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<3> {
        OC3PE_W::new(self)
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OC3M_W<4> {
        OC3M_W::new(self)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<7> {
        OC3CE_W::new(self)
    }
    ///Bit 8 - Capture/Compare 4 selection
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<10> {
        OC4FE_W::new(self)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<11> {
        OC4PE_W::new(self)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OC4M_W<12> {
        OC4M_W::new(self)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<15> {
        OC4CE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_output](index.html) module
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr2_output::R](R) reader structure
impl crate::Readable for CCMR2_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr2_output::W](W) writer structure
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR2_Output to value 0
impl crate::Resettable for CCMR2_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
