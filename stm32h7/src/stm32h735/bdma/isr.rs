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
///Field `GIF0` reader - Global interrupt flag for channel x
pub type GIF0_R = crate::BitReader<GIF0_A>;
///Global interrupt flag for channel x
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF0_A {
    ///0: No TE, HT or TC event on channel x
    NoEvent = 0,
    ///1: A TE, HT or TC event occurred on channel x
    Event = 1,
}
impl From<GIF0_A> for bool {
    #[inline(always)]
    fn from(variant: GIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GIF0_A {
        match self.bits {
            false => GIF0_A::NoEvent,
            true => GIF0_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF0_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF0_A::Event
    }
}
///Field `TCIF0` reader - Transfer complete (TC) flag for channel x
pub type TCIF0_R = crate::BitReader<TCIF0_A>;
///Transfer complete (TC) flag for channel x
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF0_A {
    ///0: No transfer complete event on channel x
    NotComplete = 0,
    ///1: A transfer complete event occurred on channel x
    Complete = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::NotComplete,
            true => TCIF0_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF0_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF0_A::Complete
    }
}
///Field `HTIF0` reader - Half transfer (HT) flag for channel x
pub type HTIF0_R = crate::BitReader<HTIF0_A>;
///Half transfer (HT) flag for channel x
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF0_A {
    ///0: No half transfer event on channel x
    NotHalf = 0,
    ///1: A half transfer event occurred on channel x
    Half = 1,
}
impl From<HTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HTIF0_A {
        match self.bits {
            false => HTIF0_A::NotHalf,
            true => HTIF0_A::Half,
        }
    }
    ///Checks if the value of the field is `NotHalf`
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF0_A::NotHalf
    }
    ///Checks if the value of the field is `Half`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF0_A::Half
    }
}
///Field `TEIF0` reader - Transfer error (TE) flag for channel x
pub type TEIF0_R = crate::BitReader<TEIF0_A>;
///Transfer error (TE) flag for channel x
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF0_A {
    ///0: No transfer error on channel x
    NoError = 0,
    ///1: A transfer error occurred on channel x
    Error = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::NoError,
            true => TEIF0_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF0_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF0_A::Error
    }
}
///Field `GIF1` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF1_R;
///Field `GIF2` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF2_R;
///Field `GIF3` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF3_R;
///Field `GIF4` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF4_R;
///Field `GIF5` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF5_R;
///Field `GIF6` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF6_R;
///Field `GIF7` reader - Global interrupt flag for channel x
pub use GIF0_R as GIF7_R;
///Field `HTIF1` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF1_R;
///Field `HTIF2` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF2_R;
///Field `HTIF3` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF3_R;
///Field `HTIF4` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF4_R;
///Field `HTIF5` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF5_R;
///Field `HTIF6` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF6_R;
///Field `HTIF7` reader - Half transfer (HT) flag for channel x
pub use HTIF0_R as HTIF7_R;
///Field `TCIF1` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF1_R;
///Field `TCIF2` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF2_R;
///Field `TCIF3` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF3_R;
///Field `TCIF4` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF4_R;
///Field `TCIF5` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF5_R;
///Field `TCIF6` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF6_R;
///Field `TCIF7` reader - Transfer complete (TC) flag for channel x
pub use TCIF0_R as TCIF7_R;
///Field `TEIF1` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF1_R;
///Field `TEIF2` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF2_R;
///Field `TEIF3` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF3_R;
///Field `TEIF4` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF4_R;
///Field `TEIF5` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF5_R;
///Field `TEIF6` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF6_R;
///Field `TEIF7` reader - Transfer error (TE) flag for channel x
pub use TEIF0_R as TEIF7_R;
impl R {
    ///Bit 0 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Global interrupt flag for channel x
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transfer complete (TC) flag for channel x
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Half transfer (HT) flag for channel x
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Transfer error (TE) flag for channel x
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///Interrupt status register
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
