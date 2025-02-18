///Register `BSEC_IPIDR` reader
pub struct R(crate::R<BSEC_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
///BSEC identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_ipidr](index.html) module
pub struct BSEC_IPIDR_SPEC;
impl crate::RegisterSpec for BSEC_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_ipidr::R](R) reader structure
impl crate::Readable for BSEC_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets BSEC_IPIDR to value 0x0010_0032
impl crate::Resettable for BSEC_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0032;
}
