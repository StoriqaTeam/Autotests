package Shop;

import helper.Addproduct;
import helper.Autorization;
import helper.Createshop;
import helper.SeleniumRunner;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

@RunWith(SeleniumRunner.class)

public class StorestatusTest {

    WebDriver driver = new  ChromeDriver();

    @Test
    public void Storestatus_Test() throws InterruptedException {

        Autorization user = new Autorization (driver);
        user.autorization_login();

        Createshop first = new Createshop (driver);
        first.create_shop();

        Addproduct add = new Addproduct(driver);
        add.add_product();


    }
}
