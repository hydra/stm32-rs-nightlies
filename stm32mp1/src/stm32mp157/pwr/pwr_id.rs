///Register `PWR_ID` reader
pub struct R(crate::R<PWR_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_ID_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IPID` reader - IPID
pub type IPID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - IPID
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
///PWR IP identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_id](index.html) module
pub struct PWR_ID_SPEC;
impl crate::RegisterSpec for PWR_ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_id::R](R) reader structure
impl crate::Readable for PWR_ID_SPEC {
    type Reader = R;
}
///`reset()` method sets PWR_ID to value 0x0001_0001
impl crate::Resettable for PWR_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
