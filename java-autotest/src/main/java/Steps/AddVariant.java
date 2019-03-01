package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class AddVariant {
    WebDriver driver;
    Screenshot screenshot;

    public AddVariant(WebDriver driver){
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement addVariants() {
        return driver.findElement(By.xpath("//*[@data-test='addVariantButton']"));
    }
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement price() {
        return driver.findElement(By.xpath("//*[@data-test='priceInput']"));
    }
    public WebElement size52() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='52']"));
    }
    public WebElement materialGlass() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Glass']"));
    }
    public WebElement colourBrown() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Brown']"));
    }
    public WebElement saveVariant() {
        return driver.findElement(By.xpath("//*[@data-test='saveVariantButton']"));
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
    public WebElement size() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Size']"));
    }
    public WebElement material() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Material']"));
    }
    public WebElement colour() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Colour']"));
    }

    // Создание вариантов товара

    public void addvariant_variant(int number) throws InterruptedException {


        try {
            Thread.sleep(8000);
            addVariants().click();
            Thread.sleep(4000);
            vendorCode().sendKeys(Generator.generateVendor());
            price().sendKeys("100");
            returnRandom();
            Thread.sleep(5000);
        }
        catch (Exception e){
            System.out.println("AddVariantsError");
            addvariant_variant(number);
            screenshot.takesScreenshot("AddVariants" + Getdate.Date());
        }
    }
    private void returnRandom() {
        try {
            size().click();
            randomSize().click();
            material().click();
            randomMaterial().click();
            colour().click();
            randomColour().click();
            saveVariant().click();
        }
        catch (Exception e){
            System.out.println("AddVariantsError" + e.getMessage());
            returnRandom();
            screenshot.takesScreenshot("AddVariants" + Getdate.Date());
        }
    }
}
