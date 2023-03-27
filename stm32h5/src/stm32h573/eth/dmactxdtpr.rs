///Register `DMACTXDTPR` reader
pub struct R(crate::R<DMACTXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACTXDTPR` writer
pub struct W(crate::W<DMACTXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXDTPR_SPEC>;
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
impl From<crate::W<DMACTXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDT` reader - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
pub type TDT_R = crate::FieldReader<u32, u32>;
///Field `TDT` writer - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
pub type TDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACTXDTPR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<0> {
        TDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Tx descriptor tail pointer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmactxdtpr](index.html) module
pub struct DMACTXDTPR_SPEC;
impl crate::RegisterSpec for DMACTXDTPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmactxdtpr::R](R) reader structure
impl crate::Readable for DMACTXDTPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmactxdtpr::W](W) writer structure
impl crate::Writable for DMACTXDTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACTXDTPR to value 0
impl crate::Resettable for DMACTXDTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
