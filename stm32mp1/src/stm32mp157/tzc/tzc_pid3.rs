///Register `TZC_PID3` reader
pub struct R(crate::R<TZC_PID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PID3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PER_ID_3` reader - PER_ID_3
pub type PER_ID_3_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PER_ID_3
    #[inline(always)]
    pub fn per_id_3(&self) -> PER_ID_3_R {
        PER_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
///Peripheral ID 3.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_pid3](index.html) module
pub struct TZC_PID3_SPEC;
impl crate::RegisterSpec for TZC_PID3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_pid3::R](R) reader structure
impl crate::Readable for TZC_PID3_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_PID3 to value 0
impl crate::Resettable for TZC_PID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
