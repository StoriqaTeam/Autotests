package Shop;

import Steps.Autorization;
import Steps.Createshop;
import Helper.SeleniumRunner;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class StorestatusTest {

    WebDriver driver = new  ChromeDriver();

    @Test
    public void Storestatus_Test() throws Exception {

        Autorization user = new Autorization (driver);
        user.autorization_login();

        Createshop first = new Createshop (driver);
        first.create_shop(2);

//        BaseProductSTQ add = new BaseProductSTQ(driver);
//        add.add_product();


    }
}
