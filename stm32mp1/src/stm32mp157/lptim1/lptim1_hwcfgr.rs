///Register `LPTIM1_HWCFGR` reader
pub struct R(crate::R<LPTIM1_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM1_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM1_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM1_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader<u8, u8>;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader<u8, u8>;
///Field `CFG3` reader - CFG3
pub type CFG3_R = crate::FieldReader<u8, u8>;
///Field `CFG4` reader - CFG4
pub type CFG4_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - CFG3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:31 - CFG4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///LPTIM 1 peripheral hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptim1_hwcfgr](index.html) module
pub struct LPTIM1_HWCFGR_SPEC;
impl crate::RegisterSpec for LPTIM1_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptim1_hwcfgr::R](R) reader structure
impl crate::Readable for LPTIM1_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets LPTIM1_HWCFGR to value 0x0001_0804
impl crate::Resettable for LPTIM1_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0804;
}
