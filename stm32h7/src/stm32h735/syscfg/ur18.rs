///Register `UR18` reader
pub struct R(crate::R<UR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR18_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPU_FREQ_BOOST` reader - CPU maximum frequency boost
pub type CPU_FREQ_BOOST_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CPU maximum frequency boost
    #[inline(always)]
    pub fn cpu_freq_boost(&self) -> CPU_FREQ_BOOST_R {
        CPU_FREQ_BOOST_R::new((self.bits & 1) != 0)
    }
}
///SYSCFG user register 18
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur18](index.html) module
pub struct UR18_SPEC;
impl crate::RegisterSpec for UR18_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur18::R](R) reader structure
impl crate::Readable for UR18_SPEC {
    type Reader = R;
}
///`reset()` method sets UR18 to value 0
impl crate::Resettable for UR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
