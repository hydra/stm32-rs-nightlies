///Register `ECCR2` reader
pub struct R(crate::R<ECCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ECCx` reader - ECCx
pub type ECCX_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECCx
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new(self.bits)
    }
}
///ECC result register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr2](index.html) module
pub struct ECCR2_SPEC;
impl crate::RegisterSpec for ECCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [eccr2::R](R) reader structure
impl crate::Readable for ECCR2_SPEC {
    type Reader = R;
}
///`reset()` method sets ECCR2 to value 0
impl crate::Resettable for ECCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
