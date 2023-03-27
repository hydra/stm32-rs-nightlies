///Register `SECWM1R2` reader
pub struct R(crate::R<SECWM1R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECWM1R2` writer
pub struct W(crate::W<SECWM1R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM1R2_SPEC>;
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
impl From<crate::W<SECWM1R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM1R2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP1_PSTRT` reader - PCROP1_PSTRT
pub type PCROP1_PSTRT_R = crate::FieldReader<u8, u8>;
///Field `PCROP1_PSTRT` writer - PCROP1_PSTRT
pub type PCROP1_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM1R2_SPEC, u8, u8, 7, O>;
///Field `PCROP1EN` reader - PCROP1EN
pub type PCROP1EN_R = crate::BitReader<bool>;
///Field `PCROP1EN` writer - PCROP1EN
pub type PCROP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECWM1R2_SPEC, bool, O>;
///Field `HDP1_PEND` reader - HDP1_PEND
pub type HDP1_PEND_R = crate::FieldReader<u8, u8>;
///Field `HDP1_PEND` writer - HDP1_PEND
pub type HDP1_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM1R2_SPEC, u8, u8, 7, O>;
///Field `HDP1EN` reader - HDP1EN
pub type HDP1EN_R = crate::BitReader<bool>;
///Field `HDP1EN` writer - HDP1EN
pub type HDP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECWM1R2_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - PCROP1_PSTRT
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 15 - PCROP1EN
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - HDP1_PEND
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - HDP1EN
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - PCROP1_PSTRT
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W<0> {
        PCROP1_PSTRT_W::new(self)
    }
    ///Bit 15 - PCROP1EN
    #[inline(always)]
    #[must_use]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W<15> {
        PCROP1EN_W::new(self)
    }
    ///Bits 16:22 - HDP1_PEND
    #[inline(always)]
    #[must_use]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<16> {
        HDP1_PEND_W::new(self)
    }
    ///Bit 31 - HDP1EN
    #[inline(always)]
    #[must_use]
    pub fn hdp1en(&mut self) -> HDP1EN_W<31> {
        HDP1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash secure watermak1 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secwm1r2](index.html) module
pub struct SECWM1R2_SPEC;
impl crate::RegisterSpec for SECWM1R2_SPEC {
    type Ux = u32;
}
///`read()` method returns [secwm1r2::R](R) reader structure
impl crate::Readable for SECWM1R2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [secwm1r2::W](W) writer structure
impl crate::Writable for SECWM1R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECWM1R2 to value 0x0f00_0f00
impl crate::Resettable for SECWM1R2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_0f00;
}
