///Register `RESP2` reader
pub struct R(crate::R<RESP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS2` reader - CARDSTATUS2
pub type CARDSTATUS2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS2
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
///Bits 31:0 = CARDSTATUS2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp2](index.html) module
pub struct RESP2_SPEC;
impl crate::RegisterSpec for RESP2_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp2::R](R) reader structure
impl crate::Readable for RESP2_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP2 to value 0
impl crate::Resettable for RESP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
