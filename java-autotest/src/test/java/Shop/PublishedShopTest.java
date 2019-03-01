package Shop;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import Steps.*;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;

@RunWith(SeleniumRunner.class)

public class PublishedShopTest {

    WebDriver driver;
    Screenshot screenshot;

    public WebElement storiqaIcon() {
        return driver.findElement(By.xpath("//*[@data-test='storiqaIcon']"));
    }

    @Before

    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void PublishedShop_Test() throws Exception {

        try {
            Autorization user = new Autorization(driver);
            user.autorization_login();

            Createshop shop = new Createshop(driver);
            shop.create_shop(2);

            BaseProductSTQ productSTQ = new BaseProductSTQ(driver);
            productSTQ.base_product(1, 2);

            AllCurrency allCurrency = new AllCurrency(driver);
            allCurrency.all_currency();

            storiqaIcon().sendKeys(Keys.CONTROL + "t");

            Loginadmin loginadminka = new Loginadmin(driver);
            loginadminka.login_admin();


        }

        catch (Exception e) {
            System.out.println("PublishedShopTest Error" + e.getMessage());
            screenshot.takesScreenshot("PublishedShopTest" + Getdate.Date());
        }
    }
}
