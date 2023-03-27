///Register `L1CR` reader
pub struct R(crate::R<L1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L1CR` writer
pub struct W(crate::W<L1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CR_SPEC>;
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
impl From<crate::W<L1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LEN` reader - Layer Enable
pub type LEN_R = crate::BitReader<bool>;
///Field `LEN` writer - Layer Enable
pub type LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1CR_SPEC, bool, O>;
///Field `COLKEN` reader - Color Keying Enable
pub type COLKEN_R = crate::BitReader<bool>;
///Field `COLKEN` writer - Color Keying Enable
pub type COLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1CR_SPEC, bool, O>;
///Field `CLUTEN` reader - Color Look-Up Table Enable
pub type CLUTEN_R = crate::BitReader<bool>;
///Field `CLUTEN` writer - Color Look-Up Table Enable
pub type CLUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<1> {
        COLKEN_W::new(self)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<4> {
        CLUTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cr](index.html) module
pub struct L1CR_SPEC;
impl crate::RegisterSpec for L1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l1cr::R](R) reader structure
impl crate::Readable for L1CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l1cr::W](W) writer structure
impl crate::Writable for L1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L1CR to value 0
impl crate::Resettable for L1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
