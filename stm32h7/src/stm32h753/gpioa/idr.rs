///Register `IDR` reader
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IDR0` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub type IDR0_R = crate::BitReader<IDR0_A>;
///Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR0_A {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<IDR0_A> for bool {
    #[inline(always)]
    fn from(variant: IDR0_A) -> Self {
        variant as u8 != 0
    }
}
impl IDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDR0_A {
        match self.bits {
            false => IDR0_A::Low,
            true => IDR0_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR0_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR0_A::High
    }
}
///Field `IDR1` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR1_R;
///Field `IDR2` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR2_R;
///Field `IDR3` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR3_R;
///Field `IDR4` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR4_R;
///Field `IDR5` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR5_R;
///Field `IDR6` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR6_R;
///Field `IDR7` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR7_R;
///Field `IDR8` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR8_R;
///Field `IDR9` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR9_R;
///Field `IDR10` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR10_R;
///Field `IDR11` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR11_R;
///Field `IDR12` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR12_R;
///Field `IDR13` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR13_R;
///Field `IDR14` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR14_R;
///Field `IDR15` reader - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
pub use IDR0_R as IDR15_R;
impl R {
    ///Bit 0 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///GPIO port input data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idr](index.html) module
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [idr::R](R) reader structure
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
