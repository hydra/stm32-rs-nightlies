///Register `GRSTCTL` reader
pub struct R(crate::R<GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GRSTCTL` writer
pub struct W(crate::W<GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSRST` reader - Core soft reset
pub type CSRST_R = crate::BitReader<bool>;
///Field `CSRST` writer - Core soft reset
pub type CSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
///Field `HSRST` reader - HCLK soft reset
pub type HSRST_R = crate::BitReader<bool>;
///Field `HSRST` writer - HCLK soft reset
pub type HSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
///Field `FCRST` reader - Host frame counter reset
pub type FCRST_R = crate::BitReader<bool>;
///Field `FCRST` writer - Host frame counter reset
pub type FCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
///Field `RXFFLSH` reader - RxFIFO flush
pub type RXFFLSH_R = crate::BitReader<bool>;
///Field `RXFFLSH` writer - RxFIFO flush
pub type RXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
///Field `TXFFLSH` reader - TxFIFO flush
pub type TXFFLSH_R = crate::BitReader<bool>;
///Field `TXFFLSH` writer - TxFIFO flush
pub type TXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
///Field `TXFNUM` reader - TxFIFO number
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
///Field `TXFNUM` writer - TxFIFO number
pub type TXFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GRSTCTL_SPEC, u8, u8, 5, O>;
///Field `DMAREQ` reader - DMA request signal enabled for USB OTG HS
pub type DMAREQ_R = crate::BitReader<bool>;
///Field `AHBIDL` reader - AHB master idle
pub type AHBIDL_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    pub fn hsrst(&self) -> HSRST_R {
        HSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 30 - DMA request signal enabled for USB OTG HS
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AHB master idle
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<0> {
        CSRST_W::new(self)
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    #[must_use]
    pub fn hsrst(&mut self) -> HSRST_W<1> {
        HSRST_W::new(self)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    #[must_use]
    pub fn fcrst(&mut self) -> FCRST_W<2> {
        FCRST_W::new(self)
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<4> {
        RXFFLSH_W::new(self)
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<5> {
        TXFFLSH_W::new(self)
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<6> {
        TXFNUM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grstctl](index.html) module
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [grstctl::R](R) reader structure
impl crate::Readable for GRSTCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [grstctl::W](W) writer structure
impl crate::Writable for GRSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GRSTCTL to value 0x2000_0000
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
