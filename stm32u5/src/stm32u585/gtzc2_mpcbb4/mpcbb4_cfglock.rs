///Register `MPCBB4_CFGLOCK` reader
pub struct R(crate::R<MPCBB4_CFGLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB4_CFGLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB4_CFGLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB4_CFGLOCK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCBB4_CFGLOCK` writer
pub struct W(crate::W<MPCBB4_CFGLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB4_CFGLOCK_SPEC>;
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
impl From<crate::W<MPCBB4_CFGLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB4_CFGLOCK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPLCK0` reader - Security/privilege configuration lock for super-block 0
pub type SPLCK0_R = crate::BitReader<bool>;
///Field `SPLCK0` writer - Security/privilege configuration lock for super-block 0
pub type SPLCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB4_CFGLOCK_SPEC, bool, O>;
impl R {
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK0_R {
        SPLCK0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    #[must_use]
    pub fn splck0(&mut self) -> SPLCK0_W<0> {
        SPLCK0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GTZC2 SRAM4 MPCBB configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcbb4_cfglock](index.html) module
pub struct MPCBB4_CFGLOCK_SPEC;
impl crate::RegisterSpec for MPCBB4_CFGLOCK_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcbb4_cfglock::R](R) reader structure
impl crate::Readable for MPCBB4_CFGLOCK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcbb4_cfglock::W](W) writer structure
impl crate::Writable for MPCBB4_CFGLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCBB4_CFGLOCK to value 0
impl crate::Resettable for MPCBB4_CFGLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
