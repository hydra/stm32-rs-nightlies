///Register `AXIMC_PERIPH_ID_1` reader
pub struct R(crate::R<AXIMC_PERIPH_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PERIPH_ID_1` reader - PERIPH_ID_1
pub type PERIPH_ID_1_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PERIPH_ID_1
    #[inline(always)]
    pub fn periph_id_1(&self) -> PERIPH_ID_1_R {
        PERIPH_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
///AXIMC peripheral ID1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_periph_id_1](index.html) module
pub struct AXIMC_PERIPH_ID_1_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_periph_id_1::R](R) reader structure
impl crate::Readable for AXIMC_PERIPH_ID_1_SPEC {
    type Reader = R;
}
///`reset()` method sets AXIMC_PERIPH_ID_1 to value 0xb4
impl crate::Resettable for AXIMC_PERIPH_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
