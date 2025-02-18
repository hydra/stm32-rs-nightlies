///Register `FDCAN_ILE` reader
pub struct R(crate::R<FDCAN_ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ILE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_ILE` writer
pub struct W(crate::W<FDCAN_ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_ILE_SPEC>;
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
impl From<crate::W<FDCAN_ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_ILE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EINT0` reader - EINT0
pub type EINT0_R = crate::BitReader<bool>;
///Field `EINT0` writer - EINT0
pub type EINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILE_SPEC, bool, O>;
///Field `EINT1` reader - EINT1
pub type EINT1_R = crate::BitReader<bool>;
///Field `EINT1` writer - EINT1
pub type EINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILE_SPEC, bool, O>;
impl R {
    ///Bit 0 - EINT0
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EINT1
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EINT0
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<0> {
        EINT0_W::new(self)
    }
    ///Bit 1 - EINT1
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<1> {
        EINT1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ile](index.html) module
pub struct FDCAN_ILE_SPEC;
impl crate::RegisterSpec for FDCAN_ILE_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ile::R](R) reader structure
impl crate::Readable for FDCAN_ILE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_ile::W](W) writer structure
impl crate::Writable for FDCAN_ILE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_ILE to value 0
impl crate::Resettable for FDCAN_ILE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
