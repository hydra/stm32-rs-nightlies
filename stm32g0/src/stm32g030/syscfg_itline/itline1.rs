///Register `ITLINE1` reader
pub struct R(crate::R<ITLINE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PVDOUT` reader - PVD supply monitoring interrupt request pending (EXTI line 16).
pub type PVDOUT_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16).
    #[inline(always)]
    pub fn pvdout(&self) -> PVDOUT_R {
        PVDOUT_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 1 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline1](index.html) module
pub struct ITLINE1_SPEC;
impl crate::RegisterSpec for ITLINE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline1::R](R) reader structure
impl crate::Readable for ITLINE1_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE1 to value 0
impl crate::Resettable for ITLINE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
