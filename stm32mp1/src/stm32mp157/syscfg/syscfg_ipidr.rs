///Register `SYSCFG_IPIDR` reader
pub struct R(crate::R<SYSCFG_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_IPIDR_SPEC>) -> Self {
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
///SYSCFG identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_ipidr](index.html) module
pub struct SYSCFG_IPIDR_SPEC;
impl crate::RegisterSpec for SYSCFG_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_ipidr::R](R) reader structure
impl crate::Readable for SYSCFG_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SYSCFG_IPIDR to value 0x0003_0001
impl crate::Resettable for SYSCFG_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0001;
}
