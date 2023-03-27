///Register `LTDC_GC1R` reader
pub struct R(crate::R<LTDC_GC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_GC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_GC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_GC1R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WBCH` reader - WBCH
pub type WBCH_R = crate::FieldReader<u8, u8>;
///Field `WGCH` reader - WGCH
pub type WGCH_R = crate::FieldReader<u8, u8>;
///Field `WRCH` reader - WRCH
pub type WRCH_R = crate::FieldReader<u8, u8>;
///Field `PRBEN` reader - PRBEN
pub type PRBEN_R = crate::BitReader<bool>;
///Field `DT` reader - DT
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `GCT` reader - GCT
pub type GCT_R = crate::FieldReader<u8, u8>;
///Field `SHREN` reader - SHREN
pub type SHREN_R = crate::BitReader<bool>;
///Field `BCP` reader - BCP
pub type BCP_R = crate::BitReader<bool>;
///Field `BBEN` reader - BBEN
pub type BBEN_R = crate::BitReader<bool>;
///Field `LNIP` reader - LNIP
pub type LNIP_R = crate::BitReader<bool>;
///Field `TP` reader - TP
pub type TP_R = crate::BitReader<bool>;
///Field `IPP` reader - IPP
pub type IPP_R = crate::BitReader<bool>;
///Field `SPP` reader - SPP
pub type SPP_R = crate::BitReader<bool>;
///Field `DWP` reader - DWP
pub type DWP_R = crate::BitReader<bool>;
///Field `STREN` reader - STREN
pub type STREN_R = crate::BitReader<bool>;
///Field `BMEN` reader - BMEN
pub type BMEN_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - WBCH
    #[inline(always)]
    pub fn wbch(&self) -> WBCH_R {
        WBCH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - WGCH
    #[inline(always)]
    pub fn wgch(&self) -> WGCH_R {
        WGCH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - WRCH
    #[inline(always)]
    pub fn wrch(&self) -> WRCH_R {
        WRCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - PRBEN
    #[inline(always)]
    pub fn prben(&self) -> PRBEN_R {
        PRBEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - DT
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 17:19 - GCT
    #[inline(always)]
    pub fn gct(&self) -> GCT_R {
        GCT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 21 - SHREN
    #[inline(always)]
    pub fn shren(&self) -> SHREN_R {
        SHREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - BCP
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - BBEN
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - LNIP
    #[inline(always)]
    pub fn lnip(&self) -> LNIP_R {
        LNIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TP
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - IPP
    #[inline(always)]
    pub fn ipp(&self) -> IPP_R {
        IPP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SPP
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DWP
    #[inline(always)]
    pub fn dwp(&self) -> DWP_R {
        DWP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - STREN
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - BMEN
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///LTDC global configuration 1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltdc_gc1r](index.html) module
pub struct LTDC_GC1R_SPEC;
impl crate::RegisterSpec for LTDC_GC1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltdc_gc1r::R](R) reader structure
impl crate::Readable for LTDC_GC1R_SPEC {
    type Reader = R;
}
///`reset()` method sets LTDC_GC1R to value 0x6be2_d888
impl crate::Resettable for LTDC_GC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x6be2_d888;
}
