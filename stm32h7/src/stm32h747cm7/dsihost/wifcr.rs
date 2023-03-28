///Register `WIFCR` reader
pub struct R(crate::R<WIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WIFCR` writer
pub struct W(crate::W<WIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFCR_SPEC>;
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
impl From<crate::W<WIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTEIF` reader - Clear tearing effect interrupt flag
pub type CTEIF_R = crate::BitReader<bool>;
///Field `CTEIF` writer - Clear tearing effect interrupt flag
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CERIF` reader - Clear end of refresh interrupt flag
pub type CERIF_R = crate::BitReader<bool>;
///Field `CERIF` writer - Clear end of refresh interrupt flag
pub type CERIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CPLLLIF` reader - Clear PLL lock interrupt flag
pub type CPLLLIF_R = crate::BitReader<bool>;
///Field `CPLLLIF` writer - Clear PLL lock interrupt flag
pub type CPLLLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CPLLUIF` reader - Clear PLL unlock interrupt flag
pub type CPLLUIF_R = crate::BitReader<bool>;
///Field `CPLLUIF` writer - Clear PLL unlock interrupt flag
pub type CPLLUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CRRIF` reader - Clear regulator ready interrupt flag
pub type CRRIF_R = crate::BitReader<bool>;
///Field `CRRIF` writer - Clear regulator ready interrupt flag
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Clear tearing effect interrupt flag
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear end of refresh interrupt flag
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Clear PLL lock interrupt flag
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clear PLL unlock interrupt flag
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Clear regulator ready interrupt flag
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear tearing effect interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    ///Bit 1 - Clear end of refresh interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<1> {
        CERIF_W::new(self)
    }
    ///Bit 9 - Clear PLL lock interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<9> {
        CPLLLIF_W::new(self)
    }
    ///Bit 10 - Clear PLL unlock interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<10> {
        CPLLUIF_W::new(self)
    }
    ///Bit 13 - Clear regulator ready interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<13> {
        CRRIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI wrapper interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wifcr](index.html) module
pub struct WIFCR_SPEC;
impl crate::RegisterSpec for WIFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wifcr::R](R) reader structure
impl crate::Readable for WIFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wifcr::W](W) writer structure
impl crate::Writable for WIFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WIFCR to value 0
impl crate::Resettable for WIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
