///Register `ID` reader
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IP_ID` reader - SDMMC IP identification.
pub type IP_ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - SDMMC IP identification.
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
///SDMMC IP identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [id](index.html) module
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [id::R](R) reader structure
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
///`reset()` method sets ID to value 0x0014_0022
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0014_0022;
}
