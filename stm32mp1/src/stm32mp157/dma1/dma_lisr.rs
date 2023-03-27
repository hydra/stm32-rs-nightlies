///Register `DMA_LISR` reader
pub struct R(crate::R<DMA_LISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_LISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_LISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_LISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FEIF0` reader - FEIF0
pub type FEIF0_R = crate::BitReader<bool>;
///Field `DMEIF0` reader - DMEIF0
pub type DMEIF0_R = crate::BitReader<bool>;
///Field `TEIF0` reader - TEIF0
pub type TEIF0_R = crate::BitReader<bool>;
///Field `HTIF0` reader - HTIF0
pub type HTIF0_R = crate::BitReader<bool>;
///Field `TCIF0` reader - TCIF0
pub type TCIF0_R = crate::BitReader<bool>;
///Field `FEIF1` reader - FEIF1
pub type FEIF1_R = crate::BitReader<bool>;
///Field `DMEIF1` reader - DMEIF1
pub type DMEIF1_R = crate::BitReader<bool>;
///Field `TEIF1` reader - TEIF1
pub type TEIF1_R = crate::BitReader<bool>;
///Field `HTIF1` reader - HTIF1
pub type HTIF1_R = crate::BitReader<bool>;
///Field `TCIF1` reader - TCIF1
pub type TCIF1_R = crate::BitReader<bool>;
///Field `FEIF2` reader - FEIF2
pub type FEIF2_R = crate::BitReader<bool>;
///Field `DMEIF2` reader - DMEIF2
pub type DMEIF2_R = crate::BitReader<bool>;
///Field `TEIF2` reader - TEIF2
pub type TEIF2_R = crate::BitReader<bool>;
///Field `HTIF2` reader - HTIF2
pub type HTIF2_R = crate::BitReader<bool>;
///Field `TCIF2` reader - TCIF2
pub type TCIF2_R = crate::BitReader<bool>;
///Field `FEIF3` reader - FEIF3
pub type FEIF3_R = crate::BitReader<bool>;
///Field `DMEIF3` reader - DMEIF3
pub type DMEIF3_R = crate::BitReader<bool>;
///Field `TEIF3` reader - TEIF3
pub type TEIF3_R = crate::BitReader<bool>;
///Field `HTIF3` reader - HTIF3
pub type HTIF3_R = crate::BitReader<bool>;
///Field `TCIF3` reader - TCIF3
pub type TCIF3_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - FEIF0
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - DMEIF0
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIF0
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HTIF0
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIF0
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FEIF1
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - DMEIF1
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TEIF1
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HTIF1
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TCIF1
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - FEIF2
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - DMEIF2
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TEIF2
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HTIF2
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCIF2
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - FEIF3
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DMEIF3
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TEIF3
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HTIF3
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TCIF3
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///DMA low interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_lisr](index.html) module
pub struct DMA_LISR_SPEC;
impl crate::RegisterSpec for DMA_LISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_lisr::R](R) reader structure
impl crate::Readable for DMA_LISR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMA_LISR to value 0
impl crate::Resettable for DMA_LISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
