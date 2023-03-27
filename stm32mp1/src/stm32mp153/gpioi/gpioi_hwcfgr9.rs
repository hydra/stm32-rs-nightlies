///Register `GPIOI_HWCFGR9` reader
pub struct R(crate::R<GPIOI_HWCFGR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOI_HWCFGR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOI_HWCFGR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOI_HWCFGR9_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EN_IO` reader - EN_IO
pub type EN_IO_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - EN_IO
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioi_hwcfgr9](index.html) module
pub struct GPIOI_HWCFGR9_SPEC;
impl crate::RegisterSpec for GPIOI_HWCFGR9_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioi_hwcfgr9::R](R) reader structure
impl crate::Readable for GPIOI_HWCFGR9_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOI_HWCFGR9 to value 0xff
impl crate::Resettable for GPIOI_HWCFGR9_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
