///Register `MPCBB2_VCTR6` reader
pub struct R(crate::R<MPCBB2_VCTR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCBB2_VCTR6` writer
pub struct W(crate::W<MPCBB2_VCTR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR6_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `B192` reader - B192
pub type B192_R = crate::BitReader<bool>;
///Field `B192` writer - B192
pub type B192_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B193` reader - B193
pub type B193_R = crate::BitReader<bool>;
///Field `B193` writer - B193
pub type B193_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B194` reader - B194
pub type B194_R = crate::BitReader<bool>;
///Field `B194` writer - B194
pub type B194_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B195` reader - B195
pub type B195_R = crate::BitReader<bool>;
///Field `B195` writer - B195
pub type B195_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B196` reader - B196
pub type B196_R = crate::BitReader<bool>;
///Field `B196` writer - B196
pub type B196_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B197` reader - B197
pub type B197_R = crate::BitReader<bool>;
///Field `B197` writer - B197
pub type B197_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B198` reader - B198
pub type B198_R = crate::BitReader<bool>;
///Field `B198` writer - B198
pub type B198_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B199` reader - B199
pub type B199_R = crate::BitReader<bool>;
///Field `B199` writer - B199
pub type B199_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B200` reader - B200
pub type B200_R = crate::BitReader<bool>;
///Field `B200` writer - B200
pub type B200_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B201` reader - B201
pub type B201_R = crate::BitReader<bool>;
///Field `B201` writer - B201
pub type B201_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B202` reader - B202
pub type B202_R = crate::BitReader<bool>;
///Field `B202` writer - B202
pub type B202_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B203` reader - B203
pub type B203_R = crate::BitReader<bool>;
///Field `B203` writer - B203
pub type B203_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B204` reader - B204
pub type B204_R = crate::BitReader<bool>;
///Field `B204` writer - B204
pub type B204_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B205` reader - B205
pub type B205_R = crate::BitReader<bool>;
///Field `B205` writer - B205
pub type B205_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B206` reader - B206
pub type B206_R = crate::BitReader<bool>;
///Field `B206` writer - B206
pub type B206_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B207` reader - B207
pub type B207_R = crate::BitReader<bool>;
///Field `B207` writer - B207
pub type B207_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B208` reader - B208
pub type B208_R = crate::BitReader<bool>;
///Field `B208` writer - B208
pub type B208_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B209` reader - B209
pub type B209_R = crate::BitReader<bool>;
///Field `B209` writer - B209
pub type B209_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B210` reader - B210
pub type B210_R = crate::BitReader<bool>;
///Field `B210` writer - B210
pub type B210_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B211` reader - B211
pub type B211_R = crate::BitReader<bool>;
///Field `B211` writer - B211
pub type B211_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B212` reader - B212
pub type B212_R = crate::BitReader<bool>;
///Field `B212` writer - B212
pub type B212_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B213` reader - B213
pub type B213_R = crate::BitReader<bool>;
///Field `B213` writer - B213
pub type B213_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B214` reader - B214
pub type B214_R = crate::BitReader<bool>;
///Field `B214` writer - B214
pub type B214_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B215` reader - B215
pub type B215_R = crate::BitReader<bool>;
///Field `B215` writer - B215
pub type B215_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B216` reader - B216
pub type B216_R = crate::BitReader<bool>;
///Field `B216` writer - B216
pub type B216_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B217` reader - B217
pub type B217_R = crate::BitReader<bool>;
///Field `B217` writer - B217
pub type B217_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B218` reader - B218
pub type B218_R = crate::BitReader<bool>;
///Field `B218` writer - B218
pub type B218_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B219` reader - B219
pub type B219_R = crate::BitReader<bool>;
///Field `B219` writer - B219
pub type B219_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B220` reader - B220
pub type B220_R = crate::BitReader<bool>;
///Field `B220` writer - B220
pub type B220_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B221` reader - B221
pub type B221_R = crate::BitReader<bool>;
///Field `B221` writer - B221
pub type B221_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B222` reader - B222
pub type B222_R = crate::BitReader<bool>;
///Field `B222` writer - B222
pub type B222_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
///Field `B223` reader - B223
pub type B223_R = crate::BitReader<bool>;
///Field `B223` writer - B223
pub type B223_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_VCTR6_SPEC, bool, O>;
impl R {
    ///Bit 0 - B192
    #[inline(always)]
    pub fn b192(&self) -> B192_R {
        B192_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - B193
    #[inline(always)]
    pub fn b193(&self) -> B193_R {
        B193_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - B194
    #[inline(always)]
    pub fn b194(&self) -> B194_R {
        B194_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - B195
    #[inline(always)]
    pub fn b195(&self) -> B195_R {
        B195_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - B196
    #[inline(always)]
    pub fn b196(&self) -> B196_R {
        B196_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - B197
    #[inline(always)]
    pub fn b197(&self) -> B197_R {
        B197_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B198
    #[inline(always)]
    pub fn b198(&self) -> B198_R {
        B198_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - B199
    #[inline(always)]
    pub fn b199(&self) -> B199_R {
        B199_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - B200
    #[inline(always)]
    pub fn b200(&self) -> B200_R {
        B200_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - B201
    #[inline(always)]
    pub fn b201(&self) -> B201_R {
        B201_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - B202
    #[inline(always)]
    pub fn b202(&self) -> B202_R {
        B202_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - B203
    #[inline(always)]
    pub fn b203(&self) -> B203_R {
        B203_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - B204
    #[inline(always)]
    pub fn b204(&self) -> B204_R {
        B204_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - B205
    #[inline(always)]
    pub fn b205(&self) -> B205_R {
        B205_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - B206
    #[inline(always)]
    pub fn b206(&self) -> B206_R {
        B206_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - B207
    #[inline(always)]
    pub fn b207(&self) -> B207_R {
        B207_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - B208
    #[inline(always)]
    pub fn b208(&self) -> B208_R {
        B208_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - B209
    #[inline(always)]
    pub fn b209(&self) -> B209_R {
        B209_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - B210
    #[inline(always)]
    pub fn b210(&self) -> B210_R {
        B210_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - B211
    #[inline(always)]
    pub fn b211(&self) -> B211_R {
        B211_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - B212
    #[inline(always)]
    pub fn b212(&self) -> B212_R {
        B212_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - B213
    #[inline(always)]
    pub fn b213(&self) -> B213_R {
        B213_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - B214
    #[inline(always)]
    pub fn b214(&self) -> B214_R {
        B214_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - B215
    #[inline(always)]
    pub fn b215(&self) -> B215_R {
        B215_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - B216
    #[inline(always)]
    pub fn b216(&self) -> B216_R {
        B216_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - B217
    #[inline(always)]
    pub fn b217(&self) -> B217_R {
        B217_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - B218
    #[inline(always)]
    pub fn b218(&self) -> B218_R {
        B218_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - B219
    #[inline(always)]
    pub fn b219(&self) -> B219_R {
        B219_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - B220
    #[inline(always)]
    pub fn b220(&self) -> B220_R {
        B220_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - B221
    #[inline(always)]
    pub fn b221(&self) -> B221_R {
        B221_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - B222
    #[inline(always)]
    pub fn b222(&self) -> B222_R {
        B222_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - B223
    #[inline(always)]
    pub fn b223(&self) -> B223_R {
        B223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - B192
    #[inline(always)]
    #[must_use]
    pub fn b192(&mut self) -> B192_W<0> {
        B192_W::new(self)
    }
    ///Bit 1 - B193
    #[inline(always)]
    #[must_use]
    pub fn b193(&mut self) -> B193_W<1> {
        B193_W::new(self)
    }
    ///Bit 2 - B194
    #[inline(always)]
    #[must_use]
    pub fn b194(&mut self) -> B194_W<2> {
        B194_W::new(self)
    }
    ///Bit 3 - B195
    #[inline(always)]
    #[must_use]
    pub fn b195(&mut self) -> B195_W<3> {
        B195_W::new(self)
    }
    ///Bit 4 - B196
    #[inline(always)]
    #[must_use]
    pub fn b196(&mut self) -> B196_W<4> {
        B196_W::new(self)
    }
    ///Bit 5 - B197
    #[inline(always)]
    #[must_use]
    pub fn b197(&mut self) -> B197_W<5> {
        B197_W::new(self)
    }
    ///Bit 6 - B198
    #[inline(always)]
    #[must_use]
    pub fn b198(&mut self) -> B198_W<6> {
        B198_W::new(self)
    }
    ///Bit 7 - B199
    #[inline(always)]
    #[must_use]
    pub fn b199(&mut self) -> B199_W<7> {
        B199_W::new(self)
    }
    ///Bit 8 - B200
    #[inline(always)]
    #[must_use]
    pub fn b200(&mut self) -> B200_W<8> {
        B200_W::new(self)
    }
    ///Bit 9 - B201
    #[inline(always)]
    #[must_use]
    pub fn b201(&mut self) -> B201_W<9> {
        B201_W::new(self)
    }
    ///Bit 10 - B202
    #[inline(always)]
    #[must_use]
    pub fn b202(&mut self) -> B202_W<10> {
        B202_W::new(self)
    }
    ///Bit 11 - B203
    #[inline(always)]
    #[must_use]
    pub fn b203(&mut self) -> B203_W<11> {
        B203_W::new(self)
    }
    ///Bit 12 - B204
    #[inline(always)]
    #[must_use]
    pub fn b204(&mut self) -> B204_W<12> {
        B204_W::new(self)
    }
    ///Bit 13 - B205
    #[inline(always)]
    #[must_use]
    pub fn b205(&mut self) -> B205_W<13> {
        B205_W::new(self)
    }
    ///Bit 14 - B206
    #[inline(always)]
    #[must_use]
    pub fn b206(&mut self) -> B206_W<14> {
        B206_W::new(self)
    }
    ///Bit 15 - B207
    #[inline(always)]
    #[must_use]
    pub fn b207(&mut self) -> B207_W<15> {
        B207_W::new(self)
    }
    ///Bit 16 - B208
    #[inline(always)]
    #[must_use]
    pub fn b208(&mut self) -> B208_W<16> {
        B208_W::new(self)
    }
    ///Bit 17 - B209
    #[inline(always)]
    #[must_use]
    pub fn b209(&mut self) -> B209_W<17> {
        B209_W::new(self)
    }
    ///Bit 18 - B210
    #[inline(always)]
    #[must_use]
    pub fn b210(&mut self) -> B210_W<18> {
        B210_W::new(self)
    }
    ///Bit 19 - B211
    #[inline(always)]
    #[must_use]
    pub fn b211(&mut self) -> B211_W<19> {
        B211_W::new(self)
    }
    ///Bit 20 - B212
    #[inline(always)]
    #[must_use]
    pub fn b212(&mut self) -> B212_W<20> {
        B212_W::new(self)
    }
    ///Bit 21 - B213
    #[inline(always)]
    #[must_use]
    pub fn b213(&mut self) -> B213_W<21> {
        B213_W::new(self)
    }
    ///Bit 22 - B214
    #[inline(always)]
    #[must_use]
    pub fn b214(&mut self) -> B214_W<22> {
        B214_W::new(self)
    }
    ///Bit 23 - B215
    #[inline(always)]
    #[must_use]
    pub fn b215(&mut self) -> B215_W<23> {
        B215_W::new(self)
    }
    ///Bit 24 - B216
    #[inline(always)]
    #[must_use]
    pub fn b216(&mut self) -> B216_W<24> {
        B216_W::new(self)
    }
    ///Bit 25 - B217
    #[inline(always)]
    #[must_use]
    pub fn b217(&mut self) -> B217_W<25> {
        B217_W::new(self)
    }
    ///Bit 26 - B218
    #[inline(always)]
    #[must_use]
    pub fn b218(&mut self) -> B218_W<26> {
        B218_W::new(self)
    }
    ///Bit 27 - B219
    #[inline(always)]
    #[must_use]
    pub fn b219(&mut self) -> B219_W<27> {
        B219_W::new(self)
    }
    ///Bit 28 - B220
    #[inline(always)]
    #[must_use]
    pub fn b220(&mut self) -> B220_W<28> {
        B220_W::new(self)
    }
    ///Bit 29 - B221
    #[inline(always)]
    #[must_use]
    pub fn b221(&mut self) -> B221_W<29> {
        B221_W::new(self)
    }
    ///Bit 30 - B222
    #[inline(always)]
    #[must_use]
    pub fn b222(&mut self) -> B222_W<30> {
        B222_W::new(self)
    }
    ///Bit 31 - B223
    #[inline(always)]
    #[must_use]
    pub fn b223(&mut self) -> B223_W<31> {
        B223_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MPCBBx vector register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcbb2_vctr6](index.html) module
pub struct MPCBB2_VCTR6_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcbb2_vctr6::R](R) reader structure
impl crate::Readable for MPCBB2_VCTR6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcbb2_vctr6::W](W) writer structure
impl crate::Writable for MPCBB2_VCTR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCBB2_VCTR6 to value 0
impl crate::Resettable for MPCBB2_VCTR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
