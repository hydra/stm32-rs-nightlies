///Register `HISR` reader
pub struct R(crate::R<HISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FEIF4` reader - Stream x FIFO error interrupt flag (x=7..4)
pub type FEIF4_R = crate::BitReader<FEIF4_A>;
///Stream x FIFO error interrupt flag (x=7..4)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIF4_A {
    ///0: No FIFO error event on stream x
    NoError = 0,
    ///1: A FIFO error event occurred on stream x
    Error = 1,
}
impl From<FEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FEIF4_A {
        match self.bits {
            false => FEIF4_A::NoError,
            true => FEIF4_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF4_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF4_A::Error
    }
}
///Field `DMEIF4` reader - Stream x direct mode error interrupt flag (x=7..4)
pub type DMEIF4_R = crate::BitReader<DMEIF4_A>;
///Stream x direct mode error interrupt flag (x=7..4)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMEIF4_A {
    ///0: No Direct Mode error on stream x
    NoError = 0,
    ///1: A Direct Mode error occurred on stream x
    Error = 1,
}
impl From<DMEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl DMEIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMEIF4_A {
        match self.bits {
            false => DMEIF4_A::NoError,
            true => DMEIF4_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF4_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF4_A::Error
    }
}
///Field `TEIF4` reader - Stream x transfer error interrupt flag (x=7..4)
pub type TEIF4_R = crate::BitReader<TEIF4_A>;
///Stream x transfer error interrupt flag (x=7..4)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF4_A {
    ///0: No transfer error on stream x
    NoError = 0,
    ///1: A transfer error occurred on stream x
    Error = 1,
}
impl From<TEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEIF4_A {
        match self.bits {
            false => TEIF4_A::NoError,
            true => TEIF4_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF4_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF4_A::Error
    }
}
///Field `HTIF4` reader - Stream x half transfer interrupt flag (x=7..4)
pub type HTIF4_R = crate::BitReader<HTIF4_A>;
///Stream x half transfer interrupt flag (x=7..4)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF4_A {
    ///0: No half transfer event on stream x
    NotHalf = 0,
    ///1: A half transfer event occurred on stream x
    Half = 1,
}
impl From<HTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HTIF4_A {
        match self.bits {
            false => HTIF4_A::NotHalf,
            true => HTIF4_A::Half,
        }
    }
    ///Checks if the value of the field is `NotHalf`
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF4_A::NotHalf
    }
    ///Checks if the value of the field is `Half`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF4_A::Half
    }
}
///Field `TCIF4` reader - Stream x transfer complete interrupt flag (x=7..4)
pub type TCIF4_R = crate::BitReader<TCIF4_A>;
///Stream x transfer complete interrupt flag (x=7..4)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF4_A {
    ///0: No transfer complete event on stream x
    NotComplete = 0,
    ///1: A transfer complete event occurred on stream x
    Complete = 1,
}
impl From<TCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIF4_A {
        match self.bits {
            false => TCIF4_A::NotComplete,
            true => TCIF4_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF4_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF4_A::Complete
    }
}
///Field `DMEIF5` reader - Stream x direct mode error interrupt flag (x=7..4)
pub use DMEIF4_R as DMEIF5_R;
///Field `DMEIF6` reader - Stream x direct mode error interrupt flag (x=7..4)
pub use DMEIF4_R as DMEIF6_R;
///Field `DMEIF7` reader - Stream x direct mode error interrupt flag (x=7..4)
pub use DMEIF4_R as DMEIF7_R;
///Field `FEIF5` reader - Stream x FIFO error interrupt flag (x=7..4)
pub use FEIF4_R as FEIF5_R;
///Field `FEIF6` reader - Stream x FIFO error interrupt flag (x=7..4)
pub use FEIF4_R as FEIF6_R;
///Field `FEIF7` reader - Stream x FIFO error interrupt flag (x=7..4)
pub use FEIF4_R as FEIF7_R;
///Field `HTIF5` reader - Stream x half transfer interrupt flag (x=7..4)
pub use HTIF4_R as HTIF5_R;
///Field `HTIF6` reader - Stream x half transfer interrupt flag (x=7..4)
pub use HTIF4_R as HTIF6_R;
///Field `HTIF7` reader - Stream x half transfer interrupt flag (x=7..4)
pub use HTIF4_R as HTIF7_R;
///Field `TCIF5` reader - Stream x transfer complete interrupt flag (x=7..4)
pub use TCIF4_R as TCIF5_R;
///Field `TCIF6` reader - Stream x transfer complete interrupt flag (x=7..4)
pub use TCIF4_R as TCIF6_R;
///Field `TCIF7` reader - Stream x transfer complete interrupt flag (x=7..4)
pub use TCIF4_R as TCIF7_R;
///Field `TEIF5` reader - Stream x transfer error interrupt flag (x=7..4)
pub use TEIF4_R as TEIF5_R;
///Field `TEIF6` reader - Stream x transfer error interrupt flag (x=7..4)
pub use TEIF4_R as TEIF6_R;
///Field `TEIF7` reader - Stream x transfer error interrupt flag (x=7..4)
pub use TEIF4_R as TEIF7_R;
impl R {
    ///Bit 0 - Stream x FIFO error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Stream x direct mode error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stream x transfer error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Stream x half transfer interrupt flag (x=7..4)
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stream x transfer complete interrupt flag (x=7..4)
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stream x FIFO error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Stream x direct mode error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stream x transfer error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Stream x half transfer interrupt flag (x=7..4)
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Stream x transfer complete interrupt flag (x=7..4)
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Stream x FIFO error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Stream x direct mode error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Stream x transfer error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Stream x half transfer interrupt flag (x=7..4)
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Stream x transfer complete interrupt flag (x=7..4)
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Stream x FIFO error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Stream x direct mode error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Stream x transfer error interrupt flag (x=7..4)
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Stream x half transfer interrupt flag (x=7..4)
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Stream x transfer complete interrupt flag (x=7..4)
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///high interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hisr](index.html) module
pub struct HISR_SPEC;
impl crate::RegisterSpec for HISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hisr::R](R) reader structure
impl crate::Readable for HISR_SPEC {
    type Reader = R;
}
///`reset()` method sets HISR to value 0
impl crate::Resettable for HISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
