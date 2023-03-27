///Register `HCINT2` reader
pub struct R(crate::R<HCINT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HCINT2` writer
pub struct W(crate::W<HCINT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT2_SPEC>;
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
impl From<crate::W<HCINT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader<bool>;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `CHH` reader - CHH
pub type CHH_R = crate::BitReader<bool>;
///Field `CHH` writer - CHH
pub type CHH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `STALL` reader - STALL
pub type STALL_R = crate::BitReader<bool>;
///Field `STALL` writer - STALL
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader<bool>;
///Field `NAK` writer - NAK
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `ACK` reader - ACK
pub type ACK_R = crate::BitReader<bool>;
///Field `ACK` writer - ACK
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `TXERR` reader - TXERR
pub type TXERR_R = crate::BitReader<bool>;
///Field `TXERR` writer - TXERR
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `BBERR` reader - BBERR
pub type BBERR_R = crate::BitReader<bool>;
///Field `BBERR` writer - BBERR
pub type BBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `FRMOR` reader - FRMOR
pub type FRMOR_R = crate::BitReader<bool>;
///Field `FRMOR` writer - FRMOR
pub type FRMOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
///Field `DTERR` reader - DTERR
pub type DTERR_R = crate::BitReader<bool>;
///Field `DTERR` writer - DTERR
pub type DTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT2_SPEC, bool, O>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CHH
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STALL
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACK
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - TXERR
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BBERR
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FRMOR
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DTERR
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    ///Bit 1 - CHH
    #[inline(always)]
    #[must_use]
    pub fn chh(&mut self) -> CHH_W<1> {
        CHH_W::new(self)
    }
    ///Bit 3 - STALL
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    ///Bit 4 - NAK
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<4> {
        NAK_W::new(self)
    }
    ///Bit 5 - ACK
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<5> {
        ACK_W::new(self)
    }
    ///Bit 7 - TXERR
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<7> {
        TXERR_W::new(self)
    }
    ///Bit 8 - BBERR
    #[inline(always)]
    #[must_use]
    pub fn bberr(&mut self) -> BBERR_W<8> {
        BBERR_W::new(self)
    }
    ///Bit 9 - FRMOR
    #[inline(always)]
    #[must_use]
    pub fn frmor(&mut self) -> FRMOR_W<9> {
        FRMOR_W::new(self)
    }
    ///Bit 10 - DTERR
    #[inline(always)]
    #[must_use]
    pub fn dterr(&mut self) -> DTERR_W<10> {
        DTERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint2](index.html) module
pub struct HCINT2_SPEC;
impl crate::RegisterSpec for HCINT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hcint2::R](R) reader structure
impl crate::Readable for HCINT2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hcint2::W](W) writer structure
impl crate::Writable for HCINT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HCINT2 to value 0
impl crate::Resettable for HCINT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
