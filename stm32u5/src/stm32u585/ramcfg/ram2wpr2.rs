///Register `RAM2WPR2` reader
pub struct R(crate::R<RAM2WPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2WPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2WPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2WPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RAM2WPR2` writer
pub struct W(crate::W<RAM2WPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM2WPR2_SPEC>;
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
impl From<crate::W<RAM2WPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM2WPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `P32WP` reader - P32WP
pub type P32WP_R = crate::BitReader<bool>;
///Field `P32WP` writer - P32WP
pub type P32WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P33WP` reader - P33WP
pub type P33WP_R = crate::BitReader<bool>;
///Field `P33WP` writer - P33WP
pub type P33WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P34WP` reader - P34WP
pub type P34WP_R = crate::BitReader<bool>;
///Field `P34WP` writer - P34WP
pub type P34WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P35WP` reader - P35WP
pub type P35WP_R = crate::BitReader<bool>;
///Field `P35WP` writer - P35WP
pub type P35WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P36WP` reader - P36WP
pub type P36WP_R = crate::BitReader<bool>;
///Field `P36WP` writer - P36WP
pub type P36WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P37WP` reader - P37WP
pub type P37WP_R = crate::BitReader<bool>;
///Field `P37WP` writer - P37WP
pub type P37WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P38WP` reader - P38WP
pub type P38WP_R = crate::BitReader<bool>;
///Field `P38WP` writer - P38WP
pub type P38WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P39WP` reader - P39WP
pub type P39WP_R = crate::BitReader<bool>;
///Field `P39WP` writer - P39WP
pub type P39WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P40WP` reader - P40WP
pub type P40WP_R = crate::BitReader<bool>;
///Field `P40WP` writer - P40WP
pub type P40WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P41WP` reader - P41WP
pub type P41WP_R = crate::BitReader<bool>;
///Field `P41WP` writer - P41WP
pub type P41WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P42WP` reader - P42WP
pub type P42WP_R = crate::BitReader<bool>;
///Field `P42WP` writer - P42WP
pub type P42WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P43WP` reader - P43WP
pub type P43WP_R = crate::BitReader<bool>;
///Field `P43WP` writer - P43WP
pub type P43WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P44WP` reader - P44WP
pub type P44WP_R = crate::BitReader<bool>;
///Field `P44WP` writer - P44WP
pub type P44WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P45WP` reader - P45WP
pub type P45WP_R = crate::BitReader<bool>;
///Field `P45WP` writer - P45WP
pub type P45WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P46WP` reader - P46WP
pub type P46WP_R = crate::BitReader<bool>;
///Field `P46WP` writer - P46WP
pub type P46WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P47WP` reader - P47WP
pub type P47WP_R = crate::BitReader<bool>;
///Field `P47WP` writer - P47WP
pub type P47WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P48WP` reader - P48WP
pub type P48WP_R = crate::BitReader<bool>;
///Field `P48WP` writer - P48WP
pub type P48WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P49WP` reader - P49WP
pub type P49WP_R = crate::BitReader<bool>;
///Field `P49WP` writer - P49WP
pub type P49WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P50WP` reader - P50WP
pub type P50WP_R = crate::BitReader<bool>;
///Field `P50WP` writer - P50WP
pub type P50WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P51WP` reader - P51WP
pub type P51WP_R = crate::BitReader<bool>;
///Field `P51WP` writer - P51WP
pub type P51WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P52WP` reader - P52WP
pub type P52WP_R = crate::BitReader<bool>;
///Field `P52WP` writer - P52WP
pub type P52WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P53WP` reader - P53WP
pub type P53WP_R = crate::BitReader<bool>;
///Field `P53WP` writer - P53WP
pub type P53WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P54WP` reader - P54WP
pub type P54WP_R = crate::BitReader<bool>;
///Field `P54WP` writer - P54WP
pub type P54WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P55WP` reader - P55WP
pub type P55WP_R = crate::BitReader<bool>;
///Field `P55WP` writer - P55WP
pub type P55WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P56WP` reader - P56WP
pub type P56WP_R = crate::BitReader<bool>;
///Field `P56WP` writer - P56WP
pub type P56WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P57WP` reader - P57WP
pub type P57WP_R = crate::BitReader<bool>;
///Field `P57WP` writer - P57WP
pub type P57WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P58WP` reader - P58WP
pub type P58WP_R = crate::BitReader<bool>;
///Field `P58WP` writer - P58WP
pub type P58WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P59WP` reader - P59WP
pub type P59WP_R = crate::BitReader<bool>;
///Field `P59WP` writer - P59WP
pub type P59WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P60WP` reader - P60WP
pub type P60WP_R = crate::BitReader<bool>;
///Field `P60WP` writer - P60WP
pub type P60WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P61WP` reader - P61WP
pub type P61WP_R = crate::BitReader<bool>;
///Field `P61WP` writer - P61WP
pub type P61WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P62WP` reader - P62WP
pub type P62WP_R = crate::BitReader<bool>;
///Field `P62WP` writer - P62WP
pub type P62WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
///Field `P63WP` reader - P63WP
pub type P63WP_R = crate::BitReader<bool>;
///Field `P63WP` writer - P63WP
pub type P63WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM2WPR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - P32WP
    #[inline(always)]
    pub fn p32wp(&self) -> P32WP_R {
        P32WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - P33WP
    #[inline(always)]
    pub fn p33wp(&self) -> P33WP_R {
        P33WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - P34WP
    #[inline(always)]
    pub fn p34wp(&self) -> P34WP_R {
        P34WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - P35WP
    #[inline(always)]
    pub fn p35wp(&self) -> P35WP_R {
        P35WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - P36WP
    #[inline(always)]
    pub fn p36wp(&self) -> P36WP_R {
        P36WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - P37WP
    #[inline(always)]
    pub fn p37wp(&self) -> P37WP_R {
        P37WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - P38WP
    #[inline(always)]
    pub fn p38wp(&self) -> P38WP_R {
        P38WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - P39WP
    #[inline(always)]
    pub fn p39wp(&self) -> P39WP_R {
        P39WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - P40WP
    #[inline(always)]
    pub fn p40wp(&self) -> P40WP_R {
        P40WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - P41WP
    #[inline(always)]
    pub fn p41wp(&self) -> P41WP_R {
        P41WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - P42WP
    #[inline(always)]
    pub fn p42wp(&self) -> P42WP_R {
        P42WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - P43WP
    #[inline(always)]
    pub fn p43wp(&self) -> P43WP_R {
        P43WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - P44WP
    #[inline(always)]
    pub fn p44wp(&self) -> P44WP_R {
        P44WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - P45WP
    #[inline(always)]
    pub fn p45wp(&self) -> P45WP_R {
        P45WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - P46WP
    #[inline(always)]
    pub fn p46wp(&self) -> P46WP_R {
        P46WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - P47WP
    #[inline(always)]
    pub fn p47wp(&self) -> P47WP_R {
        P47WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - P48WP
    #[inline(always)]
    pub fn p48wp(&self) -> P48WP_R {
        P48WP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - P49WP
    #[inline(always)]
    pub fn p49wp(&self) -> P49WP_R {
        P49WP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - P50WP
    #[inline(always)]
    pub fn p50wp(&self) -> P50WP_R {
        P50WP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - P51WP
    #[inline(always)]
    pub fn p51wp(&self) -> P51WP_R {
        P51WP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - P52WP
    #[inline(always)]
    pub fn p52wp(&self) -> P52WP_R {
        P52WP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - P53WP
    #[inline(always)]
    pub fn p53wp(&self) -> P53WP_R {
        P53WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - P54WP
    #[inline(always)]
    pub fn p54wp(&self) -> P54WP_R {
        P54WP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - P55WP
    #[inline(always)]
    pub fn p55wp(&self) -> P55WP_R {
        P55WP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - P56WP
    #[inline(always)]
    pub fn p56wp(&self) -> P56WP_R {
        P56WP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - P57WP
    #[inline(always)]
    pub fn p57wp(&self) -> P57WP_R {
        P57WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - P58WP
    #[inline(always)]
    pub fn p58wp(&self) -> P58WP_R {
        P58WP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - P59WP
    #[inline(always)]
    pub fn p59wp(&self) -> P59WP_R {
        P59WP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - P60WP
    #[inline(always)]
    pub fn p60wp(&self) -> P60WP_R {
        P60WP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - P61WP
    #[inline(always)]
    pub fn p61wp(&self) -> P61WP_R {
        P61WP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - P62WP
    #[inline(always)]
    pub fn p62wp(&self) -> P62WP_R {
        P62WP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - P63WP
    #[inline(always)]
    pub fn p63wp(&self) -> P63WP_R {
        P63WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - P32WP
    #[inline(always)]
    #[must_use]
    pub fn p32wp(&mut self) -> P32WP_W<0> {
        P32WP_W::new(self)
    }
    ///Bit 1 - P33WP
    #[inline(always)]
    #[must_use]
    pub fn p33wp(&mut self) -> P33WP_W<1> {
        P33WP_W::new(self)
    }
    ///Bit 2 - P34WP
    #[inline(always)]
    #[must_use]
    pub fn p34wp(&mut self) -> P34WP_W<2> {
        P34WP_W::new(self)
    }
    ///Bit 3 - P35WP
    #[inline(always)]
    #[must_use]
    pub fn p35wp(&mut self) -> P35WP_W<3> {
        P35WP_W::new(self)
    }
    ///Bit 4 - P36WP
    #[inline(always)]
    #[must_use]
    pub fn p36wp(&mut self) -> P36WP_W<4> {
        P36WP_W::new(self)
    }
    ///Bit 5 - P37WP
    #[inline(always)]
    #[must_use]
    pub fn p37wp(&mut self) -> P37WP_W<5> {
        P37WP_W::new(self)
    }
    ///Bit 6 - P38WP
    #[inline(always)]
    #[must_use]
    pub fn p38wp(&mut self) -> P38WP_W<6> {
        P38WP_W::new(self)
    }
    ///Bit 7 - P39WP
    #[inline(always)]
    #[must_use]
    pub fn p39wp(&mut self) -> P39WP_W<7> {
        P39WP_W::new(self)
    }
    ///Bit 8 - P40WP
    #[inline(always)]
    #[must_use]
    pub fn p40wp(&mut self) -> P40WP_W<8> {
        P40WP_W::new(self)
    }
    ///Bit 9 - P41WP
    #[inline(always)]
    #[must_use]
    pub fn p41wp(&mut self) -> P41WP_W<9> {
        P41WP_W::new(self)
    }
    ///Bit 10 - P42WP
    #[inline(always)]
    #[must_use]
    pub fn p42wp(&mut self) -> P42WP_W<10> {
        P42WP_W::new(self)
    }
    ///Bit 11 - P43WP
    #[inline(always)]
    #[must_use]
    pub fn p43wp(&mut self) -> P43WP_W<11> {
        P43WP_W::new(self)
    }
    ///Bit 12 - P44WP
    #[inline(always)]
    #[must_use]
    pub fn p44wp(&mut self) -> P44WP_W<12> {
        P44WP_W::new(self)
    }
    ///Bit 13 - P45WP
    #[inline(always)]
    #[must_use]
    pub fn p45wp(&mut self) -> P45WP_W<13> {
        P45WP_W::new(self)
    }
    ///Bit 14 - P46WP
    #[inline(always)]
    #[must_use]
    pub fn p46wp(&mut self) -> P46WP_W<14> {
        P46WP_W::new(self)
    }
    ///Bit 15 - P47WP
    #[inline(always)]
    #[must_use]
    pub fn p47wp(&mut self) -> P47WP_W<15> {
        P47WP_W::new(self)
    }
    ///Bit 16 - P48WP
    #[inline(always)]
    #[must_use]
    pub fn p48wp(&mut self) -> P48WP_W<16> {
        P48WP_W::new(self)
    }
    ///Bit 17 - P49WP
    #[inline(always)]
    #[must_use]
    pub fn p49wp(&mut self) -> P49WP_W<17> {
        P49WP_W::new(self)
    }
    ///Bit 18 - P50WP
    #[inline(always)]
    #[must_use]
    pub fn p50wp(&mut self) -> P50WP_W<18> {
        P50WP_W::new(self)
    }
    ///Bit 19 - P51WP
    #[inline(always)]
    #[must_use]
    pub fn p51wp(&mut self) -> P51WP_W<19> {
        P51WP_W::new(self)
    }
    ///Bit 20 - P52WP
    #[inline(always)]
    #[must_use]
    pub fn p52wp(&mut self) -> P52WP_W<20> {
        P52WP_W::new(self)
    }
    ///Bit 21 - P53WP
    #[inline(always)]
    #[must_use]
    pub fn p53wp(&mut self) -> P53WP_W<21> {
        P53WP_W::new(self)
    }
    ///Bit 22 - P54WP
    #[inline(always)]
    #[must_use]
    pub fn p54wp(&mut self) -> P54WP_W<22> {
        P54WP_W::new(self)
    }
    ///Bit 23 - P55WP
    #[inline(always)]
    #[must_use]
    pub fn p55wp(&mut self) -> P55WP_W<23> {
        P55WP_W::new(self)
    }
    ///Bit 24 - P56WP
    #[inline(always)]
    #[must_use]
    pub fn p56wp(&mut self) -> P56WP_W<24> {
        P56WP_W::new(self)
    }
    ///Bit 25 - P57WP
    #[inline(always)]
    #[must_use]
    pub fn p57wp(&mut self) -> P57WP_W<25> {
        P57WP_W::new(self)
    }
    ///Bit 26 - P58WP
    #[inline(always)]
    #[must_use]
    pub fn p58wp(&mut self) -> P58WP_W<26> {
        P58WP_W::new(self)
    }
    ///Bit 27 - P59WP
    #[inline(always)]
    #[must_use]
    pub fn p59wp(&mut self) -> P59WP_W<27> {
        P59WP_W::new(self)
    }
    ///Bit 28 - P60WP
    #[inline(always)]
    #[must_use]
    pub fn p60wp(&mut self) -> P60WP_W<28> {
        P60WP_W::new(self)
    }
    ///Bit 29 - P61WP
    #[inline(always)]
    #[must_use]
    pub fn p61wp(&mut self) -> P61WP_W<29> {
        P61WP_W::new(self)
    }
    ///Bit 30 - P62WP
    #[inline(always)]
    #[must_use]
    pub fn p62wp(&mut self) -> P62WP_W<30> {
        P62WP_W::new(self)
    }
    ///Bit 31 - P63WP
    #[inline(always)]
    #[must_use]
    pub fn p63wp(&mut self) -> P63WP_W<31> {
        P63WP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG SRAM2 write protection register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram2wpr2](index.html) module
pub struct RAM2WPR2_SPEC;
impl crate::RegisterSpec for RAM2WPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram2wpr2::R](R) reader structure
impl crate::Readable for RAM2WPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ram2wpr2::W](W) writer structure
impl crate::Writable for RAM2WPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM2WPR2 to value 0
impl crate::Resettable for RAM2WPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
