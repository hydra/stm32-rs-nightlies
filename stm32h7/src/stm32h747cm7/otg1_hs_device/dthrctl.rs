///Register `DTHRCTL` reader
pub struct R(crate::R<DTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTHRCTL` writer
pub struct W(crate::W<DTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTHRCTL_SPEC>;
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
impl From<crate::W<DTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NONISOTHREN` reader - Nonisochronous IN endpoints threshold enable
pub type NONISOTHREN_R = crate::BitReader<bool>;
///Field `NONISOTHREN` writer - Nonisochronous IN endpoints threshold enable
pub type NONISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, O>;
///Field `ISOTHREN` reader - ISO IN endpoint threshold enable
pub type ISOTHREN_R = crate::BitReader<bool>;
///Field `ISOTHREN` writer - ISO IN endpoint threshold enable
pub type ISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, O>;
///Field `TXTHRLEN` reader - Transmit threshold length
pub type TXTHRLEN_R = crate::FieldReader<u16, u16>;
///Field `TXTHRLEN` writer - Transmit threshold length
pub type TXTHRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTHRCTL_SPEC, u16, u16, 9, O>;
///Field `RXTHREN` reader - Receive threshold enable
pub type RXTHREN_R = crate::BitReader<bool>;
///Field `RXTHREN` writer - Receive threshold enable
pub type RXTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, O>;
///Field `RXTHRLEN` reader - Receive threshold length
pub type RXTHRLEN_R = crate::FieldReader<u16, u16>;
///Field `RXTHRLEN` writer - Receive threshold length
pub type RXTHRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTHRCTL_SPEC, u16, u16, 9, O>;
///Field `ARPEN` reader - Arbiter parking enable
pub type ARPEN_R = crate::BitReader<bool>;
///Field `ARPEN` writer - Arbiter parking enable
pub type ARPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, O>;
impl R {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<0> {
        NONISOTHREN_W::new(self)
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<1> {
        ISOTHREN_W::new(self)
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<2> {
        TXTHRLEN_W::new(self)
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<16> {
        RXTHREN_W::new(self)
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<17> {
        RXTHRLEN_W::new(self)
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<27> {
        ARPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS Device threshold control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dthrctl](index.html) module
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [dthrctl::R](R) reader structure
impl crate::Readable for DTHRCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dthrctl::W](W) writer structure
impl crate::Writable for DTHRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTHRCTL to value 0
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
