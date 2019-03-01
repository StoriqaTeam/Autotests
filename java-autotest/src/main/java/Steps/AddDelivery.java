package Steps;

import Helper.BaseURL;
import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class AddDelivery {

    WebDriver driver;
    Screenshot screenshot;

    public AddDelivery(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }
    public WebElement userDropdown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement menuMyShops() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']"));
    }
    public WebElement product_1() {
        return driver.findElement(By.xpath("//*[@data-test='productRow_1']"));
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


    public void add_delivery() throws InterruptedException {

        try {
            driver.get(BaseURL.getBaseUrl());
            userDropdown().click();
            menuMyShops().click();
            goods().click();
            product_1().click();
            deliveryTab().click();
            localShipping().click();
//            Thread.sleep(3000);
            shippingAddLocal().click();
            interShipping().click();
//            Thread.sleep(3000);
            selectAll().click();
//            Thread.sleep(3000);
            shippingAddInt().click();
//            Thread.sleep(4000);
            saveShipping().click();
        }

        catch (Exception e) {
            System.out.println("Add Delivery ERROR" + e.getMessage());
            screenshot.takesScreenshot("Add Delivery" + Getdate.Date());
        }
    }
}
