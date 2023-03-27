///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AWD` reader - Analog watchdog flag
pub type AWD_R = crate::BitReader<AWDR_A>;
///Analog watchdog flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDR_A {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWDR_A> for bool {
    #[inline(always)]
    fn from(variant: AWDR_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDR_A {
        match self.bits {
            false => AWDR_A::NoEvent,
            true => AWDR_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWDR_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWDR_A::Event
    }
}
///Analog watchdog flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<AWDW_AW> for bool {
    #[inline(always)]
    fn from(variant: AWDW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` writer - Analog watchdog flag
pub type AWD_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, AWDW_AW, O>;
impl<'a, const O: u8> AWD_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWDW_AW::Clear)
    }
}
///Field `EOC` reader - Regular channel end of conversion
pub type EOC_R = crate::BitReader<EOCR_A>;
///Regular channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
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
///Regular channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - Regular channel end of conversion
pub type EOC_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, EOCW_AW, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCW_AW::Clear)
    }
}
///Field `JEOC` reader - Injected channel end of conversion
pub type JEOC_R = crate::BitReader<JEOCR_A>;
///Injected channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR_A {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
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
///Injected channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<JEOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` writer - Injected channel end of conversion
pub type JEOC_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, JEOCW_AW, O>;
impl<'a, const O: u8> JEOC_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOCW_AW::Clear)
    }
}
///Field `JSTRT` reader - Injected channel start flag
pub type JSTRT_R = crate::BitReader<JSTRTR_A>;
///Injected channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTR_A {
    ///0: No injected channel conversion started
    NotStarted = 0,
    ///1: Injected channel conversion has started
    Started = 1,
}
impl From<JSTRTR_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl JSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JSTRTR_A {
        match self.bits {
            false => JSTRTR_A::NotStarted,
            true => JSTRTR_A::Started,
        }
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRTR_A::NotStarted
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRTR_A::Started
    }
}
///Injected channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<JSTRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: JSTRTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSTRT` writer - Injected channel start flag
pub type JSTRT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, JSTRTW_AW, O>;
impl<'a, const O: u8> JSTRT_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JSTRTW_AW::Clear)
    }
}
///Field `STRT` reader - Regular channel start flag
pub type STRT_R = crate::BitReader<STRTR_A>;
///Regular channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR_A {
    ///0: No regular channel conversion started
    NotStarted = 0,
    ///1: Regular channel conversion has started
    Started = 1,
}
impl From<STRTR_A> for bool {
    #[inline(always)]
    fn from(variant: STRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STRTR_A {
        match self.bits {
            false => STRTR_A::NotStarted,
            true => STRTR_A::Started,
        }
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRTR_A::NotStarted
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRTR_A::Started
    }
}
///Regular channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<STRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STRTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` writer - Regular channel start flag
pub type STRT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, STRTW_AW, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STRTW_AW::Clear)
    }
}
///Field `OVR` reader - Overrun
pub type OVR_R = crate::BitReader<bool>;
///Field `OVR` writer - Overrun
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    #[must_use]
    pub fn awd(&mut self) -> AWD_W<0> {
        AWD_W::new(self)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<1> {
        EOC_W::new(self)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<2> {
        JEOC_W::new(self)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    #[must_use]
    pub fn jstrt(&mut self) -> JSTRT_W<3> {
        JSTRT_W::new(self)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<4> {
        STRT_W::new(self)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<5> {
        OVR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
