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
///Field `WUF1` reader - Wakeup flag 1
pub type WUF1_R = crate::BitReader<WUF1_A>;
///Wakeup flag 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1_A {
    ///0: No wakeup event detected on WKUP1
    Clear = 0,
    ///1: Wakeup event detected on WKUP1
    Wakeup = 1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::Clear,
            true => WUF1_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF1_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF1_A::Wakeup
    }
}
///Field `WUF2` reader - Wakeup flag 2
pub type WUF2_R = crate::BitReader<WUF2_A>;
///Wakeup flag 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF2_A {
    ///0: No wakeup event detected on WKUP2
    Clear = 0,
    ///1: Wakeup event detected on WKUP2
    Wakeup = 1,
}
impl From<WUF2_A> for bool {
    #[inline(always)]
    fn from(variant: WUF2_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF2_A {
        match self.bits {
            false => WUF2_A::Clear,
            true => WUF2_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF2_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF2_A::Wakeup
    }
}
///Field `WUF3` reader - Wakeup flag 3
pub type WUF3_R = crate::BitReader<WUF3_A>;
///Wakeup flag 3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF3_A {
    ///0: No wakeup event detected on WKUP3
    Clear = 0,
    ///1: Wakeup event detected on WKUP3
    Wakeup = 1,
}
impl From<WUF3_A> for bool {
    #[inline(always)]
    fn from(variant: WUF3_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF3_A {
        match self.bits {
            false => WUF3_A::Clear,
            true => WUF3_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF3_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF3_A::Wakeup
    }
}
///Field `WPVDF` reader - Wakeup PVD flag
pub type WPVDF_R = crate::BitReader<WPVDF_A>;
///Wakeup PVD flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPVDF_A {
    ///0: No wakeup event detected on PVD
    Clear = 0,
    ///1: Wakeup event detected on PVD
    Wakeup = 1,
}
impl From<WPVDF_A> for bool {
    #[inline(always)]
    fn from(variant: WPVDF_A) -> Self {
        variant as u8 != 0
    }
}
impl WPVDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WPVDF_A {
        match self.bits {
            false => WPVDF_A::Clear,
            true => WPVDF_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WPVDF_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WPVDF_A::Wakeup
    }
}
///Field `WRFBUSYF` reader - Radio BUSY wakeup flag
pub type WRFBUSYF_R = crate::BitReader<WRFBUSYF_A>;
///Radio BUSY wakeup flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYF_A {
    ///0: No wakeup event detected on radio busy
    Clear = 0,
    ///1: Wakeup event detected on radio busy
    Wakeup = 1,
}
impl From<WRFBUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYF_A) -> Self {
        variant as u8 != 0
    }
}
impl WRFBUSYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WRFBUSYF_A {
        match self.bits {
            false => WRFBUSYF_A::Clear,
            true => WRFBUSYF_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WRFBUSYF_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WRFBUSYF_A::Wakeup
    }
}
///Field `C2HF` reader - PU2 Hold interrupt flag
pub type C2HF_R = crate::BitReader<bool>;
///Field `WUFI` reader - Internal wakeup interrupt flag
pub type WUFI_R = crate::BitReader<WUFI_A>;
///Internal wakeup interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI_A {
    ///0: All internal wakeup sources are cleared
    Clear = 0,
    ///1: wakeup is detected on the internal wakeup line
    Wakeup = 1,
}
impl From<WUFI_A> for bool {
    #[inline(always)]
    fn from(variant: WUFI_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFI_A {
        match self.bits {
            false => WUFI_A::Clear,
            true => WUFI_A::Wakeup,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUFI_A::Clear
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUFI_A::Wakeup
    }
}
impl R {
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Wakeup PVD flag
    #[inline(always)]
    pub fn wpvdf(&self) -> WPVDF_R {
        WPVDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Radio BUSY wakeup flag
    #[inline(always)]
    pub fn wrfbusyf(&self) -> WRFBUSYF_R {
        WRFBUSYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - PU2 Hold interrupt flag
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Internal wakeup interrupt flag
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
