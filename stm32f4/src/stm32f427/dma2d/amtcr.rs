///Register `AMTCR` reader
pub struct R(crate::R<AMTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AMTCR` writer
pub struct W(crate::W<AMTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMTCR_SPEC>;
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
impl From<crate::W<AMTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Enable Enables the dead time functionality.
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - Enable Enables the dead time functionality.
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMTCR_SPEC, bool, O>;
///Field `DT` reader - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `DT` writer - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMTCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Enable Enables the dead time functionality.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Enable Enables the dead time functionality.
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 8:15 - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<8> {
        DT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D AXI master timer configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [amtcr](index.html) module
pub struct AMTCR_SPEC;
impl crate::RegisterSpec for AMTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [amtcr::R](R) reader structure
impl crate::Readable for AMTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [amtcr::W](W) writer structure
impl crate::Writable for AMTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AMTCR to value 0
impl crate::Resettable for AMTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
