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
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_R = crate::FieldReader<u8, u8>;
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC1FE` reader - Output compare 1 fast enable
pub type OC1FE_R = crate::BitReader<bool>;
///Field `OC1FE` writer - Output compare 1 fast enable
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1PE` reader - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_R = crate::BitReader<bool>;
///Field `OC1PE` writer - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1M1` reader - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from âfrozenâ mode to âPWMâ mode. Note: The OC1M\[3\]
///bit is not contiguous, located in bit 16.
pub type OC1M1_R = crate::FieldReader<u8, u8>;
///Field `OC1M1` writer - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from âfrozenâ mode to âPWMâ mode. Note: The OC1M\[3\]
///bit is not contiguous, located in bit 16.
pub type OC1M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 3, O>;
///Field `OC1CE` reader - Output compare 1 clear enable
pub type OC1CE_R = crate::BitReader<bool>;
///Field `OC1CE` writer - Output compare 1 clear enable
pub type OC1CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `CC2S` reader - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_R = crate::FieldReader<u8, u8>;
///Field `CC2S` writer - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC2FE` reader - Output compare 2 fast enable
pub type OC2FE_R = crate::BitReader<bool>;
///Field `OC2FE` writer - Output compare 2 fast enable
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2PE` reader - Output compare 2 preload enable
pub type OC2PE_R = crate::BitReader<bool>;
///Field `OC2PE` writer - Output compare 2 preload enable
pub type OC2PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2M` reader - Output compare 2 mode
pub type OC2M_R = crate::FieldReader<u8, OC2M_A>;
///Output compare 2 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC2M_A {
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
impl From<OC2M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC2M_A) -> Self {
        variant as _
    }
}
impl OC2M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC2M_A {
        match self.bits {
            0 => OC2M_A::Frozen,
            1 => OC2M_A::ActiveOnMatch,
            2 => OC2M_A::InactiveOnMatch,
            3 => OC2M_A::Toggle,
            4 => OC2M_A::ForceInactive,
            5 => OC2M_A::ForceActive,
            6 => OC2M_A::PwmMode1,
            7 => OC2M_A::PwmMode2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC2M_A::Frozen
    }
    ///Checks if the value of the field is `ActiveOnMatch`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC2M_A::ActiveOnMatch
    }
    ///Checks if the value of the field is `InactiveOnMatch`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC2M_A::InactiveOnMatch
    }
    ///Checks if the value of the field is `Toggle`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC2M_A::Toggle
    }
    ///Checks if the value of the field is `ForceInactive`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC2M_A::ForceInactive
    }
    ///Checks if the value of the field is `ForceActive`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC2M_A::ForceActive
    }
    ///Checks if the value of the field is `PwmMode1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC2M_A::PwmMode1
    }
    ///Checks if the value of the field is `PwmMode2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC2M_A::PwmMode2
    }
}
///Field `OC2M` writer - Output compare 2 mode
pub type OC2M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_OUTPUT_SPEC, u8, OC2M_A, 3, O>;
impl<'a, const O: u8> OC2M_W<'a, O> {
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC2M_A::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC2M_A::Toggle)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC2M_A::ForceInactive)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC2M_A::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::PwmMode1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::PwmMode2)
    }
}
///Field `OC2CE` reader - Output compare 2 clear enable
pub type OC2CE_R = crate::BitReader<bool>;
///Field `OC2CE` writer - Output compare 2 clear enable
pub type OC2CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1M_3` reader - Output Compare 1 mode - bit 3
pub type OC1M_3_R = crate::BitReader<OC1M_3_A>;
///Output Compare 1 mode - bit 3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1M_3_A {
    ///0: Normal output compare mode (modes 0-7)
    Normal = 0,
    ///1: Extended output compare mode (modes 7-15)
    Extended = 1,
}
impl From<OC1M_3_A> for bool {
    #[inline(always)]
    fn from(variant: OC1M_3_A) -> Self {
        variant as u8 != 0
    }
}
impl OC1M_3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1M_3_A {
        match self.bits {
            false => OC1M_3_A::Normal,
            true => OC1M_3_A::Extended,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC1M_3_A::Normal
    }
    ///Checks if the value of the field is `Extended`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC1M_3_A::Extended
    }
}
///Field `OC1M_3` writer - Output Compare 1 mode - bit 3
pub type OC1M_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, OC1M_3_A, O>;
impl<'a, const O: u8> OC1M_3_W<'a, O> {
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OC1M_3_A::Normal)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(OC1M_3_A::Extended)
    }
}
///Field `OC2M_3` reader - Output Compare 2 mode - bit 3
pub use OC1M_3_R as OC2M_3_R;
///Field `OC2M_3` writer - Output Compare 2 mode - bit 3
pub use OC1M_3_W as OC2M_3_W;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from âfrozenâ mode to âPWMâ mode. Note: The OC1M\[3\]
    ///bit is not contiguous, located in bit 16.
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output Compare 1 mode - bit 3
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output Compare 2 mode - bit 3
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    ///Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    ///Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from âfrozenâ mode to âPWMâ mode. Note: The OC1M\[3\]
    ///bit is not contiguous, located in bit 16.
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> OC1M1_W<4> {
        OC1M1_W::new(self)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<7> {
        OC1CE_W::new(self)
    }
    ///Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<10> {
        OC2FE_W::new(self)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<11> {
        OC2PE_W::new(self)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<12> {
        OC2M_W::new(self)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<15> {
        OC2CE_W::new(self)
    }
    ///Bit 16 - Output Compare 1 mode - bit 3
    #[inline(always)]
    #[must_use]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<16> {
        OC1M_3_W::new(self)
    }
    ///Bit 24 - Output Compare 2 mode - bit 3
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<24> {
        OC2M_3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 1 (output mode)
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
