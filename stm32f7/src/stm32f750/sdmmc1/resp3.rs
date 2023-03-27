///Register `RESP3` reader
pub struct R(crate::R<RESP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS3` reader - see Table 132
pub type CARDSTATUS3_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - see Table 132
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp3](index.html) module
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp3::R](R) reader structure
impl crate::Readable for RESP3_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP3 to value 0
impl crate::Resettable for RESP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
