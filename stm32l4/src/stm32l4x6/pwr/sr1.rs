///Register `SR1` reader
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CWUF1` reader - Wakeup flag 1
pub type CWUF1_R = crate::BitReader<bool>;
///Field `CWUF2` reader - Wakeup flag 2
pub type CWUF2_R = crate::BitReader<bool>;
///Field `CWUF3` reader - Wakeup flag 3
pub type CWUF3_R = crate::BitReader<bool>;
///Field `CWUF4` reader - Wakeup flag 4
pub type CWUF4_R = crate::BitReader<bool>;
///Field `CWUF5` reader - Wakeup flag 5
pub type CWUF5_R = crate::BitReader<bool>;
///Field `CSBF` reader - Standby flag
pub type CSBF_R = crate::BitReader<bool>;
///Field `WUFI` reader - Wakeup flag internal
pub type WUFI_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn cwuf1(&self) -> CWUF1_R {
        CWUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn cwuf2(&self) -> CWUF2_R {
        CWUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn cwuf3(&self) -> CWUF3_R {
        CWUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4
    #[inline(always)]
    pub fn cwuf4(&self) -> CWUF4_R {
        CWUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5
    #[inline(always)]
    pub fn cwuf5(&self) -> CWUF5_R {
        CWUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Wakeup flag internal
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///Power status register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr1](index.html) module
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr1::R](R) reader structure
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
