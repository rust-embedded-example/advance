#[doc = "Register `DHR12R1` reader"]
pub type R = crate::R<Dhr12r1Spec>;
#[doc = "Register `DHR12R1` writer"]
pub type W = crate::W<Dhr12r1Spec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data"]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data"]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<'_, Dhr12r1Spec> {
        Dacc1dhrW::new(self, 0)
    }
}
#[doc = "channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12r1Spec;
impl crate::RegisterSpec for Dhr12r1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12r1::R`](R) reader structure"]
impl crate::Readable for Dhr12r1Spec {}
#[doc = "`write(|w| ..)` method takes [`dhr12r1::W`](W) writer structure"]
impl crate::Writable for Dhr12r1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12R1 to value 0"]
impl crate::Resettable for Dhr12r1Spec {}
