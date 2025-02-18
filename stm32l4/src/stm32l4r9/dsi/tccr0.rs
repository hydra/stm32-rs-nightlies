///Register `TCCR0` reader
pub struct R(crate::R<TCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TCCR0` writer
pub struct W(crate::W<TCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0_SPEC>;
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
impl From<crate::W<TCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPRX_TOCNT` reader - Low-power Reception Timeout Counter
pub type LPRX_TOCNT_R = crate::FieldReader<u16, u16>;
///Field `LPRX_TOCNT` writer - Low-power Reception Timeout Counter
pub type LPRX_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR0_SPEC, u16, u16, 16, O>;
///Field `HSTX_TOCNT` reader - High-Speed Transmission Timeout Counter
pub type HSTX_TOCNT_R = crate::FieldReader<u16, u16>;
///Field `HSTX_TOCNT` writer - High-Speed Transmission Timeout Counter
pub type HSTX_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR0_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    #[must_use]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<0> {
        LPRX_TOCNT_W::new(self)
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    #[must_use]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<16> {
        HSTX_TOCNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Timeout Counter Configuration Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tccr0](index.html) module
pub struct TCCR0_SPEC;
impl crate::RegisterSpec for TCCR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [tccr0::R](R) reader structure
impl crate::Readable for TCCR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tccr0::W](W) writer structure
impl crate::Writable for TCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TCCR0 to value 0
impl crate::Resettable for TCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
