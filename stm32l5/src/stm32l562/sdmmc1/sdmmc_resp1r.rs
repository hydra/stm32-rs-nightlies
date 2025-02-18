///Register `SDMMC_RESP1R` reader
pub struct R(crate::R<SDMMC_RESP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_RESP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_RESP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_RESP1R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS1` reader - see Table 432
pub type CARDSTATUS1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - see Table 432
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_resp1r](index.html) module
pub struct SDMMC_RESP1R_SPEC;
impl crate::RegisterSpec for SDMMC_RESP1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_resp1r::R](R) reader structure
impl crate::Readable for SDMMC_RESP1R_SPEC {
    type Reader = R;
}
///`reset()` method sets SDMMC_RESP1R to value 0
impl crate::Resettable for SDMMC_RESP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
