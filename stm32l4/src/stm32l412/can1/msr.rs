///Register `MSR` reader
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MSR` writer
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INAK` reader - INAK
pub type INAK_R = crate::BitReader<bool>;
///Field `SLAK` reader - SLAK
pub type SLAK_R = crate::BitReader<bool>;
///Field `ERRI` reader - ERRI
pub type ERRI_R = crate::BitReader<bool>;
///Field `ERRI` writer - ERRI
pub type ERRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSR_SPEC, bool, O>;
///Field `WKUI` reader - WKUI
pub type WKUI_R = crate::BitReader<bool>;
///Field `WKUI` writer - WKUI
pub type WKUI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSR_SPEC, bool, O>;
///Field `SLAKI` reader - SLAKI
pub type SLAKI_R = crate::BitReader<bool>;
///Field `SLAKI` writer - SLAKI
pub type SLAKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSR_SPEC, bool, O>;
///Field `TXM` reader - TXM
pub type TXM_R = crate::BitReader<bool>;
///Field `RXM` reader - RXM
pub type RXM_R = crate::BitReader<bool>;
///Field `SAMP` reader - SAMP
pub type SAMP_R = crate::BitReader<bool>;
///Field `RX` reader - RX
pub type RX_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - INAK
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SLAK
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRI
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SLAKI
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - TXM
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXM
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SAMP
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - ERRI
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ERRI_W<2> {
        ERRI_W::new(self)
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    #[must_use]
    pub fn wkui(&mut self) -> WKUI_W<3> {
        WKUI_W::new(self)
    }
    ///Bit 4 - SLAKI
    #[inline(always)]
    #[must_use]
    pub fn slaki(&mut self) -> SLAKI_W<4> {
        SLAKI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///master status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [msr](index.html) module
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [msr::R](R) reader structure
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [msr::W](W) writer structure
impl crate::Writable for MSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MSR to value 0x0c02
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c02;
}
