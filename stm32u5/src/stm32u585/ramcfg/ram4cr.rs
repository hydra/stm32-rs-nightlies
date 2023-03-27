///Register `RAM4CR` reader
pub struct R(crate::R<RAM4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM4CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RAM4CR` writer
pub struct W(crate::W<RAM4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM4CR_SPEC>;
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
impl From<crate::W<RAM4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM4CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ECCE` reader - ECCE
pub type ECCE_R = crate::BitReader<bool>;
///Field `ECCE` writer - ECCE
pub type ECCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM4CR_SPEC, bool, O>;
///Field `ALE` reader - ALE
pub type ALE_R = crate::BitReader<bool>;
///Field `ALE` writer - ALE
pub type ALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM4CR_SPEC, bool, O>;
///Field `SRAMER` reader - SRAMER
pub type SRAMER_R = crate::BitReader<bool>;
///Field `SRAMER` writer - SRAMER
pub type SRAMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM4CR_SPEC, bool, O>;
///Field `WSC` reader - WSC
pub type WSC_R = crate::FieldReader<u8, u8>;
///Field `WSC` writer - WSC
pub type WSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM4CR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - ECCE
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ALE
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAMER
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:18 - WSC
    #[inline(always)]
    pub fn wsc(&self) -> WSC_R {
        WSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - ECCE
    #[inline(always)]
    #[must_use]
    pub fn ecce(&mut self) -> ECCE_W<0> {
        ECCE_W::new(self)
    }
    ///Bit 4 - ALE
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<4> {
        ALE_W::new(self)
    }
    ///Bit 8 - SRAMER
    #[inline(always)]
    #[must_use]
    pub fn sramer(&mut self) -> SRAMER_W<8> {
        SRAMER_W::new(self)
    }
    ///Bits 16:18 - WSC
    #[inline(always)]
    #[must_use]
    pub fn wsc(&mut self) -> WSC_W<16> {
        WSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG SRAM x control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram4cr](index.html) module
pub struct RAM4CR_SPEC;
impl crate::RegisterSpec for RAM4CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram4cr::R](R) reader structure
impl crate::Readable for RAM4CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ram4cr::W](W) writer structure
impl crate::Writable for RAM4CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM4CR to value 0
impl crate::Resettable for RAM4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
