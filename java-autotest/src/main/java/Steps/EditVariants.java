package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;


import static Helper.Constants.DELL;

@RunWith(SeleniumRunner.class)

public class EditVariants {

    WebDriver driver;
    Screenshot screenshot;

    public EditVariants (WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement selectVariant() {
        return driver.findElement(By.xpath("//*[@data-test='" + SelectionVariant.getRandonVariant() + "']"));
    }
    public WebElement productName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement shotrDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement seoTitle() {
        return driver.findElement(By.xpath("//*[@data-test='seoTitle']"));
    }
    public WebElement seoDescription() {
        return driver.findElement(By.xpath("//*[@data-test='seoDescription']"));
    }
    public WebElement price() {
        return driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']"));
    }
    public WebElement cashback() {
        return driver.findElement(By.xpath("//*[@data-test='variantCashbackInput']"));
    }
    public WebElement size() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Size']"));
    }
    public WebElement material() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Material']"));
    }
    public WebElement colour() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Colour']"));
    }
    public WebElement randomSize() {
        return driver.findElement(By.xpath("//*[@data-test='" + Variants.getRandomSize() + "']"));
    }
    public WebElement randomMaterial() {
        return driver.findElement(By.xpath("//*[@data-test='" + Variants.getRandomMaterial() + "']"));
    }
    public WebElement randomColour() {
        return driver.findElement(By.xpath("//*[@data-test='" + Variants.getRandomColour() + "']"));
    }
    public WebElement saveProductButton() {
        return driver.findElement(By.xpath("//*[@data-test='saveProductButton']"));
    }

    public void edit_variants() throws InterruptedException {

        try {
            Thread.sleep(3000);
            selectVariant().click();
            Thread.sleep(2000);
            productName().sendKeys(DELL);
            productName().sendKeys("product" + Getdate.Date());
            shotrDescription().sendKeys(DELL);
            shotrDescription().sendKeys(Generator.generateShortDescription());
            vendorCode().sendKeys(DELL);
            vendorCode().sendKeys(Generator.generateVendor());
            seoTitle().sendKeys(Generator.generateSeoTitle());
            seoDescription().sendKeys(Generator.generateText());
            price().sendKeys(DELL);
            price().sendKeys("90");
            cashback().sendKeys(DELL);
            cashback().sendKeys("10");
            size().click();
            randomSize().click();
            material().click();
            randomMaterial().click();
            colour().click();
            randomColour().click();
            saveProductButton().click();
            Thread.sleep(3000);
        }

        catch (Exception e) {
            System.out.print("TEST_LOG EditVariants Error" + e.getMessage());
            screenshot.takesScreenshot("EditVariants" + Getdate.Date());
        }
    }
}
