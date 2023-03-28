///Register `AXIMC_COMP_ID_2` reader
pub struct R(crate::R<AXIMC_COMP_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_COMP_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_COMP_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_COMP_ID_2_SPEC>) -> Self {
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
///AXIMC component ID2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_comp_id_2](index.html) module
pub struct AXIMC_COMP_ID_2_SPEC;
impl crate::RegisterSpec for AXIMC_COMP_ID_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_comp_id_2::R](R) reader structure
impl crate::Readable for AXIMC_COMP_ID_2_SPEC {
    type Reader = R;
}
///`reset()` method sets AXIMC_COMP_ID_2 to value 0x05
impl crate::Resettable for AXIMC_COMP_ID_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
