///Register `OAR1` reader
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OAR1` writer
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD` reader - Interface address
pub type ADD_R = crate::FieldReader<u16, u16>;
///Field `ADD` writer - Interface address
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OAR1_SPEC, u16, u16, 10, O>;
///Field `ADDMODE` reader - ADDMODE
pub type ADDMODE_R = crate::BitReader<ADDMODE_A>;
///ADDMODE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDMODE_A {
    ///0: 7-bit slave address
    Add7 = 0,
    ///1: 10-bit slave address
    Add10 = 1,
}
impl From<ADDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDMODE_A {
        match self.bits {
            false => ADDMODE_A::Add7,
            true => ADDMODE_A::Add10,
        }
    }
    ///Checks if the value of the field is `Add7`
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDMODE_A::Add7
    }
    ///Checks if the value of the field is `Add10`
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDMODE_A::Add10
    }
}
///Field `ADDMODE` writer - ADDMODE
pub type ADDMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, ADDMODE_A, O>;
impl<'a, const O: u8> ADDMODE_W<'a, O> {
    ///7-bit slave address
    #[inline(always)]
    pub fn add7(self) -> &'a mut W {
        self.variant(ADDMODE_A::Add7)
    }
    ///10-bit slave address
    #[inline(always)]
    pub fn add10(self) -> &'a mut W {
        self.variant(ADDMODE_A::Add10)
    }
}
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 15 - ADDMODE
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    ///Bit 15 - ADDMODE
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> ADDMODE_W<15> {
        ADDMODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OAR1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oar1](index.html) module
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [oar1::R](R) reader structure
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [oar1::W](W) writer structure
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OAR1 to value 0
impl crate::Resettable for OAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
