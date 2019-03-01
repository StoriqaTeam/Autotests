package AdminPanel;

import Steps.Loginadmin;
import Helper.SeleniumRunner;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

@RunWith(SeleniumRunner.class)

public class LogoutadminTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
//        System.setProperty("webdriver.opera.driver", "/Users/user/operadriver");
//        driver = new OperaDriver();
//        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
//        driver = new FirefoxDriver();
    }
    //WebDriver driver = new ChromeDriver();

    @Test
    public void Logoutadmin_Test() throws InterruptedException {

        Loginadmin admin = new Loginadmin(driver);
        admin.login_admin();

    }
}
