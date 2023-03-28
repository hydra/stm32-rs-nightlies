///Register `DMACRXDLAR` reader
pub struct R(crate::R<DMACRXDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRXDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRXDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRXDLAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACRXDLAR` writer
pub struct W(crate::W<DMACRXDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRXDLAR_SPEC>;
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
impl From<crate::W<DMACRXDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRXDLAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDESLA` reader - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type RDESLA_R = crate::FieldReader<u32, u32>;
///Field `RDESLA` writer - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type RDESLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRXDLAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    #[must_use]
    pub fn rdesla(&mut self) -> RDESLA_W<0> {
        RDESLA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Rx descriptor list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacrxdlar](index.html) module
pub struct DMACRXDLAR_SPEC;
impl crate::RegisterSpec for DMACRXDLAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacrxdlar::R](R) reader structure
impl crate::Readable for DMACRXDLAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacrxdlar::W](W) writer structure
impl crate::Writable for DMACRXDLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACRXDLAR to value 0
impl crate::Resettable for DMACRXDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
