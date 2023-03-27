///Register `ODISR` writer
pub struct W(crate::W<ODISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODISR_SPEC>;
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
impl From<crate::W<ODISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TA1ODIS` writer - TA1ODIS
pub type TA1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TA2ODIS` writer - TA2ODIS
pub type TA2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TB1ODIS` writer - TB1ODIS
pub type TB1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TB2ODIS` writer - TB2ODIS
pub type TB2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TC1ODIS` writer - TC1ODIS
pub type TC1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TC2ODIS` writer - TC2ODIS
pub type TC2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TD1ODIS` writer - TD1ODIS
pub type TD1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TD2ODIS` writer - TD2ODIS
pub type TD2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TE1ODIS` writer - TE1ODIS
pub type TE1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TE2ODIS` writer - TE2ODIS
pub type TE2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TF1ODIS` writer - TF1ODIS
pub type TF1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
///Field `TF2ODIS` writer - TF2ODIS
pub type TF2ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, bool, O>;
impl W {
    ///Bit 0 - TA1ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<0> {
        TA1ODIS_W::new(self)
    }
    ///Bit 1 - TA2ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<1> {
        TA2ODIS_W::new(self)
    }
    ///Bit 2 - TB1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<2> {
        TB1ODIS_W::new(self)
    }
    ///Bit 3 - TB2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<3> {
        TB2ODIS_W::new(self)
    }
    ///Bit 4 - TC1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<4> {
        TC1ODIS_W::new(self)
    }
    ///Bit 5 - TC2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<5> {
        TC2ODIS_W::new(self)
    }
    ///Bit 6 - TD1ODIS
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<6> {
        TD1ODIS_W::new(self)
    }
    ///Bit 7 - TD2ODIS
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<7> {
        TD2ODIS_W::new(self)
    }
    ///Bit 8 - TE1ODIS
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<8> {
        TE1ODIS_W::new(self)
    }
    ///Bit 9 - TE2ODIS
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<9> {
        TE2ODIS_W::new(self)
    }
    ///Bit 10 - TF1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tf1odis(&mut self) -> TF1ODIS_W<10> {
        TF1ODIS_W::new(self)
    }
    ///Bit 11 - TF2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tf2odis(&mut self) -> TF2ODIS_W<11> {
        TF2ODIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ODISR
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odisr](index.html) module
pub struct ODISR_SPEC;
impl crate::RegisterSpec for ODISR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [odisr::W](W) writer structure
impl crate::Writable for ODISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ODISR to value 0
impl crate::Resettable for ODISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
