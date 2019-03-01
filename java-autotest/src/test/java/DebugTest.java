import Steps.*;
import Helper.SeleniumRunner;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.firefox.FirefoxDriver;


@RunWith(SeleniumRunner.class)

public class DebugTest {
    private static final int PRODUCTS_AMOUNT = 10;

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
    public void Debug_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

//        Createshop shop = new Createshop(driver);
//        shop.create_shop(2);
//
//        BaseProductSTQ productSTQ = new BaseProductSTQ(driver);
//        productSTQ.base_product(1, 2);
//
//        AddProductETH productETH = new AddProductETH(driver);
//        productETH.product_eth(1, 2);
//
//        AddProductBTC productBTC = new AddProductBTC(driver);
//        productBTC.product_btc(1, 2);
//
//        AddProductEUR productEUR = new AddProductEUR(driver);
//        productEUR.product_eur(1, 2);
//
//        AddItem item = new AddItem(driver);
//        for (int i = 0; i < PRODUCTS_AMOUNT; i++) {
//            item.additem_product(i);
//        }
//
//        EditShop editshop = new EditShop(driver);
//        editshop.edit_shop();

//        SendModerationShop moderationShop = new SendModerationShop(driver);
//        moderationShop.sendmoderation_shop();
//
//        EditBaseProduct editbase = new EditBaseProduct(driver);
//        editbase.edit_base_product();
//
//        EditVariants editvariants = new EditVariants(driver);
//        editvariants.edit_variants();
//
//        AddContacts contacts = new AddContacts(driver);
//        contacts.add_contacts();
//
//        EditStorage storage = new EditStorage(driver);
//        storage.edit_storage();
//
//        AddDelivery delivery = new AddDelivery(driver);
//        delivery.add_delivery();

//        DeletVariant deletevariant = new DeletVariant(driver);
//        deletevariant.delete_variant();

//        BuyNowSTQ buynowSTQ = new BuyNowSTQ(driver);
//        buynowSTQ.buy_now();

//        BuyNowEURVisa buyNowVisa = new BuyNowEURVisa(driver);
//        buyNowVisa.buy_nowEUR_Visa();

//        BuyNowEURmc buyNowMC = new BuyNowEURmc(driver);
//        buyNowMC.buy_nowEUR_MC();

//        BuyNowAmerExpr buyNowAmerExpr = new BuyNowAmerExpr(driver);
//        buyNowAmerExpr.buy_now_AmerExpr();

//        BuyNowDiscover buyNowDiscover = new BuyNowDiscover(driver);
//        buyNowDiscover.buy_now_Discover();

//        BuyNowUnionPay buyNowUnionPay = new BuyNowUnionPay(driver);
//        buyNowUnionPay.buy_now_UnionPay();

//        AddToCart addcart = new AddToCart(driver);
//        addcart.add_cart();
//
//        Loginadmin loginadminka = new Loginadmin(driver);
//        loginadminka.login_admin();

//        ShangeStatusShop statusshop = new ShangeStatusShop(driver);
//        statusshop.status_shop();

        SearchProducts searchProducts = new SearchProducts(driver);
        searchProducts.search_products();

//        driver.close();
    }
}
