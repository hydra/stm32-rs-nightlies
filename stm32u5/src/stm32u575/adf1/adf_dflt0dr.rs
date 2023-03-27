///Register `ADF_DFLT0DR` reader
pub struct R(crate::R<ADF_DFLT0DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DR` reader - DR
pub type DR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 8:31 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///ADF digital filter data register 0
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0dr](index.html) module
pub struct ADF_DFLT0DR_SPEC;
impl crate::RegisterSpec for ADF_DFLT0DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0dr::R](R) reader structure
impl crate::Readable for ADF_DFLT0DR_SPEC {
    type Reader = R;
}
///`reset()` method sets ADF_DFLT0DR to value 0
impl crate::Resettable for ADF_DFLT0DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
