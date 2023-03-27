///Register `RESP2R` reader
pub struct R(crate::R<RESP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP2R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS2` reader - see Table 347
pub type CARDSTATUS2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - see Table 347
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp2r](index.html) module
pub struct RESP2R_SPEC;
impl crate::RegisterSpec for RESP2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp2r::R](R) reader structure
impl crate::Readable for RESP2R_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP2R to value 0
impl crate::Resettable for RESP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
