package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class Createshop {

    WebDriver driver;
    Screenshot screenshot;

    private int attempts;

    public Createshop(WebDriver driver){
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement storeName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement webAddress() {
        return driver.findElement(By.xpath("//*[@data-test='slug']"));
    }
    public WebElement shortDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement nextStep() {
        return driver.findElement(By.xpath("//*[@data-test='wizardBackButton']"));
    }
    public WebElement mainLanguages() {
        return driver.findElement(By.xpath("//*[@data-test='wizardLanguagesSelect']"));
    }
    public WebElement address() {
        return driver.findElement(By.xpath("//*[@data-test='autocompleteAddress']"));
    }
    public WebElement english() {
        return driver.findElement(By.xpath("//*[@data-test='wizardBackButton']"));
    }
    public WebElement country() {
        return driver.findElement(By.xpath("//*[@data-test='AddressFormSelectInput']"));
    }
    public WebElement russia() {
        return driver.findElement(By.xpath("//div[@id='RUS']"));
    }
    public WebElement selectAddress() {
        return driver.findElement(By.xpath("//*[@data-test='AddressFormSelect']"));
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
    public WebElement addProdukt() {
        return driver.findElement(By.xpath("//*[@data-test='wizardUploaderProductFotoFirst']"));
    }

    public void create_shop(int attempts) throws InterruptedException {

        step1(2);
        step2(2);

    }

    private void step1(int attempts) {
        try {
            driver.get("https://stage.stq.cloud/manage/wizard?");
            storeName().sendKeys(DELL);
            Wait.sleep(driver, 15);
            webAddress().sendKeys(DELL);
            Wait.sleep(driver, 15);
            shortDescription().sendKeys(DELL);

            storeName().sendKeys(Generator.generateShopName());
            Wait.sleep(driver, 15);
            webAddress().sendKeys(Generator.generateSite());
            Wait.sleep(driver, 15);
            shortDescription().sendKeys(Generator.generateShortDescription());
            Thread.sleep(3000);
            nextStep().click();
        }
        catch (Exception e) {
            e.printStackTrace();
            System.out.println("CreateShop error" + e.getMessage());
            if (attempts > 1)
                step1(--attempts);
            screenshot.takesScreenshot("CreateShop" + Getdate.Date());
        }
    }
    private void step2(int attemts) {
        try {
//            driver.findElement(By.xpath("//div[normalize-space(text())='Set up store']"));
            mainLanguages().click();
            english().click();
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
            administrativeArea().sendKeys(LOCALITY);
            route().sendKeys(STREET1);
            postalCode().sendKeys(INDEX);
            Thread.sleep(5000);
            nextStep().click();
            Thread.sleep(5000);
            addProdukt().click();
        }
        catch (Exception e) {
            e.printStackTrace();
            System.out.println("CreateShop error" + e.getMessage());
            if (attempts > 1)
                step2(--attempts);
            screenshot.takesScreenshot("CreateShop" + Getdate.Date());
        }
    }
}
