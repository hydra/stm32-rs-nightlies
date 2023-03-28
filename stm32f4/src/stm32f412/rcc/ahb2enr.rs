///Register `AHB2ENR` reader
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2ENR` writer
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RNGEN` reader - RNGEN
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
///RNGEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::Disabled,
            true => RNGEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN_A::Enabled
    }
}
///Field `RNGEN` writer - RNGEN
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, RNGEN_A, O>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Enabled)
    }
}
///Field `OTGFSEN` reader - USB OTG FS clock enable
pub use RNGEN_R as OTGFSEN_R;
///Field `OTGFSEN` writer - USB OTG FS clock enable
pub use RNGEN_W as OTGFSEN_W;
impl R {
    ///Bit 6 - RNGEN
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - RNGEN
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<6> {
        RNGEN_W::new(self)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<7> {
        OTGFSEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](index.html) module
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2enr::R](R) reader structure
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2enr::W](W) writer structure
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
