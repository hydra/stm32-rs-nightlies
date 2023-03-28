///Register `CRYP_IV0RR` reader
pub struct R(crate::R<CRYP_IV0RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IV0RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IV0RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IV0RR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRYP_IV0RR` writer
pub struct W(crate::W<CRYP_IV0RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IV0RR_SPEC>;
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
impl From<crate::W<CRYP_IV0RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IV0RR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IV63` reader - IV63
pub type IV63_R = crate::BitReader<bool>;
///Field `IV63` writer - IV63
pub type IV63_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV62` reader - IV62
pub type IV62_R = crate::BitReader<bool>;
///Field `IV62` writer - IV62
pub type IV62_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV61` reader - IV61
pub type IV61_R = crate::BitReader<bool>;
///Field `IV61` writer - IV61
pub type IV61_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV60` reader - IV60
pub type IV60_R = crate::BitReader<bool>;
///Field `IV60` writer - IV60
pub type IV60_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV59` reader - IV59
pub type IV59_R = crate::BitReader<bool>;
///Field `IV59` writer - IV59
pub type IV59_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV58` reader - IV58
pub type IV58_R = crate::BitReader<bool>;
///Field `IV58` writer - IV58
pub type IV58_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV57` reader - IV57
pub type IV57_R = crate::BitReader<bool>;
///Field `IV57` writer - IV57
pub type IV57_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV56` reader - IV56
pub type IV56_R = crate::BitReader<bool>;
///Field `IV56` writer - IV56
pub type IV56_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV55` reader - IV55
pub type IV55_R = crate::BitReader<bool>;
///Field `IV55` writer - IV55
pub type IV55_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV54` reader - IV54
pub type IV54_R = crate::BitReader<bool>;
///Field `IV54` writer - IV54
pub type IV54_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV53` reader - IV53
pub type IV53_R = crate::BitReader<bool>;
///Field `IV53` writer - IV53
pub type IV53_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV52` reader - IV52
pub type IV52_R = crate::BitReader<bool>;
///Field `IV52` writer - IV52
pub type IV52_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV51` reader - IV51
pub type IV51_R = crate::BitReader<bool>;
///Field `IV51` writer - IV51
pub type IV51_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV50` reader - IV50
pub type IV50_R = crate::BitReader<bool>;
///Field `IV50` writer - IV50
pub type IV50_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV49` reader - IV49
pub type IV49_R = crate::BitReader<bool>;
///Field `IV49` writer - IV49
pub type IV49_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV48` reader - IV48
pub type IV48_R = crate::BitReader<bool>;
///Field `IV48` writer - IV48
pub type IV48_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV47` reader - IV47
pub type IV47_R = crate::BitReader<bool>;
///Field `IV47` writer - IV47
pub type IV47_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV46` reader - IV46
pub type IV46_R = crate::BitReader<bool>;
///Field `IV46` writer - IV46
pub type IV46_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV45` reader - IV45
pub type IV45_R = crate::BitReader<bool>;
///Field `IV45` writer - IV45
pub type IV45_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV44` reader - IV44
pub type IV44_R = crate::BitReader<bool>;
///Field `IV44` writer - IV44
pub type IV44_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV43` reader - IV43
pub type IV43_R = crate::BitReader<bool>;
///Field `IV43` writer - IV43
pub type IV43_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV42` reader - IV42
pub type IV42_R = crate::BitReader<bool>;
///Field `IV42` writer - IV42
pub type IV42_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV41` reader - IV41
pub type IV41_R = crate::BitReader<bool>;
///Field `IV41` writer - IV41
pub type IV41_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV40` reader - IV40
pub type IV40_R = crate::BitReader<bool>;
///Field `IV40` writer - IV40
pub type IV40_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV39` reader - IV39
pub type IV39_R = crate::BitReader<bool>;
///Field `IV39` writer - IV39
pub type IV39_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV38` reader - IV38
pub type IV38_R = crate::BitReader<bool>;
///Field `IV38` writer - IV38
pub type IV38_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV37` reader - IV37
pub type IV37_R = crate::BitReader<bool>;
///Field `IV37` writer - IV37
pub type IV37_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV36` reader - IV36
pub type IV36_R = crate::BitReader<bool>;
///Field `IV36` writer - IV36
pub type IV36_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV35` reader - IV35
pub type IV35_R = crate::BitReader<bool>;
///Field `IV35` writer - IV35
pub type IV35_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV34` reader - IV34
pub type IV34_R = crate::BitReader<bool>;
///Field `IV34` writer - IV34
pub type IV34_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV33` reader - IV33
pub type IV33_R = crate::BitReader<bool>;
///Field `IV33` writer - IV33
pub type IV33_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
///Field `IV32` reader - IV32
pub type IV32_R = crate::BitReader<bool>;
///Field `IV32` writer - IV32
pub type IV32_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0RR_SPEC, bool, O>;
impl R {
    ///Bit 0 - IV63
    #[inline(always)]
    pub fn iv63(&self) -> IV63_R {
        IV63_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IV62
    #[inline(always)]
    pub fn iv62(&self) -> IV62_R {
        IV62_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IV61
    #[inline(always)]
    pub fn iv61(&self) -> IV61_R {
        IV61_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IV60
    #[inline(always)]
    pub fn iv60(&self) -> IV60_R {
        IV60_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IV59
    #[inline(always)]
    pub fn iv59(&self) -> IV59_R {
        IV59_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IV58
    #[inline(always)]
    pub fn iv58(&self) -> IV58_R {
        IV58_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IV57
    #[inline(always)]
    pub fn iv57(&self) -> IV57_R {
        IV57_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IV56
    #[inline(always)]
    pub fn iv56(&self) -> IV56_R {
        IV56_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IV55
    #[inline(always)]
    pub fn iv55(&self) -> IV55_R {
        IV55_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IV54
    #[inline(always)]
    pub fn iv54(&self) -> IV54_R {
        IV54_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IV53
    #[inline(always)]
    pub fn iv53(&self) -> IV53_R {
        IV53_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - IV52
    #[inline(always)]
    pub fn iv52(&self) -> IV52_R {
        IV52_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IV51
    #[inline(always)]
    pub fn iv51(&self) -> IV51_R {
        IV51_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IV50
    #[inline(always)]
    pub fn iv50(&self) -> IV50_R {
        IV50_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IV49
    #[inline(always)]
    pub fn iv49(&self) -> IV49_R {
        IV49_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IV48
    #[inline(always)]
    pub fn iv48(&self) -> IV48_R {
        IV48_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IV47
    #[inline(always)]
    pub fn iv47(&self) -> IV47_R {
        IV47_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - IV46
    #[inline(always)]
    pub fn iv46(&self) -> IV46_R {
        IV46_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IV45
    #[inline(always)]
    pub fn iv45(&self) -> IV45_R {
        IV45_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - IV44
    #[inline(always)]
    pub fn iv44(&self) -> IV44_R {
        IV44_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IV43
    #[inline(always)]
    pub fn iv43(&self) -> IV43_R {
        IV43_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IV42
    #[inline(always)]
    pub fn iv42(&self) -> IV42_R {
        IV42_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - IV41
    #[inline(always)]
    pub fn iv41(&self) -> IV41_R {
        IV41_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - IV40
    #[inline(always)]
    pub fn iv40(&self) -> IV40_R {
        IV40_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - IV39
    #[inline(always)]
    pub fn iv39(&self) -> IV39_R {
        IV39_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - IV38
    #[inline(always)]
    pub fn iv38(&self) -> IV38_R {
        IV38_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - IV37
    #[inline(always)]
    pub fn iv37(&self) -> IV37_R {
        IV37_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IV36
    #[inline(always)]
    pub fn iv36(&self) -> IV36_R {
        IV36_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IV35
    #[inline(always)]
    pub fn iv35(&self) -> IV35_R {
        IV35_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - IV34
    #[inline(always)]
    pub fn iv34(&self) -> IV34_R {
        IV34_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - IV33
    #[inline(always)]
    pub fn iv33(&self) -> IV33_R {
        IV33_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - IV32
    #[inline(always)]
    pub fn iv32(&self) -> IV32_R {
        IV32_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IV63
    #[inline(always)]
    #[must_use]
    pub fn iv63(&mut self) -> IV63_W<0> {
        IV63_W::new(self)
    }
    ///Bit 1 - IV62
    #[inline(always)]
    #[must_use]
    pub fn iv62(&mut self) -> IV62_W<1> {
        IV62_W::new(self)
    }
    ///Bit 2 - IV61
    #[inline(always)]
    #[must_use]
    pub fn iv61(&mut self) -> IV61_W<2> {
        IV61_W::new(self)
    }
    ///Bit 3 - IV60
    #[inline(always)]
    #[must_use]
    pub fn iv60(&mut self) -> IV60_W<3> {
        IV60_W::new(self)
    }
    ///Bit 4 - IV59
    #[inline(always)]
    #[must_use]
    pub fn iv59(&mut self) -> IV59_W<4> {
        IV59_W::new(self)
    }
    ///Bit 5 - IV58
    #[inline(always)]
    #[must_use]
    pub fn iv58(&mut self) -> IV58_W<5> {
        IV58_W::new(self)
    }
    ///Bit 6 - IV57
    #[inline(always)]
    #[must_use]
    pub fn iv57(&mut self) -> IV57_W<6> {
        IV57_W::new(self)
    }
    ///Bit 7 - IV56
    #[inline(always)]
    #[must_use]
    pub fn iv56(&mut self) -> IV56_W<7> {
        IV56_W::new(self)
    }
    ///Bit 8 - IV55
    #[inline(always)]
    #[must_use]
    pub fn iv55(&mut self) -> IV55_W<8> {
        IV55_W::new(self)
    }
    ///Bit 9 - IV54
    #[inline(always)]
    #[must_use]
    pub fn iv54(&mut self) -> IV54_W<9> {
        IV54_W::new(self)
    }
    ///Bit 10 - IV53
    #[inline(always)]
    #[must_use]
    pub fn iv53(&mut self) -> IV53_W<10> {
        IV53_W::new(self)
    }
    ///Bit 11 - IV52
    #[inline(always)]
    #[must_use]
    pub fn iv52(&mut self) -> IV52_W<11> {
        IV52_W::new(self)
    }
    ///Bit 12 - IV51
    #[inline(always)]
    #[must_use]
    pub fn iv51(&mut self) -> IV51_W<12> {
        IV51_W::new(self)
    }
    ///Bit 13 - IV50
    #[inline(always)]
    #[must_use]
    pub fn iv50(&mut self) -> IV50_W<13> {
        IV50_W::new(self)
    }
    ///Bit 14 - IV49
    #[inline(always)]
    #[must_use]
    pub fn iv49(&mut self) -> IV49_W<14> {
        IV49_W::new(self)
    }
    ///Bit 15 - IV48
    #[inline(always)]
    #[must_use]
    pub fn iv48(&mut self) -> IV48_W<15> {
        IV48_W::new(self)
    }
    ///Bit 16 - IV47
    #[inline(always)]
    #[must_use]
    pub fn iv47(&mut self) -> IV47_W<16> {
        IV47_W::new(self)
    }
    ///Bit 17 - IV46
    #[inline(always)]
    #[must_use]
    pub fn iv46(&mut self) -> IV46_W<17> {
        IV46_W::new(self)
    }
    ///Bit 18 - IV45
    #[inline(always)]
    #[must_use]
    pub fn iv45(&mut self) -> IV45_W<18> {
        IV45_W::new(self)
    }
    ///Bit 19 - IV44
    #[inline(always)]
    #[must_use]
    pub fn iv44(&mut self) -> IV44_W<19> {
        IV44_W::new(self)
    }
    ///Bit 20 - IV43
    #[inline(always)]
    #[must_use]
    pub fn iv43(&mut self) -> IV43_W<20> {
        IV43_W::new(self)
    }
    ///Bit 21 - IV42
    #[inline(always)]
    #[must_use]
    pub fn iv42(&mut self) -> IV42_W<21> {
        IV42_W::new(self)
    }
    ///Bit 22 - IV41
    #[inline(always)]
    #[must_use]
    pub fn iv41(&mut self) -> IV41_W<22> {
        IV41_W::new(self)
    }
    ///Bit 23 - IV40
    #[inline(always)]
    #[must_use]
    pub fn iv40(&mut self) -> IV40_W<23> {
        IV40_W::new(self)
    }
    ///Bit 24 - IV39
    #[inline(always)]
    #[must_use]
    pub fn iv39(&mut self) -> IV39_W<24> {
        IV39_W::new(self)
    }
    ///Bit 25 - IV38
    #[inline(always)]
    #[must_use]
    pub fn iv38(&mut self) -> IV38_W<25> {
        IV38_W::new(self)
    }
    ///Bit 26 - IV37
    #[inline(always)]
    #[must_use]
    pub fn iv37(&mut self) -> IV37_W<26> {
        IV37_W::new(self)
    }
    ///Bit 27 - IV36
    #[inline(always)]
    #[must_use]
    pub fn iv36(&mut self) -> IV36_W<27> {
        IV36_W::new(self)
    }
    ///Bit 28 - IV35
    #[inline(always)]
    #[must_use]
    pub fn iv35(&mut self) -> IV35_W<28> {
        IV35_W::new(self)
    }
    ///Bit 29 - IV34
    #[inline(always)]
    #[must_use]
    pub fn iv34(&mut self) -> IV34_W<29> {
        IV34_W::new(self)
    }
    ///Bit 30 - IV33
    #[inline(always)]
    #[must_use]
    pub fn iv33(&mut self) -> IV33_W<30> {
        IV33_W::new(self)
    }
    ///Bit 31 - IV32
    #[inline(always)]
    #[must_use]
    pub fn iv32(&mut self) -> IV32_W<31> {
        IV32_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_iv0rr](index.html) module
pub struct CRYP_IV0RR_SPEC;
impl crate::RegisterSpec for CRYP_IV0RR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_iv0rr::R](R) reader structure
impl crate::Readable for CRYP_IV0RR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cryp_iv0rr::W](W) writer structure
impl crate::Writable for CRYP_IV0RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRYP_IV0RR to value 0
impl crate::Resettable for CRYP_IV0RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
