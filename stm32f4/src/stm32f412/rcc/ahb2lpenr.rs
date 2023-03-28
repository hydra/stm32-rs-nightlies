///Register `AHB2LPENR` reader
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2LPENR` writer
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RNGLPEN` reader - RNGLPEN
pub type RNGLPEN_R = crate::BitReader<RNGLPEN_A>;
///RNGLPEN
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGLPEN_A {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<RNGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGLPEN_A {
        match self.bits {
            false => RNGLPEN_A::DisabledInSleep,
            true => RNGLPEN_A::EnabledInSleep,
        }
    }
    ///Checks if the value of the field is `DisabledInSleep`
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == RNGLPEN_A::DisabledInSleep
    }
    ///Checks if the value of the field is `EnabledInSleep`
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == RNGLPEN_A::EnabledInSleep
    }
}
///Field `RNGLPEN` writer - RNGLPEN
pub type RNGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, RNGLPEN_A, O>;
impl<'a, const O: u8> RNGLPEN_W<'a, O> {
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(RNGLPEN_A::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(RNGLPEN_A::EnabledInSleep)
    }
}
///Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode
pub use RNGLPEN_R as OTGFSLPEN_R;
///Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode
pub use RNGLPEN_W as OTGFSLPEN_W;
impl R {
    ///Bit 6 - RNGLPEN
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS clock enable during Sleep mode
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - RNGLPEN
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<6> {
        RNGLPEN_W::new(self)
    }
    ///Bit 7 - USB OTG FS clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<7> {
        OTGFSLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2lpenr](index.html) module
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2lpenr::R](R) reader structure
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2LPENR to value 0xf1
impl crate::Resettable for AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf1;
}
