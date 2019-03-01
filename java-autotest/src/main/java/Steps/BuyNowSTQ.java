package Steps;


import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class BuyNowSTQ {

    WebDriver driver;
    Screenshot screenshot;

    public BuyNowSTQ(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

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
        return driver.findElement(By.xpath("//*[@data-test='phone']"));
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
        return driver.findElement(By.xpath("//div[normalize-space(text())='product-addToCart']"));
    }
    public WebElement myCart() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='My Cart']"));
    }
    public WebElement comment() {
        return driver.findElement(By.xpath("//*[@data-test='customerComment']"));
    }


    public void buy_now() throws InterruptedException {
        try {
            driver.get("https://stage.stq.cloud/store/339/products/988");
            countryDellivery().click();
//            countryDellivery().sendKeys(RUSS);
            countryDellivery().sendKeys(Keys.ENTER);
            buyNow().click();
            receiverName().sendKeys(DELL);
            receiverName().sendKeys(Generator.generateName() + " " + Generator.generateName());
            phone().sendKeys(DELL);
            phone().sendKeys(Generator.generatePhone());
            newAddress().click();
            country().click();
            Thread.sleep(3000);
            country().sendKeys(RUSS);
            Thread.sleep(3000);
            country().sendKeys(Keys.ENTER);
            address().click();
            address().sendKeys(STREET2);
            address().sendKeys(Keys.ENTER);
            Wait.sleep(driver, 10);
            street().click();
            street().sendKeys(APART);
            locality().sendKeys(LOCALITY);
            administrativeArea().sendKeys("Moscow");
            route().sendKeys("Semenovskaya Ploshchad' 1");
            postalCode().sendKeys("125009");
            Thread.sleep(4000);
            nextButton().click();
            Thread.sleep(3000);
            nextButton().click();
            payment();
        }

        catch (Exception e) {
            System.out.println("BuyNowSTQ Error" + e.getMessage());
            screenshot.takesScreenshot("BuyNowSTQ" + Getdate.Date());
        }
    }
}
