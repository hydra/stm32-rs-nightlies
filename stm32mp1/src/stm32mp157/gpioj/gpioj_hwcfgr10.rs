///Register `GPIOJ_HWCFGR10` reader
pub struct R(crate::R<GPIOJ_HWCFGR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOJ_HWCFGR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOJ_HWCFGR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOJ_HWCFGR10_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AHB_IOP` reader - AHB_IOP
pub type AHB_IOP_R = crate::FieldReader<u8, u8>;
///Field `AF_SIZE` reader - AF_SIZE
pub type AF_SIZE_R = crate::FieldReader<u8, u8>;
///Field `SPEED_CFG` reader - SPEED_CFG
pub type SPEED_CFG_R = crate::FieldReader<u8, u8>;
///Field `LOCK_CFG` reader - LOCK_CFG
pub type LOCK_CFG_R = crate::FieldReader<u8, u8>;
///Field `SEC_CFG` reader - SEC_CFG
pub type SEC_CFG_R = crate::FieldReader<u8, u8>;
///Field `OR_CFG` reader - OR_CFG
pub type OR_CFG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - AHB_IOP
    #[inline(always)]
    pub fn ahb_iop(&self) -> AHB_IOP_R {
        AHB_IOP_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AF_SIZE
    #[inline(always)]
    pub fn af_size(&self) -> AF_SIZE_R {
        AF_SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SPEED_CFG
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - LOCK_CFG
    #[inline(always)]
    pub fn lock_cfg(&self) -> LOCK_CFG_R {
        LOCK_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SEC_CFG
    #[inline(always)]
    pub fn sec_cfg(&self) -> SEC_CFG_R {
        SEC_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - OR_CFG
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioj_hwcfgr10](index.html) module
pub struct GPIOJ_HWCFGR10_SPEC;
impl crate::RegisterSpec for GPIOJ_HWCFGR10_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioj_hwcfgr10::R](R) reader structure
impl crate::Readable for GPIOJ_HWCFGR10_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOJ_HWCFGR10 to value 0x0001_1240
impl crate::Resettable for GPIOJ_HWCFGR10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1240;
}
