///Register `TZC_CID1` reader
pub struct R(crate::R<TZC_CID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CID1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COMP_ID_1` reader - COMP_ID_1
pub type COMP_ID_1_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - COMP_ID_1
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
///Component ID 1.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_cid1](index.html) module
pub struct TZC_CID1_SPEC;
impl crate::RegisterSpec for TZC_CID1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_cid1::R](R) reader structure
impl crate::Readable for TZC_CID1_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_CID1 to value 0xf0
impl crate::Resettable for TZC_CID1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
