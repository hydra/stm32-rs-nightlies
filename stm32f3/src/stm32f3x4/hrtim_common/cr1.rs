///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MUDIS` reader - Master Update Disable
pub type MUDIS_R = crate::BitReader<MUDIS_A>;
///Master Update Disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUDIS_A {
    ///0: Timer update enabled
    Enabled = 0,
    ///1: Timer update disabled
    Disabled = 1,
}
impl From<MUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MUDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUDIS_A {
        match self.bits {
            false => MUDIS_A::Enabled,
            true => MUDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUDIS_A::Disabled
    }
}
///Field `MUDIS` writer - Master Update Disable
pub type MUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MUDIS_A, O>;
impl<'a, const O: u8> MUDIS_W<'a, O> {
    ///Timer update enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUDIS_A::Enabled)
    }
    ///Timer update disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUDIS_A::Disabled)
    }
}
///Field `TAUDIS` reader - Timer A Update Disable
pub use MUDIS_R as TAUDIS_R;
///Field `TBUDIS` reader - Timer B Update Disable
pub use MUDIS_R as TBUDIS_R;
///Field `TCUDIS` reader - Timer C Update Disable
pub use MUDIS_R as TCUDIS_R;
///Field `TDUDIS` reader - Timer D Update Disable
pub use MUDIS_R as TDUDIS_R;
///Field `TEUDIS` reader - Timer E Update Disable
pub use MUDIS_R as TEUDIS_R;
///Field `TAUDIS` writer - Timer A Update Disable
pub use MUDIS_W as TAUDIS_W;
///Field `TBUDIS` writer - Timer B Update Disable
pub use MUDIS_W as TBUDIS_W;
///Field `TCUDIS` writer - Timer C Update Disable
pub use MUDIS_W as TCUDIS_W;
///Field `TDUDIS` writer - Timer D Update Disable
pub use MUDIS_W as TDUDIS_W;
///Field `TEUDIS` writer - Timer E Update Disable
pub use MUDIS_W as TEUDIS_W;
///Field `AD1USRC` reader - ADC Trigger 1 Update Source
pub type AD1USRC_R = crate::FieldReader<u8, AD1USRC_A>;
///ADC Trigger 1 Update Source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AD1USRC_A {
    ///0: ADC trigger update from master timer
    Master = 0,
    ///1: ADC trigger update from timer A
    TimerA = 1,
    ///2: ADC trigger update from timer B
    TimerB = 2,
    ///3: ADC trigger update from timer C
    TimerC = 3,
    ///4: ADC trigger update from timer D
    TimerD = 4,
    ///5: ADC trigger update from timer E
    TimerE = 5,
}
impl From<AD1USRC_A> for u8 {
    #[inline(always)]
    fn from(variant: AD1USRC_A) -> Self {
        variant as _
    }
}
impl AD1USRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<AD1USRC_A> {
        match self.bits {
            0 => Some(AD1USRC_A::Master),
            1 => Some(AD1USRC_A::TimerA),
            2 => Some(AD1USRC_A::TimerB),
            3 => Some(AD1USRC_A::TimerC),
            4 => Some(AD1USRC_A::TimerD),
            5 => Some(AD1USRC_A::TimerE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Master`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == AD1USRC_A::Master
    }
    ///Checks if the value of the field is `TimerA`
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == AD1USRC_A::TimerA
    }
    ///Checks if the value of the field is `TimerB`
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == AD1USRC_A::TimerB
    }
    ///Checks if the value of the field is `TimerC`
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == AD1USRC_A::TimerC
    }
    ///Checks if the value of the field is `TimerD`
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == AD1USRC_A::TimerD
    }
    ///Checks if the value of the field is `TimerE`
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == AD1USRC_A::TimerE
    }
}
///Field `AD1USRC` writer - ADC Trigger 1 Update Source
pub type AD1USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, AD1USRC_A, 3, O>;
impl<'a, const O: u8> AD1USRC_W<'a, O> {
    ///ADC trigger update from master timer
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(AD1USRC_A::Master)
    }
    ///ADC trigger update from timer A
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD1USRC_A::TimerA)
    }
    ///ADC trigger update from timer B
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD1USRC_A::TimerB)
    }
    ///ADC trigger update from timer C
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD1USRC_A::TimerC)
    }
    ///ADC trigger update from timer D
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD1USRC_A::TimerD)
    }
    ///ADC trigger update from timer E
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD1USRC_A::TimerE)
    }
}
///Field `AD2USRC` reader - ADC Trigger 2 Update Source
pub use AD1USRC_R as AD2USRC_R;
///Field `AD3USRC` reader - ADC Trigger 3 Update Source
pub use AD1USRC_R as AD3USRC_R;
///Field `AD4USRC` reader - ADC Trigger 4 Update Source
pub use AD1USRC_R as AD4USRC_R;
///Field `AD2USRC` writer - ADC Trigger 2 Update Source
pub use AD1USRC_W as AD2USRC_W;
///Field `AD3USRC` writer - ADC Trigger 3 Update Source
pub use AD1USRC_W as AD3USRC_W;
///Field `AD4USRC` writer - ADC Trigger 4 Update Source
pub use AD1USRC_W as AD4USRC_W;
impl R {
    ///Bit 0 - Master Update Disable
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A Update Disable
    #[inline(always)]
    pub fn taudis(&self) -> TAUDIS_R {
        TAUDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Update Disable
    #[inline(always)]
    pub fn tbudis(&self) -> TBUDIS_R {
        TBUDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer C Update Disable
    #[inline(always)]
    pub fn tcudis(&self) -> TCUDIS_R {
        TCUDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer D Update Disable
    #[inline(always)]
    pub fn tdudis(&self) -> TDUDIS_R {
        TDUDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer E Update Disable
    #[inline(always)]
    pub fn teudis(&self) -> TEUDIS_R {
        TEUDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 16:18 - ADC Trigger 1 Update Source
    #[inline(always)]
    pub fn ad1usrc(&self) -> AD1USRC_R {
        AD1USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - ADC Trigger 2 Update Source
    #[inline(always)]
    pub fn ad2usrc(&self) -> AD2USRC_R {
        AD2USRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:24 - ADC Trigger 3 Update Source
    #[inline(always)]
    pub fn ad3usrc(&self) -> AD3USRC_R {
        AD3USRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:27 - ADC Trigger 4 Update Source
    #[inline(always)]
    pub fn ad4usrc(&self) -> AD4USRC_R {
        AD4USRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Master Update Disable
    #[inline(always)]
    #[must_use]
    pub fn mudis(&mut self) -> MUDIS_W<0> {
        MUDIS_W::new(self)
    }
    ///Bit 1 - Timer A Update Disable
    #[inline(always)]
    #[must_use]
    pub fn taudis(&mut self) -> TAUDIS_W<1> {
        TAUDIS_W::new(self)
    }
    ///Bit 2 - Timer B Update Disable
    #[inline(always)]
    #[must_use]
    pub fn tbudis(&mut self) -> TBUDIS_W<2> {
        TBUDIS_W::new(self)
    }
    ///Bit 3 - Timer C Update Disable
    #[inline(always)]
    #[must_use]
    pub fn tcudis(&mut self) -> TCUDIS_W<3> {
        TCUDIS_W::new(self)
    }
    ///Bit 4 - Timer D Update Disable
    #[inline(always)]
    #[must_use]
    pub fn tdudis(&mut self) -> TDUDIS_W<4> {
        TDUDIS_W::new(self)
    }
    ///Bit 5 - Timer E Update Disable
    #[inline(always)]
    #[must_use]
    pub fn teudis(&mut self) -> TEUDIS_W<5> {
        TEUDIS_W::new(self)
    }
    ///Bits 16:18 - ADC Trigger 1 Update Source
    #[inline(always)]
    #[must_use]
    pub fn ad1usrc(&mut self) -> AD1USRC_W<16> {
        AD1USRC_W::new(self)
    }
    ///Bits 19:21 - ADC Trigger 2 Update Source
    #[inline(always)]
    #[must_use]
    pub fn ad2usrc(&mut self) -> AD2USRC_W<19> {
        AD2USRC_W::new(self)
    }
    ///Bits 22:24 - ADC Trigger 3 Update Source
    #[inline(always)]
    #[must_use]
    pub fn ad3usrc(&mut self) -> AD3USRC_W<22> {
        AD3USRC_W::new(self)
    }
    ///Bits 25:27 - ADC Trigger 4 Update Source
    #[inline(always)]
    #[must_use]
    pub fn ad4usrc(&mut self) -> AD4USRC_W<25> {
        AD4USRC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
