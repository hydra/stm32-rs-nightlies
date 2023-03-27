///Register `DDRCTRL_DRAMTMG8` reader
pub struct R(crate::R<DDRCTRL_DRAMTMG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG8_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_DRAMTMG8` writer
pub struct W(crate::W<DDRCTRL_DRAMTMG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG8_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG8_SPEC>) -> Self {
        W(writer)
    }
}
///Field `T_XS_X32` reader - T_XS_X32
pub type T_XS_X32_R = crate::FieldReader<u8, u8>;
///Field `T_XS_X32` writer - T_XS_X32
pub type T_XS_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG8_SPEC, u8, u8, 7, O>;
///Field `T_XS_DLL_X32` reader - T_XS_DLL_X32
pub type T_XS_DLL_X32_R = crate::FieldReader<u8, u8>;
///Field `T_XS_DLL_X32` writer - T_XS_DLL_X32
pub type T_XS_DLL_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG8_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - T_XS_X32
    #[inline(always)]
    pub fn t_xs_x32(&self) -> T_XS_X32_R {
        T_XS_X32_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - T_XS_DLL_X32
    #[inline(always)]
    pub fn t_xs_dll_x32(&self) -> T_XS_DLL_X32_R {
        T_XS_DLL_X32_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - T_XS_X32
    #[inline(always)]
    #[must_use]
    pub fn t_xs_x32(&mut self) -> T_XS_X32_W<0> {
        T_XS_X32_W::new(self)
    }
    ///Bits 8:14 - T_XS_DLL_X32
    #[inline(always)]
    #[must_use]
    pub fn t_xs_dll_x32(&mut self) -> T_XS_DLL_X32_W<8> {
        T_XS_DLL_X32_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL SDRAM timing register 8
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_dramtmg8](index.html) module
pub struct DDRCTRL_DRAMTMG8_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG8_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_dramtmg8::R](R) reader structure
impl crate::Readable for DDRCTRL_DRAMTMG8_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_dramtmg8::W](W) writer structure
impl crate::Writable for DDRCTRL_DRAMTMG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_DRAMTMG8 to value 0x4405
impl crate::Resettable for DDRCTRL_DRAMTMG8_SPEC {
    const RESET_VALUE: Self::Ux = 0x4405;
}
