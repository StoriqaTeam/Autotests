package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import Helper.Wait;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.LOGIN_SHOP;
import static Helper.Constants.PASS;


@RunWith(SeleniumRunner.class)

public class Loginadmin {

    WebDriver driver;
    Screenshot screenshot;

    public Loginadmin(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement email() {
        return driver.findElement(By.xpath("//*[@data-test='email']"));
    }
    public WebElement password() {
        return driver.findElement(By.xpath("//*[@data-test='password']"));
    }
    public WebElement loginButton() {
        return driver.findElement(By.xpath("//*[@data-test='loginButton']"));
    }

//    @Before
//    public void setUp(){
//        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
//        driver = new ChromeDriver();
//        System.setProperty("webdriver.opera.driver", "/Users/user/operadriver");
//        driver = new OperaDriver();
//        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
//        driver = new FirefoxDriver();
//    }
    //WebDriver driver = new ChromeDriver();

    public void login_admin() throws InterruptedException {

//        driver.manage().window().maximize();
//        driver.get("https://stqAdmin:yDYErXs4J4MuWvSC@stage.stq.cloud/storiqatools/auth");
//        Thread.sleep(3000);
//        driver.findElement(By.xpath("//*[@data-test='email']")).sendKeys("22684@crapmail.tld");
//        driver.findElement(By.xpath("//*[@data-test='password']")).sendKeys("M4n3b2v1");
//        driver.findElement(By.xpath("//*[@data-test='loginButton']")).click();

        try {
            driver.manage().window().maximize();
            Wait.sleep(driver, 10);
            driver.get("https://stqAdmin:yDYErXs4J4MuWvSC@stage.stq.cloud/storiqatools/auth");
            Thread.sleep(3000);
            email().sendKeys(LOGIN_SHOP);
            password().sendKeys(PASS);
            loginButton().click();
            Thread.sleep(3000);
            driver.findElement(By.xpath("//div[normalize-space(text())='Storiqa admin panel']"));
        }

        catch (Exception e) {
            System.out.println("TEST_LOG AdminAutorization Error" + e.getMessage());
            screenshot.takesScreenshot("AdminAutorization" + Getdate.Date());
        }
    }
}