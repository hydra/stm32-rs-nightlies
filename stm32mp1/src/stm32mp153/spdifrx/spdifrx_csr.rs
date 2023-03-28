///Register `SPDIFRX_CSR` reader
pub struct R(crate::R<SPDIFRX_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `USR` reader - USR
pub type USR_R = crate::FieldReader<u16, u16>;
///Field `CS` reader - CS
pub type CS_R = crate::FieldReader<u8, u8>;
///Field `SOB` reader - SOB
pub type SOB_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:15 - USR
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - CS
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - SOB
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
///Channel status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spdifrx_csr](index.html) module
pub struct SPDIFRX_CSR_SPEC;
impl crate::RegisterSpec for SPDIFRX_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spdifrx_csr::R](R) reader structure
impl crate::Readable for SPDIFRX_CSR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPDIFRX_CSR to value 0
impl crate::Resettable for SPDIFRX_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
