///Register `OPFCCR` reader
pub struct R(crate::R<OPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPFCCR` writer
pub struct W(crate::W<OPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPFCCR_SPEC>;
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
impl From<crate::W<OPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CM` reader - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
pub type CM_R = crate::FieldReader<u8, u8>;
///Field `CM` writer - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPFCCR_SPEC, u8, u8, 3, O>;
///Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
pub type AI_R = crate::BitReader<bool>;
///Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, bool, O>;
///Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
pub type RBS_R = crate::BitReader<bool>;
///Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
pub type RBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<20> {
        AI_W::new(self)
    }
    ///Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<21> {
        RBS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D output PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opfccr](index.html) module
pub struct OPFCCR_SPEC;
impl crate::RegisterSpec for OPFCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opfccr::R](R) reader structure
impl crate::Readable for OPFCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opfccr::W](W) writer structure
impl crate::Writable for OPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPFCCR to value 0
impl crate::Resettable for OPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
