///Register `TZC_CID3` reader
pub struct R(crate::R<TZC_CID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CID3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COMP_ID_3` reader - COMP_ID_3
pub type COMP_ID_3_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - COMP_ID_3
    #[inline(always)]
    pub fn comp_id_3(&self) -> COMP_ID_3_R {
        COMP_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
///Component ID 3.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_cid3](index.html) module
pub struct TZC_CID3_SPEC;
impl crate::RegisterSpec for TZC_CID3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_cid3::R](R) reader structure
impl crate::Readable for TZC_CID3_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_CID3 to value 0xb1
impl crate::Resettable for TZC_CID3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
