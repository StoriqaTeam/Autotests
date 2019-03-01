package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class BuyNowEURVisa {

    WebDriver driver;
    Screenshot screenshot;

    public BuyNowEURVisa(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement countryDellivery() {
        return driver.findElement(By.xpath("//*[@data-test='productDeliveryCountrySelectInput']"));
    }
    public WebElement userDropdown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement orders() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileOrdersLink']"));
    }
    public WebElement orders_1() {
        return driver.findElement(By.xpath("//*[@class='TableRow__container___1FadI']"));
    }
    public WebElement paymentInfo() {
        return driver.findElement(By.xpath("//*[@class='Button__container___1-f_l Button__big___1fBlG Button__fullWidth___1cZbQ']"));
    }
    public WebElement buyNow() {
        return driver.findElement(By.xpath("//*[@data-test='product-BuyNow']"));
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
    public WebElement choose_company() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Choose company']"));
    }
    public WebElement cardNumber() {
        return driver.findElement(By.xpath("//*[@class='InputElement is-empty']"));
    }
    public WebElement iFrame() {
        return driver.findElement(By.xpath("//*[@name='__privateStripeFrame4']"));
    }
    public WebElement success() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Success']"));
    }
    public WebElement exp_date() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='MM / YY']"));
    }
    public WebElement cvc() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='CVC']"));
    }
    public WebElement payButton() {
        return driver.findElement(By.xpath("//*[@data-test='payButton']"));
    }
    public WebElement postal() {
        return driver.findElement(By.xpath("//*[@name='postal']"));
    }


    public void buy_nowEUR_Visa() throws InterruptedException {
        try {
            driver.get("https://stage.stq.cloud/store/917/products/2441");
            countryDellivery().click();
            countryDellivery().sendKeys(DELL);
            countryDellivery().sendKeys(RUSS);
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
            Thread.sleep(3000);
            iFrame().click();
            iFrame().sendKeys("4242");
            iFrame().sendKeys("4242");
            iFrame().sendKeys("4242");
            iFrame().sendKeys("4242");
            iFrame().sendKeys("1234");
            iFrame().sendKeys("424");
            iFrame().sendKeys("42421");
            payButton().click();
            success();
        }

        catch (Exception e) {
            System.out.println("BuyNowVisa Error" + e.getMessage());
            screenshot.takesScreenshot("BuyNowVisa" + Getdate.Date());
        }
    }
}
