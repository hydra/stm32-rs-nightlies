///Register `OAR2` reader
pub struct R(crate::R<OAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OAR2` writer
pub struct W(crate::W<OAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR2_SPEC>;
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
impl From<crate::W<OAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ENDUAL` reader - Dual addressing mode enable
pub type ENDUAL_R = crate::BitReader<ENDUAL_A>;
///Dual addressing mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDUAL_A {
    ///0: Single addressing mode
    Single = 0,
    ///1: Dual addressing mode
    Dual = 1,
}
impl From<ENDUAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENDUAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDUAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENDUAL_A {
        match self.bits {
            false => ENDUAL_A::Single,
            true => ENDUAL_A::Dual,
        }
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ENDUAL_A::Single
    }
    ///Checks if the value of the field is `Dual`
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == ENDUAL_A::Dual
    }
}
///Field `ENDUAL` writer - Dual addressing mode enable
pub type ENDUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR2_SPEC, ENDUAL_A, O>;
impl<'a, const O: u8> ENDUAL_W<'a, O> {
    ///Single addressing mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ENDUAL_A::Single)
    }
    ///Dual addressing mode
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(ENDUAL_A::Dual)
    }
}
///Field `ADD2` reader - Interface address
pub type ADD2_R = crate::FieldReader<u8, u8>;
///Field `ADD2` writer - Interface address
pub type ADD2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OAR2_SPEC, u8, u8, 7, O>;
impl R {
    ///Bit 0 - Dual addressing mode enable
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - Interface address
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    ///Bit 0 - Dual addressing mode enable
    #[inline(always)]
    #[must_use]
    pub fn endual(&mut self) -> ENDUAL_W<0> {
        ENDUAL_W::new(self)
    }
    ///Bits 1:7 - Interface address
    #[inline(always)]
    #[must_use]
    pub fn add2(&mut self) -> ADD2_W<1> {
        ADD2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OAR2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oar2](index.html) module
pub struct OAR2_SPEC;
impl crate::RegisterSpec for OAR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [oar2::R](R) reader structure
impl crate::Readable for OAR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [oar2::W](W) writer structure
impl crate::Writable for OAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OAR2 to value 0
impl crate::Resettable for OAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
