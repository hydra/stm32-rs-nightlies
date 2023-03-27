///Register `DMACTXDLAR` reader
pub struct R(crate::R<DMACTXDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXDLAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACTXDLAR` writer
pub struct W(crate::W<DMACTXDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXDLAR_SPEC>;
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
impl From<crate::W<DMACTXDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXDLAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type TDESLA_R = crate::FieldReader<u32, u32>;
///Field `TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type TDESLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACTXDLAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<0> {
        TDESLA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Tx descriptor list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmactxdlar](index.html) module
pub struct DMACTXDLAR_SPEC;
impl crate::RegisterSpec for DMACTXDLAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmactxdlar::R](R) reader structure
impl crate::Readable for DMACTXDLAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmactxdlar::W](W) writer structure
impl crate::Writable for DMACTXDLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACTXDLAR to value 0
impl crate::Resettable for DMACTXDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
