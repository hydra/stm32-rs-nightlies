///Register `SPDIFRX_DIR` reader
pub struct R(crate::R<SPDIFRX_DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_DIR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `THI` reader - THI
pub type THI_R = crate::FieldReader<u16, u16>;
///Field `TLO` reader - TLO
pub type TLO_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:12 - THI
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - TLO
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
///Debug information register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spdifrx_dir](index.html) module
pub struct SPDIFRX_DIR_SPEC;
impl crate::RegisterSpec for SPDIFRX_DIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spdifrx_dir::R](R) reader structure
impl crate::Readable for SPDIFRX_DIR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPDIFRX_DIR to value 0
impl crate::Resettable for SPDIFRX_DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
