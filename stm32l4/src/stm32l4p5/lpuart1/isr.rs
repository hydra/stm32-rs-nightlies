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
///Field `PE` reader - PE
pub type PE_R = crate::BitReader<bool>;
///Field `FE` reader - FE
pub type FE_R = crate::BitReader<bool>;
///Field `NF` reader - NF
pub type NF_R = crate::BitReader<bool>;
///Field `ORE` reader - ORE
pub type ORE_R = crate::BitReader<bool>;
///Field `IDLE` reader - IDLE
pub type IDLE_R = crate::BitReader<bool>;
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader<bool>;
///Field `TC` reader - TC
pub type TC_R = crate::BitReader<bool>;
///Field `TXE` reader - TXE
pub type TXE_R = crate::BitReader<bool>;
///Field `CTSIF` reader - CTSIF
pub type CTSIF_R = crate::BitReader<bool>;
///Field `CTS` reader - CTS
pub type CTS_R = crate::BitReader<bool>;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<bool>;
///Field `CMF` reader - CMF
pub type CMF_R = crate::BitReader<bool>;
///Field `SBKF` reader - SBKF
pub type SBKF_R = crate::BitReader<bool>;
///Field `RWU` reader - RWU
pub type RWU_R = crate::BitReader<bool>;
///Field `WUF` reader - WUF
pub type WUF_R = crate::BitReader<bool>;
///Field `TEACK` reader - TEACK
pub type TEACK_R = crate::BitReader<bool>;
///Field `REACK` reader - REACK
pub type REACK_R = crate::BitReader<bool>;
///Field `TXFF` reader - ??
pub type TXFF_R = crate::BitReader<bool>;
///Field `RXFF` reader - ??
pub type RXFF_R = crate::BitReader<bool>;
///Field `RXFT` reader - RXFIFO threshold flag
pub type RXFT_R = crate::BitReader<bool>;
///Field `TXFT` reader - TXFIFO threshold flag
pub type TXFT_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NF
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXE
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WUF
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ??
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ??
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RXFIFO threshold flag
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFIFO threshold flag
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///Interrupt &amp; status register
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
///`reset()` method sets ISR to value 0xc0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
