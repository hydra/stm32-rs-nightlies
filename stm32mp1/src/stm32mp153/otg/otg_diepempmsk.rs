///Register `OTG_DIEPEMPMSK` reader
pub struct R(crate::R<OTG_DIEPEMPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DIEPEMPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DIEPEMPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DIEPEMPMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_DIEPEMPMSK` writer
pub struct W(crate::W<OTG_DIEPEMPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DIEPEMPMSK_SPEC>;
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
impl From<crate::W<OTG_DIEPEMPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DIEPEMPMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INEPTXFEM` reader - INEPTXFEM
pub type INEPTXFEM_R = crate::FieldReader<u16, u16>;
///Field `INEPTXFEM` writer - INEPTXFEM
pub type INEPTXFEM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DIEPEMPMSK_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - INEPTXFEM
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - INEPTXFEM
    #[inline(always)]
    #[must_use]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W<0> {
        INEPTXFEM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_diepempmsk](index.html) module
pub struct OTG_DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for OTG_DIEPEMPMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_diepempmsk::R](R) reader structure
impl crate::Readable for OTG_DIEPEMPMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_diepempmsk::W](W) writer structure
impl crate::Writable for OTG_DIEPEMPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_DIEPEMPMSK to value 0
impl crate::Resettable for OTG_DIEPEMPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
