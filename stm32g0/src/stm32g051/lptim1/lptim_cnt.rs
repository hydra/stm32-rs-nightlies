///Register `LPTIM_CNT` reader
pub struct R(crate::R<LPTIM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
///Counter Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptim_cnt](index.html) module
pub struct LPTIM_CNT_SPEC;
impl crate::RegisterSpec for LPTIM_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptim_cnt::R](R) reader structure
impl crate::Readable for LPTIM_CNT_SPEC {
    type Reader = R;
}
///`reset()` method sets LPTIM_CNT to value 0
impl crate::Resettable for LPTIM_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
