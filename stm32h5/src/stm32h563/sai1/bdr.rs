///Register `BDR` reader
pub struct R(crate::R<BDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDR` writer
pub struct W(crate::W<BDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDR_SPEC>;
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
impl From<crate::W<BDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA` reader - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
pub type DATA_R = crate::FieldReader<u32, u32>;
///Field `DATA` writer - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAI data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdr](index.html) module
pub struct BDR_SPEC;
impl crate::RegisterSpec for BDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdr::R](R) reader structure
impl crate::Readable for BDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdr::W](W) writer structure
impl crate::Writable for BDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDR to value 0
impl crate::Resettable for BDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
