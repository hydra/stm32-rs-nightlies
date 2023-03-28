///Register `RESP1` reader
pub struct R(crate::R<RESP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS1` reader - see Table 132
pub type CARDSTATUS1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - see Table 132
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp1](index.html) module
pub struct RESP1_SPEC;
impl crate::RegisterSpec for RESP1_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp1::R](R) reader structure
impl crate::Readable for RESP1_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP1 to value 0
impl crate::Resettable for RESP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
