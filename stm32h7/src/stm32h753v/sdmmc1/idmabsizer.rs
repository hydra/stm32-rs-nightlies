///Register `IDMABSIZER` reader
pub struct R(crate::R<IDMABSIZER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMABSIZER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMABSIZER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMABSIZER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IDMABSIZER` writer
pub struct W(crate::W<IDMABSIZER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMABSIZER_SPEC>;
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
impl From<crate::W<IDMABSIZER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDMABSIZER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDMABNDT` reader - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_R = crate::FieldReader<u8, u8>;
///Field `IDMABNDT` writer - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDMABSIZER_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0xff) as u8)
    }
}
impl W {
    ///Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
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
///The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idmabsizer](index.html) module
pub struct IDMABSIZER_SPEC;
impl crate::RegisterSpec for IDMABSIZER_SPEC {
    type Ux = u32;
}
///`read()` method returns [idmabsizer::R](R) reader structure
impl crate::Readable for IDMABSIZER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [idmabsizer::W](W) writer structure
impl crate::Writable for IDMABSIZER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IDMABSIZER to value 0
impl crate::Resettable for IDMABSIZER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
