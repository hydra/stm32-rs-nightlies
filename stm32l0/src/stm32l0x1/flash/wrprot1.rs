///Register `WRPROT1` reader
pub struct R(crate::R<WRPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROT1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WRPROT1` reader - Write Protection
pub type WRPROT1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Write Protection
    #[inline(always)]
    pub fn wrprot1(&self) -> WRPROT1_R {
        WRPROT1_R::new(self.bits)
    }
}
///Write Protection Register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrprot1](index.html) module
pub struct WRPROT1_SPEC;
impl crate::RegisterSpec for WRPROT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrprot1::R](R) reader structure
impl crate::Readable for WRPROT1_SPEC {
    type Reader = R;
}
///`reset()` method sets WRPROT1 to value 0
impl crate::Resettable for WRPROT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
