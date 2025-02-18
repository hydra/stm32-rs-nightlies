///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LIF` reader - Line Interrupt flag
pub type LIF_R = crate::BitReader<LIF_A>;
///Line Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIF_A {
    ///0: Programmed line not reached
    NotReached = 0,
    ///1: Line interrupt generated when a programmed line is reached
    Reached = 1,
}
impl From<LIF_A> for bool {
    #[inline(always)]
    fn from(variant: LIF_A) -> Self {
        variant as u8 != 0
    }
}
impl LIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LIF_A {
        match self.bits {
            false => LIF_A::NotReached,
            true => LIF_A::Reached,
        }
    }
    ///Checks if the value of the field is `NotReached`
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == LIF_A::NotReached
    }
    ///Checks if the value of the field is `Reached`
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == LIF_A::Reached
    }
}
///Field `FUIF` reader - FIFO Underrun Interrupt flag
pub type FUIF_R = crate::BitReader<FUIF_A>;
///FIFO Underrun Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIF_A {
    ///0: No FIFO underrun
    NoUnderrun = 0,
    ///1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
    Underrun = 1,
}
impl From<FUIF_A> for bool {
    #[inline(always)]
    fn from(variant: FUIF_A) -> Self {
        variant as u8 != 0
    }
}
impl FUIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FUIF_A {
        match self.bits {
            false => FUIF_A::NoUnderrun,
            true => FUIF_A::Underrun,
        }
    }
    ///Checks if the value of the field is `NoUnderrun`
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == FUIF_A::NoUnderrun
    }
    ///Checks if the value of the field is `Underrun`
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == FUIF_A::Underrun
    }
}
///Field `TERRIF` reader - Transfer Error interrupt flag
pub type TERRIF_R = crate::BitReader<TERRIF_A>;
///Transfer Error interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIF_A {
    ///0: No transfer error
    NoError = 0,
    ///1: Transfer error interrupt generated when a bus error occurs
    Error = 1,
}
impl From<TERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TERRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TERRIF_A {
        match self.bits {
            false => TERRIF_A::NoError,
            true => TERRIF_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TERRIF_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TERRIF_A::Error
    }
}
///Field `RRIF` reader - Register Reload Interrupt Flag
pub type RRIF_R = crate::BitReader<RRIF_A>;
///Register Reload Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIF_A {
    ///0: No register reload
    NoReload = 0,
    ///1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
    Reload = 1,
}
impl From<RRIF_A> for bool {
    #[inline(always)]
    fn from(variant: RRIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRIF_A {
        match self.bits {
            false => RRIF_A::NoReload,
            true => RRIF_A::Reload,
        }
    }
    ///Checks if the value of the field is `NoReload`
    #[inline(always)]
    pub fn is_no_reload(&self) -> bool {
        *self == RRIF_A::NoReload
    }
    ///Checks if the value of the field is `Reload`
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == RRIF_A::Reload
    }
}
impl R {
    ///Bit 0 - Line Interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer Error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Register Reload Interrupt Flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
///LTDC Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
