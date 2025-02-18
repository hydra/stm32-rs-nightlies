///Register `CICR` writer
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
///LSI ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC_AW {
    ///1: Clear the LSIRDYF flag
    Clear = 1,
}
impl From<LSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSIRDYC_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    ///Clear the LSIRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::Clear)
    }
}
///LSE ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC_AW {
    ///1: Clear the LSERDYF flag
    Clear = 1,
}
impl From<LSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSERDYC_AW, O>;
impl<'a, const O: u8> LSERDYC_W<'a, O> {
    ///Clear the LSERDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSERDYC_AW::Clear)
    }
}
///MSI ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYC_AW {
    ///1: Clear the MSIRDYF flag
    Clear = 1,
}
impl From<MSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub type MSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, MSIRDYC_AW, O>;
impl<'a, const O: u8> MSIRDYC_W<'a, O> {
    ///Clear the MSIRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MSIRDYC_AW::Clear)
    }
}
///HSI ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC_AW {
    ///1: Clear HSIRDYF flag
    Clear = 1,
}
impl From<HSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSIRDYC_AW, O>;
impl<'a, const O: u8> HSIRDYC_W<'a, O> {
    ///Clear HSIRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSIRDYC_AW::Clear)
    }
}
///HSE ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC_AW {
    ///1: Clear HSERDYF flag
    Clear = 1,
}
impl From<HSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYC` writer - HSE ready interrupt clear
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSERDYC_AW, O>;
impl<'a, const O: u8> HSERDYC_W<'a, O> {
    ///Clear HSERDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSERDYC_AW::Clear)
    }
}
///PLL ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYC_AW {
    ///1: Clear PLLRDYF flag
    Clear = 1,
}
impl From<PLLRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLRDYC_AW, O>;
impl<'a, const O: u8> PLLRDYC_W<'a, O> {
    ///Clear PLLRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLRDYC_AW::Clear)
    }
}
///PLLSAI1 ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYC_AW {
    ///1: Clear PLLSAI1RDYF flag
    Clear = 1,
}
impl From<PLLSAI1RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear
pub type PLLSAI1RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLSAI1RDYC_AW, O>;
impl<'a, const O: u8> PLLSAI1RDYC_W<'a, O> {
    ///Clear PLLSAI1RDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLSAI1RDYC_AW::Clear)
    }
}
///PLLSAI2 ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYC_AW {
    ///1: Clear PLLSAI2RDYF flag
    Clear = 1,
}
impl From<PLLSAI2RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear
pub type PLLSAI2RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLSAI2RDYC_AW, O>;
impl<'a, const O: u8> PLLSAI2RDYC_W<'a, O> {
    ///Clear PLLSAI2RDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLSAI2RDYC_AW::Clear)
    }
}
///Clock security system interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC_AW {
    ///1: Clear the CSSF flag
    Clear = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, CSSC_AW, O>;
impl<'a, const O: u8> CSSC_W<'a, O> {
    ///Clear the CSSF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSC_AW::Clear)
    }
}
///LSE Clock security system interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC_AW {
    ///1: Clear the LSECSSF flag
    Clear = 1,
}
impl From<LSECSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub type LSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSECSSC_AW, O>;
impl<'a, const O: u8> LSECSSC_W<'a, O> {
    ///Clear the LSECSSF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSECSSC_AW::Clear)
    }
}
///HSI48 oscillator ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC_AW {
    ///1: Clear the HSI48RDYC flag
    Clear = 1,
}
impl From<HSI48RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSI48RDYC_AW, O>;
impl<'a, const O: u8> HSI48RDYC_W<'a, O> {
    ///Clear the HSI48RDYC flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSI48RDYC_AW::Clear)
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<2> {
        MSIRDYC_W::new(self)
    }
    ///Bit 3 - HSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<3> {
        HSIRDYC_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<4> {
        HSERDYC_W::new(self)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<5> {
        PLLRDYC_W::new(self)
    }
    ///Bit 6 - PLLSAI1 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<6> {
        PLLSAI1RDYC_W::new(self)
    }
    ///Bit 7 - PLLSAI2 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllsai2rdyc(&mut self) -> PLLSAI2RDYC_W<7> {
        PLLSAI2RDYC_W::new(self)
    }
    ///Bit 8 - Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<8> {
        CSSC_W::new(self)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<9> {
        LSECSSC_W::new(self)
    }
    ///Bit 10 - HSI48 oscillator ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<10> {
        HSI48RDYC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cicr](index.html) module
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [cicr::W](W) writer structure
impl crate::Writable for CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
