///Register `CCCSR` reader
pub struct R(crate::R<CCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCCSR` writer
pub struct W(crate::W<CCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCSR_SPEC>;
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
impl From<crate::W<CCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN1` reader - EN1
pub type EN1_R = crate::BitReader<bool>;
///Field `EN1` writer - EN1
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
///Field `CS1` reader - CS1
pub type CS1_R = crate::BitReader<bool>;
///Field `CS1` writer - CS1
pub type CS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
///Field `EN2` reader - EN2
pub type EN2_R = crate::BitReader<bool>;
///Field `EN2` writer - EN2
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
///Field `CS2` reader - CS2
pub type CS2_R = crate::BitReader<bool>;
///Field `CS2` writer - CS2
pub type CS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
///Field `RDY1` reader - RDY1
pub type RDY1_R = crate::BitReader<bool>;
///Field `RDY2` reader - RDY2
pub type RDY2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - EN1
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CS1
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EN2
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CS2
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - RDY1
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RDY2
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EN1
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    ///Bit 1 - CS1
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<1> {
        CS1_W::new(self)
    }
    ///Bit 2 - EN2
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<2> {
        EN2_W::new(self)
    }
    ///Bit 3 - CS2
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<3> {
        CS2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///compensation cell control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cccsr](index.html) module
pub struct CCCSR_SPEC;
impl crate::RegisterSpec for CCCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cccsr::R](R) reader structure
impl crate::Readable for CCCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cccsr::W](W) writer structure
impl crate::Writable for CCCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCCSR to value 0x0a
impl crate::Resettable for CCCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
