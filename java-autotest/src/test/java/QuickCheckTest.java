import Helper.SeleniumRunner;
import Steps.AllCurrency;
import Steps.*;
import Steps.BaseProductSTQ;
import Steps.Createshop;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

@RunWith(SeleniumRunner.class)

public class QuickCheckTest {

    WebDriver driver;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
//        System.setProperty("webdriver.opera.driver", "/Users/user/operadriver");
//        driver = new OperaDriver();
//        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
//        driver = new FirefoxDriver();
    }

    @Test
    public void QuickCheck_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

//        Createshop shop = new Createshop(driver);
//        shop.create_shop(2);
//
//        BaseProductSTQ productSTQ = new BaseProductSTQ(driver);
//        productSTQ.base_product(1, 2);
//
//        AllCurrency allCurrency = new AllCurrency(driver);
//        allCurrency.all_currency();


    }
}
