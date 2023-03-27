///Register `RESP%s` reader
pub struct R(crate::R<RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP_SPEC>) -> Self {
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
///For information about available fields see [resp](index.html) module
pub struct RESP_SPEC;
impl crate::RegisterSpec for RESP_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp::R](R) reader structure
impl crate::Readable for RESP_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP%s to value 0
impl crate::Resettable for RESP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
