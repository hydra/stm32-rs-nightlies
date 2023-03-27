///Register `OTG_DIEPTXF4` reader
pub struct R(crate::R<OTG_DIEPTXF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DIEPTXF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DIEPTXF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DIEPTXF4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_DIEPTXF4` writer
pub struct W(crate::W<OTG_DIEPTXF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DIEPTXF4_SPEC>;
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
impl From<crate::W<OTG_DIEPTXF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DIEPTXF4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INEPTXSA` reader - INEPTXSA
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
///Field `INEPTXSA` writer - INEPTXSA
pub type INEPTXSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DIEPTXF4_SPEC, u16, u16, 16, O>;
///Field `INEPTXFD` reader - INEPTXFD
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
///Field `INEPTXFD` writer - INEPTXFD
pub type INEPTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DIEPTXF4_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<16> {
        INEPTXFD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG device IN endpoint transmit FIFO 4 size register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_dieptxf4](index.html) module
pub struct OTG_DIEPTXF4_SPEC;
impl crate::RegisterSpec for OTG_DIEPTXF4_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_dieptxf4::R](R) reader structure
impl crate::Readable for OTG_DIEPTXF4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_dieptxf4::W](W) writer structure
impl crate::Writable for OTG_DIEPTXF4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_DIEPTXF4 to value 0x0200_0400
impl crate::Resettable for OTG_DIEPTXF4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
