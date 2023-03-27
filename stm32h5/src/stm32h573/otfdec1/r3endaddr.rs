///Register `R3ENDADDR` reader
pub struct R(crate::R<R3ENDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R3ENDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R3ENDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R3ENDADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R3ENDADDR` writer
pub struct W(crate::W<R3ENDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R3ENDADDR_SPEC>;
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
impl From<crate::W<R3ENDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R3ENDADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_END_ADDR` reader - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones.
pub type REGX_END_ADDR_R = crate::FieldReader<u32, u32>;
///Field `REGx_END_ADDR` writer - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones.
pub type REGX_END_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R3ENDADDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones.
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones.
    #[inline(always)]
    #[must_use]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<0> {
        REGX_END_ADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region 3 end address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r3endaddr](index.html) module
pub struct R3ENDADDR_SPEC;
impl crate::RegisterSpec for R3ENDADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [r3endaddr::R](R) reader structure
impl crate::Readable for R3ENDADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r3endaddr::W](W) writer structure
impl crate::Writable for R3ENDADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R3ENDADDR to value 0x0fff
impl crate::Resettable for R3ENDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
