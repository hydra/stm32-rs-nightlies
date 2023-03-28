///Register `TZC_CID2` reader
pub struct R(crate::R<TZC_CID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CID2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COMP_ID_2` reader - COMP_ID_2
pub type COMP_ID_2_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - COMP_ID_2
    #[inline(always)]
    pub fn comp_id_2(&self) -> COMP_ID_2_R {
        COMP_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
///Component ID 2.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_cid2](index.html) module
pub struct TZC_CID2_SPEC;
impl crate::RegisterSpec for TZC_CID2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_cid2::R](R) reader structure
impl crate::Readable for TZC_CID2_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_CID2 to value 0x05
impl crate::Resettable for TZC_CID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
