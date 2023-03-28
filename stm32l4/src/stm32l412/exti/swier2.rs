///Register `SWIER2` reader
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER2` writer
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWI35` reader - Software interrupt on line 35
pub type SWI35_R = crate::BitReader<SWI35W_A>;
///Software interrupt on line 35
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI35W_A {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SWI35W_A> for bool {
    #[inline(always)]
    fn from(variant: SWI35W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI35_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI35W_A> {
        match self.bits {
            true => Some(SWI35W_A::Pend),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pend`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI35W_A::Pend
    }
}
///Field `SWI35` writer - Software interrupt on line 35
pub type SWI35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWI35W_A, O>;
impl<'a, const O: u8> SWI35_W<'a, O> {
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35W_A::Pend)
    }
}
///Field `SWI36` reader - Software interrupt on line 36
pub use SWI35_R as SWI36_R;
///Field `SWI37` reader - Software interrupt on line 37
pub use SWI35_R as SWI37_R;
///Field `SWI38` reader - Software interrupt on line 38
pub use SWI35_R as SWI38_R;
///Field `SWI36` writer - Software interrupt on line 36
pub use SWI35_W as SWI36_W;
///Field `SWI37` writer - Software interrupt on line 37
pub use SWI35_W as SWI37_W;
///Field `SWI38` writer - Software interrupt on line 38
pub use SWI35_W as SWI38_W;
impl R {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    #[must_use]
    pub fn swi35(&mut self) -> SWI35_W<3> {
        SWI35_W::new(self)
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    #[must_use]
    pub fn swi36(&mut self) -> SWI36_W<4> {
        SWI36_W::new(self)
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    #[must_use]
    pub fn swi37(&mut self) -> SWI37_W<5> {
        SWI37_W::new(self)
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    #[must_use]
    pub fn swi38(&mut self) -> SWI38_W<6> {
        SWI38_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier2](index.html) module
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier2::R](R) reader structure
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier2::W](W) writer structure
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
