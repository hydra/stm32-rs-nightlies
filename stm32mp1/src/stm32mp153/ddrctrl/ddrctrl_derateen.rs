///Register `DDRCTRL_DERATEEN` reader
pub struct R(crate::R<DDRCTRL_DERATEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DERATEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DERATEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DERATEEN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_DERATEEN` writer
pub struct W(crate::W<DDRCTRL_DERATEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DERATEEN_SPEC>;
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
impl From<crate::W<DDRCTRL_DERATEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DERATEEN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DERATE_ENABLE` reader - DERATE_ENABLE
pub type DERATE_ENABLE_R = crate::BitReader<bool>;
///Field `DERATE_ENABLE` writer - DERATE_ENABLE
pub type DERATE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DERATEEN_SPEC, bool, O>;
///Field `DERATE_VALUE` reader - DERATE_VALUE
pub type DERATE_VALUE_R = crate::FieldReader<u8, u8>;
///Field `DERATE_VALUE` writer - DERATE_VALUE
pub type DERATE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DERATEEN_SPEC, u8, u8, 2, O>;
///Field `DERATE_BYTE` reader - DERATE_BYTE
pub type DERATE_BYTE_R = crate::FieldReader<u8, u8>;
///Field `DERATE_BYTE` writer - DERATE_BYTE
pub type DERATE_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DERATEEN_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - DERATE_ENABLE
    #[inline(always)]
    pub fn derate_enable(&self) -> DERATE_ENABLE_R {
        DERATE_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DERATE_VALUE
    #[inline(always)]
    pub fn derate_value(&self) -> DERATE_VALUE_R {
        DERATE_VALUE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:7 - DERATE_BYTE
    #[inline(always)]
    pub fn derate_byte(&self) -> DERATE_BYTE_R {
        DERATE_BYTE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - DERATE_ENABLE
    #[inline(always)]
    #[must_use]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W<0> {
        DERATE_ENABLE_W::new(self)
    }
    ///Bits 1:2 - DERATE_VALUE
    #[inline(always)]
    #[must_use]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W<1> {
        DERATE_VALUE_W::new(self)
    }
    ///Bits 4:7 - DERATE_BYTE
    #[inline(always)]
    #[must_use]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W<4> {
        DERATE_BYTE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL temperature derate enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_derateen](index.html) module
pub struct DDRCTRL_DERATEEN_SPEC;
impl crate::RegisterSpec for DDRCTRL_DERATEEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_derateen::R](R) reader structure
impl crate::Readable for DDRCTRL_DERATEEN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_derateen::W](W) writer structure
impl crate::Writable for DDRCTRL_DERATEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_DERATEEN to value 0
impl crate::Resettable for DDRCTRL_DERATEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
