///Register `FIFO` reader
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFO` writer
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FIF0Data` reader - FIF0Data
pub type FIF0DATA_R = crate::FieldReader<u32, u32>;
///Field `FIF0Data` writer - FIF0Data
pub type FIF0DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - FIF0Data
    #[inline(always)]
    pub fn fif0data(&self) -> FIF0DATA_R {
        FIF0DATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - FIF0Data
    #[inline(always)]
    #[must_use]
    pub fn fif0data(&mut self) -> FIF0DATA_W<0> {
        FIF0DATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data FIFO register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifo](index.html) module
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifo::R](R) reader structure
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifo::W](W) writer structure
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
