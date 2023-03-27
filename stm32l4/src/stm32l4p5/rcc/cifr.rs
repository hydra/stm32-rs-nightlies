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
pub type LSIRDYF_R = crate::BitReader<LSIRDYF_A>;
///LSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF_A {
    ///0: No clock ready interrupt caused by the LSI oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the LSI oscillator
    Interrupt = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::NoInterrupt,
            true => LSIRDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSIRDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSIRDYF_A::Interrupt
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = crate::BitReader<LSERDYF_A>;
///LSE ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF_A {
    ///0: No clock ready interrupt caused by the LSE oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the LSE oscillator
    Interrupt = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::NoInterrupt,
            true => LSERDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSERDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSERDYF_A::Interrupt
    }
}
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = crate::BitReader<MSIRDYF_A>;
///MSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF_A {
    ///0: No clock ready interrupt caused by the MSI oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the MSI oscillator
    Interrupt = 1,
}
impl From<MSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIRDYF_A {
        match self.bits {
            false => MSIRDYF_A::NoInterrupt,
            true => MSIRDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == MSIRDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == MSIRDYF_A::Interrupt
    }
}
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub type HSIRDYF_R = crate::BitReader<HSIRDYF_A>;
///HSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF_A {
    ///0: No clock ready interrupt caused by the HSI16 oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSI16 oscillator
    Interrupt = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::NoInterrupt,
            true => HSIRDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSIRDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSIRDYF_A::Interrupt
    }
}
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = crate::BitReader<HSERDYF_A>;
///HSE ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF_A {
    ///0: No clock ready interrupt caused by the HSE oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSE oscillator
    Interrupt = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::NoInterrupt,
            true => HSERDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSERDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSERDYF_A::Interrupt
    }
}
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub type PLLRDYF_R = crate::BitReader<PLLRDYF_A>;
///PLL ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF_A {
    ///0: No clock ready interrupt caused by PLL lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLL lock
    Interrupt = 1,
}
impl From<PLLRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYF_A {
        match self.bits {
            false => PLLRDYF_A::NoInterrupt,
            true => PLLRDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLRDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLRDYF_A::Interrupt
    }
}
///Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag
pub type PLLSAI1RDYF_R = crate::BitReader<PLLSAI1RDYF_A>;
///PLLSAI1 ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYF_A {
    ///0: No clock ready interrupt caused by PLLSAI1 lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLLSAI1 lock
    Interrupt = 1,
}
impl From<PLLSAI1RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1RDYF_A {
        match self.bits {
            false => PLLSAI1RDYF_A::NoInterrupt,
            true => PLLSAI1RDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF_A::Interrupt
    }
}
///Field `PLLSAI2RDYF` reader - PLLSAI2 ready interrupt flag
pub type PLLSAI2RDYF_R = crate::BitReader<PLLSAI2RDYF_A>;
///PLLSAI2 ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYF_A {
    ///0: No clock ready interrupt caused by PLLSAI2 lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLLSAI2 lock
    Interrupt = 1,
}
impl From<PLLSAI2RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2RDYF_A {
        match self.bits {
            false => PLLSAI2RDYF_A::NoInterrupt,
            true => PLLSAI2RDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF_A::Interrupt
    }
}
///Field `CSSF` reader - Clock security system interrupt flag
pub type CSSF_R = crate::BitReader<CSSF_A>;
///Clock security system interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF_A {
    ///0: No clock security interrupt caused by HSE clock failure
    NoInterrupt = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interrupt = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::NoInterrupt,
            true => CSSF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == CSSF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == CSSF_A::Interrupt
    }
}
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub type LSECSSF_R = crate::BitReader<LSECSSF_A>;
///LSE Clock security system interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF_A {
    ///0: No clock security interrupt caused by LSE clock failure
    NoInterrupt = 0,
    ///1: Clock security interrupt caused by LSE clock failure
    Interrupt = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::NoInterrupt,
            true => LSECSSF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSECSSF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSECSSF_A::Interrupt
    }
}
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub type HSI48RDYF_R = crate::BitReader<HSI48RDYF_A>;
///HSI48 ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYF_A {
    ///0: No clock ready interrupt caused by the HSI48 oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSI48 oscillator
    Interrupt = 1,
}
impl From<HSI48RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYF_A {
        match self.bits {
            false => HSI48RDYF_A::NoInterrupt,
            true => HSI48RDYF_A::Interrupt,
        }
    }
    ///Checks if the value of the field is `NoInterrupt`
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSI48RDYF_A::NoInterrupt
    }
    ///Checks if the value of the field is `Interrupt`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSI48RDYF_A::Interrupt
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
    ///Bit 2 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt flag
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLLSAI2 ready interrupt flag
    #[inline(always)]
    pub fn pllsai2rdyf(&self) -> PLLSAI2RDYF_R {
        PLLSAI2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
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
