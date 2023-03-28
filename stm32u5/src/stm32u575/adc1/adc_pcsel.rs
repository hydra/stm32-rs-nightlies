///Register `ADC_PCSEL` reader
pub struct R(crate::R<ADC_PCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_PCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_PCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_PCSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_PCSEL` writer
pub struct W(crate::W<ADC_PCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_PCSEL_SPEC>;
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
impl From<crate::W<ADC_PCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_PCSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCSEL0` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL0_R = crate::BitReader<bool>;
///Field `PCSEL0` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL1` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL1_R = crate::BitReader<bool>;
///Field `PCSEL1` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL2` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL2_R = crate::BitReader<bool>;
///Field `PCSEL2` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL3` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL3_R = crate::BitReader<bool>;
///Field `PCSEL3` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL4` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL4_R = crate::BitReader<bool>;
///Field `PCSEL4` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL5` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL5_R = crate::BitReader<bool>;
///Field `PCSEL5` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL6` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL6_R = crate::BitReader<bool>;
///Field `PCSEL6` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL7` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL7_R = crate::BitReader<bool>;
///Field `PCSEL7` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL8` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL8_R = crate::BitReader<bool>;
///Field `PCSEL8` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL9` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL9_R = crate::BitReader<bool>;
///Field `PCSEL9` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL10` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL10_R = crate::BitReader<bool>;
///Field `PCSEL10` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL11` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL11_R = crate::BitReader<bool>;
///Field `PCSEL11` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL12` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL12_R = crate::BitReader<bool>;
///Field `PCSEL12` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL13` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL13_R = crate::BitReader<bool>;
///Field `PCSEL13` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL14` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL14_R = crate::BitReader<bool>;
///Field `PCSEL14` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL15` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL15_R = crate::BitReader<bool>;
///Field `PCSEL15` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL16` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL16_R = crate::BitReader<bool>;
///Field `PCSEL16` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL17` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL17_R = crate::BitReader<bool>;
///Field `PCSEL17` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL18` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL18_R = crate::BitReader<bool>;
///Field `PCSEL18` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
///Field `PCSEL19` reader - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL19_R = crate::BitReader<bool>;
///Field `PCSEL19` writer - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type PCSEL19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PCSEL_SPEC, bool, O>;
impl R {
    ///Bit 0 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel4(&self) -> PCSEL4_R {
        PCSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel5(&self) -> PCSEL5_R {
        PCSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel6(&self) -> PCSEL6_R {
        PCSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel7(&self) -> PCSEL7_R {
        PCSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel8(&self) -> PCSEL8_R {
        PCSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel9(&self) -> PCSEL9_R {
        PCSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel10(&self) -> PCSEL10_R {
        PCSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel11(&self) -> PCSEL11_R {
        PCSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel12(&self) -> PCSEL12_R {
        PCSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel13(&self) -> PCSEL13_R {
        PCSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel14(&self) -> PCSEL14_R {
        PCSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel15(&self) -> PCSEL15_R {
        PCSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel16(&self) -> PCSEL16_R {
        PCSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel17(&self) -> PCSEL17_R {
        PCSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel18(&self) -> PCSEL18_R {
        PCSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn pcsel19(&self) -> PCSEL19_R {
        PCSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel0(&mut self) -> PCSEL0_W<0> {
        PCSEL0_W::new(self)
    }
    ///Bit 1 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel1(&mut self) -> PCSEL1_W<1> {
        PCSEL1_W::new(self)
    }
    ///Bit 2 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel2(&mut self) -> PCSEL2_W<2> {
        PCSEL2_W::new(self)
    }
    ///Bit 3 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel3(&mut self) -> PCSEL3_W<3> {
        PCSEL3_W::new(self)
    }
    ///Bit 4 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel4(&mut self) -> PCSEL4_W<4> {
        PCSEL4_W::new(self)
    }
    ///Bit 5 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel5(&mut self) -> PCSEL5_W<5> {
        PCSEL5_W::new(self)
    }
    ///Bit 6 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel6(&mut self) -> PCSEL6_W<6> {
        PCSEL6_W::new(self)
    }
    ///Bit 7 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel7(&mut self) -> PCSEL7_W<7> {
        PCSEL7_W::new(self)
    }
    ///Bit 8 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel8(&mut self) -> PCSEL8_W<8> {
        PCSEL8_W::new(self)
    }
    ///Bit 9 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel9(&mut self) -> PCSEL9_W<9> {
        PCSEL9_W::new(self)
    }
    ///Bit 10 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel10(&mut self) -> PCSEL10_W<10> {
        PCSEL10_W::new(self)
    }
    ///Bit 11 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel11(&mut self) -> PCSEL11_W<11> {
        PCSEL11_W::new(self)
    }
    ///Bit 12 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel12(&mut self) -> PCSEL12_W<12> {
        PCSEL12_W::new(self)
    }
    ///Bit 13 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel13(&mut self) -> PCSEL13_W<13> {
        PCSEL13_W::new(self)
    }
    ///Bit 14 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel14(&mut self) -> PCSEL14_W<14> {
        PCSEL14_W::new(self)
    }
    ///Bit 15 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel15(&mut self) -> PCSEL15_W<15> {
        PCSEL15_W::new(self)
    }
    ///Bit 16 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel16(&mut self) -> PCSEL16_W<16> {
        PCSEL16_W::new(self)
    }
    ///Bit 17 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel17(&mut self) -> PCSEL17_W<17> {
        PCSEL17_W::new(self)
    }
    ///Bit 18 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel18(&mut self) -> PCSEL18_W<18> {
        PCSEL18_W::new(self)
    }
    ///Bit 19 - Channel i (VINP\[i\]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn pcsel19(&mut self) -> PCSEL19_W<19> {
        PCSEL19_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC channel preselection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_pcsel](index.html) module
pub struct ADC_PCSEL_SPEC;
impl crate::RegisterSpec for ADC_PCSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_pcsel::R](R) reader structure
impl crate::Readable for ADC_PCSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_pcsel::W](W) writer structure
impl crate::Writable for ADC_PCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_PCSEL to value 0
impl crate::Resettable for ADC_PCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
