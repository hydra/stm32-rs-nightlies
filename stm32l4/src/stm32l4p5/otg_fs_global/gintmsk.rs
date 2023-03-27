///Register `GINTMSK` reader
pub struct R(crate::R<GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GINTMSK` writer
pub struct W(crate::W<GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_SPEC>;
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
impl From<crate::W<GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MMISM` reader - Mode mismatch interrupt mask
pub type MMISM_R = crate::BitReader<bool>;
///Field `MMISM` writer - Mode mismatch interrupt mask
pub type MMISM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `OTGINT` reader - OTG interrupt mask
pub type OTGINT_R = crate::BitReader<bool>;
///Field `OTGINT` writer - OTG interrupt mask
pub type OTGINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `SOFM` reader - Start of frame mask
pub type SOFM_R = crate::BitReader<bool>;
///Field `SOFM` writer - Start of frame mask
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `RXFLVLM` reader - Receive FIFO non-empty mask
pub type RXFLVLM_R = crate::BitReader<bool>;
///Field `RXFLVLM` writer - Receive FIFO non-empty mask
pub type RXFLVLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `NPTXFEM` reader - Non-periodic TxFIFO empty mask
pub type NPTXFEM_R = crate::BitReader<bool>;
///Field `NPTXFEM` writer - Non-periodic TxFIFO empty mask
pub type NPTXFEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `GINAKEFFM` reader - Global non-periodic IN NAK effective mask
pub type GINAKEFFM_R = crate::BitReader<bool>;
///Field `GINAKEFFM` writer - Global non-periodic IN NAK effective mask
pub type GINAKEFFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `GONAKEFFM` reader - Global OUT NAK effective mask
pub type GONAKEFFM_R = crate::BitReader<bool>;
///Field `GONAKEFFM` writer - Global OUT NAK effective mask
pub type GONAKEFFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `ESUSPM` reader - Early suspend mask
pub type ESUSPM_R = crate::BitReader<bool>;
///Field `ESUSPM` writer - Early suspend mask
pub type ESUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `USBSUSPM` reader - USB suspend mask
pub type USBSUSPM_R = crate::BitReader<bool>;
///Field `USBSUSPM` writer - USB suspend mask
pub type USBSUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `USBRST` reader - USB reset mask
pub type USBRST_R = crate::BitReader<bool>;
///Field `USBRST` writer - USB reset mask
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `ENUMDNEM` reader - Enumeration done mask
pub type ENUMDNEM_R = crate::BitReader<bool>;
///Field `ENUMDNEM` writer - Enumeration done mask
pub type ENUMDNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask
pub type ISOODRPM_R = crate::BitReader<bool>;
///Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask
pub type ISOODRPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `EOPFM` reader - End of periodic frame interrupt mask
pub type EOPFM_R = crate::BitReader<bool>;
///Field `EOPFM` writer - End of periodic frame interrupt mask
pub type EOPFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `EPMISM` reader - Endpoint mismatch interrupt mask
pub type EPMISM_R = crate::BitReader<bool>;
///Field `EPMISM` writer - Endpoint mismatch interrupt mask
pub type EPMISM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `IEPINT` reader - IN endpoints interrupt mask
pub type IEPINT_R = crate::BitReader<bool>;
///Field `IEPINT` writer - IN endpoints interrupt mask
pub type IEPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `OEPINT` reader - OUT endpoints interrupt mask
pub type OEPINT_R = crate::BitReader<bool>;
///Field `OEPINT` writer - OUT endpoints interrupt mask
pub type OEPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask
pub type IISOIXFRM_R = crate::BitReader<bool>;
///Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask
pub type IISOIXFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `IPXFRM_IISOOXFRM` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)
pub type IPXFRM_IISOOXFRM_R = crate::BitReader<bool>;
///Field `IPXFRM_IISOOXFRM` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)
pub type IPXFRM_IISOOXFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `RSTDETM` reader - Reset detected interrupt mask
pub type RSTDETM_R = crate::BitReader<bool>;
///Field `RSTDETM` writer - Reset detected interrupt mask
pub type RSTDETM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `PRTIM` reader - Host port interrupt mask
pub type PRTIM_R = crate::BitReader<bool>;
///Field `HCIM` reader - Host channels interrupt mask
pub type HCIM_R = crate::BitReader<bool>;
///Field `HCIM` writer - Host channels interrupt mask
pub type HCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `PTXFEM` reader - Periodic TxFIFO empty mask
pub type PTXFEM_R = crate::BitReader<bool>;
///Field `PTXFEM` writer - Periodic TxFIFO empty mask
pub type PTXFEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `LPMINTM` reader - LPM interrupt mask
pub type LPMINTM_R = crate::BitReader<bool>;
///Field `LPMINTM` writer - LPM interrupt mask
pub type LPMINTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `CIDSCHGM` reader - Connector ID status change mask
pub type CIDSCHGM_R = crate::BitReader<bool>;
///Field `CIDSCHGM` writer - Connector ID status change mask
pub type CIDSCHGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `DISCINT` reader - Disconnect detected interrupt mask
pub type DISCINT_R = crate::BitReader<bool>;
///Field `DISCINT` writer - Disconnect detected interrupt mask
pub type DISCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `SRQIM` reader - Session request/new session detected interrupt mask
pub type SRQIM_R = crate::BitReader<bool>;
///Field `SRQIM` writer - Session request/new session detected interrupt mask
pub type SRQIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
///Field `WUIM` reader - Resume/remote wakeup detected interrupt mask
pub type WUIM_R = crate::BitReader<bool>;
///Field `WUIM` writer - Resume/remote wakeup detected interrupt mask
pub type WUIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
impl R {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-periodic TxFIFO empty mask
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Global non-periodic IN NAK effective mask
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global OUT NAK effective mask
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Early suspend mask
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USB suspend mask
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB reset mask
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enumeration done mask
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt mask
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - End of periodic frame interrupt mask
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Endpoint mismatch interrupt mask
    #[inline(always)]
    pub fn epmism(&self) -> EPMISM_R {
        EPMISM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IN endpoints interrupt mask
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OUT endpoints interrupt mask
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Incomplete isochronous IN transfer mask
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)
    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(&self) -> IPXFRM_IISOOXFRM_R {
        IPXFRM_IISOOXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Reset detected interrupt mask
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Host port interrupt mask
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Host channels interrupt mask
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Periodic TxFIFO empty mask
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Disconnect detected interrupt mask
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resume/remote wakeup detected interrupt mask
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn mmism(&mut self) -> MMISM_W<1> {
        MMISM_W::new(self)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn otgint(&mut self) -> OTGINT_W<2> {
        OTGINT_W::new(self)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<3> {
        SOFM_W::new(self)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    #[must_use]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<4> {
        RXFLVLM_W::new(self)
    }
    ///Bit 5 - Non-periodic TxFIFO empty mask
    #[inline(always)]
    #[must_use]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<5> {
        NPTXFEM_W::new(self)
    }
    ///Bit 6 - Global non-periodic IN NAK effective mask
    #[inline(always)]
    #[must_use]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<6> {
        GINAKEFFM_W::new(self)
    }
    ///Bit 7 - Global OUT NAK effective mask
    #[inline(always)]
    #[must_use]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<7> {
        GONAKEFFM_W::new(self)
    }
    ///Bit 10 - Early suspend mask
    #[inline(always)]
    #[must_use]
    pub fn esuspm(&mut self) -> ESUSPM_W<10> {
        ESUSPM_W::new(self)
    }
    ///Bit 11 - USB suspend mask
    #[inline(always)]
    #[must_use]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<11> {
        USBSUSPM_W::new(self)
    }
    ///Bit 12 - USB reset mask
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<12> {
        USBRST_W::new(self)
    }
    ///Bit 13 - Enumeration done mask
    #[inline(always)]
    #[must_use]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<13> {
        ENUMDNEM_W::new(self)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<14> {
        ISOODRPM_W::new(self)
    }
    ///Bit 15 - End of periodic frame interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn eopfm(&mut self) -> EOPFM_W<15> {
        EOPFM_W::new(self)
    }
    ///Bit 17 - Endpoint mismatch interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn epmism(&mut self) -> EPMISM_W<17> {
        EPMISM_W::new(self)
    }
    ///Bit 18 - IN endpoints interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn iepint(&mut self) -> IEPINT_W<18> {
        IEPINT_W::new(self)
    }
    ///Bit 19 - OUT endpoints interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<19> {
        OEPINT_W::new(self)
    }
    ///Bit 20 - Incomplete isochronous IN transfer mask
    #[inline(always)]
    #[must_use]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<20> {
        IISOIXFRM_W::new(self)
    }
    ///Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)
    #[inline(always)]
    #[must_use]
    pub fn ipxfrm_iisooxfrm(&mut self) -> IPXFRM_IISOOXFRM_W<21> {
        IPXFRM_IISOOXFRM_W::new(self)
    }
    ///Bit 23 - Reset detected interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn rstdetm(&mut self) -> RSTDETM_W<23> {
        RSTDETM_W::new(self)
    }
    ///Bit 25 - Host channels interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn hcim(&mut self) -> HCIM_W<25> {
        HCIM_W::new(self)
    }
    ///Bit 26 - Periodic TxFIFO empty mask
    #[inline(always)]
    #[must_use]
    pub fn ptxfem(&mut self) -> PTXFEM_W<26> {
        PTXFEM_W::new(self)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn lpmintm(&mut self) -> LPMINTM_W<27> {
        LPMINTM_W::new(self)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    #[must_use]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<28> {
        CIDSCHGM_W::new(self)
    }
    ///Bit 29 - Disconnect detected interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<29> {
        DISCINT_W::new(self)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn srqim(&mut self) -> SRQIM_W<30> {
        SRQIM_W::new(self)
    }
    ///Bit 31 - Resume/remote wakeup detected interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn wuim(&mut self) -> WUIM_W<31> {
        WUIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gintmsk](index.html) module
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [gintmsk::R](R) reader structure
impl crate::Readable for GINTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gintmsk::W](W) writer structure
impl crate::Writable for GINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GINTMSK to value 0
impl crate::Resettable for GINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
