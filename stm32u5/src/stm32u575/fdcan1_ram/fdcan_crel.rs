///Register `FDCAN_CREL` reader
pub struct R(crate::R<FDCAN_CREL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CREL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CREL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CREL_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DAY` reader - Timestamp Day
pub type DAY_R = crate::FieldReader<u8, u8>;
///Field `MON` reader - Timestamp Month
pub type MON_R = crate::FieldReader<u8, u8>;
///Field `YEAR` reader - Timestamp Year
pub type YEAR_R = crate::FieldReader<u8, u8>;
///Field `SUBSTEP` reader - Sub-step of Core release
pub type SUBSTEP_R = crate::FieldReader<u8, u8>;
///Field `STEP` reader - Step of Core release
pub type STEP_R = crate::FieldReader<u8, u8>;
///Field `REL` reader - Core release
pub type REL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Timestamp Day
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Timestamp Month
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Timestamp Year
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Sub-step of Core release
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Step of Core release
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Core release
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
///FDCAN Core Release Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_crel](index.html) module
pub struct FDCAN_CREL_SPEC;
impl crate::RegisterSpec for FDCAN_CREL_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_crel::R](R) reader structure
impl crate::Readable for FDCAN_CREL_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_CREL to value 0x3214_1218
impl crate::Resettable for FDCAN_CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3214_1218;
}
