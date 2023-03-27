///Register `OTG_DAINTMSK` reader
pub struct R(crate::R<OTG_DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_DAINTMSK` writer
pub struct W(crate::W<OTG_DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DAINTMSK_SPEC>;
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
impl From<crate::W<OTG_DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IEPM` reader - IEPM
pub type IEPM_R = crate::FieldReader<u16, u16>;
///Field `IEPM` writer - IEPM
pub type IEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DAINTMSK_SPEC, u16, u16, 16, O>;
///Field `OEPM` reader - OEPM
pub type OEPM_R = crate::FieldReader<u16, u16>;
///Field `OEPM` writer - OEPM
pub type OEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - IEPM
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OEPM
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IEPM
    #[inline(always)]
    #[must_use]
    pub fn iepm(&mut self) -> IEPM_W<0> {
        IEPM_W::new(self)
    }
    ///Bits 16:31 - OEPM
    #[inline(always)]
    #[must_use]
    pub fn oepm(&mut self) -> OEPM_W<16> {
        OEPM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_daintmsk](index.html) module
pub struct OTG_DAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_DAINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_daintmsk::R](R) reader structure
impl crate::Readable for OTG_DAINTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_daintmsk::W](W) writer structure
impl crate::Writable for OTG_DAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_DAINTMSK to value 0
impl crate::Resettable for OTG_DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
