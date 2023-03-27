///Register `HWCFGR2` reader
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CFG1` reader - peripheral hardware configuration 1
pub type CFG1_R = crate::FieldReader<u8, u8>;
///Field `CFG2` reader - peripheral hardware configuration 2
pub type CFG2_R = crate::FieldReader<u8, u8>;
///Field `CFG3` reader - peripheral hardware configuration 3
pub type CFG3_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - peripheral hardware configuration 1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:15 - peripheral hardware configuration 2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - peripheral hardware configuration 3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///LPTIM peripheral hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr2](index.html) module
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr2::R](R) reader structure
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR2 to value 0
impl crate::Resettable for HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
