///Register `QUADSPI_HWCFGR` reader
pub struct R(crate::R<QUADSPI_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FIFOSIZE` reader - FIFOSIZE
pub type FIFOSIZE_R = crate::FieldReader<u8, u8>;
///Field `FIFOPTR` reader - FIFOPTR
pub type FIFOPTR_R = crate::FieldReader<u8, u8>;
///Field `PRESCVAL` reader - PRESCVAL
pub type PRESCVAL_R = crate::FieldReader<u8, u8>;
///Field `IDLENGTH` reader - IDLENGTH
pub type IDLENGTH_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - FIFOSIZE
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - FIFOPTR
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PRESCVAL
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - IDLENGTH
    #[inline(always)]
    pub fn idlength(&self) -> IDLENGTH_R {
        IDLENGTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
///QUADSPI HW configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [quadspi_hwcfgr](index.html) module
pub struct QUADSPI_HWCFGR_SPEC;
impl crate::RegisterSpec for QUADSPI_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [quadspi_hwcfgr::R](R) reader structure
impl crate::Readable for QUADSPI_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets QUADSPI_HWCFGR to value 0xb058
impl crate::Resettable for QUADSPI_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb058;
}
