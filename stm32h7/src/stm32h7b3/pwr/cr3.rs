///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BYPASS` reader - Power management unit bypass
pub type BYPASS_R = crate::BitReader<bool>;
///Field `BYPASS` writer - Power management unit bypass
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `LDOEN` reader - Low drop-out regulator enable
pub type LDOEN_R = crate::BitReader<bool>;
///Field `LDOEN` writer - Low drop-out regulator enable
pub type LDOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SMPSEN` reader - SMPSEN
pub type SMPSEN_R = crate::BitReader<bool>;
///Field `SMPSEN` writer - SMPSEN
pub type SMPSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SMPSEXTHP` reader - SMPSEXTHP
pub type SMPSEXTHP_R = crate::BitReader<bool>;
///Field `SMPSEXTHP` writer - SMPSEXTHP
pub type SMPSEXTHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SMPSLEVEL` reader - SMPSLEVEL
pub type SMPSLEVEL_R = crate::FieldReader<u8, u8>;
///Field `SMPSLEVEL` writer - SMPSLEVEL
pub type SMPSLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 2, O>;
///Field `VBE` reader - VBAT charging enable
pub type VBE_R = crate::BitReader<bool>;
///Field `VBE` writer - VBAT charging enable
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `VBRS` reader - VBAT charging resistor selection
pub type VBRS_R = crate::BitReader<bool>;
///Field `VBRS` writer - VBAT charging resistor selection
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SMPSEXTRDY` reader - SMPSEXTRDY
pub type SMPSEXTRDY_R = crate::BitReader<bool>;
///Field `USB33DEN` reader - VDD33USB voltage level detector enable.
pub type USB33DEN_R = crate::BitReader<bool>;
///Field `USB33DEN` writer - VDD33USB voltage level detector enable.
pub type USB33DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `USBREGEN` reader - USB regulator enable.
pub type USBREGEN_R = crate::BitReader<bool>;
///Field `USBREGEN` writer - USB regulator enable.
pub type USBREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `USB33RDY` reader - USB supply ready.
pub type USB33RDY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Power management unit bypass
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low drop-out regulator enable
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SMPSEN
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SMPSEXTHP
    #[inline(always)]
    pub fn smpsexthp(&self) -> SMPSEXTHP_R {
        SMPSEXTHP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SMPSLEVEL
    #[inline(always)]
    pub fn smpslevel(&self) -> SMPSLEVEL_R {
        SMPSLEVEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - SMPSEXTRDY
    #[inline(always)]
    pub fn smpsextrdy(&self) -> SMPSEXTRDY_R {
        SMPSEXTRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - VDD33USB voltage level detector enable.
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB supply ready.
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power management unit bypass
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    ///Bit 1 - Low drop-out regulator enable
    #[inline(always)]
    #[must_use]
    pub fn ldoen(&mut self) -> LDOEN_W<1> {
        LDOEN_W::new(self)
    }
    ///Bit 2 - SMPSEN
    #[inline(always)]
    #[must_use]
    pub fn smpsen(&mut self) -> SMPSEN_W<2> {
        SMPSEN_W::new(self)
    }
    ///Bit 3 - SMPSEXTHP
    #[inline(always)]
    #[must_use]
    pub fn smpsexthp(&mut self) -> SMPSEXTHP_W<3> {
        SMPSEXTHP_W::new(self)
    }
    ///Bits 4:5 - SMPSLEVEL
    #[inline(always)]
    #[must_use]
    pub fn smpslevel(&mut self) -> SMPSLEVEL_W<4> {
        SMPSLEVEL_W::new(self)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    ///Bit 24 - VDD33USB voltage level detector enable.
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<24> {
        USB33DEN_W::new(self)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    #[must_use]
    pub fn usbregen(&mut self) -> USBREGEN_W<25> {
        USBREGEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR3 to value 0x06
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
