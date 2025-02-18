///Register `TXD` reader
pub struct R(crate::R<TXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXD` writer
pub struct W(crate::W<TXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXD_SPEC>;
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
impl From<crate::W<TXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXD` reader - Tx Data register
pub type TXD_R = crate::FieldReader<u8, u8>;
///Field `TXD` writer - Tx Data register
pub type TXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXD_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<0> {
        TXD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CEC Tx data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txd](index.html) module
pub struct TXD_SPEC;
impl crate::RegisterSpec for TXD_SPEC {
    type Ux = u32;
}
///`read()` method returns [txd::R](R) reader structure
impl crate::Readable for TXD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txd::W](W) writer structure
impl crate::Writable for TXD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TXD to value 0
impl crate::Resettable for TXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
