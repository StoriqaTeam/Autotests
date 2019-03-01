package Steps;

import Helper.BaseURL;
import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Generator.generateShortDescription;
import static Helper.Generator.generateVendor;

@RunWith(SeleniumRunner.class)

public class AllCurrency {

    WebDriver driver;
    Screenshot screenshot;

    public AllCurrency(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

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
    public WebElement eth() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect_item_Currency_ETH']"));
    }
    public WebElement btc() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect_item_Currency_BTC']"));
    }
    public WebElement eur() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect_item_Currency_EUR']"));
    }
    public WebElement currencySelect() {
        return driver.findElement(By.xpath("//*[@data-test='productCurrencySelect']"));
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
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement deliveryTab() {
        return driver.findElement(By.xpath("//*[@data-test='deliveryTab']"));
    }
    public WebElement shippingAddLocal() {
        return driver.findElement(By.xpath("//*[@data-test='shippingAddCompanyButton_local']"));
    }
    public WebElement shippingAddInt() {
        return driver.findElement(By.xpath("//*[@data-test='shippingAddCompanyButton_inter']"));
    }
    public WebElement localShipping() {
        return driver.findElement(By.xpath("//*[@data-test='localShippingFixPrice']"));
    }
    public WebElement interShipping() {
        return driver.findElement(By.xpath("//*[@data-test='interShippingFixPrice']"));
    }
    public WebElement selectAll() {
        return driver.findElement(By.xpath("//*[@data-test='shipping-all-countries']"));
    }
    public WebElement saveShipping() {
        return driver.findElement(By.xpath("//*[@data-test='saveShippingButton']"));
    }

    public void all_currency() throws InterruptedException {

        try {
            storiqaIcon().click();
            driver.get(BaseURL.getBaseUrl());
            userDropdown().click();
            menuMyShops().click();
            goods().click();
            addProduct().click();
            produktName().sendKeys("ETH");
            shortDescription().sendKeys(generateShortDescription());
            currencySelect().click();
            eth().click();
            produktCategory().click();
            officeSchool();
            categoryItem46().click();
            calendars();
            categoryItem47().click();
            postcards();
            categoryItem48().click();
            price().sendKeys("0,00000001");
            vendorCode().sendKeys(generateVendor());
            deliveryTab().click();
            localShipping().click();
            shippingAddLocal().click();
            interShipping().click();
            selectAll().click();
            shippingAddInt().click();
            saveShipping().click();
            storiqaIcon().click();
            driver.get(BaseURL.getBaseUrl());
            userDropdown().click();
            menuMyShops().click();
            goods().click();
            addProduct().click();
            produktName().sendKeys("BTC");
            shortDescription().sendKeys(generateShortDescription());
            currencySelect().click();
            btc().click();
            produktCategory().click();
            officeSchool();
            categoryItem46().click();
            calendars();
            categoryItem47().click();
            postcards();
            categoryItem48().click();
            price().sendKeys("0,00000001");
            vendorCode().sendKeys(generateVendor());
            deliveryTab().click();
            localShipping().click();
            shippingAddLocal().click();
            interShipping().click();
            selectAll().click();
            shippingAddInt().click();
            saveShipping().click();
            storiqaIcon().click();
            driver.get(BaseURL.getBaseUrl());
            userDropdown().click();
            menuMyShops().click();
            goods().click();
            addProduct().click();
            produktName().sendKeys("EUR");
            shortDescription().sendKeys(generateShortDescription());
            currencySelect().click();
            eur().click();
            produktCategory().click();
            officeSchool();
            categoryItem46().click();
            calendars();
            categoryItem47().click();
            postcards();
            categoryItem48().click();
            price().sendKeys("100");
            vendorCode().sendKeys(generateVendor());
            deliveryTab().click();
            localShipping().click();
            shippingAddLocal().click();
            interShipping().click();
            selectAll().click();
            shippingAddInt().click();
            saveShipping().click();
            storiqaIcon().click();
        }

        catch (Exception e) {
            System.out.println("AllCurrency Error" + e.getMessage());
            screenshot.takesScreenshot("AllCurrency" + Getdate.Date());
        }
    }
}
