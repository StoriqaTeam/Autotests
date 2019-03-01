package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class EditBaseProduct {

    WebDriver driver;
    Screenshot screenshot;

    public EditBaseProduct(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }
    public WebElement produktName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement shortDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement price() {
        return driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']"));
    }
    public WebElement cashback() {
        return driver.findElement(By.xpath("//*[@data-test='variantCashbackInput']"));
    }
    public WebElement saveProductButton() {
        return driver.findElement(By.xpath("//*[@data-test='saveProductButton']"));
    }
    public WebElement product1() {
        return driver.findElement(By.xpath("//*[@data-test='productRow_1']"));
    }
    public WebElement seoTitle() {
        return driver.findElement(By.xpath("//*[@data-test='seoTitle']"));
    }
    public WebElement seoDescription() {
        return driver.findElement(By.xpath("//*[@data-test='seoDescription']"));
    }
    public WebElement variantDiscount() {
        return driver.findElement(By.xpath("//*[@data-test='variantDiscountInput']"));
    }

    public void edit_base_product() throws InterruptedException {

        try {
            Thread.sleep(3000);
            goods().click();
            product1().click();
            produktName().sendKeys(DELL);
            produktName().sendKeys(PRODUKT + Getdate.Date());
            shortDescription().sendKeys(DELL);
            shortDescription().sendKeys(SHORT);
            vendorCode().sendKeys(DELL);
            vendorCode().sendKeys(VENDOR);
            seoTitle().sendKeys(TEXT);
            seoDescription().sendKeys(TEXT);
            price().sendKeys(DELL);
            price().sendKeys("99");
            cashback().sendKeys(DELL);
            cashback().sendKeys("10");
            variantDiscount().sendKeys(DELL);
            variantDiscount().sendKeys("10");
            saveProductButton().click();
            Thread.sleep(3000);
            goods().click();

        } catch (Exception e) {
            System.out.print("TEST_LOG EditBaseProduct Error" + e.getMessage());
            screenshot.takesScreenshot("EditBaseProduct" + Getdate.Date());
        }
    }
}
