///Register `IFCR` reader
pub struct R(crate::R<IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CGIF1` reader - Clear channel 1 global interrupt flag
pub type CGIF1_R = crate::BitReader<CGIF1_A>;
///Clear channel 1 global interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGIF1_A {
    ///1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    Clear = 1,
}
impl From<CGIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CGIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CGIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CGIF1_A> {
        match self.bits {
            true => Some(CGIF1_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CGIF1_A::Clear
    }
}
///Field `CTCIF1` reader - Clear channel 1 transfer complete flag
pub type CTCIF1_R = crate::BitReader<CTCIF1_A>;
///Clear channel 1 transfer complete flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF1_A {
    ///1: Clears the TCIF flag in the ISR register
    Clear = 1,
}
impl From<CTCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTCIF1_A> {
        match self.bits {
            true => Some(CTCIF1_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF1_A::Clear
    }
}
///Field `CHTIF1` reader - Clear channel 1 half transfer flag
pub type CHTIF1_R = crate::BitReader<CHTIF1_A>;
///Clear channel 1 half transfer flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF1_A {
    ///1: Clears the HTIF flag in the ISR register
    Clear = 1,
}
impl From<CHTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CHTIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHTIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CHTIF1_A> {
        match self.bits {
            true => Some(CHTIF1_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CHTIF1_A::Clear
    }
}
///Field `CTEIF1` reader - Clear channel 1 transfer error flag
pub type CTEIF1_R = crate::BitReader<CTEIF1_A>;
///Clear channel 1 transfer error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF1_A {
    ///1: Clears the TEIF flag in the ISR register
    Clear = 1,
}
impl From<CTEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CTEIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEIF1_A> {
        match self.bits {
            true => Some(CTEIF1_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF1_A::Clear
    }
}
///Field `CGIF2` reader - Clear channel 2 global interrupt flag
pub use CGIF1_R as CGIF2_R;
///Field `CGIF3` reader - Clear channel 3 global interrupt flag
pub use CGIF1_R as CGIF3_R;
///Field `CGIF4` reader - Clear channel 4 global interrupt flag
pub use CGIF1_R as CGIF4_R;
///Field `CGIF5` reader - Clear channel 5 global interrupt flag
pub use CGIF1_R as CGIF5_R;
///Field `CGIF6` reader - Clear channel 6 global interrupt flag
pub use CGIF1_R as CGIF6_R;
///Field `CGIF7` reader - Clear channel 7 global interrupt flag
pub use CGIF1_R as CGIF7_R;
///Field `CHTIF2` reader - Clear channel 2 half transfer flag
pub use CHTIF1_R as CHTIF2_R;
///Field `CHTIF3` reader - Clear channel 3 half transfer flag
pub use CHTIF1_R as CHTIF3_R;
///Field `CHTIF4` reader - Clear channel 4 half transfer flag
pub use CHTIF1_R as CHTIF4_R;
///Field `CHTIF5` reader - Clear channel 5 half transfer flag
pub use CHTIF1_R as CHTIF5_R;
///Field `CHTIF6` reader - Clear channel 6 half transfer flag
pub use CHTIF1_R as CHTIF6_R;
///Field `CHTIF7` reader - Clear channel 7 half transfer flag
pub use CHTIF1_R as CHTIF7_R;
///Field `CTCIF2` reader - Clear channel 2 transfer complete flag
pub use CTCIF1_R as CTCIF2_R;
///Field `CTCIF3` reader - Clear channel 3 transfer complete flag
pub use CTCIF1_R as CTCIF3_R;
///Field `CTCIF4` reader - Clear channel 4 transfer complete flag
pub use CTCIF1_R as CTCIF4_R;
///Field `CTCIF5` reader - Clear channel 5 transfer complete flag
pub use CTCIF1_R as CTCIF5_R;
///Field `CTCIF6` reader - Clear channel 6 transfer complete flag
pub use CTCIF1_R as CTCIF6_R;
///Field `CTCIF7` reader - Clear channel 7 transfer complete flag
pub use CTCIF1_R as CTCIF7_R;
///Field `CTEIF2` reader - Clear channel 2 transfer error flag
pub use CTEIF1_R as CTEIF2_R;
///Field `CTEIF3` reader - Clear channel 3 transfer error flag
pub use CTEIF1_R as CTEIF3_R;
///Field `CTEIF4` reader - Clear channel 4 transfer error flag
pub use CTEIF1_R as CTEIF4_R;
///Field `CTEIF5` reader - Clear channel 5 transfer error flag
pub use CTEIF1_R as CTEIF5_R;
///Field `CTEIF6` reader - Clear channel 6 transfer error flag
pub use CTEIF1_R as CTEIF6_R;
///Field `CTEIF7` reader - Clear channel 7 transfer error flag
pub use CTEIF1_R as CTEIF7_R;
impl R {
    ///Bit 0 - Clear channel 1 global interrupt flag
    #[inline(always)]
    pub fn cgif1(&self) -> CGIF1_R {
        CGIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear channel 1 transfer complete flag
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear channel 1 half transfer flag
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear channel 1 transfer error flag
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear channel 2 global interrupt flag
    #[inline(always)]
    pub fn cgif2(&self) -> CGIF2_R {
        CGIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear channel 2 transfer complete flag
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clear channel 2 half transfer flag
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clear channel 2 transfer error flag
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Clear channel 3 global interrupt flag
    #[inline(always)]
    pub fn cgif3(&self) -> CGIF3_R {
        CGIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clear channel 3 transfer complete flag
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clear channel 3 half transfer flag
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clear channel 3 transfer error flag
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Clear channel 4 global interrupt flag
    #[inline(always)]
    pub fn cgif4(&self) -> CGIF4_R {
        CGIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Clear channel 4 transfer complete flag
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Clear channel 4 half transfer flag
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF4_R {
        CHTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Clear channel 4 transfer error flag
    #[inline(always)]
    pub fn cteif4(&self) -> CTEIF4_R {
        CTEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Clear channel 5 global interrupt flag
    #[inline(always)]
    pub fn cgif5(&self) -> CGIF5_R {
        CGIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clear channel 5 transfer complete flag
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Clear channel 5 half transfer flag
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF5_R {
        CHTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clear channel 5 transfer error flag
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF5_R {
        CTEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Clear channel 6 global interrupt flag
    #[inline(always)]
    pub fn cgif6(&self) -> CGIF6_R {
        CGIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Clear channel 6 transfer complete flag
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Clear channel 6 half transfer flag
    #[inline(always)]
    pub fn chtif6(&self) -> CHTIF6_R {
        CHTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Clear channel 6 transfer error flag
    #[inline(always)]
    pub fn cteif6(&self) -> CTEIF6_R {
        CTEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Clear channel 7 global interrupt flag
    #[inline(always)]
    pub fn cgif7(&self) -> CGIF7_R {
        CGIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Clear channel 7 transfer complete flag
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Clear channel 7 half transfer flag
    #[inline(always)]
    pub fn chtif7(&self) -> CHTIF7_R {
        CHTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Clear channel 7 transfer error flag
    #[inline(always)]
    pub fn cteif7(&self) -> CTEIF7_R {
        CTEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///high interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](index.html) module
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ifcr::R](R) reader structure
impl crate::Readable for IFCR_SPEC {
    type Reader = R;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
