///Register `TIM6_CR2` reader
pub struct R(crate::R<TIM6_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM6_CR2` writer
pub struct W(crate::W<TIM6_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CR2_SPEC>;
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
impl From<crate::W<TIM6_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCPC` reader - CCPC
pub type CCPC_R = crate::BitReader<bool>;
///Field `CCPC` writer - CCPC
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `CCUS` reader - CCUS
pub type CCUS_R = crate::BitReader<bool>;
///Field `CCUS` writer - CCUS
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `CCDS` reader - CCDS
pub type CCDS_R = crate::BitReader<bool>;
///Field `CCDS` writer - CCDS
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `MMS` reader - MMS
pub type MMS_R = crate::FieldReader<u8, u8>;
///Field `MMS` writer - MMS
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM6_CR2_SPEC, u8, u8, 3, O>;
///Field `TI1S` reader - TI1S
pub type TI1S_R = crate::BitReader<bool>;
///Field `TI1S` writer - TI1S
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS1` reader - OIS1
pub type OIS1_R = crate::BitReader<bool>;
///Field `OIS1` writer - OIS1
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS1N` reader - OIS1N
pub type OIS1N_R = crate::BitReader<bool>;
///Field `OIS1N` writer - OIS1N
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS2` reader - OIS2
pub type OIS2_R = crate::BitReader<bool>;
///Field `OIS2` writer - OIS2
pub type OIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS2N` reader - OIS2N
pub type OIS2N_R = crate::BitReader<bool>;
///Field `OIS2N` writer - OIS2N
pub type OIS2N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS3` reader - OIS3
pub type OIS3_R = crate::BitReader<bool>;
///Field `OIS3` writer - OIS3
pub type OIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS3N` reader - OIS3N
pub type OIS3N_R = crate::BitReader<bool>;
///Field `OIS3N` writer - OIS3N
pub type OIS3N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS4` reader - OIS4
pub type OIS4_R = crate::BitReader<bool>;
///Field `OIS4` writer - OIS4
pub type OIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS5` reader - OIS5
pub type OIS5_R = crate::BitReader<bool>;
///Field `OIS5` writer - OIS5
pub type OIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `OIS6` reader - OIS6
pub type OIS6_R = crate::BitReader<bool>;
///Field `OIS6` writer - OIS6
pub type OIS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CR2_SPEC, bool, O>;
///Field `MMS2` reader - MMS2
pub type MMS2_R = crate::FieldReader<u8, u8>;
///Field `MMS2` writer - MMS2
pub type MMS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM6_CR2_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - CCPC
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - CCUS
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CCDS
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OIS1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OIS1N
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - OIS2
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OIS2N
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - OIS3
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - OIS3N
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OIS4
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - OIS5
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - OIS6
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - MMS2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - CCPC
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    ///Bit 2 - CCUS
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    ///Bit 3 - CCDS
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    ///Bits 4:6 - MMS
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    ///Bit 8 - OIS1
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    ///Bit 9 - OIS1N
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    ///Bit 10 - OIS2
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<10> {
        OIS2_W::new(self)
    }
    ///Bit 11 - OIS2N
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<11> {
        OIS2N_W::new(self)
    }
    ///Bit 12 - OIS3
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<12> {
        OIS3_W::new(self)
    }
    ///Bit 13 - OIS3N
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<13> {
        OIS3N_W::new(self)
    }
    ///Bit 14 - OIS4
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<14> {
        OIS4_W::new(self)
    }
    ///Bit 16 - OIS5
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<16> {
        OIS5_W::new(self)
    }
    ///Bit 18 - OIS6
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<18> {
        OIS6_W::new(self)
    }
    ///Bits 20:23 - MMS2
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<20> {
        MMS2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM6 control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim6_cr2](index.html) module
pub struct TIM6_CR2_SPEC;
impl crate::RegisterSpec for TIM6_CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim6_cr2::R](R) reader structure
impl crate::Readable for TIM6_CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim6_cr2::W](W) writer structure
impl crate::Writable for TIM6_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM6_CR2 to value 0
impl crate::Resettable for TIM6_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
