package Steps;

import Helper.BaseURL;
import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Generator.*;

@RunWith(SeleniumRunner.class)

public class AddProductEUR {

    WebDriver driver;
    Screenshot screenshot;

    public AddProductEUR(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement storiqaIcon() {
        return driver.findElement(By.xpath("//*[@data-test='storiqaIcon']"));
    }
    public WebElement userDropdown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement menuMyShops() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']"));
    }
    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }
    public WebElement addProduct() {
        return driver.findElement(By.xpath("//*[@data-test='addProductButton']"));
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
        return driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']"));
    }
    public WebElement currencySelect() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect']"));
    }
    public WebElement eur() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect_item_Currency_EUR']"));
    }
    public WebElement createProduct() {
        return driver.findElement(By.xpath("//*[@data-test='saveProductButton']"));
    }


    public void product_eur(int number, int attempts) throws InterruptedException {

        try {
            storiqaIcon().click();
            driver.get(BaseURL.getBaseUrl());
            userDropdown().click();
            menuMyShops().click();
            goods().click();
            addProduct().click();
            produktName().sendKeys(generateName());
            shortDescription().sendKeys(generateShortDescription());
            vendorCode().sendKeys(generateVendor());
            produktCategory().click();
            officeSchool();
            categoryItem46().click();
            calendars();
            categoryItem47().click();
            postcards();
            categoryItem48().click();
            price().sendKeys(generatePrice());
            currencySelect().click();
            eur().click();
            createProduct().click();
            Thread.sleep(3000);
        }

        catch (Exception e) {
            System.out.println("AddProductETH Error" + e.getMessage());
            if (attempts > 1)product_eur(number, --attempts);
            screenshot.takesScreenshot("AddProductETH" + Getdate.Date());
        }

    }
}
