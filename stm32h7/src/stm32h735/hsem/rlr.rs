///Register `RLR%s` reader
pub struct R(crate::R<RLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PROCID` reader - Semaphore ProcessID
pub type PROCID_R = crate::FieldReader<u8, u8>;
///Field `MASTERID` reader - Semaphore MasterID
pub type MASTERID_R = crate::FieldReader<u8, u8>;
///Field `LOCK` reader - Lock indication
pub type LOCK_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Semaphore MasterID
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///HSEM Read lock register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rlr](index.html) module
pub struct RLR_SPEC;
impl crate::RegisterSpec for RLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rlr::R](R) reader structure
impl crate::Readable for RLR_SPEC {
    type Reader = R;
}
///`reset()` method sets RLR%s to value 0
impl crate::Resettable for RLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
