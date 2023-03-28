///Register `DIEPTXF0` reader
pub struct R(crate::R<DIEPTXF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPTXF0` writer
pub struct W(crate::W<DIEPTXF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF0_SPEC>;
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
impl From<crate::W<DIEPTXF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TX0FSA` reader - Endpoint 0 transmit RAM start address
pub type TX0FSA_R = crate::FieldReader<u16, u16>;
///Field `TX0FSA` writer - Endpoint 0 transmit RAM start address
pub type TX0FSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF0_SPEC, u16, u16, 16, O>;
///Field `TX0FD` reader - Endpoint 0 TxFIFO depth
pub type TX0FD_R = crate::FieldReader<u16, u16>;
///Field `TX0FD` writer - Endpoint 0 TxFIFO depth
pub type TX0FD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF0_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<0> {
        TX0FSA_W::new(self)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<16> {
        TX0FD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Endpoint 0 transmit FIFO size (peripheral mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptxf0](index.html) module
pub struct DIEPTXF0_SPEC;
impl crate::RegisterSpec for DIEPTXF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [dieptxf0::R](R) reader structure
impl crate::Readable for DIEPTXF0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dieptxf0::W](W) writer structure
impl crate::Writable for DIEPTXF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPTXF0 to value 0x0200
impl crate::Resettable for DIEPTXF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
