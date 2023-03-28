///Register `AXIMC_COMP_ID_3` reader
pub struct R(crate::R<AXIMC_COMP_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_COMP_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_COMP_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_COMP_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PREAMBLE` reader - PREAMBLE
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PREAMBLE
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
///AXIMC component ID3 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_comp_id_3](index.html) module
pub struct AXIMC_COMP_ID_3_SPEC;
impl crate::RegisterSpec for AXIMC_COMP_ID_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_comp_id_3::R](R) reader structure
impl crate::Readable for AXIMC_COMP_ID_3_SPEC {
    type Reader = R;
}
///`reset()` method sets AXIMC_COMP_ID_3 to value 0xb1
impl crate::Resettable for AXIMC_COMP_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
