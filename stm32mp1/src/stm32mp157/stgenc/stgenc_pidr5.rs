///Register `STGENC_PIDR5` reader
pub struct R(crate::R<STGENC_PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PIDR5` reader - PIDR5
pub type PIDR5_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - PIDR5
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
///STGENC peripheral ID5 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenc_pidr5](index.html) module
pub struct STGENC_PIDR5_SPEC;
impl crate::RegisterSpec for STGENC_PIDR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenc_pidr5::R](R) reader structure
impl crate::Readable for STGENC_PIDR5_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENC_PIDR5 to value 0
impl crate::Resettable for STGENC_PIDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
