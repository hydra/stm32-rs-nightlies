///Register `UR12` reader
pub struct R(crate::R<UR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR12_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IWDG2M` reader - Independent Watchdog 2 mode
pub type IWDG2M_R = crate::BitReader<bool>;
///Field `SECURE` reader - Secure mode
pub type SECURE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Independent Watchdog 2 mode
    #[inline(always)]
    pub fn iwdg2m(&self) -> IWDG2M_R {
        IWDG2M_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Secure mode
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///SYSCFG user register 12
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur12](index.html) module
pub struct UR12_SPEC;
impl crate::RegisterSpec for UR12_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur12::R](R) reader structure
impl crate::Readable for UR12_SPEC {
    type Reader = R;
}
///`reset()` method sets UR12 to value 0
impl crate::Resettable for UR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
