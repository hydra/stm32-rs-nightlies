///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCF` writer - Computation complete flag clear
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `RWEIF` writer - Read or write error interrupt flag clear
pub type RWEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `KEIF` writer - Key error interrupt flag clear
pub type KEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Computation complete flag clear
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<0> {
        CCF_W::new(self)
    }
    ///Bit 1 - Read or write error interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn rweif(&mut self) -> RWEIF_W<1> {
        RWEIF_W::new(self)
    }
    ///Bit 2 - Key error interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn keif(&mut self) -> KEIF_W<2> {
        KEIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
