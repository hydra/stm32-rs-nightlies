///Register `IPCC_C2TOC1SR` reader
pub struct R(crate::R<IPCC_C2TOC1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C2TOC1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C2TOC1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C2TOC1SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CHxF` reader - CHxF
pub type CHX_F_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:5 - CHxF
    #[inline(always)]
    pub fn chx_f(&self) -> CHX_F_R {
        CHX_F_R::new((self.bits & 0x3f) as u8)
    }
}
///IPCC processor 2 to processor 1 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ipcc_c2toc1sr](index.html) module
pub struct IPCC_C2TOC1SR_SPEC;
impl crate::RegisterSpec for IPCC_C2TOC1SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ipcc_c2toc1sr::R](R) reader structure
impl crate::Readable for IPCC_C2TOC1SR_SPEC {
    type Reader = R;
}
///`reset()` method sets IPCC_C2TOC1SR to value 0
impl crate::Resettable for IPCC_C2TOC1SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
