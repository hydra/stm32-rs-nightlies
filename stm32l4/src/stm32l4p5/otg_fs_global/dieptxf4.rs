///Register `DIEPTXF4` reader
pub struct R(crate::R<DIEPTXF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPTXF4` writer
pub struct W(crate::W<DIEPTXF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF4_SPEC>;
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
impl From<crate::W<DIEPTXF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INEPTXSA` reader - IN endpoint FIFO5 transmit RAM start address
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
///Field `INEPTXSA` writer - IN endpoint FIFO5 transmit RAM start address
pub type INEPTXSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF4_SPEC, u16, u16, 16, O>;
///Field `INEPTXFD` reader - IN endpoint TxFIFO depth
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
///Field `INEPTXFD` writer - IN endpoint TxFIFO depth
pub type INEPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF4_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - IN endpoint FIFO5 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFO5 transmit RAM start address
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
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
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptxf4](index.html) module
pub struct DIEPTXF4_SPEC;
impl crate::RegisterSpec for DIEPTXF4_SPEC {
    type Ux = u32;
}
///`read()` method returns [dieptxf4::R](R) reader structure
impl crate::Readable for DIEPTXF4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dieptxf4::W](W) writer structure
impl crate::Writable for DIEPTXF4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPTXF4 to value 0
impl crate::Resettable for DIEPTXF4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
