///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AWDCH` reader - Analog watchdog channel select bits
pub type AWDCH_R = crate::FieldReader<u8, u8>;
///Field `AWDCH` writer - Analog watchdog channel select bits
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
///Field `EOCIE` reader - Interrupt enable for EOC
pub type EOCIE_R = crate::BitReader<bool>;
///Field `EOCIE` writer - Interrupt enable for EOC
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<bool>;
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `JEOCIE` reader - Interrupt enable for injected channels
pub type JEOCIE_R = crate::BitReader<bool>;
///Field `JEOCIE` writer - Interrupt enable for injected channels
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `SCAN` reader - Scan mode
pub type SCAN_R = crate::BitReader<bool>;
///Field `SCAN` writer - Scan mode
pub type SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode
pub type AWDSGL_R = crate::BitReader<bool>;
///Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode
pub type AWDSGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `JAUTO` reader - Automatic injected group conversion
pub type JAUTO_R = crate::BitReader<bool>;
///Field `JAUTO` writer - Automatic injected group conversion
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `DISCEN` reader - Discontinuous mode on regular channels
pub type DISCEN_R = crate::BitReader<bool>;
///Field `DISCEN` writer - Discontinuous mode on regular channels
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `JDISCEN` reader - Discontinuous mode on injected channels
pub type JDISCEN_R = crate::BitReader<bool>;
///Field `JDISCEN` writer - Discontinuous mode on injected channels
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `DISCNUM` reader - Discontinuous mode channel count
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
///Field `DISCNUM` writer - Discontinuous mode channel count
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
///Field `PDD` reader - Power down during the delay phase
pub type PDD_R = crate::BitReader<bool>;
///Field `PDD` writer - Power down during the delay phase
pub type PDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `PDI` reader - Power down during the idle phase
pub type PDI_R = crate::BitReader<bool>;
///Field `PDI` writer - Power down during the idle phase
pub type PDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `JAWDEN` reader - Analog watchdog enable on injected channels
pub type JAWDEN_R = crate::BitReader<bool>;
///Field `JAWDEN` writer - Analog watchdog enable on injected channels
pub type JAWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `AWDEN` reader - Analog watchdog enable on regular channels
pub type AWDEN_R = crate::BitReader<bool>;
///Field `AWDEN` writer - Analog watchdog enable on regular channels
pub type AWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `RES` reader - Resolution
pub type RES_R = crate::FieldReader<u8, u8>;
///Field `RES` writer - Resolution
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
///Field `OVRIE` reader - Overrun interrupt enable
pub type OVRIE_R = crate::BitReader<bool>;
///Field `OVRIE` writer - Overrun interrupt enable
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - Power down during the delay phase
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Power down during the idle phase
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<0> {
        AWDCH_W::new(self)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<5> {
        EOCIE_W::new(self)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<6> {
        AWDIE_W::new(self)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<7> {
        JEOCIE_W::new(self)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<8> {
        SCAN_W::new(self)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<9> {
        AWDSGL_W::new(self)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<10> {
        JAUTO_W::new(self)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<11> {
        DISCEN_W::new(self)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<12> {
        JDISCEN_W::new(self)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<13> {
        DISCNUM_W::new(self)
    }
    ///Bit 16 - Power down during the delay phase
    #[inline(always)]
    #[must_use]
    pub fn pdd(&mut self) -> PDD_W<16> {
        PDD_W::new(self)
    }
    ///Bit 17 - Power down during the idle phase
    #[inline(always)]
    #[must_use]
    pub fn pdi(&mut self) -> PDI_W<17> {
        PDI_W::new(self)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    #[must_use]
    pub fn jawden(&mut self) -> JAWDEN_W<22> {
        JAWDEN_W::new(self)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<23> {
        AWDEN_W::new(self)
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<24> {
        RES_W::new(self)
    }
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<26> {
        OVRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
