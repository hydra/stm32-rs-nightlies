///Register `OTG_HPRT` reader
pub struct R(crate::R<OTG_HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HPRT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_HPRT` writer
pub struct W(crate::W<OTG_HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HPRT_SPEC>;
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
impl From<crate::W<OTG_HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HPRT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCSTS` reader - PCSTS
pub type PCSTS_R = crate::BitReader<bool>;
///Field `PCDET` reader - PCDET
pub type PCDET_R = crate::BitReader<bool>;
///Field `PCDET` writer - PCDET
pub type PCDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PENA` reader - PENA
pub type PENA_R = crate::BitReader<bool>;
///Field `PENA` writer - PENA
pub type PENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PENCHNG` reader - PENCHNG
pub type PENCHNG_R = crate::BitReader<bool>;
///Field `PENCHNG` writer - PENCHNG
pub type PENCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `POCA` reader - POCA
pub type POCA_R = crate::BitReader<bool>;
///Field `POCCHNG` reader - POCCHNG
pub type POCCHNG_R = crate::BitReader<bool>;
///Field `POCCHNG` writer - POCCHNG
pub type POCCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PRES` reader - PRES
pub type PRES_R = crate::BitReader<bool>;
///Field `PRES` writer - PRES
pub type PRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PSUSP` reader - PSUSP
pub type PSUSP_R = crate::BitReader<bool>;
///Field `PSUSP` writer - PSUSP
pub type PSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PRST` reader - PRST
pub type PRST_R = crate::BitReader<bool>;
///Field `PRST` writer - PRST
pub type PRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PLSTS` reader - PLSTS
pub type PLSTS_R = crate::FieldReader<u8, u8>;
///Field `PPWR` reader - PPWR
pub type PPWR_R = crate::BitReader<bool>;
///Field `PPWR` writer - PPWR
pub type PPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HPRT_SPEC, bool, O>;
///Field `PTCTL` reader - PTCTL
pub type PTCTL_R = crate::FieldReader<u8, u8>;
///Field `PTCTL` writer - PTCTL
pub type PTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HPRT_SPEC, u8, u8, 4, O>;
///Field `PSPD` reader - PSPD
pub type PSPD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - PCSTS
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PCDET
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PENA
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PENCHNG
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - POCA
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - POCCHNG
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRES
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PSUSP
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PRST
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - PLSTS
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - PPWR
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - PTCTL
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:18 - PSPD
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    ///Bit 1 - PCDET
    #[inline(always)]
    #[must_use]
    pub fn pcdet(&mut self) -> PCDET_W<1> {
        PCDET_W::new(self)
    }
    ///Bit 2 - PENA
    #[inline(always)]
    #[must_use]
    pub fn pena(&mut self) -> PENA_W<2> {
        PENA_W::new(self)
    }
    ///Bit 3 - PENCHNG
    #[inline(always)]
    #[must_use]
    pub fn penchng(&mut self) -> PENCHNG_W<3> {
        PENCHNG_W::new(self)
    }
    ///Bit 5 - POCCHNG
    #[inline(always)]
    #[must_use]
    pub fn pocchng(&mut self) -> POCCHNG_W<5> {
        POCCHNG_W::new(self)
    }
    ///Bit 6 - PRES
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<6> {
        PRES_W::new(self)
    }
    ///Bit 7 - PSUSP
    #[inline(always)]
    #[must_use]
    pub fn psusp(&mut self) -> PSUSP_W<7> {
        PSUSP_W::new(self)
    }
    ///Bit 8 - PRST
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<8> {
        PRST_W::new(self)
    }
    ///Bit 12 - PPWR
    #[inline(always)]
    #[must_use]
    pub fn ppwr(&mut self) -> PPWR_W<12> {
        PPWR_W::new(self)
    }
    ///Bits 13:16 - PTCTL
    #[inline(always)]
    #[must_use]
    pub fn ptctl(&mut self) -> PTCTL_W<13> {
        PTCTL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hprt](index.html) module
pub struct OTG_HPRT_SPEC;
impl crate::RegisterSpec for OTG_HPRT_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_hprt::R](R) reader structure
impl crate::Readable for OTG_HPRT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_hprt::W](W) writer structure
impl crate::Writable for OTG_HPRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_HPRT to value 0
impl crate::Resettable for OTG_HPRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
