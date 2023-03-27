///Register `CIFR` reader
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR_A>;
///LSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR_A {
    ///0: No clock ready interrupt
    NotInterrupted = 0,
    ///1: Clock ready interrupt
    Interrupted = 1,
}
impl From<LSIRDYFR_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYFR_A {
        match self.bits {
            false => LSIRDYFR_A::NotInterrupted,
            true => LSIRDYFR_A::Interrupted,
        }
    }
    ///Checks if the value of the field is `NotInterrupted`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::NotInterrupted
    }
    ///Checks if the value of the field is `Interrupted`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::Interrupted
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `HSI16RDYF` reader - HSI16 ready interrupt flag
pub use LSIRDYF_R as HSI16RDYF_R;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub use LSIRDYF_R as MSIRDYF_R;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub use LSIRDYF_R as HSI48RDYF_R;
///Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag
pub type CSSLSEF_R = crate::BitReader<CSSLSEF_A>;
///LSE Clock Security System Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSEF_A {
    ///0: No failure detected on LSE clock failure
    NoFailure = 0,
    ///1: Failure detected on LSE clock failure
    Failure = 1,
}
impl From<CSSLSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSLSEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSLSEF_A {
        match self.bits {
            false => CSSLSEF_A::NoFailure,
            true => CSSLSEF_A::Failure,
        }
    }
    ///Checks if the value of the field is `NoFailure`
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSEF_A::NoFailure
    }
    ///Checks if the value of the field is `Failure`
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSEF_A::Failure
    }
}
///Field `CSSHSEF` reader - Clock Security System Interrupt flag
pub type CSSHSEF_R = crate::BitReader<CSSHSEF_A>;
///Clock Security System Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSHSEF_A {
    ///0: No clock security interrupt caused by HSE clock failure
    NoClock = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Clock = 1,
}
impl From<CSSHSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSHSEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSHSEF_A {
        match self.bits {
            false => CSSHSEF_A::NoClock,
            true => CSSHSEF_A::Clock,
        }
    }
    ///Checks if the value of the field is `NoClock`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CSSHSEF_A::NoClock
    }
    ///Checks if the value of the field is `Clock`
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == CSSHSEF_A::Clock
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///Clock interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cifr](index.html) module
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cifr::R](R) reader structure
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
