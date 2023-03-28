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
///Field `ID0` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID0_R = crate::BitReader<bool>;
///Field `ID1` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID1_R = crate::BitReader<bool>;
///Field `ID2` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID2_R = crate::BitReader<bool>;
///Field `ID3` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID3_R = crate::BitReader<bool>;
///Field `ID4` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID4_R = crate::BitReader<bool>;
///Field `ID5` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID5_R = crate::BitReader<bool>;
///Field `ID6` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID6_R = crate::BitReader<bool>;
///Field `ID7` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID7_R = crate::BitReader<bool>;
///Field `ID8` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID8_R = crate::BitReader<bool>;
///Field `ID9` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID9_R = crate::BitReader<bool>;
///Field `ID10` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID10_R = crate::BitReader<bool>;
///Field `ID11` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID11_R = crate::BitReader<bool>;
///Field `ID12` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID12_R = crate::BitReader<bool>;
///Field `ID13` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID13_R = crate::BitReader<bool>;
///Field `ID14` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID14_R = crate::BitReader<bool>;
///Field `ID15` reader - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type ID15_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x input data I/O pin y (y = 15 to 0) These bits are read-only. They contain the input value of the corresponding I/O port. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
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
