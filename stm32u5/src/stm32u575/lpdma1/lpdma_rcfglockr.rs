///Register `LPDMA_RCFGLOCKR` reader
pub struct R(crate::R<LPDMA_RCFGLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_RCFGLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_RCFGLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_RCFGLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_RCFGLOCKR` writer
pub struct W(crate::W<LPDMA_RCFGLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_RCFGLOCKR_SPEC>;
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
impl From<crate::W<LPDMA_RCFGLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_RCFGLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LOCK0` reader - LOCK0
pub type LOCK0_R = crate::BitReader<bool>;
///Field `LOCK0` writer - LOCK0
pub type LOCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_RCFGLOCKR_SPEC, bool, O>;
///Field `LOCK1` reader - LOCK1
pub type LOCK1_R = crate::BitReader<bool>;
///Field `LOCK1` writer - LOCK1
pub type LOCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_RCFGLOCKR_SPEC, bool, O>;
///Field `LOCK2` reader - LOCK2
pub type LOCK2_R = crate::BitReader<bool>;
///Field `LOCK2` writer - LOCK2
pub type LOCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_RCFGLOCKR_SPEC, bool, O>;
///Field `LOCK3` reader - LOCK3
pub type LOCK3_R = crate::BitReader<bool>;
///Field `LOCK3` writer - LOCK3
pub type LOCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_RCFGLOCKR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LOCK0
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LOCK0
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<0> {
        LOCK0_W::new(self)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<1> {
        LOCK1_W::new(self)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<2> {
        LOCK2_W::new(self)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<3> {
        LOCK3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_rcfglockr](index.html) module
pub struct LPDMA_RCFGLOCKR_SPEC;
impl crate::RegisterSpec for LPDMA_RCFGLOCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_rcfglockr::R](R) reader structure
impl crate::Readable for LPDMA_RCFGLOCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_rcfglockr::W](W) writer structure
impl crate::Writable for LPDMA_RCFGLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_RCFGLOCKR to value 0
impl crate::Resettable for LPDMA_RCFGLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
