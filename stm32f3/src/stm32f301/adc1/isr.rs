///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISR` writer
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADRDY` reader - ADC ready
pub type ADRDY_R = crate::BitReader<ADRDYR_A>;
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR_A {
    ///0: ADC is not ready to start conversion
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDYR_A {
        match self.bits {
            false => ADRDYR_A::NotReady,
            true => ADRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR_A::Ready
    }
}
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW_AW {
    ///1: Clear ADC is ready to start conversion flag
    Clear = 1,
}
impl From<ADRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADC ready
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, ADRDYW_AW, O>;
impl<'a, const O: u8> ADRDY_W<'a, O> {
    ///Clear ADC is ready to start conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDYW_AW::Clear)
    }
}
///Field `EOSMP` reader - End of sampling flag
pub type EOSMP_R = crate::BitReader<EOSMPR_A>;
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR_A {
    ///0: End of sampling phase no yet reached
    NotEnded = 0,
    ///1: End of sampling phase reached
    Ended = 1,
}
impl From<EOSMPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMPR_A {
        match self.bits {
            false => EOSMPR_A::NotEnded,
            true => EOSMPR_A::Ended,
        }
    }
    ///Checks if the value of the field is `NotEnded`
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMPR_A::NotEnded
    }
    ///Checks if the value of the field is `Ended`
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMPR_A::Ended
    }
}
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW_AW {
    ///1: Clear end of sampling phase reached flag
    Clear = 1,
}
impl From<EOSMPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - End of sampling flag
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOSMPW_AW, O>;
impl<'a, const O: u8> EOSMP_W<'a, O> {
    ///Clear end of sampling phase reached flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMPW_AW::Clear)
    }
}
///Field `EOC` reader - End of conversion flag
pub type EOC_R = crate::BitReader<EOCR_A>;
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    ///0: Regular conversion is not complete
    NotComplete = 0,
    ///1: Regular conversion complete
    Complete = 1,
}
impl From<EOCR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCR_A {
        match self.bits {
            false => EOCR_A::NotComplete,
            true => EOCR_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR_A::Complete
    }
}
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    ///1: Clear regular conversion complete flag
    Clear = 1,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - End of conversion flag
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOCW_AW, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    ///Clear regular conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCW_AW::Clear)
    }
}
///Field `EOS` reader - End of regular sequence flag
pub type EOS_R = crate::BitReader<EOSR_A>;
///End of regular sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR_A {
    ///0: Regular sequence is not complete
    NotComplete = 0,
    ///1: Regular sequence complete
    Complete = 1,
}
impl From<EOSR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSR_A {
        match self.bits {
            false => EOSR_A::NotComplete,
            true => EOSR_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR_A::Complete
    }
}
///End of regular sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW_AW {
    ///1: Clear regular sequence complete flag
    Clear = 1,
}
impl From<EOSW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - End of regular sequence flag
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOSW_AW, O>;
impl<'a, const O: u8> EOS_W<'a, O> {
    ///Clear regular sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSW_AW::Clear)
    }
}
///Field `OVR` reader - ADC overrun
pub type OVR_R = crate::BitReader<OVRR_A>;
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NoOverrun,
            true => OVRR_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::Overrun
    }
}
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW_AW {
    ///1: Clear overrun occurred flag
    Clear = 1,
}
impl From<OVRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - ADC overrun
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, OVRW_AW, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    ///Clear overrun occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRW_AW::Clear)
    }
}
///Field `JEOC` reader - Injected channel end of conversion flag
pub type JEOC_R = crate::BitReader<JEOCR_A>;
///Injected channel end of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR_A {
    ///0: Injected conversion is not complete
    NotComplete = 0,
    ///1: Injected conversion complete
    Complete = 1,
}
impl From<JEOCR_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOCR_A {
        match self.bits {
            false => JEOCR_A::NotComplete,
            true => JEOCR_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR_A::Complete
    }
}
///Injected channel end of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW_AW {
    ///1: Clear injected conversion complete flag
    Clear = 1,
}
impl From<JEOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` writer - Injected channel end of conversion flag
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, JEOCW_AW, O>;
impl<'a, const O: u8> JEOC_W<'a, O> {
    ///Clear injected conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOCW_AW::Clear)
    }
}
///Field `JEOS` reader - Injected channel end of sequence flag
pub type JEOS_R = crate::BitReader<JEOSR_A>;
///Injected channel end of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSR_A {
    ///0: Injected sequence is not complete
    NotComplete = 0,
    ///1: Injected sequence complete
    Complete = 1,
}
impl From<JEOSR_A> for bool {
    #[inline(always)]
    fn from(variant: JEOSR_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOSR_A {
        match self.bits {
            false => JEOSR_A::NotComplete,
            true => JEOSR_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOSR_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOSR_A::Complete
    }
}
///Injected channel end of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSW_AW {
    ///1: Clear Injected sequence complete flag
    Clear = 1,
}
impl From<JEOSW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOSW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS` writer - Injected channel end of sequence flag
pub type JEOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, JEOSW_AW, O>;
impl<'a, const O: u8> JEOS_W<'a, O> {
    ///Clear Injected sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOSW_AW::Clear)
    }
}
///Field `AWD1` reader - Analog watchdog 1 flag
pub type AWD1_R = crate::BitReader<AWD1R_A>;
///Analog watchdog 1 flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R_A {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1R_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1R_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1R_A {
        match self.bits {
            false => AWD1R_A::NoEvent,
            true => AWD1R_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R_A::Event
    }
}
///Analog watchdog 1 flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W_AW {
    ///1: Clear analog watchdog event occurred flag
    Clear = 1,
}
impl From<AWD1W_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD1W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1` writer - Analog watchdog 1 flag
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, AWD1W_AW, O>;
impl<'a, const O: u8> AWD1_W<'a, O> {
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD1W_AW::Clear)
    }
}
///Field `AWD2` reader - Analog watchdog 2 flag
pub use AWD1_R as AWD2_R;
///Field `AWD3` reader - Analog watchdog 3 flag
pub use AWD1_R as AWD3_R;
///Field `AWD2` writer - Analog watchdog 2 flag
pub use AWD1_W as AWD2_W;
///Field `AWD3` writer - Analog watchdog 3 flag
pub use AWD1_W as AWD3_W;
///Field `JQOVF` reader - Injected context queue overflow
pub type JQOVF_R = crate::BitReader<JQOVFR_A>;
///Injected context queue overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFR_A {
    ///0: No injected context queue overflow has occurred
    NoOverflow = 0,
    ///1: Injected context queue overflow has occurred
    Overflow = 1,
}
impl From<JQOVFR_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVFR_A) -> Self {
        variant as u8 != 0
    }
}
impl JQOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JQOVFR_A {
        match self.bits {
            false => JQOVFR_A::NoOverflow,
            true => JQOVFR_A::Overflow,
        }
    }
    ///Checks if the value of the field is `NoOverflow`
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVFR_A::NoOverflow
    }
    ///Checks if the value of the field is `Overflow`
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVFR_A::Overflow
    }
}
///Injected context queue overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFW_AW {
    ///1: Clear injected context queue overflow flag
    Clear = 1,
}
impl From<JQOVFW_AW> for bool {
    #[inline(always)]
    fn from(variant: JQOVFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF` writer - Injected context queue overflow
pub type JQOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, JQOVFW_AW, O>;
impl<'a, const O: u8> JQOVF_W<'a, O> {
    ///Clear injected context queue overflow flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JQOVFW_AW::Clear)
    }
}
impl R {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADC ready
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<5> {
        JEOC_W::new(self)
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JEOS_W<6> {
        JEOS_W::new(self)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    #[must_use]
    pub fn jqovf(&mut self) -> JQOVF_W<10> {
        JQOVF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [isr::W](W) writer structure
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
