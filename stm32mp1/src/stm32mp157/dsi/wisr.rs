///Register `WISR` reader
pub struct R(crate::R<WISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TEIF` reader - TEIF
pub type TEIF_R = crate::BitReader<bool>;
///Field `ERIF` reader - ERIF
pub type ERIF_R = crate::BitReader<bool>;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<bool>;
///Field `PLLLS` reader - PLLLS
pub type PLLLS_R = crate::BitReader<bool>;
///Field `PLLLIF` reader - PLLLIF
pub type PLLLIF_R = crate::BitReader<bool>;
///Field `PLLUIF` reader - PLLUIF
pub type PLLUIF_R = crate::BitReader<bool>;
///Field `RRS` reader - RRS
pub type RRS_R = crate::BitReader<bool>;
///Field `RRIF` reader - RRIF
pub type RRIF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TEIF
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ERIF
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - PLLLS
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLLLIF
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLLUIF
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - RRS
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RRIF
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
///DSI wrapper interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wisr](index.html) module
pub struct WISR_SPEC;
impl crate::RegisterSpec for WISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wisr::R](R) reader structure
impl crate::Readable for WISR_SPEC {
    type Reader = R;
}
///`reset()` method sets WISR to value 0
impl crate::Resettable for WISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
