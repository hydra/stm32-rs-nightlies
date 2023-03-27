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
///Field `ID0` reader - Port input data (y = 0..7)
pub type ID0_R = crate::BitReader<bool>;
///Field `ID1` reader - Port input data (y = 0..7)
pub type ID1_R = crate::BitReader<bool>;
///Field `ID2` reader - Port input data (y = 0..7)
pub type ID2_R = crate::BitReader<bool>;
///Field `ID3` reader - Port input data (y = 0..7)
pub type ID3_R = crate::BitReader<bool>;
///Field `ID4` reader - Port input data (y = 0..7)
pub type ID4_R = crate::BitReader<bool>;
///Field `ID5` reader - Port input data (y = 0..7)
pub type ID5_R = crate::BitReader<bool>;
///Field `ID6` reader - Port input data (y = 0..7)
pub type ID6_R = crate::BitReader<bool>;
///Field `ID7` reader - Port input data (y = 0..7)
pub type ID7_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data (y = 0..7)
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
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
