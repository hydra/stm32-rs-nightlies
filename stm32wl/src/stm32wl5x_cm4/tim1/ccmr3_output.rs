///Register `CCMR3_Output` reader
pub struct R(crate::R<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR3_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR3_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR3_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR3_Output` writer
pub struct W(crate::W<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR3_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR3_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR3_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OC5FE` reader - OC5FE
pub type OC5FE_R = crate::BitReader<OC5FE_A>;
///OC5FE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5FE_A {
    ///0: Fast output disabled
    Disabled = 0,
    ///1: Fast output enabled
    Enabled = 1,
}
impl From<OC5FE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5FE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC5FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5FE_A {
        match self.bits {
            false => OC5FE_A::Disabled,
            true => OC5FE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5FE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5FE_A::Enabled
    }
}
///Field `OC5FE` writer - OC5FE
pub type OC5FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR3_OUTPUT_SPEC, OC5FE_A, O>;
impl<'a, const O: u8> OC5FE_W<'a, O> {
    ///Fast output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5FE_A::Disabled)
    }
    ///Fast output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5FE_A::Enabled)
    }
}
///Field `OC5PE` reader - OC5PE
pub type OC5PE_R = crate::BitReader<OC5PE_A>;
///OC5PE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5PE_A {
    ///0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC5PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5PE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC5PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5PE_A {
        match self.bits {
            false => OC5PE_A::Disabled,
            true => OC5PE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5PE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5PE_A::Enabled
    }
}
///Field `OC5PE` writer - OC5PE
pub type OC5PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR3_OUTPUT_SPEC, OC5PE_A, O>;
impl<'a, const O: u8> OC5PE_W<'a, O> {
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5PE_A::Disabled)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5PE_A::Enabled)
    }
}
///Field `OC5M` reader - OC5M
pub type OC5M_R = crate::FieldReader<u8, OC5M_A>;
///OC5M
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC5M_A {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    Frozen = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    ActiveOnMatch = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    InactiveOnMatch = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    Toggle = 3,
    ///4: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    ForceInactive = 4,
    ///5: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    ForceActive = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    PwmMode1 = 6,
    ///7: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    PwmMode2 = 7,
}
impl From<OC5M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC5M_A) -> Self {
        variant as _
    }
}
impl OC5M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5M_A {
        match self.bits {
            0 => OC5M_A::Frozen,
            1 => OC5M_A::ActiveOnMatch,
            2 => OC5M_A::InactiveOnMatch,
            3 => OC5M_A::Toggle,
            4 => OC5M_A::ForceInactive,
            5 => OC5M_A::ForceActive,
            6 => OC5M_A::PwmMode1,
            7 => OC5M_A::PwmMode2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC5M_A::Frozen
    }
    ///Checks if the value of the field is `ActiveOnMatch`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC5M_A::ActiveOnMatch
    }
    ///Checks if the value of the field is `InactiveOnMatch`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC5M_A::InactiveOnMatch
    }
    ///Checks if the value of the field is `Toggle`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC5M_A::Toggle
    }
    ///Checks if the value of the field is `ForceInactive`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC5M_A::ForceInactive
    }
    ///Checks if the value of the field is `ForceActive`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC5M_A::ForceActive
    }
    ///Checks if the value of the field is `PwmMode1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC5M_A::PwmMode1
    }
    ///Checks if the value of the field is `PwmMode2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC5M_A::PwmMode2
    }
}
///Field `OC5M` writer - OC5M
pub type OC5M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR3_OUTPUT_SPEC, u8, OC5M_A, 3, O>;
impl<'a, const O: u8> OC5M_W<'a, O> {
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC5M_A::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC5M_A::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC5M_A::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC5M_A::Toggle)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC5M_A::ForceInactive)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC5M_A::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC5M_A::PwmMode1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC5M_A::PwmMode2)
    }
}
///Field `OC5CE` reader - OC5CE
pub type OC5CE_R = crate::BitReader<OC5CE_A>;
///OC5CE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5CE_A {
    ///0: OCxRef is not affected by the ocref_clr_int signal
    Disabled = 0,
    ///1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
    Enabled = 1,
}
impl From<OC5CE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5CE_A) -> Self {
        variant as u8 != 0
    }
}
impl OC5CE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5CE_A {
        match self.bits {
            false => OC5CE_A::Disabled,
            true => OC5CE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5CE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5CE_A::Enabled
    }
}
///Field `OC5CE` writer - OC5CE
pub type OC5CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR3_OUTPUT_SPEC, OC5CE_A, O>;
impl<'a, const O: u8> OC5CE_W<'a, O> {
    ///OCxRef is not affected by the ocref_clr_int signal
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5CE_A::Disabled)
    }
    ///OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5CE_A::Enabled)
    }
}
///Field `OC6CE` reader - OC6CE
pub use OC5CE_R as OC6CE_R;
///Field `OC6CE` writer - OC6CE
pub use OC5CE_W as OC6CE_W;
///Field `OC6FE` reader - OC6FE
pub use OC5FE_R as OC6FE_R;
///Field `OC6FE` writer - OC6FE
pub use OC5FE_W as OC6FE_W;
///Field `OC6M` reader - OC6M
pub use OC5M_R as OC6M_R;
///Field `OC6M` writer - OC6M
pub use OC5M_W as OC6M_W;
///Field `OC6PE` reader - OC6PE
pub use OC5PE_R as OC6PE_R;
///Field `OC6PE` writer - OC6PE
pub use OC5PE_W as OC6PE_W;
///Field `OC5M_3` reader - OC5M
pub type OC5M_3_R = crate::BitReader<OC5M_3_A>;
///OC5M
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5M_3_A {
    ///0: Normal output compare mode (modes 0-7)
    Normal = 0,
    ///1: Extended output compare mode (modes 7-15)
    Extended = 1,
}
impl From<OC5M_3_A> for bool {
    #[inline(always)]
    fn from(variant: OC5M_3_A) -> Self {
        variant as u8 != 0
    }
}
impl OC5M_3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5M_3_A {
        match self.bits {
            false => OC5M_3_A::Normal,
            true => OC5M_3_A::Extended,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC5M_3_A::Normal
    }
    ///Checks if the value of the field is `Extended`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC5M_3_A::Extended
    }
}
///Field `OC5M_3` writer - OC5M
pub type OC5M_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR3_OUTPUT_SPEC, OC5M_3_A, O>;
impl<'a, const O: u8> OC5M_3_W<'a, O> {
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OC5M_3_A::Normal)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(OC5M_3_A::Extended)
    }
}
///Field `OC6M_3` reader - OC6M
pub use OC5M_3_R as OC6M_3_R;
///Field `OC6M_3` writer - OC6M
pub use OC5M_3_W as OC6M_3_W;
impl R {
    ///Bit 2 - OC5FE
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC5PE
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC5M
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - OC5CE
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - OC6FE
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OC6PE
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC6M
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - OC6CE
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC5M
    #[inline(always)]
    pub fn oc5m_3(&self) -> OC5M_3_R {
        OC5M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC6M
    #[inline(always)]
    pub fn oc6m_3(&self) -> OC6M_3_R {
        OC6M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - OC5FE
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<2> {
        OC5FE_W::new(self)
    }
    ///Bit 3 - OC5PE
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<3> {
        OC5PE_W::new(self)
    }
    ///Bits 4:6 - OC5M
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> OC5M_W<4> {
        OC5M_W::new(self)
    }
    ///Bit 7 - OC5CE
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<7> {
        OC5CE_W::new(self)
    }
    ///Bit 10 - OC6FE
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<10> {
        OC6FE_W::new(self)
    }
    ///Bit 11 - OC6PE
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<11> {
        OC6PE_W::new(self)
    }
    ///Bits 12:14 - OC6M
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> OC6M_W<12> {
        OC6M_W::new(self)
    }
    ///Bit 15 - OC6CE
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<15> {
        OC6CE_W::new(self)
    }
    ///Bit 16 - OC5M
    #[inline(always)]
    #[must_use]
    pub fn oc5m_3(&mut self) -> OC5M_3_W<16> {
        OC5M_3_W::new(self)
    }
    ///Bit 24 - OC6M
    #[inline(always)]
    #[must_use]
    pub fn oc6m_3(&mut self) -> OC6M_3_W<24> {
        OC6M_3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr3_output](index.html) module
pub struct CCMR3_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR3_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr3_output::R](R) reader structure
impl crate::Readable for CCMR3_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr3_output::W](W) writer structure
impl crate::Writable for CCMR3_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR3_Output to value 0
impl crate::Resettable for CCMR3_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
