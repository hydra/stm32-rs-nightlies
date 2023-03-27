///Register `FIFOR11` reader
pub struct R(crate::R<FIFOR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOR11_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFOR11` writer
pub struct W(crate::W<FIFOR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOR11_SPEC>;
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
impl From<crate::W<FIFOR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOR11_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FIFODATA` reader - Receive and transmit FIFO data
pub type FIFODATA_R = crate::FieldReader<u32, u32>;
///Field `FIFODATA` writer - Receive and transmit FIFO data
pub type FIFODATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOR11_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<0> {
        FIFODATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data FIFO register 11
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifor11](index.html) module
pub struct FIFOR11_SPEC;
impl crate::RegisterSpec for FIFOR11_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifor11::R](R) reader structure
impl crate::Readable for FIFOR11_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifor11::W](W) writer structure
impl crate::Writable for FIFOR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FIFOR11 to value 0
impl crate::Resettable for FIFOR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
