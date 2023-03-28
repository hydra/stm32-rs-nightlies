///Register `SDMMC_IDMABSIZER` reader
pub struct R(crate::R<SDMMC_IDMABSIZER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABSIZER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABSIZER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABSIZER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDMMC_IDMABSIZER` writer
pub struct W(crate::W<SDMMC_IDMABSIZER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABSIZER_SPEC>;
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
impl From<crate::W<SDMMC_IDMABSIZER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABSIZER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDMABNDT` reader - IDMABNDT
pub type IDMABNDT_R = crate::FieldReader<u16, u16>;
///Field `IDMABNDT` writer - IDMABNDT
pub type IDMABNDT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_IDMABSIZER_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 5:16 - IDMABNDT
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 5:16 - IDMABNDT
    #[inline(always)]
    #[must_use]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<5> {
        IDMABNDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_idmabsizer](index.html) module
pub struct SDMMC_IDMABSIZER_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABSIZER_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_idmabsizer::R](R) reader structure
impl crate::Readable for SDMMC_IDMABSIZER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdmmc_idmabsizer::W](W) writer structure
impl crate::Writable for SDMMC_IDMABSIZER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDMMC_IDMABSIZER to value 0
impl crate::Resettable for SDMMC_IDMABSIZER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
