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
///Field `HSYNC` reader - HSYNC
pub type HSYNC_R = crate::BitReader<HSYNC_A>;
///HSYNC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC_A {
    ///0: Active line
    ActiveLine = 0,
    ///1: Synchronization between lines
    BetweenLines = 1,
}
impl From<HSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl HSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSYNC_A {
        match self.bits {
            false => HSYNC_A::ActiveLine,
            true => HSYNC_A::BetweenLines,
        }
    }
    ///Checks if the value of the field is `ActiveLine`
    #[inline(always)]
    pub fn is_active_line(&self) -> bool {
        *self == HSYNC_A::ActiveLine
    }
    ///Checks if the value of the field is `BetweenLines`
    #[inline(always)]
    pub fn is_between_lines(&self) -> bool {
        *self == HSYNC_A::BetweenLines
    }
}
///Field `VSYNC` reader - VSYNC
pub type VSYNC_R = crate::BitReader<VSYNC_A>;
///VSYNC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_A {
    ///0: Active frame
    ActiveFrame = 0,
    ///1: Synchronization between frames
    BetweenFrames = 1,
}
impl From<VSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_A {
        match self.bits {
            false => VSYNC_A::ActiveFrame,
            true => VSYNC_A::BetweenFrames,
        }
    }
    ///Checks if the value of the field is `ActiveFrame`
    #[inline(always)]
    pub fn is_active_frame(&self) -> bool {
        *self == VSYNC_A::ActiveFrame
    }
    ///Checks if the value of the field is `BetweenFrames`
    #[inline(always)]
    pub fn is_between_frames(&self) -> bool {
        *self == VSYNC_A::BetweenFrames
    }
}
///Field `FNE` reader - FIFO not empty
pub type FNE_R = crate::BitReader<FNE_A>;
///FIFO not empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FNE_A {
    ///0: FIFO contains valid data
    NotEmpty = 0,
    ///1: FIFO empty
    Empty = 1,
}
impl From<FNE_A> for bool {
    #[inline(always)]
    fn from(variant: FNE_A) -> Self {
        variant as u8 != 0
    }
}
impl FNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FNE_A {
        match self.bits {
            false => FNE_A::NotEmpty,
            true => FNE_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FNE_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FNE_A::Empty
    }
}
impl R {
    ///Bit 0 - HSYNC
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSYNC
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO not empty
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///status register
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
