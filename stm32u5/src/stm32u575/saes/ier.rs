///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCFIE` reader - Computation complete flag interrupt enable
pub type CCFIE_R = crate::BitReader<bool>;
///Field `CCFIE` writer - Computation complete flag interrupt enable
pub type CCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `RWEIE` reader - Read or write error interrupt enable
pub type RWEIE_R = crate::BitReader<bool>;
///Field `RWEIE` writer - Read or write error interrupt enable
pub type RWEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `KEIE` reader - Key error interrupt enable
pub type KEIE_R = crate::BitReader<bool>;
///Field `KEIE` writer - Key error interrupt enable
pub type KEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `RNGEIE` reader - RNGEIE
pub type RNGEIE_R = crate::BitReader<bool>;
///Field `RNGEIE` writer - RNGEIE
pub type RNGEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Computation complete flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt enable
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt enable
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGEIE
    #[inline(always)]
    pub fn rngeie(&self) -> RNGEIE_R {
        RNGEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Computation complete flag interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<0> {
        CCFIE_W::new(self)
    }
    ///Bit 1 - Read or write error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rweie(&mut self) -> RWEIE_W<1> {
        RWEIE_W::new(self)
    }
    ///Bit 2 - Key error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KEIE_W<2> {
        KEIE_W::new(self)
    }
    ///Bit 3 - RNGEIE
    #[inline(always)]
    #[must_use]
    pub fn rngeie(&mut self) -> RNGEIE_W<3> {
        RNGEIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
