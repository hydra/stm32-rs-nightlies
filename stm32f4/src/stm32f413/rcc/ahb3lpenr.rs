///Register `AHB3LPENR` reader
pub struct R(crate::R<AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3LPENR` writer
pub struct W(crate::W<AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3LPENR_SPEC>;
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
impl From<crate::W<AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FSMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode
pub type FSMCLPEN_R = crate::BitReader<FSMCLPEN_A>;
///Flexible memory controller module clock enable during Sleep mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCLPEN_A {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<FSMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FSMCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSMCLPEN_A {
        match self.bits {
            false => FSMCLPEN_A::DisabledInSleep,
            true => FSMCLPEN_A::EnabledInSleep,
        }
    }
    ///Checks if the value of the field is `DisabledInSleep`
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::DisabledInSleep
    }
    ///Checks if the value of the field is `EnabledInSleep`
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::EnabledInSleep
    }
}
///Field `FSMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode
pub type FSMCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3LPENR_SPEC, FSMCLPEN_A, O>;
impl<'a, const O: u8> FSMCLPEN_W<'a, O> {
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::EnabledInSleep)
    }
}
///Field `QSPILPEN` reader - QUADSPI memory controller module clock enable during Sleep mode
pub use FSMCLPEN_R as QSPILPEN_R;
///Field `QSPILPEN` writer - QUADSPI memory controller module clock enable during Sleep mode
pub use FSMCLPEN_W as QSPILPEN_W;
impl R {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - QUADSPI memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W<0> {
        FSMCLPEN_W::new(self)
    }
    ///Bit 1 - QUADSPI memory controller module clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<1> {
        QSPILPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB3 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3lpenr](index.html) module
pub struct AHB3LPENR_SPEC;
impl crate::RegisterSpec for AHB3LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3lpenr::R](R) reader structure
impl crate::Readable for AHB3LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3lpenr::W](W) writer structure
impl crate::Writable for AHB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3LPENR to value 0x03
impl crate::Resettable for AHB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
