///Register `OR1` reader
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR1` writer
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OCREF_CLR` reader - Ocref_clr source selection This bit selects the ocref_clr input source.
pub type OCREF_CLR_R = crate::BitReader<bool>;
///Field `OCREF_CLR` writer - Ocref_clr source selection This bit selects the ocref_clr input source.
pub type OCREF_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source.
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source.
    #[inline(always)]
    #[must_use]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W<0> {
        OCREF_CLR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or1](index.html) module
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [or1::R](R) reader structure
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or1::W](W) writer structure
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
