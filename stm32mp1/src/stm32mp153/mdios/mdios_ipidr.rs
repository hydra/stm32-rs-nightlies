///Register `MDIOS_IPIDR` reader
pub struct R(crate::R<MDIOS_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
///MDIOS identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdios_ipidr](index.html) module
pub struct MDIOS_IPIDR_SPEC;
impl crate::RegisterSpec for MDIOS_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_ipidr::R](R) reader structure
impl crate::Readable for MDIOS_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_IPIDR to value 0x0018_0001
impl crate::Resettable for MDIOS_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0018_0001;
}
