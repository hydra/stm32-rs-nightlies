///Register `R4STARTADDR` reader
pub struct R(crate::R<R4STARTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R4STARTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R4STARTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R4STARTADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R4STARTADDR` writer
pub struct W(crate::W<R4STARTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R4STARTADDR_SPEC>;
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
impl From<crate::W<R4STARTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R4STARTADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_START_ADDR` reader - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros .
pub type REGX_START_ADDR_R = crate::FieldReader<u32, u32>;
///Field `REGx_START_ADDR` writer - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros .
pub type REGX_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R4STARTADDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros .
    #[inline(always)]
    pub fn regx_start_addr(&self) -> REGX_START_ADDR_R {
        REGX_START_ADDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros .
    #[inline(always)]
    #[must_use]
    pub fn regx_start_addr(&mut self) -> REGX_START_ADDR_W<0> {
        REGX_START_ADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region 4 start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r4startaddr](index.html) module
pub struct R4STARTADDR_SPEC;
impl crate::RegisterSpec for R4STARTADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [r4startaddr::R](R) reader structure
impl crate::Readable for R4STARTADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r4startaddr::W](W) writer structure
impl crate::Writable for R4STARTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R4STARTADDR to value 0
impl crate::Resettable for R4STARTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
