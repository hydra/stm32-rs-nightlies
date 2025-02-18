///Register `RESPI1` reader
pub struct R(crate::R<RESPI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPI1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUS1` reader - CARDSTATUS1
pub type CARDSTATUS1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS1
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
///Bits 31:0 = CARDSTATUS1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [respi1](index.html) module
pub struct RESPI1_SPEC;
impl crate::RegisterSpec for RESPI1_SPEC {
    type Ux = u32;
}
///`read()` method returns [respi1::R](R) reader structure
impl crate::Readable for RESPI1_SPEC {
    type Reader = R;
}
///`reset()` method sets RESPI1 to value 0
impl crate::Resettable for RESPI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
