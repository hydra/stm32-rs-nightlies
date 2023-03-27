///Register `WWDG_CFR` reader
pub struct R(crate::R<WWDG_CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WWDG_CFR` writer
pub struct W(crate::W<WWDG_CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CFR_SPEC>;
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
impl From<crate::W<WWDG_CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `W` reader - W
pub type W_R = crate::FieldReader<u8, u8>;
///Field `W` writer - W
pub type W_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WWDG_CFR_SPEC, u8, u8, 7, O>;
///Field `EWI` reader - EWI
pub type EWI_R = crate::BitReader<bool>;
///Field `EWI` writer - EWI
pub type EWI_W<'a, const O: u8> = crate::BitWriter<'a, u32, WWDG_CFR_SPEC, bool, O>;
///Field `WDGTB` reader - WDGTB
pub type WDGTB_R = crate::FieldReader<u8, u8>;
///Field `WDGTB` writer - WDGTB
pub type WDGTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WWDG_CFR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:6 - W
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - EWI
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - WDGTB
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    ///Bits 0:6 - W
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<0> {
        W_W::new(self)
    }
    ///Bit 9 - EWI
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<9> {
        EWI_W::new(self)
    }
    ///Bits 11:13 - WDGTB
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<11> {
        WDGTB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wwdg_cfr](index.html) module
pub struct WWDG_CFR_SPEC;
impl crate::RegisterSpec for WWDG_CFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wwdg_cfr::R](R) reader structure
impl crate::Readable for WWDG_CFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wwdg_cfr::W](W) writer structure
impl crate::Writable for WWDG_CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WWDG_CFR to value 0x7f
impl crate::Resettable for WWDG_CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
