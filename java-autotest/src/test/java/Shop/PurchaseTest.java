package Shop;

import Helper.Generator;
import Helper.Wait;
import Page.shop.Shop;
import Steps.Autorization;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class PurchaseTest {

    WebDriver driver;

    public WebElement countryDellivery() {
        return driver.findElement(By.xpath("//*[@data-test='productDeliveryCountrySelectInput']"));
    }
    public WebElement buyNow() {
        return driver.findElement(By.xpath("//*[@data-test='product-BuyNowSTQ']"));
    }
    public WebElement receiverName() {
        return driver.findElement(By.xpath("//*[@data-test='receiverName']"));
    }
    public WebElement phone() {
        return driver.findElement(By.xpath("//*[@data-test='receiverPhone']"));
    }
    public WebElement newAddress() {
        return driver.findElement(By.xpath("//*[@data-test='newAddressCheckbox']"));
    }
    public WebElement country() {
        return driver.findElement(By.xpath("//*[@data-test='AddressFormSelectInput']"));
    }
    public WebElement address() {
        return driver.findElement(By.xpath("//*[@data-test='autocompleteAddress']"));
    }
    public WebElement street() {
        return driver.findElement(By.xpath("//*[@data-test='streetNumber']"));
    }
    public WebElement locality() {
        return driver.findElement(By.xpath("//*[@data-test='locality']"));
    }
    public WebElement administrativeArea() {
        return driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']"));
    }
    public WebElement route() {
        return driver.findElement(By.xpath("//*[@data-test='route']"));
    }
    public WebElement postalCode() {
        return driver.findElement(By.xpath("//*[@data-test='postalCode']"));
    }
    public WebElement nextButton() {
        return driver.findElement(By.xpath("//*[@data-test='checkoutNext']"));
    }
    public WebElement payment() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Payment']"));
    }
    public WebElement addCart() {
    return driver.findElement(By.xpath("//*[@data-test='product-addToCart']"));
}
    public WebElement myCart() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='My Cart']"));
    }
    public WebElement comment() {
        return driver.findElement(By.xpath("//*[@data-test='customerComment']"));
    }

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void Purchase_Test() throws Exception {


        //By now.

        Autorization user = new Autorization(driver);
        user.autorization_login();

        try {
//            driver.get("https://stage.stq.cloud/store/339/products/988");
//            countryDellivery().click();
//            countryDellivery().sendKeys(RUSS);
//            countryDellivery().sendKeys(Keys.ENTER);
//            buyNow().click();
//            receiverName().sendKeys(DELL);
//            receiverName().sendKeys(Generator.generateName() + " " + Generator.generateName());
//            phone().sendKeys(DELL);
//            phone().sendKeys(Generator.generatePhone());
//            newAddress().click();
//            country().click();
//            Thread.sleep(3000);
//            country().sendKeys(RUSS);
//            Thread.sleep(3000);
//            country().sendKeys(Keys.ENTER);
//            address().click();
//            address().sendKeys(STREET2);
//            address().sendKeys(Keys.ENTER);
//            Wait.sleep(driver, 10);
//            street().click();
//            street().sendKeys(APART);
//            locality().sendKeys(LOCALITY);
//            administrativeArea().sendKeys("Moscow");
//            route().sendKeys("Semenovskaya Ploshchad' 1");
//            postalCode().sendKeys("125009");
//            Thread.sleep(4000);
//            nextButton().click();
//            Thread.sleep(3000);
//            nextButton().click();
//            payment();

            driver.get("https://stage.stq.cloud/store/339/products/988");
            addCart().click();
            Thread.sleep(2000);
            addCart().click();
            myCart();
            comment().sendKeys(TEXT);
            nextButton().click();
            receiverName().sendKeys(DELL);
            receiverName().sendKeys(Generator.generateName() + " " + Generator.generateName());
            phone().sendKeys(DELL);
            phone().sendKeys(Generator.generatePhone());
            newAddress().click();
            country().click();
            Thread.sleep(5000);
            country().sendKeys(RUSS);
            Thread.sleep(9000);
            country().sendKeys(Keys.ENTER);
            Wait.sleep(driver, 20);
            address().click();
            address().sendKeys(STREET1);
            address().sendKeys(Keys.ENTER);
            Wait.sleep(driver, 15);
            street().click();
            street().sendKeys(APART);
            locality().sendKeys(LOCALITY);
            administrativeArea().sendKeys("Moscow");
            route().sendKeys("Tverskaya Street");
            postalCode().sendKeys("125009");
            Thread.sleep(5000);
            nextButton().click();
            Thread.sleep(3000);
            nextButton().click();
            payment();
        }

        catch (Exception e) {
            System.out.println("BuyNowSTQ Error " + e.getMessage());
        }


        //Корзина.

//        driver.get("https://stage.stq.cloud/");
//        driver.get("https://stage.stq.cloud/store/339/products/988");
////        driver.findElement(By.xpath("//*[@data-test='productMaterialSelect']")).click();
////        driver.findElement(By.xpath("//*[@data-test='productMaterialSelect_item']")).click();
//        driver.findElement(By.xpath("//*[@data-test='product-addToCart']")).click();
//        driver.findElement(By.xpath("//*[@data-test='logoLink']")).click();
//        driver.findElement(By.xpath("//*[@data-test='header-cart-button']")).click();
//        driver.findElement(By.xpath("//*[@data-test='checkoutNext']")).click();
//
//        driver.findElement(By.xpath("//*[@data-test='newAddressCheckbox']")).click();
//        Thread.sleep(8000);
//        //driver.findElement(By.xpath("//div[normalize-space(text())='DELIVERY INFO']"));
////        driver.findElement(By.xpath("//*[@data-test='phone']")).click();
//        driver.findElement(By.xpath("//*[@data-test='receiverPhone']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='receiverPhone']")).sendKeys("812731298371289371");
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
//        Thread.sleep(5000);
////        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
////        driver.findElement(By.xpath("//*[@data-test='streetNumber'='1']"));
////        driver.findElement(By.xpath("//*[@data-test='locality'='Moscow']"));
////        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2'='Moscow']"));
////        driver.findElement(By.xpath("//*[@data-test='route'='Tverskaya Street']"));
////        driver.findElement(By.xpath("//*[@data-test='postalCode'='125009']"));
//        driver.findElement(By.xpath("//*[@data-test='streetNumber']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='streetNumber']")).sendKeys("1");
//        driver.findElement(By.xpath("//*[@data-test='route']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='route']")).sendKeys("Tverskaya Street");
//        driver.findElement(By.xpath("//*[@data-test='locality']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='locality']")).sendKeys("Moskva");
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']")).sendKeys("Moskva");
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel1']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel1']")).sendKeys("Alll");
//        driver.findElement(By.xpath("//*[@data-test='postalCode']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='postalCode']")).sendKeys("125009");
//        Thread.sleep(5000);
////        driver.findElement(By.xpath("//*[@data-test='phone']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='checkoutNext']")).click();
//        Thread.sleep(5000);
//        driver.findElement(By.xpath("//*[@data-test='checkoutNext']")).click();
//        Thread.sleep(5000);
//        driver.findElement(By.xpath("//div[normalize-space(text())='Payment']"));
    }

}
