///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ENABLE` reader - Counter enable
pub type ENABLE_R = crate::BitReader<bool>;
///Field `ENABLE` writer - Counter enable
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `TICKINT` reader - SysTick exception request enable
pub type TICKINT_R = crate::BitReader<bool>;
///Field `TICKINT` writer - SysTick exception request enable
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `CLKSOURCE` reader - Clock source selection
pub type CLKSOURCE_R = crate::BitReader<bool>;
///Field `CLKSOURCE` writer - Clock source selection
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `COUNTFLAG` reader - COUNTFLAG
pub type COUNTFLAG_R = crate::BitReader<bool>;
///Field `COUNTFLAG` writer - COUNTFLAG
pub type COUNTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SysTick exception request enable
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clock source selection
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - COUNTFLAG
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counter enable
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    ///Bit 1 - SysTick exception request enable
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    ///Bit 2 - Clock source selection
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    ///Bit 16 - COUNTFLAG
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<16> {
        COUNTFLAG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SysTick control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
