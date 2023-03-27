///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `YEMPTY` reader - Y buffer empty flag
pub type YEMPTY_R = crate::BitReader<bool>;
///Field `YEMPTY` writer - Y buffer empty flag
pub type YEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `X1FULL` reader - X1 buffer full flag
pub type X1FULL_R = crate::BitReader<bool>;
///Field `X1FULL` writer - X1 buffer full flag
pub type X1FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OVFL` reader - Overflow error flag
pub type OVFL_R = crate::BitReader<bool>;
///Field `OVFL` writer - Overflow error flag
pub type OVFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `UNFL` reader - Underflow error flag
pub type UNFL_R = crate::BitReader<bool>;
///Field `UNFL` writer - Underflow error flag
pub type UNFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `SAT` reader - Saturation error flag
pub type SAT_R = crate::BitReader<bool>;
///Field `SAT` writer - Saturation error flag
pub type SAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Y buffer empty flag
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - X1 buffer full flag
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Overflow error flag
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Underflow error flag
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Saturation error flag
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Y buffer empty flag
    #[inline(always)]
    #[must_use]
    pub fn yempty(&mut self) -> YEMPTY_W<0> {
        YEMPTY_W::new(self)
    }
    ///Bit 1 - X1 buffer full flag
    #[inline(always)]
    #[must_use]
    pub fn x1full(&mut self) -> X1FULL_W<1> {
        X1FULL_W::new(self)
    }
    ///Bit 8 - Overflow error flag
    #[inline(always)]
    #[must_use]
    pub fn ovfl(&mut self) -> OVFL_W<8> {
        OVFL_W::new(self)
    }
    ///Bit 9 - Underflow error flag
    #[inline(always)]
    #[must_use]
    pub fn unfl(&mut self) -> UNFL_W<9> {
        UNFL_W::new(self)
    }
    ///Bit 10 - Saturation error flag
    #[inline(always)]
    #[must_use]
    pub fn sat(&mut self) -> SAT_W<10> {
        SAT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
