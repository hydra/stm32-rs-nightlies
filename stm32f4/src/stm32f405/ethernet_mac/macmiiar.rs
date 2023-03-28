///Register `MACMIIAR` reader
pub struct R(crate::R<MACMIIAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACMIIAR` writer
pub struct W(crate::W<MACMIIAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIAR_SPEC>;
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
impl From<crate::W<MACMIIAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MB` reader - MB
pub type MB_R = crate::BitReader<bool>;
///Field `MB` writer - MB
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, bool, O>;
///Field `MW` reader - MW
pub type MW_R = crate::BitReader<bool>;
///Field `MW` writer - MW
pub type MW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, bool, O>;
///Field `CR` reader - CR
pub type CR_R = crate::FieldReader<u8, u8>;
///Field `CR` writer - CR
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 3, O>;
///Field `MR` reader - MR
pub type MR_R = crate::FieldReader<u8, u8>;
///Field `MR` writer - MR
pub type MR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
///Field `PA` reader - PA
pub type PA_R = crate::FieldReader<u8, u8>;
///Field `PA` writer - PA
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bit 0 - MB
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MW
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 6:10 - MR
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - PA
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - MB
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    ///Bit 1 - MW
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<1> {
        MW_W::new(self)
    }
    ///Bits 2:4 - CR
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<2> {
        CR_W::new(self)
    }
    ///Bits 6:10 - MR
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<6> {
        MR_W::new(self)
    }
    ///Bits 11:15 - PA
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<11> {
        PA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC MII address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macmiiar](index.html) module
pub struct MACMIIAR_SPEC;
impl crate::RegisterSpec for MACMIIAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macmiiar::R](R) reader structure
impl crate::Readable for MACMIIAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macmiiar::W](W) writer structure
impl crate::Writable for MACMIIAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACMIIAR to value 0
impl crate::Resettable for MACMIIAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
