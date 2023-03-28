///Register `CIER` reader
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CIER` writer
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_R = crate::BitReader<bool>;
///Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_R = crate::BitReader<bool>;
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
pub type HSIRDYIE_R = crate::BitReader<bool>;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_R = crate::BitReader<bool>;
///Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cier](index.html) module
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cier::R](R) reader structure
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cier::W](W) writer structure
impl crate::Writable for CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
