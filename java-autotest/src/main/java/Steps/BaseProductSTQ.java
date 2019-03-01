package Steps;

import Helper.Generator;
import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class BaseProductSTQ {

    WebDriver driver;
    Screenshot screenshot;
    private int attempts;

    public BaseProductSTQ(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement produktName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement shortDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement produktCategory() {
        return driver.findElement(By.xpath("//*[@data-test='categorySelector']"));
    }
    public WebElement officeSchool() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Office & School Supplies']"));
    }
    public WebElement categoryItem46() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_46']"));
    }
    public WebElement calendars() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Calendars, Planners & Cards']"));
    }
    public WebElement categoryItem47() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_47']"));
    }
    public WebElement postcards() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Postcards']"));
    }
    public WebElement categoryItem48() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_48']"));
    }
    public WebElement price() {
        return driver.findElement(By.xpath("//*[@data-test='wizardProductPriceInputInput']"));
    }
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement cashback() {
        return driver.findElement(By.xpath("//*[@data-test='cashback']"));
    }
    public WebElement quantity() {
        return driver.findElement(By.xpath("//*[@data-test='quantity']"));
    }
    public WebElement saveButton() {
        return driver.findElement(By.xpath("//*[@data-test='wizardSaveProductButton']"));
    }
    public WebElement publishStore() {
        return driver.findElement(By.xpath("//*[@data-test='wizardBackButton']"));
    }
    public WebElement publishPopup() {
        return driver.findElement(By.xpath("//*[@data-test='publishPopupWrapper']"));
    }
//    public WebElement quantity() {
//        return driver.findElement(By.xpath("//*[@data-test='quantity']"));
//    }
    public WebElement cancelPublish() {
        return driver.findElement(By.xpath("//*[@data-test='cancelPublishWizardButton']"));
    }

    public void base_product(int number, int attempts) throws InterruptedException {

        try {
            produktName().sendKeys("variant" + number);
            shortDescription().sendKeys("test produkt" + number);
            produktCategory().click();
            officeSchool();
            categoryItem46().click();
            calendars();
            categoryItem47().click();
            postcards();
            categoryItem48().click();
            price().sendKeys("100");
            vendorCode().sendKeys(Generator.generateVendor());
            cashback().sendKeys("10");
            quantity().sendKeys("1000");
            saveButton().click();
            publishStore().click();
            publishPopup().click();
            cancelPublish().click();
            Thread.sleep(5000);
        }

        catch (Exception e){
            System.out.println("BaseProduktSTQError" + e.getMessage());
            if (attempts > 1)base_product(number, --attempts);
            screenshot.takesScreenshot("BaseProduktSTQ" + Getdate.Date());
        }
    }
}
