package Page.shop;

public enum Shop {
    SELLONSTORIQA("//a[contains(text(),'Sell on Storiqa')]"),
    STARTSELLING("//*[@data-test='startSelling']"),
    STORENAME("//*[@data-test='name']"),
    WEBADDRESS("//*[@data-test='slug']"),
    SHORTDESCRIPTION("//*[@data-test='shortDescription']"),
    LONGDESCRIPTION("//*[@data-test='longDescription']"),
    NEXTSTEPBUTTON("//*[@data-test='wizardBackButton']"),
    MAINLANGUAGE("//*[@data-test='wizardLanguagesSelect']"),
    ENGLISH("//*[@data-test='wizardLanguagesSelect']//*[contains(text(),'English')]"),
    COUNTRY("//*[@data-test='AddressFormSelect']"),
    ALBANIA("//div[@id='ALB']"),
    RUSSIA("//div[@id='RUS']"),
    ADDRES("//*[@data-test='autocompleteAddress']"),
    LOCALITY("//*[@data-test='locality']"),
    STREET("//*[@data-test='streetNumber']"),
    STREETADDRESS("//*[@data-test='route']"),
    REGION("//*[@data-test='administrativeAreaLevel2']"),
    POSTALCODE("//*[@data-test='postalCode']"),
    ADDFIRSTPRODUCTFOTO("//*[@data-test='wizardUploaderProductFotoFirst']"),
    PRODUCTNAME("//*[@data-test='name']"),
    PRODUKTCATEGORY("//*[@data-test='categorySelector']"),
    PRICE("//*[@data-test='wizardProductPriceInputInput']"),
    VENDORCODE("//*[@data-test='vendorCode']"),
    CASHEBACK("//*[@data-test='cashback']"),
    QUANTITY("//*[@data-test='quantity']"),
    CURRENCY("//*[@data-test='step3Currency']"),
    STQ("step3Currency_item"),
    SAVEPRODUCTBUTTON("//*[@data-test='wizardSaveProductButton']"),
    SLOGAN("//*[@data-test='slogan']"),
    SAVE("//*[@data-test='saveButton']");

    private final String cl;

    Shop(String cl){
        this.cl = cl;
    }

    public String getCl(){
        return cl;
    }
}
