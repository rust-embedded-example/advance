#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<Ccmr1OutputSpec>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<Ccmr1OutputSpec>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Output Compare 1 fast enable"]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output Compare 1 fast enable"]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output Compare 1 preload enable"]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output Compare 1 preload enable"]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - Output Compare 1 mode"]
pub type Oc1mR = crate::FieldReader;
#[doc = "Field `OC1M` writer - Output Compare 1 mode"]
pub type Oc1mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1M_3` reader - Output Compare 1 mode"]
pub type Oc1m3R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - Output Compare 1 mode"]
pub type Oc1m3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> Oc1mR {
        Oc1mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> Oc1m3R {
        Oc1m3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1OutputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<'_, Ccmr1OutputSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, Ccmr1OutputSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> Oc1mW<'_, Ccmr1OutputSpec> {
        Oc1mW::new(self, 4)
    }
    #[doc = "Bit 16 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> Oc1m3W<'_, Ccmr1OutputSpec> {
        Oc1m3W::new(self, 16)
    }
}
#[doc = "capture/compare mode register (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1OutputSpec;
impl crate::RegisterSpec for Ccmr1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for Ccmr1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for Ccmr1OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for Ccmr1OutputSpec {}
