///Register `WRPROT2` reader
pub struct R(crate::R<WRPROT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROT2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WRPROT2` reader - Write Protection
pub type WRPROT2_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Write Protection
    #[inline(always)]
    pub fn wrprot2(&self) -> WRPROT2_R {
        WRPROT2_R::new((self.bits & 0xffff) as u16)
    }
}
///Write Protection Register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrprot2](index.html) module
pub struct WRPROT2_SPEC;
impl crate::RegisterSpec for WRPROT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrprot2::R](R) reader structure
impl crate::Readable for WRPROT2_SPEC {
    type Reader = R;
}
///`reset()` method sets WRPROT2 to value 0
impl crate::Resettable for WRPROT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
