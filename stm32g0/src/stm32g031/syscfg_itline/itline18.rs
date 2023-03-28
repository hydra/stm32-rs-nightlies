///Register `ITLINE18` reader
pub struct R(crate::R<ITLINE18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE18_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LPTIM2` reader - LPTIM2
pub type LPTIM2_R = crate::BitReader<bool>;
impl R {
    ///Bit 1 - LPTIM2
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///interrupt line 18 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline18](index.html) module
pub struct ITLINE18_SPEC;
impl crate::RegisterSpec for ITLINE18_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline18::R](R) reader structure
impl crate::Readable for ITLINE18_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE18 to value 0
impl crate::Resettable for ITLINE18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
