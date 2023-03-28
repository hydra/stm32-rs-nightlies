///Register `ITLINE17` reader
pub struct R(crate::R<ITLINE17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE17_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LPTIM1` reader - LPTIM1
pub type LPTIM1_R = crate::BitReader<bool>;
impl R {
    ///Bit 2 - LPTIM1
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///interrupt line 17 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline17](index.html) module
pub struct ITLINE17_SPEC;
impl crate::RegisterSpec for ITLINE17_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline17::R](R) reader structure
impl crate::Readable for ITLINE17_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE17 to value 0
impl crate::Resettable for ITLINE17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
