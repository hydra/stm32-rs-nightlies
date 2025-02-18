///Register `AXIMC_PERIPH_ID_0` reader
pub struct R(crate::R<AXIMC_PERIPH_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PERIPH_ID_0` reader - PERIPH_ID_0
pub type PERIPH_ID_0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PERIPH_ID_0
    #[inline(always)]
    pub fn periph_id_0(&self) -> PERIPH_ID_0_R {
        PERIPH_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
///AXIMC peripheral ID0 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_periph_id_0](index.html) module
pub struct AXIMC_PERIPH_ID_0_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_periph_id_0::R](R) reader structure
impl crate::Readable for AXIMC_PERIPH_ID_0_SPEC {
    type Reader = R;
}
///`reset()` method sets AXIMC_PERIPH_ID_0 to value 0
impl crate::Resettable for AXIMC_PERIPH_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
