///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PVU` reader - Watchdog prescaler value update
pub type PVU_R = crate::BitReader<PVU_A>;
///Watchdog prescaler value update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVU_A {
    ///0: No update on-going
    Idle = 0,
    ///1: Update on-going
    Busy = 1,
}
impl From<PVU_A> for bool {
    #[inline(always)]
    fn from(variant: PVU_A) -> Self {
        variant as u8 != 0
    }
}
impl PVU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVU_A {
        match self.bits {
            false => PVU_A::Idle,
            true => PVU_A::Busy,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PVU_A::Idle
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PVU_A::Busy
    }
}
///Field `RVU` reader - Watchdog counter reload value update
pub type RVU_R = crate::BitReader<RVU_A>;
///Watchdog counter reload value update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RVU_A {
    ///0: No update on-going
    Idle = 0,
    ///1: Update on-going
    Busy = 1,
}
impl From<RVU_A> for bool {
    #[inline(always)]
    fn from(variant: RVU_A) -> Self {
        variant as u8 != 0
    }
}
impl RVU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RVU_A {
        match self.bits {
            false => RVU_A::Idle,
            true => RVU_A::Busy,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RVU_A::Idle
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RVU_A::Busy
    }
}
///Field `WVU` reader - Watchdog counter window value update
pub type WVU_R = crate::BitReader<WVU_A>;
///Watchdog counter window value update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WVU_A {
    ///0: No update on-going
    Idle = 0,
    ///1: Update on-going
    Busy = 1,
}
impl From<WVU_A> for bool {
    #[inline(always)]
    fn from(variant: WVU_A) -> Self {
        variant as u8 != 0
    }
}
impl WVU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WVU_A {
        match self.bits {
            false => WVU_A::Idle,
            true => WVU_A::Busy,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WVU_A::Idle
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == WVU_A::Busy
    }
}
impl R {
    ///Bit 0 - Watchdog prescaler value update
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
