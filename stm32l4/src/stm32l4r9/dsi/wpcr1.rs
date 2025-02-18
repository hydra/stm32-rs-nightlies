///Register `WPCR1` reader
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WPCR1` writer
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSTXDCL` reader - High-Speed Transmission Delay on Clock Lane
pub type HSTXDCL_R = crate::FieldReader<u8, u8>;
///Field `HSTXDCL` writer - High-Speed Transmission Delay on Clock Lane
pub type HSTXDCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `HSTXDLL` reader - High-Speed Transmission Delay on Data Lanes
pub type HSTXDLL_R = crate::FieldReader<u8, u8>;
///Field `HSTXDLL` writer - High-Speed Transmission Delay on Data Lanes
pub type HSTXDLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `LPSRCL` reader - Low-Power transmission Slew Rate Compensation on Clock Lane
pub type LPSRCL_R = crate::FieldReader<u8, u8>;
///Field `LPSRCL` writer - Low-Power transmission Slew Rate Compensation on Clock Lane
pub type LPSRCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `LPSRDL` reader - Low-Power transmission Slew Rate Compensation on Data Lanes
pub type LPSRDL_R = crate::FieldReader<u8, u8>;
///Field `LPSRDL` writer - Low-Power transmission Slew Rate Compensation on Data Lanes
pub type LPSRDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `SDCC` reader - SDD Control
pub type SDCC_R = crate::BitReader<bool>;
///Field `SDCC` writer - SDD Control
pub type SDCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
///Field `HSTXSRCCL` reader - High-Speed Transmission Slew Rate Control on Clock Lane
pub type HSTXSRCCL_R = crate::FieldReader<u8, u8>;
///Field `HSTXSRCCL` writer - High-Speed Transmission Slew Rate Control on Clock Lane
pub type HSTXSRCCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `HSTXSRCDL` reader - High-Speed Transmission Slew Rate Control on Data Lanes
pub type HSTXSRCDL_R = crate::FieldReader<u8, u8>;
///Field `HSTXSRCDL` writer - High-Speed Transmission Slew Rate Control on Data Lanes
pub type HSTXSRCDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
///Field `FLPRXLPM` reader - Forces LP Receiver in Low-Power Mode
pub type FLPRXLPM_R = crate::BitReader<bool>;
///Field `FLPRXLPM` writer - Forces LP Receiver in Low-Power Mode
pub type FLPRXLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
///Field `LPRXFT` reader - Low-Power RX low-pass Filtering Tuning
pub type LPRXFT_R = crate::FieldReader<u8, u8>;
///Field `LPRXFT` writer - Low-Power RX low-pass Filtering Tuning
pub type LPRXFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - High-Speed Transmission Delay on Clock Lane
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - High-Speed Transmission Delay on Data Lanes
    #[inline(always)]
    pub fn hstxdll(&self) -> HSTXDLL_R {
        HSTXDLL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane
    #[inline(always)]
    pub fn lpsrcl(&self) -> LPSRCL_R {
        LPSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes
    #[inline(always)]
    pub fn lpsrdl(&self) -> LPSRDL_R {
        LPSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 12 - SDD Control
    #[inline(always)]
    pub fn sdcc(&self) -> SDCC_R {
        SDCC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 22 - Forces LP Receiver in Low-Power Mode
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 25:26 - Low-Power RX low-pass Filtering Tuning
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - High-Speed Transmission Delay on Clock Lane
    #[inline(always)]
    #[must_use]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<0> {
        HSTXDCL_W::new(self)
    }
    ///Bits 2:3 - High-Speed Transmission Delay on Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn hstxdll(&mut self) -> HSTXDLL_W<2> {
        HSTXDLL_W::new(self)
    }
    ///Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane
    #[inline(always)]
    #[must_use]
    pub fn lpsrcl(&mut self) -> LPSRCL_W<6> {
        LPSRCL_W::new(self)
    }
    ///Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn lpsrdl(&mut self) -> LPSRDL_W<8> {
        LPSRDL_W::new(self)
    }
    ///Bit 12 - SDD Control
    #[inline(always)]
    #[must_use]
    pub fn sdcc(&mut self) -> SDCC_W<12> {
        SDCC_W::new(self)
    }
    ///Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane
    #[inline(always)]
    #[must_use]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<16> {
        HSTXSRCCL_W::new(self)
    }
    ///Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<18> {
        HSTXSRCDL_W::new(self)
    }
    ///Bit 22 - Forces LP Receiver in Low-Power Mode
    #[inline(always)]
    #[must_use]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<22> {
        FLPRXLPM_W::new(self)
    }
    ///Bits 25:26 - Low-Power RX low-pass Filtering Tuning
    #[inline(always)]
    #[must_use]
    pub fn lprxft(&mut self) -> LPRXFT_W<25> {
        LPRXFT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Wrapper PHY Configuration Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpcr1](index.html) module
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [wpcr1::R](R) reader structure
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wpcr1::W](W) writer structure
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WPCR1 to value 0
impl crate::Resettable for WPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
