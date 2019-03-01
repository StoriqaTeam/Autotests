///*
//package Regress;
//
//import Page.autorization_page.AutorizationLoc;
//import Page.shop.Shop;
//import Steps.Autorization;
//import Helper.Generator;
//import Helper.SeleniumRunner;
//import org.junit.Assert;
//import org.junit.Before;
//import org.junit.Alll;
//import org.junit.runner.RunWith;
//import org.openqa.selenium.By;
//import org.openqa.selenium.Keys;
//import org.openqa.selenium.WebDriver;
//import org.openqa.selenium.chrome.ChromeDriver;
//
//import java.util.concurrent.TimeUnit;
//
//@RunWith(SeleniumRunner.class)
//
//public class RegressTest {
//
//    WebDriver driver;
//
//    @Before
//    public void setUp(){
//        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
//        driver = new ChromeDriver();
//    }
//
//    @Alll
//    public void Regress_Test() throws InterruptedException {
//
//        //Регистрация новго пользователя
//        driver.get("https://stage:whenLambo%3F@stage.stq.cloud/auth");
//        //driver.get("https://storiqateam:s3cur3passw0rd@nightly.stq.cloud/auth");
//        driver.findElement(By.xpath(AutorizationLoc.SIGNUP.getCl())).click();
//        //driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).isDisplayed();
//        driver.findElement(By.xpath("//div[normalize-space(text())='Sign Up']"));
//        driver.findElement(By.xpath(AutorizationLoc.FIRSTNAME.getCl())).sendKeys("Ivan");
//        driver.findElement(By.xpath(AutorizationLoc.LASTNAME.getCl())).sendKeys("Petrov");
//        driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys(Generator.generateEmail());
//        driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("1234567Qq");
//        driver.manage().timeouts().implicitlyWait(90, TimeUnit.SECONDS);
//        //driver.findElement(By.xpath(AutorizationLoc.TERMS.getCl())).click();
//        //driver.manage().timeouts().implicitlyWait(40, TimeUnit.SECONDS);
//        //driver.findElement(By.xpath(AutorizationLoc.PRIVASY.getCl())).click();
//        Thread.sleep(12000);
//        driver.findElement(By.xpath("//*[@data-test='terms']")).click();
//        driver.findElement(By.xpath("//*[@data-test='privacy']")).click();
//        driver.findElement(By.xpath(AutorizationLoc.UPBUTTON.getCl())).click();
//
//        driver.close();
//    }
//
//    @Alll
//    public void Autorization_Test() throws InterruptedException {
//
//        // Авторизация
//
//        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
//        Assert.assertEquals("Storiqa", driver.getTitle());
//        //driver.findElement(By.xpath("//div[@class='AuthButtons__signUpButton___2jzhw']")).click();
//        driver.findElement(By.xpath(AutorizationLoc.SIGNIN.getCl())).click();
//        driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys("22684@crapmail.tld");
//        driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("M4n3b2v1");
//        driver.manage().timeouts().implicitlyWait(15, TimeUnit.SECONDS);
//        driver.findElement(By.xpath(AutorizationLoc.INBUTTON.getCl())).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
//
//        driver.close();
//    }
//
//    @Alll
//    public void Createshop_Test() throws InterruptedException {
//
//        Autorization user = new Autorization(driver);
//        user.autorization_login();
//
//
//        Assert.assertEquals("Storiqa", driver.getTitle());
//        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
//        Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
//        driver.findElement(By.xpath(Shop.SELLONSTORIQA.getCl())).click();
//        //Assert.assertEquals("https://stage.stq.cloud/start-selling/en", driver.getCurrentUrl());
//        //driver.findElement(By.xpath("//div[normalize-space(text())='Online crypto marketplace']"));
////        driver.findElement(By.xpath(Shop.STARTSELLING.getCl())).click();
//
//        driver.get("https://stage.stq.cloud/manage/wizard?");
//
//        // Переходим в визард
////        Assert.assertEquals("https://stage.stq.cloud/manage/wizard", driver.getCurrentUrl());
////        driver.findElement(By.xpath("//div[normalize-space(text())='Give your store a name']"));
//
////        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
////        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
////        Thread.sleep(7000);
////        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//
//        //Thread.sleep(10000);
//
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("my test shop");
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("test123qwe");
//        //Thread.sleep(7000);
//        //driver.findElement(By.xpath("//div[normalize-space(text())='Vacant']"));
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("my test shop");
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isEnabled();
//        //Thread.sleep(7000);
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//        Thread.sleep(10000);
//
//        driver.findElement(By.xpath("//div[normalize-space(text())='Set up store']"));
//        driver.findElement(By.xpath(Shop.MAINLANGUAGE.getCl())).click();
//        driver.findElement(By.xpath(Shop.ENGLISH.getCl())).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        //Thread.sleep(5000);
//        //driver.findElement(By.xpath("//*[@data-test='AddressFormSelect']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ALBANIA.getCl())).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
//        Thread.sleep(5000);
//        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='streetNumber'='1']"));
//        driver.findElement(By.xpath("//*[@data-test='locality'='Moscow']"));
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2'='Moscow']"));
//        driver.findElement(By.xpath("//*[@data-test='route'='Tverskaya Street']"));
//        driver.findElement(By.xpath("//*[@data-test='postalCode'='125009']"));
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//
//        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
//        driver.findElement(By.xpath(Shop.ADDFIRSTPRODUCTFOTO.getCl())).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));
//
//        driver.close();
//    }
//
//
//    @Alll
//    public void AddProduct_Test() throws InterruptedException {
//
//        Autorization user = new Autorization(driver);
//        user.autorization_login();
//
//        org.junit.Assert.assertEquals("Storiqa", driver.getTitle());
//        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
//        org.junit.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
//        driver.findElement(By.xpath(Shop.SELLONSTORIQA.getCl())).click();
//
//        driver.get("https://stage.stq.cloud/manage/wizard?");
//
//        // Переходим в визард
//
//        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        //Thread.sleep(7000);
//        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//
//        //Thread.sleep(10000);
//
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("my test shop");
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        Thread.sleep(7000);
//        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys(Generator.generateSite());
//        Thread.sleep(7000);
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("my test shop");
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isEnabled();
//        Thread.sleep(7000);
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//        Thread.sleep(10000);
//
//        driver.findElement(By.xpath("//div[normalize-space(text())='Set up store']"));
//        driver.findElement(By.xpath(Shop.MAINLANGUAGE.getCl())).click();
//        driver.findElement(By.xpath(Shop.ENGLISH.getCl())).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='stq-input']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ALBANIA.getCl())).click();
//        driver.findElement(By.xpath("//*[.='Albania']")).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='stq-input']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
//        Thread.sleep(4000);
//        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='streetNumber'='1']"));
//        driver.findElement(By.xpath("//*[@data-test='locality'='Moscow']"));
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2'='Moscow']"));
//        driver.findElement(By.xpath("//*[@data-test='route'='Tverskaya Street']"));
//        driver.findElement(By.xpath("//*[@data-test='postalCode'='125009']"));
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//
//        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
//        //driver.findElement(By.xpath(Shop.ADDFIRSTPRODUCTFOTO.getCl())).click();
//        //driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));
//        //driver.findElement(By.xpath("//*[@data-test='wizardBackButton']")).click();
//        //driver.findElement(By.xpath("//*[@data-test='closeWizard']")).click();
//        //driver.findElement(By.xpath("//*[@data-test='wizardUploaderProductFotoFirst']")).click();
////
////        //Создание продукта
////
//        //driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));
////        driver.findElement(By.xpath(Shop.PRODUCTNAME.getCl())).sendKeys("my produkt");
////        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("test produkt");
////        driver.findElement(By.xpath(Shop.PRODUKTCATEGORY.getCl())).click();
////        driver.findElement(By.xpath("//div[normalize-space(text())='Office & School Supplies']"));
////        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
////        driver.findElement(By.xpath("//div[normalize-space(text())='Calendars, Planners & Cards']"));
////        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
////        driver.findElement(By.xpath("//div[normalize-space(text())='Postcards']"));
////        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
////        driver.findElement(By.xpath(Shop.PRICE.getCl())).sendKeys("1000");
////        driver.findElement(By.xpath(Shop.VENDORCODE.getCl())).sendKeys("123");
////        driver.findElement(By.xpath(Shop.CASHEBACK.getCl())).sendKeys("10");
////        driver.findElement(By.xpath(Shop.QUANTITY.getCl())).sendKeys("100");
////        //driver.findElement(By.xpath(Shop.CURRENCY.getCl())).click();
////        //driver.manage().timeouts().implicitlyWait(40, TimeUnit.SECONDS);
////        //driver.findElement(By.xpath(Shop.STQ.getCl())).sendKeys(Keys.ENTER);
////        driver.findElement(By.xpath(Shop.SAVEPRODUCTBUTTON.getCl())).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
//        driver.findElement(By.xpath("//*[@data-test='wizardBackButton']")).click();
//        driver.findElement(By.xpath("//*[@data-test='wizardUploaderProductFotoFirst']")).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='my produkt']"));
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//        //driver.switchTo().window()
//        driver.findElement(By.xpath("//*[@data-test='closeWizard']")).click();
//
//        driver.close();
//    }
//
//
//}
//*/
