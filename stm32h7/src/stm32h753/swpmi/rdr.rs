///Register `RDR` reader
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RD` reader - received data
pub type RD_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - received data
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(self.bits)
    }
}
///SWPMI Receive data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdr](index.html) module
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdr::R](R) reader structure
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
