package Shop;

import helper.Autorization;
import helper.Createshop;
import helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.testng.annotations.Test;

import java.util.ArrayList;

@RunWith(SeleniumRunner.class)

public class EditShopTest {

    WebDriver driver = new ChromeDriver();

    @Test
    public void EditShop_Test() throws InterruptedException {

//        Autorization user = new Autorization(driver);
//        user.autorization_login();
//
//        Createshop first =  new Createshop(driver);
//        first.create_shop();

        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
        ArrayList<String> tabs = new ArrayList<String> (driver.getWindowHandles());
        driver.switchTo().window(tabs.get(1));

    }
}
