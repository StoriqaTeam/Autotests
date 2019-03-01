package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class AddToCart {

    WebDriver driver;
    Screenshot screenshot;

    public AddToCart(WebDriver driver) {
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

    public void add_cart() throws InterruptedException {
        try {
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
            System.out.println("AddToCart Error" + e.getMessage());
            screenshot.takesScreenshot("AddToCart" + Getdate.Date());
        }
    }


}
