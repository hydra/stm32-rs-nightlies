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
///Field `GIF1` reader - Channel 1 Global interrupt flag
pub type GIF1_R = crate::BitReader<GIF1_A>;
///Channel 1 Global interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1_A {
    ///0: No transfer error, half event, complete event
    NoEvent = 0,
    ///1: A transfer error, half event or complete event has occured
    Event = 1,
}
impl From<GIF1_A> for bool {
    #[inline(always)]
    fn from(variant: GIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GIF1_A {
        match self.bits {
            false => GIF1_A::NoEvent,
            true => GIF1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF1_A::Event
    }
}
///Field `TCIF1` reader - Channel 1 Transfer Complete flag
pub type TCIF1_R = crate::BitReader<TCIF1_A>;
///Channel 1 Transfer Complete flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1_A {
    ///0: No transfer complete event
    NotComplete = 0,
    ///1: A transfer complete event has occured
    Complete = 1,
}
impl From<TCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIF1_A {
        match self.bits {
            false => TCIF1_A::NotComplete,
            true => TCIF1_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF1_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF1_A::Complete
    }
}
///Field `HTIF1` reader - Channel 1 Half Transfer Complete flag
pub type HTIF1_R = crate::BitReader<HTIF1_A>;
///Channel 1 Half Transfer Complete flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1_A {
    ///0: No half transfer event
    NotHalf = 0,
    ///1: A half transfer event has occured
    Half = 1,
}
impl From<HTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HTIF1_A {
        match self.bits {
            false => HTIF1_A::NotHalf,
            true => HTIF1_A::Half,
        }
    }
    ///Checks if the value of the field is `NotHalf`
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF1_A::NotHalf
    }
    ///Checks if the value of the field is `Half`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF1_A::Half
    }
}
///Field `TEIF1` reader - Channel 1 Transfer Error flag
pub type TEIF1_R = crate::BitReader<TEIF1_A>;
///Channel 1 Transfer Error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1_A {
    ///0: No transfer error
    NoError = 0,
    ///1: A transfer error has occured
    Error = 1,
}
impl From<TEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEIF1_A {
        match self.bits {
            false => TEIF1_A::NoError,
            true => TEIF1_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF1_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF1_A::Error
    }
}
///Field `GIF2` reader - Channel 2 Global interrupt flag
pub use GIF1_R as GIF2_R;
///Field `GIF3` reader - Channel 3 Global interrupt flag
pub use GIF1_R as GIF3_R;
///Field `GIF4` reader - Channel 4 Global interrupt flag
pub use GIF1_R as GIF4_R;
///Field `GIF5` reader - Channel 5 Global interrupt flag
pub use GIF1_R as GIF5_R;
///Field `GIF6` reader - Channel 6 Global interrupt flag
pub use GIF1_R as GIF6_R;
///Field `GIF7` reader - Channel 7 Global interrupt flag
pub use GIF1_R as GIF7_R;
///Field `HTIF2` reader - Channel 2 Half Transfer Complete flag
pub use HTIF1_R as HTIF2_R;
///Field `HTIF3` reader - Channel 3 Half Transfer Complete flag
pub use HTIF1_R as HTIF3_R;
///Field `HTIF4` reader - Channel 4 Half Transfer Complete flag
pub use HTIF1_R as HTIF4_R;
///Field `HTIF5` reader - Channel 5 Half Transfer Complete flag
pub use HTIF1_R as HTIF5_R;
///Field `HTIF6` reader - Channel 6 Half Transfer Complete flag
pub use HTIF1_R as HTIF6_R;
///Field `HTIF7` reader - Channel 7 Half Transfer Complete flag
pub use HTIF1_R as HTIF7_R;
///Field `TCIF2` reader - Channel 2 Transfer Complete flag
pub use TCIF1_R as TCIF2_R;
///Field `TCIF3` reader - Channel 3 Transfer Complete flag
pub use TCIF1_R as TCIF3_R;
///Field `TCIF4` reader - Channel 4 Transfer Complete flag
pub use TCIF1_R as TCIF4_R;
///Field `TCIF5` reader - Channel 5 Transfer Complete flag
pub use TCIF1_R as TCIF5_R;
///Field `TCIF6` reader - Channel 6 Transfer Complete flag
pub use TCIF1_R as TCIF6_R;
///Field `TCIF7` reader - Channel 7 Transfer Complete flag
pub use TCIF1_R as TCIF7_R;
///Field `TEIF2` reader - Channel 2 Transfer Error flag
pub use TEIF1_R as TEIF2_R;
///Field `TEIF3` reader - Channel 3 Transfer Error flag
pub use TEIF1_R as TEIF3_R;
///Field `TEIF4` reader - Channel 4 Transfer Error flag
pub use TEIF1_R as TEIF4_R;
///Field `TEIF5` reader - Channel 5 Transfer Error flag
pub use TEIF1_R as TEIF5_R;
///Field `TEIF6` reader - Channel 6 Transfer Error flag
pub use TEIF1_R as TEIF6_R;
///Field `TEIF7` reader - Channel 7 Transfer Error flag
pub use TEIF1_R as TEIF7_R;
impl R {
    ///Bit 0 - Channel 1 Global interrupt flag
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 Transfer Complete flag
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 1 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 1 Transfer Error flag
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 2 Global interrupt flag
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 2 Transfer Complete flag
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 2 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 2 Transfer Error flag
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel 3 Global interrupt flag
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel 3 Transfer Complete flag
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel 3 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel 3 Transfer Error flag
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel 4 Global interrupt flag
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel 4 Transfer Complete flag
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel 4 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel 4 Transfer Error flag
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel 5 Global interrupt flag
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel 5 Transfer Complete flag
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel 5 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel 5 Transfer Error flag
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel 6 Global interrupt flag
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel 6 Transfer Complete flag
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel 6 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel 6 Transfer Error flag
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel 7 Global interrupt flag
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel 7 Transfer Complete flag
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel 7 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel 7 Transfer Error flag
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///DMA interrupt status register (DMA_ISR)
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
